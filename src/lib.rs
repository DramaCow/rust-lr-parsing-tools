#![warn(missing_docs)]

//! Core formal language analysis tools.

mod grammar;
pub use grammar::{
    Grammar,
    GrammarBuilder,
    GrammarBuildError,
    Symbol,
    nullability,
    First,
    Follow,
};

mod automata;
pub use automata::{
    LR0A,
    LALR1A,
    LR1A,
    LR0Item,
    LR1Item,
};

pub mod table;

// =================
// === INTERNALS ===
// =================

mod transitive_closure;
use transitive_closure::transitive_closure;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
