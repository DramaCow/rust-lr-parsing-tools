//! Context free grammar.

#![allow(non_snake_case)]

mod grammar;
pub use self::grammar::{
    Grammar,
    GrammarBuilder,
    GrammarBuildError,
    Symbol,
};

mod nullability;
pub use self::nullability::nullability;

mod first;
pub use self::first::First;

mod follow;
pub use self::follow::Follow;

// =================
// === INTERNALS ===
// =================

#[cfg(test)]
mod tests;