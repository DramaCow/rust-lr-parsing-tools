use std::iter::{Once, once};
use super::{super::inner};
use crate::{LR1A, LR1Item};
use crate::{Grammar, Symbol};
use super::super::LR1TableConstruction;
use super::super::{Action, NaiveLR1Table, Conflict, ConstructionError};

pub struct LR1;

impl LR1TableConstruction for LR1 {
    fn construct<F>(grammar: &Grammar, conflict_resolution: F) -> Result<NaiveLR1Table, ConstructionError>
    where
        F: FnMut(Conflict) -> Result<Action, Conflict>
    {
        NaiveLR1Table::build(&LR1A::new(grammar), conflict_resolution)
    }
}

// =================
// === INTERNALS ===
// =================

impl inner::ItemSets for LR1A<'_> {
    type Item = LR1Item;

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
}

impl inner::Lookaheads<'_> for LR1A<'_> {
    type Output = Once<Option<usize>>;

    fn lookaheads(&self, _: usize, item: &Self::Item) -> Self::Output {
        once(item.lookahead)
    }
}
