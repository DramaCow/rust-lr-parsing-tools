#![allow(non_snake_case)]

use std::collections::{HashSet, HashMap};
use super::{LR0A, lr0a::State};
use crate::grammar::Grammar;

pub struct LALR1A<'a> {
    lr0a: LR0A<'a>,
    lookahead: HashMap<StateReductionPair, HashSet<Option<usize>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateReductionPair {
    pub state: usize,
    pub production: usize,
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
        let pair = StateReductionPair { state, production };
        &self.lookahead[&pair]
    }
}

// =================
// === INTERNALS ===
// =================

mod builder;
use self::builder::LALR1ABuilder;