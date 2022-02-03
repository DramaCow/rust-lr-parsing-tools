use super::LR0Item;
use crate::grammar::{Grammar, Symbol};

pub trait LRAutomaton<'a> 
where
    <Self::ItemSet as IntoIterator>::Item: DottedItem
{
    type ItemSet: IntoIterator + 'a;

    fn grammar(&self) -> &Grammar;
    
    fn state_count(&self) -> usize;

    fn items(&'a self, state: usize) -> Self::ItemSet;
    
    fn transition(&self, state: usize, symbol: Symbol) -> Option<usize>;
    
    // /// For some state q of an LRk automaton, the longest common preceding subpath is the longest
    // /// sequence of edges a_1, .., a_n such that all paths from start node s to q are of the form
    // /// b.., a_1, ..., a_n.
    // fn longest_common_preceding_subpath(&self, state: usize) -> &[Symbol] {
    //     let max_item = self.items(state).iter().max_by_key(|&item| self.pos(item)).unwrap();
    //     &self.grammar().productions().get(self.production(max_item)).1[..self.pos(max_item)]
    // }
}

pub trait DottedItem {
    type Lookaheads: IntoIterator<Item = Option<usize>>;

    fn production(&self) -> usize;

    fn is_kernel_item(&self) -> bool;
    
    fn is_complete(&self) -> bool;
    
    fn symbol_at_dot(&self) -> Option<Symbol>;
    
    // fn symbols(&self) -> &[Symbol];

    fn lookaheads(&self) -> Self::Lookaheads;
}