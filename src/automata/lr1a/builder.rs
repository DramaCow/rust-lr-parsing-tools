#![allow(non_snake_case)]

use std::collections::BTreeSet;
use std::iter::once;
use super::{inner, LR1Item, LR1A, State};
use crate::grammar::{Grammar, Symbol, Nullable, First};

pub struct LR1ABuilder<'a> {
    grammar: &'a Grammar,
    nullable: Nullable,
    first: First,
}

impl inner::BuildItemSets<LR1Item> for LR1ABuilder<'_> {
    fn start_item(&self) -> LR1Item {
        LR1Item::new(self.grammar.productions().len() - 1, 0, None)
    }

    fn advance(&self, item: &LR1Item) -> LR1Item {
        LR1Item::new(item.lr0_item.production, item.lr0_item.pos + 1, item.lookahead)
    }

    fn symbol_at_dot(&self, item: &LR1Item) -> Option<Symbol> {
        item.lr0_item.symbol_at_dot(self.grammar)
    }

    /// Performs the following:
    /// * for item `i` with variable `B` at dot:
    /// * * for production with `B` on lhs:
    /// * * * for symbol `b` in :
    /// * * * * add item that is the production with dot at start and lookahead `b`
    fn closure(&self, old_items: &BTreeSet<LR1Item>) -> BTreeSet<LR1Item> {
        let mut items     = old_items.clone();
        let mut new_items = old_items.clone();
        
        let mut done = false;
        
        while !done {
            done = true;

            for item in &items {
                if let Some(Symbol::Variable(var)) = item.lr0_item.symbol_at_dot(self.grammar) {
                    match item.lr0_item.symbol_after_dot(self.grammar) {
                        None => {
                            for alt in self.grammar.rules().get(var).production_ids() {
                                if new_items.insert(LR1Item::new(alt, 0, item.lookahead)) {
                                    done = false;
                                }
                            }
                        },
                        Some(Symbol::Terminal(a)) => {
                            for alt in self.grammar.rules().get(var).production_ids() {
                                if new_items.insert(LR1Item::new(alt, 0, Some(a))) {
                                    done = false;
                                }
                            }
                        },
                        Some(Symbol::Variable(A)) => {
                            let first_A = self.first.get(A);

                            if self.nullable.get(A) {
                                for lookahead in first_A.iter().copied().map(Some).chain(once(item.lookahead)) {
                                    for alt in self.grammar.rules().get(var).production_ids() {
                                        if new_items.insert(LR1Item::new(alt, 0, lookahead)) {
                                            done = false;
                                        }
                                    }
                                }
                            } else {
                                for &lookahead in first_A {
                                    for alt in self.grammar.rules().get(var).production_ids() {
                                        if new_items.insert(LR1Item::new(alt, 0, Some(lookahead))) {
                                            done = false;
                                        }
                                    }
                                }
                            } 
                        },
                    };
                }
            }

            items = new_items.clone();
        }
    
        items
    }
}

impl<'a> LR1ABuilder<'a> {
    #[must_use]
    pub fn new(grammar: &'a Grammar) -> Self {
        let (first, nullable) = grammar.first_set();
        LR1ABuilder { grammar, nullable, first }
    }

    #[must_use]
    pub fn build(self) -> LR1A<'a> {
        let (itemsets, gotos) = <Self as inner::BuildItemSets<LR1Item>>::build(&self);

        LR1A {
            grammar: self.grammar,
            states: itemsets.into_iter()
                .zip(gotos)
                .map(|(items, next)| State { items: items.into_iter().collect(), next })
                .collect()
        }
    }
}