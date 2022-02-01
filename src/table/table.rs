#![allow(non_snake_case)]

use super::{Conflict, ConstructionError};
use crate::grammar::Symbol;
use crate::automata::{LR0Item, LRAutomaton};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Invalid,
    Accept,
    /// shift to a *state*
    Shift(usize),
    /// reduce via a *production*
    Reduce(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct Reduction {
    pub var: usize,
    pub count: usize,
}

pub trait LR1Table {
    const START_STATE: usize = 0;
    fn action(&self, state: usize, word: Option<usize>) -> Action;
    fn goto(&self, state: usize, var: usize) -> Option<usize>;
    fn reduction(&self, production: usize) -> Reduction;
}

#[derive(Debug)]
pub struct NaiveLR1Table {
    actions:    Vec<Action>,        // lookup what action to perform given state and word
    gotos:      Vec<Option<usize>>, // lookup what state should be transitioned to after reduction
    reductions: Vec<Reduction>,     // production --> rule and number of symbols
    word_count: usize,
    var_count:  usize,
}

impl NaiveLR1Table {
    /// # Errors
    pub fn build<'a, T, F>(automaton: &'a T, mut conflict_resolution: F) -> Result<NaiveLR1Table, ConstructionError>
    where
        T: LRAutomaton<'a>,
        T::Item: AsRef<LR0Item>,
        T::Lookaheads: IntoIterator<Item = Option<usize>>,
        F: FnMut(Conflict) -> Result<Action, Conflict>,
    {
        let grammar = automaton.grammar();

        let word_count = grammar.word_count() + 1; // +1 for eof
        let var_count = grammar.rules().len() - 1; // implicit start variable not needed in goto table
        let num_states = automaton.state_count();

        let mut table = NaiveLR1Table {
            actions: vec![Action::Invalid; word_count * num_states],
            gotos: vec![None; var_count * num_states],
            reductions: //grammar.productions().into_iter().map(|(lhs, rhs)| Reduction { var: lhs, count: rhs.len() }).collect(),
                grammar.rules().into_iter().enumerate().flat_map(|(i, rule)| {
                    rule.alts().map(move |alt| Reduction { var: i, count: alt.len() })
                }).collect(),
            word_count,
            var_count,
        };

        for i in 0..num_states {
            for item in automaton.items(i) {
                if !item.as_ref().is_complete(&grammar) {
                    let symbol = item.as_ref().symbol_at_dot(&grammar).unwrap();
                    if let Symbol::Terminal(word) = symbol {
                        // CASE 1: item is incomplete and has a terminal symbol at dot.

                        let action = table.actions.get_mut(i * word_count + word + 1).unwrap();
                        let next_state = automaton.transition(i, symbol).unwrap();
    
                        // Note: shift-shift conflicts cannot occur
                        if let Action::Reduce(production) = *action {
                            *action = conflict_resolution(Conflict::ShiftReduce { word, next_state, production })
                                .map_err(|conflict| ConstructionError { state: i, conflict })?;
                        } else {
                            *action = Action::Shift(next_state);
                        }
                    }
                } else if table.reductions[item.as_ref().production].var < var_count {
                    // CASE 2: item is complete and does not have the start symbol on LHS.

                    for lookahead in automaton.lookaheads(i, item) {
                        let column = lookahead.map_or(0, |a| a + 1);
                        let action = table.actions.get_mut(i * word_count + column).unwrap();
                        
                        match *action {
                            Action::Shift(state) => {
                                *action = conflict_resolution(Conflict::ShiftReduce { word: column - 1, next_state: state, production: item.as_ref().production })
                                    .map_err(|conflict| ConstructionError { state: i, conflict })?;
                            }
                            Action::Reduce(production1) => {
                                *action = conflict_resolution(Conflict::ReduceReduce { production1, production2: item.as_ref().production })
                                    .map_err(|conflict| ConstructionError { state: i, conflict })?;
                            }
                            _ => {
                                *action = Action::Reduce(item.as_ref().production);
                            }
                        }
                    }
                } else {
                    // CASE 3: item is complete and has start symbol on LHS (lookahead will always be eof).
                    table.actions[i * word_count] = Action::Accept;
                }
            }

            for (var, A) in (0..var_count).map(|A| (Symbol::Variable(A), A)) {
                table.gotos[i * var_count + A] = automaton.transition(i, var);
            }
        }

        Ok(table)
    }
}

impl LR1Table for NaiveLR1Table {
    fn action(&self, state: usize, word: Option<usize>) -> Action {
        self.actions[state * self.word_count + word.map_or(0, |a| a + 1)]
    }

    fn goto(&self, state: usize, var: usize) -> Option<usize> {
        self.gotos[state * self.var_count + var]
    }

    fn reduction(&self, production: usize) -> Reduction {
        self.reductions[production]
    }
}