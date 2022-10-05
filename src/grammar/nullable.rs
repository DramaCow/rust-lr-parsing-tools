use super::{Grammar, Symbol};

pub struct Nullable(Vec<bool>);

impl Nullable {
    pub(crate) fn new(grammar: &Grammar) -> Self {
        let mut nullable = vec![false; grammar.rules().len()];

        let mut done = false;

        while !done {
            done = true;

            for (A, rule) in grammar.rules().into_iter().enumerate() {
                if !nullable[A] {
                    nullable[A] = rule.alts().any(|alt| {
                        alt.iter().all(|&symbol| {
                            match symbol {
                                Symbol::Terminal(_) => false,
                                Symbol::Variable(B) => nullable[B],
                            }
                        })
                    });
        
                    if nullable[A] {
                        done = false;
                    }
                }
            }
        }

        Nullable(nullable)
    }

    pub fn get(&self, var: usize) -> bool {
        self.0[var]
    }
}