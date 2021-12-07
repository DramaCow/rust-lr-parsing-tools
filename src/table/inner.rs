use crate::grammar::{Grammar, Symbol};

pub trait ItemSets {
    type Item;

    fn grammar(&self) -> &Grammar;
    fn state_count(&self) -> usize;

    fn items(&self, state: usize) -> &[Self::Item];
    fn transition(&self, state: usize, symbol: Symbol) -> Option<usize>;
    
    fn production(&self, item: &Self::Item) -> usize;
    fn pos(&self, item: &Self::Item) -> usize;
    fn is_complete(&self, item: &Self::Item) -> bool;
    fn symbol_at_dot(&self, item: &Self::Item) -> Option<Symbol>;

    /// For some state q of an LRk automaton, the longest common preceding subpath is the longest
    /// sequence of edges a_1, .., a_n such that all paths from start node s to q are of the form
    /// b.., a_1, .., a_n.
    fn longest_common_preceding_subpath(&self, state: usize) -> &[Symbol] {
        let max_item = self.items(state).iter().max_by_key(|&item| self.pos(item)).unwrap();
        &self.grammar().productions().get(self.production(max_item)).1[..self.pos(max_item)]
    }
}

pub trait Lookaheads<'a>: ItemSets {
    type Output;
    fn lookaheads(&'a self, state: usize, item: &Self::Item) -> Self::Output;
}