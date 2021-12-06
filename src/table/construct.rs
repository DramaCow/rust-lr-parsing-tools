use super::{inner, Action, NaiveLR1Table};
use crate::{Grammar, Symbol};

#[derive(Debug)]
pub struct ConstructionError {
    pub state: usize,
    pub conflict: Conflict,
}

#[derive(Debug)]
pub enum Conflict {
    ShiftReduce { word: usize, next_state: usize, production: usize },
    ReduceReduce { production1: usize, production2: usize },
}

pub trait LR1TableConstruction {
    /// # Errors 
    fn construct<F>(grammar: &Grammar, conflict_resolution: F) -> Result<NaiveLR1Table, ConstructionError>
    where
        F: FnMut(Conflict) -> Result<Action, Conflict>;
}