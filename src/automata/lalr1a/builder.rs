#![allow(non_snake_case)]

use std::collections::{HashSet, HashMap};
use super::LR0A;
use super::{LALR1A, StateReductionPair};
use crate::grammar::{Grammar, Symbol, Nullable};
use crate::transitive_closure;

pub struct LALR1ABuilder<'a> {
    grammar: &'a Grammar,
    lr0a: LR0A<'a>,
    nullable: Nullable,
    nonterminal_transitions: Vec<NonterminalTransition>,
    nonterminal_transition_map: HashMap<NonterminalTransition, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NonterminalTransition {
    pub state: usize,
    pub var: usize,
}

impl<'a> LALR1ABuilder<'a> {
    #[must_use]
    pub fn new(grammar: &'a Grammar) -> Self {
        let lr0a = LR0A::new(grammar);

        let nonterminal_transitions: Vec<_> = lr0a.states().iter().enumerate().flat_map(|(p, state)| {
            state.next.keys().filter_map(move |&symbol| {
                if let Symbol::Variable(A) = symbol {
                    Some(NonterminalTransition { state: p, var: A })
                } else {
                    None
                }
            })
        }).collect();

        let nonterminal_transition_map = nonterminal_transitions.iter().enumerate()
            .map(|(i, &transition)| (transition, i)).collect();

        Self {
            grammar,
            lr0a,
            nullable: grammar.nullability(),
            nonterminal_transitions,
            nonterminal_transition_map,
        }
    }

    #[must_use]
    pub fn build(self) -> LALR1A<'a> {
        let lookahead = self.lookahead();
        LALR1A {
            lr0a: self.lr0a,
            lookahead,
        }
    }
}

impl LALR1ABuilder<'_> {
    #[must_use]
    pub fn nonterminal_transitions(&self) -> &[NonterminalTransition] {
        &self.nonterminal_transitions
    }

    #[must_use]
    pub fn direct_read(&self) -> Vec<HashSet<Option<usize>>> {
        let states = self.lr0a.states();
        let mut direct_read: Vec<HashSet<Option<usize>>> = 
            vec![HashSet::new(); self.nonterminal_transitions.len()];        
        for (i, &transition) in self.nonterminal_transitions.iter().enumerate() {
            let NonterminalTransition { state: p, var: A } = transition;
            // The only "transition" to the "accept state" is from the state
            // reached by shifting the start variable from the start state.
            // TODO: lift outside the loop
            if (p, A) == (0, 0) {
                direct_read[i].insert(None);
            }
            let q = states[p].next[&Symbol::Variable(A)];
            for &symbol in states[q].next.keys() {
                if let Symbol::Terminal(t) = symbol {
                    // (p, A) directly-reads t
                    direct_read[i].insert(Some(t));
                }
            }
        }
        direct_read
    }

    #[must_use]
    pub fn read(&self) -> Vec<HashSet<Option<usize>>> {
        let mut read = self.direct_read();
        let reads = self.reads();
        if transitive_closure(&mut read, |i| reads[i].iter().copied(), extend) {
            // cycle detected, TODO: handle errors
        }
        read
    }

    #[must_use]
    pub fn follow(&self) -> Vec<HashSet<Option<usize>>> {
        let mut follow = self.read();
        let includes = self.includes();
        if transitive_closure(&mut follow, |i| includes[i].iter().copied(), extend) {
            // cycle detected, TODO: handle errors
        }
        follow
    }

    #[must_use]
    pub fn lookahead(&self) -> HashMap<StateReductionPair, HashSet<Option<usize>>> {
        let follow = self.follow();
        self.lookback().into_iter().map(|(key, value)| {
            (key, value.into_iter().fold(HashSet::new(), |mut acc, x| {
                acc.extend(&follow[x]);
                acc
            }))
        }).collect()
    }

    #[must_use]
    pub fn reads(&self) -> Vec<HashSet<usize>> {
        // NOTE: this doesn't need to be stored: can be computed on the fly.
        let states = self.lr0a.states();
        self.nonterminal_transitions.iter().map(|&transition| {
            let NonterminalTransition { state: p, var: A } = transition;
            let q = states[p].next[&Symbol::Variable(A)];
            states[q].next.keys().filter_map(|&symbol| {
                if let Symbol::Variable(B) = symbol {
                    if self.nullable.get(B) {
                        // (p, A) reads (q, B)
                        let transition = NonterminalTransition { state: q, var: B };
                        let j = self.nonterminal_transition_map[&transition];
                        return Some(j);
                    }
                }
                None
            }).collect()
        }).collect()
    }

    #[must_use]
    pub fn includes(&self) -> Vec<HashSet<usize>> {
        let states = self.lr0a.states();
        let mut successors = vec![HashSet::new(); self.nonterminal_transitions.len()];
        for transition in self.nonterminal_transitions() {
            let NonterminalTransition { state: p, var: B } = *transition;
            for alt in self.grammar.rules().get(B).alts() {
                let mut q = p;
                for (i, &symbol) in alt.iter().enumerate() {
                    if let Symbol::Variable(A) = symbol {
                        let nullable_gamma = alt[i+1..].iter().all(|&symbol| match symbol {
                            Symbol::Terminal(_) => false,
                            Symbol::Variable(C) => self.nullable.get(C),
                        });
                        if nullable_gamma {
                            let i = self.nonterminal_transition_map[&NonterminalTransition { state: q, var: A }];
                            successors[i].insert(self.nonterminal_transition_map[transition]);
                        }
                    }
                    q = states[q].next[&symbol];
                }
            }
        }

        successors
    }

    #[must_use]
    pub fn lookback(&self) -> HashMap<StateReductionPair, HashSet<usize>> {
        // let inconsistent_state_reduction_pairs: Vec<(usize, usize)> = self.lr0a.states().iter()
        //     .enumerate()
        //     .filter_map(|(q, state)| {
        //         if state.items.len() > 1 {
        //             Some(state.items.iter().filter_map(move |item| {
        //                 if item.is_complete(self.grammar) {
        //                     Some((q, item.production))
        //                 } else {
        //                     None
        //                 }
        //             }))
        //         } else {
        //             None
        //         }
        //     }).flatten().collect();
        // println!("{:?}", inconsistent_state_reduction_pairs);
        let states = self.lr0a.states();
        let mut map: HashMap<StateReductionPair, HashSet<usize>> = HashMap::new();
        for (i, &transition) in self.nonterminal_transitions().iter().enumerate() {
            let NonterminalTransition { state: p, var: A } = transition;
            let rule = self.grammar.rules().get(A);
            for (alt_index, alt) in rule.production_ids().zip(rule.alts()) {
                let q = alt.iter().fold(p, |q, symbol| states[q].next[symbol]);
                map.entry(StateReductionPair { state: q, production: alt_index }).or_default().insert(i);
            }
        }
        map
    }
}

// =================
// === INTERNALS === 
// =================

fn extend(a: &mut HashSet<Option<usize>>, b: &HashSet<Option<usize>>) {
    a.extend(b);
}