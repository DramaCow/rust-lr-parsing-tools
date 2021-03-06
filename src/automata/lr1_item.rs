use super::LR0Item;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LR1Item {
    pub lr0_item: LR0Item,
    pub lookahead: Option<usize>, // class of lookahead terminal
}

impl AsRef<LR0Item> for LR1Item {
    fn as_ref(&self) -> &LR0Item { 
        &self.lr0_item
    }
}

impl LR1Item {
    #[must_use]
    pub fn new(alt: usize, pos: usize, lookahead: Option<usize>) -> Self {
        Self {
            lr0_item: LR0Item::new(alt, pos),
            lookahead,
        }
    }
}