#![allow(non_snake_case)]

use std::iter::Copied;
use std::collections::{hash_set, HashSet, HashMap};
use super::{LR0A, lr0a::State, LR0Item, LRAutomaton};
use crate::grammar::{Grammar, Symbol};

pub struct LALR1A<'a> {
    lr0a: LR0A<'a>,
    lookahead: Vec<HashMap<usize, HashSet<Option<usize>>>>,
}

impl<'a> LALR1A<'a> {
    #[must_use]
    pub fn new(grammar: &'a Grammar) -> Self {
        LALR1ABuilder::new(grammar).build()
    }

    #[must_use]
    pub fn grammar(&self) -> &'a Grammar {
        self.lr0a.grammar()
    }

    #[must_use]
    pub fn states(&self) -> &[State] {
        self.lr0a.states()
    }

    #[must_use]
    pub fn lookaheads(&self, state: usize, production: usize) -> &HashSet<Option<usize>> {
        &self.lookahead[state][&production]
    }
}

impl<'a> LRAutomaton<'a> for LALR1A<'_> {
    type Item = LR0Item;
    type Lookaheads = Copied<hash_set::Iter<'a, Option<usize>>>;

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

    fn lookaheads(&'a self, state: usize, item: &Self::Item) -> Self::Lookaheads {
        self.lookaheads(state, item.production).iter().copied()
    }
}

// =================
// === INTERNALS ===
// =================

mod builder;
use self::builder::LALR1ABuilder;