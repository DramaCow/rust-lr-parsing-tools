use std::collections::BTreeSet;
use std::iter::once;
use super::{Grammar, Symbol, Nullable, First};

/// A utility struct that, for each unique variable present in a 
/// grammar, stores the set of terminals (the follow set) that can
/// appear immediately after in a sentence.
/// 
/// Sets of tokens are represented as slices of type `Option<usize>`,
/// where `None` represents EOF (End Of File).  
#[derive(Debug)]
pub struct Follow {
    follows: Vec<Option<usize>>,
    var_ranges: Vec<usize>,
}

impl Follow {
    #[must_use]
    pub(super) fn new(grammar: &Grammar, nullable: &Nullable, first: &First) -> Self {
        let var_follows = compute_var_follows(grammar, nullable, first);
        let var_ranges = once(0)
            .chain(
                var_follows.iter()
                .map(BTreeSet::len)
                .scan(0, |cumsum, len| { *cumsum += len; Some(*cumsum) }))
            .collect();

        Self {
            follows: var_follows.into_iter().flatten().collect(),
            var_ranges,
        }
    }

    pub fn get(&self, var: usize) -> &[Option<usize>] {
        &self.follows[self.var_ranges[var]..self.var_ranges[var+1]]
    }
}

// =================
// === INTERNALS ===
// =================

/// Constructs the follow sets for each unique variable in grammar.
fn compute_var_follows(grammar: &Grammar, nullable: &Nullable, first: &First) -> Vec<BTreeSet<Option<usize>>> {
    let mut follow = vec![BTreeSet::<Option<usize>>::new(); grammar.rules().len()];
    follow.last_mut().unwrap().insert(None);

    let mut done = false;
    while !done {
        done = true;
        for (A, beta) in grammar.productions() {
            let mut trailer = follow[A].clone();
            for &symbol in beta.iter().rev() {
                match symbol {
                    Symbol::Terminal(b) => {
                        trailer = once(Some(b)).collect();
                    }
                    Symbol::Variable(B) => {
                        if !trailer.is_subset(&follow[B]) {
                            follow[B].extend(&trailer);
                            done = false;
                        }
                        let first_B = &first.get(B);
                        if nullable.get(B) {
                            trailer.extend(first_B.iter().copied().map(Some));
                        } else {
                            trailer = first_B.iter().copied().map(Some).collect();
                        }
                    }
                }
            }
        }
    }

    follow
}