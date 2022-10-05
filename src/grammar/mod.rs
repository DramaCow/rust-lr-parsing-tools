//! Context free grammar.

#![allow(non_snake_case)]

mod grammar;
pub use self::grammar::{
    Grammar,
    GrammarBuilder,
    GrammarBuildError,
    Symbol,
};

mod first;
pub use self::first::First;

mod follow;
pub use self::follow::Follow;

mod nullable;
pub use self::nullable::Nullable;

// =================
// === INTERNALS ===
// =================

#[cfg(test)]
mod tests;