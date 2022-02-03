use crate::grammar::{GrammarBuilder, Symbol::Terminal as Word, Symbol::Variable as Var};
use super::{LALR1A, LRAutomaton, DottedItem};

#[test]
fn parentheses_grammar() {
    let wordnames = &["c", "d"];
    let varnames = &["S", "C", "S'"];
    let grammar = GrammarBuilder::new()
        .rule(&[&[Var(1), Var(1)]])
        .rule(&[&[Word(0), Var(1)], &[Word(1)]])
        .build().unwrap();

    let lalr1a = LALR1A::new(&grammar);

    println!("{:?}", lalr1a.lookahead);

    for i in 0..lalr1a.state_count() {
        for item in lalr1a.items(i) {
            let (lhs, rhs) = grammar.productions().get(item.production());
            print!("[{} -->", varnames[lhs]);
            for symbol in rhs {
                match *symbol {
                    Word(a) => print!(" {}", wordnames[a]),
                    Var(A) => print!(" {}", varnames[A]),
                }
            }
            print!(", {{");

            // for lookahead in item.lookaheads() {
            //     match lookahead {
            //         Some(a) => print!(" {},", wordnames[a]),
            //         None => print!(" #,"),
            //     }
            // }
            println!(" }}]");
        }
        println!("----------------");
    }
}