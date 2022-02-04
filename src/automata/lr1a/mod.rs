#![allow(non_snake_case)]

use std::iter::{Once, once};
use std::collections::HashMap;
use super::{inner, LR1Item, LRAutomaton, DottedItem};
use crate::grammar::{Grammar, Symbol};

pub struct LR1A<'a> {
    grammar: &'a Grammar,
    states: Vec<State>,
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
}

impl<'a> LRAutomaton<'a> for LR1A<'a> {
    type ItemSet = LR1ItemSet<'a>;

    fn grammar(&self) -> &Grammar {
        self.grammar()
    }

    fn state_count(&self) -> usize {
        self.states.len()
    }

    fn items(&'a self, state: usize) -> Self::ItemSet {
        LR1ItemSet::new(self, state)
    }

    fn transition(&self, state: usize, symbol: Symbol) -> Option<usize> {
        self.states[state].next.get(&symbol).copied()
    }
}

pub struct LR1ItemSet<'a> {
    grammar: &'a Grammar,
    iter: std::slice::Iter<'a, LR1Item>,
}

impl<'a> Iterator for LR1ItemSet<'a> {
    type Item = LR1ItemProxy<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(LR1ItemProxy {
            grammar: self.grammar,
            item: *self.iter.next()?,
        })
    }
}

impl<'a> LR1ItemSet<'a> {
    pub fn new(lr1a: &'a LR1A, state: usize) -> Self {
        Self { grammar: lr1a.grammar, iter: lr1a.states[state].items.iter() }
    }
}

pub struct LR1ItemProxy<'a> {
    grammar: &'a Grammar,
    item: LR1Item,
}

impl DottedItem for LR1ItemProxy<'_> {
    type Lookaheads = Once<Option<usize>>;

    fn production(&self) -> usize {
        self.item.lr0_item.production
    }
    
    fn is_kernel_item(&self) -> bool {
        self.item.lr0_item.is_kernel_item(self.grammar)
    }
    
    fn is_complete(&self) -> bool {
        self.item.lr0_item.is_complete(self.grammar)
    }
    
    fn symbol_at_dot(&self) -> Option<Symbol> {
        self.item.lr0_item.symbol_at_dot(self.grammar)
    }

    fn lookaheads(&self) -> Self::Lookaheads {
        once(self.item.lookahead)
    }
}

// =================
// === INTERNALS ===
// =================

struct State {
    next: HashMap<Symbol, usize>,
    items: Vec<LR1Item>,
}

mod builder;
use self::builder::LR1ABuilder;