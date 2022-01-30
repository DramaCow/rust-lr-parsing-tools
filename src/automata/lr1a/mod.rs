#![allow(non_snake_case)]

use std::iter::{Once, once};
use std::collections::HashMap;
use super::{inner, LR1Item, LRAutomaton};
use crate::grammar::{Grammar, Symbol};

pub struct LR1A<'a> {
    grammar: &'a Grammar,
    states: Vec<State>,
}

pub struct State {
    pub next: HashMap<Symbol, usize>,
    pub items: Vec<LR1Item>,
}

impl<'a> LR1A<'a> {
    #[must_use]
    pub fn new(grammar: &'a Grammar) -> Self {
        LR1ABuilder::new(grammar).build()
    }

    #[must_use]
    pub fn grammar(&self) -> &'a Grammar {
        self.grammar
    }

    #[must_use]
    pub fn states(&self) -> &[State] {
        &self.states
    }
}

impl LRAutomaton<'_> for LR1A<'_> {
    type Item = LR1Item;
    type Lookaheads = Once<Option<usize>>;

    fn grammar(&self) -> &Grammar {
        self.grammar()
    }

    fn state_count(&self) -> usize {
        self.states().len()
    }

    fn items(&self, state: usize) -> &[Self::Item] {
        &self.states()[state].items
    }

    fn transition(&self, state: usize, symbol: Symbol) -> Option<usize> {
        self.states()[state].next.get(&symbol).copied()
    }

    fn production(&self, item: &Self::Item) -> usize {
        item.lr0_item.production
    }

    fn pos(&self, item: &Self::Item) -> usize {
        item.lr0_item.pos
    }

    fn is_complete(&self, item: &Self::Item) -> bool {
        item.lr0_item.is_complete(self.grammar())
    }

    fn symbol_at_dot(&self, item: &Self::Item) -> Option<Symbol> {
        item.lr0_item.symbol_at_dot(self.grammar())
    }
    
    fn lookaheads(&self, _: usize, item: &Self::Item) -> Self::Lookaheads {
        once(item.lookahead)
    }
}

// =================
// === INTERNALS ===
// =================

mod builder;
use self::builder::LR1ABuilder;