#![allow(non_snake_case)]

use std::collections::HashMap;
use super::{inner, LR0Item};
use crate::grammar::{Grammar, Symbol};

pub struct LR0A<'a> {
    grammar: &'a Grammar,
    states: Vec<State>,
}

pub struct State {
    pub next: HashMap<Symbol, usize>,
    pub items: Vec<LR0Item>,
}

impl<'a> LR0A<'a> {
    #[must_use]
    pub fn new(grammar: &'a Grammar) -> Self {
        LR0ABuilder::new(grammar).build()
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

// =================
// === INTERNALS ===
// =================

mod builder;
use self::builder::LR0ABuilder;