mod table;
pub use self::table::{
    Action,
    LR1Table,
    NaiveLR1Table,
};

mod construct;
pub use self::construct::{
    ConstructionError,
    Conflict,
};

mod parse;
pub use self::parse::{
    Event,
    Parse,
    ParseError,
};

// =================
// === INTERNALS ===
// =================

#[cfg(test)]
mod tests;