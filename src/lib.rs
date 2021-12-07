#![warn(missing_docs)]

//! Core formal language analysis tools.

pub mod grammar;
pub mod automata;
pub mod table;

// =================
// === INTERNALS ===
// =================

mod transitive_closure;
use transitive_closure::transitive_closure;