use crate::grammar::{Grammar, Symbol};

pub trait LRAutomaton<'a> {
    type Item;
    type Lookaheads;

    fn grammar(&self) -> &Grammar;
    
    fn state_count(&self) -> usize;

    fn items(&self, state: usize) -> &[Self::Item];
    
    fn transition(&self, state: usize, symbol: Symbol) -> Option<usize>;
    
    fn lookaheads(&'a self, state: usize, item: &Self::Item) -> Self::Lookaheads;

    // /// For some state q of an LRk automaton, the longest common preceding subpath is the longest
    // /// sequence of edges a_1, .., a_n such that all paths from start node s to q are of the form
    // /// b.., a_1, ..., a_n.
    // fn longest_common_preceding_subpath(&self, state: usize) -> &[Symbol] {
    //     let max_item = self.items(state).iter().max_by_key(|&item| self.pos(item)).unwrap();
    //     &self.grammar().productions().get(self.production(max_item)).1[..self.pos(max_item)]
    // }
}