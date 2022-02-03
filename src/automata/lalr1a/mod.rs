#![allow(non_snake_case)]

use std::iter::Copied;
use std::collections::{hash_set, HashSet, HashMap};
use super::{LR0A, lr0a::State, LR0Item, LRAutomaton, DottedItem};
use crate::grammar::{Grammar, Symbol};

pub struct LALR1A<'a> {
    lr0a: LR0A<'a>,
    // pub lookahead: Vec<HashMap<usize, HashSet<Option<usize>>>>,
    pub lookahead: HashMap<StateReductionPair, HashSet<Option<usize>>>,
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
}

impl<'a> LRAutomaton<'a> for LALR1A<'_> {
    type ItemSet = LALR1ItemSet<'a>;

    fn grammar(&self) -> &Grammar {
        self.grammar()
    }

    fn state_count(&self) -> usize {
        self.lr0a.states().len()
    }

    fn items(&'a self, state: usize) -> Self::ItemSet {
        LALR1ItemSet::new(self, state)
    }

    fn transition(&self, state: usize, symbol: Symbol) -> Option<usize> {
        self.lr0a.states()[state].next.get(&symbol).copied()
    }
}

pub struct LALR1ItemSet<'a> {
    lalr1a: &'a LALR1A<'a>,
    state: usize,
    iter: std::slice::Iter<'a, LR0Item>,
}

impl<'a> Iterator for LALR1ItemSet<'a> {
    type Item = LALR1ItemProxy<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(LALR1ItemProxy {
            lalr1a: self.lalr1a,
            state: self.state,
            item: *self.iter.next()?,
        })
    }
}

impl<'a> LALR1ItemSet<'a> {
    pub fn new(lalr1a: &'a LALR1A, state: usize) -> Self {
        Self { lalr1a, state, iter: lalr1a.lr0a.states()[state].items.iter() }
    }
}


pub struct LALR1ItemProxy<'a> {
    lalr1a: &'a LALR1A<'a>,
    state: usize,
    item: LR0Item,
}

impl<'a> DottedItem for LALR1ItemProxy<'a> {
    type Lookaheads = Copied<hash_set::Iter<'a, Option<usize>>>;

    fn production(&self) -> usize {
        self.item.production
    }
    
    fn is_kernel_item(&self) -> bool {
        self.item.is_kernel_item(self.lalr1a.grammar())
    }
    
    fn is_complete(&self) -> bool {
        self.item.is_complete(self.lalr1a.grammar())
    }
    
    fn symbol_at_dot(&self) -> Option<Symbol> {
        self.item.symbol_at_dot(self.lalr1a.grammar())
    }
    
    // fn symbols(&self) -> &[Symbol] {
    //     self.lalr1a.grammar().productions().get(self.item.production).1
    // }

    fn lookaheads(&self) -> Self::Lookaheads {
        // self.lalr1a.lookahead[self.state][&self.item.production].iter().copied()
        let pair = StateReductionPair { state: self.state, production: self.item.production };
        self.lalr1a.lookahead[&pair].iter().copied()
    }
}

// =================
// === INTERNALS ===
// =================

mod builder;
use self::builder::LALR1ABuilder;