use std::iter::Copied;
use std::collections::hash_set;
use crate::{Grammar, Symbol};
use crate::{LALR1A, LR0Item};
use super::{super::inner};
use super::super::LR1TableConstruction;
use super::super::{Action, NaiveLR1Table, Conflict, ConstructionError};

pub struct LALR1;

impl LR1TableConstruction for LALR1 {
    fn construct<F>(grammar: &Grammar, conflict_resolution: F) -> Result<NaiveLR1Table, ConstructionError>
    where
        F: FnMut(Conflict) -> Result<Action, Conflict>
    {
        NaiveLR1Table::build(&LALR1A::new(grammar), conflict_resolution)
    }
}

// =================
// === INTERNALS ===
// =================

impl inner::ItemSets for LALR1A<'_> {
    type Item = LR0Item;

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
        item.production
    }

    fn pos(&self, item: &Self::Item) -> usize {
        item.pos
    }

    fn is_complete(&self, item: &Self::Item) -> bool {
        item.is_complete(self.grammar())
    }

    fn symbol_at_dot(&self, item: &Self::Item) -> Option<Symbol> {
        item.symbol_at_dot(self.grammar())
    }
}

impl<'a> inner::Lookaheads<'a> for LALR1A<'_> {
    type Output = Copied<hash_set::Iter<'a, Option<usize>>>;

    fn lookaheads(&'a self, state: usize, item: &Self::Item) -> Self::Output {
        self.lookaheads(state, item.production).iter().copied()
    }
}