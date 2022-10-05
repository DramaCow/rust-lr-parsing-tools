#![allow(non_upper_case_globals)]

use super::{Symbol, Grammar, GrammarBuilder};

fn rr_expr_grammar() -> Grammar {
    const add: Symbol    = Symbol::Terminal(0);
    const sub: Symbol    = Symbol::Terminal(1);
    const mul: Symbol    = Symbol::Terminal(2);
    const div: Symbol    = Symbol::Terminal(3);
    const lparen: Symbol = Symbol::Terminal(4);
    const rparen: Symbol = Symbol::Terminal(5);
    const name: Symbol   = Symbol::Terminal(6);
    const num: Symbol    = Symbol::Terminal(7);
    // ---
    const Expr: Symbol   = Symbol::Variable(0);
    const Expr_: Symbol  = Symbol::Variable(1);
    const Term: Symbol   = Symbol::Variable(2);
    const Term_: Symbol  = Symbol::Variable(3);
    const Factor: Symbol = Symbol::Variable(4);

    // Expr : Term Expr_,
    // Expr_ : + Term Expr_
    //       | - Term Expr_
    //       | ,
    // Term : Factor Term_,
    // Term_ : * Factor Term_
    //       | / Factor Term_
    //       | ,
    // Factor : ( Expr )
    //        | name
    //        | num,
    GrammarBuilder::new()
        .rule(&[&[Term, Expr_]])
        .rule(&[&[add, Term, Expr_],
                &[sub, Term, Expr_],
                &[]])
        .rule(&[&[Factor, Term_]])
        .rule(&[&[mul, Factor, Term_],
                &[div, Factor, Term_],
                &[]])
        .rule(&[&[lparen, Expr, rparen],
                &[name],
                &[num]])
        .build().unwrap()
}

// ---
const Expr: usize   = 0;
const Expr_: usize  = 1;
const Term: usize   = 2;
const Term_: usize  = 3;
const Factor: usize = 4;

#[test]
fn test_nullability() {
    let grammar = rr_expr_grammar();
    let nullable = grammar.nullability();
    assert!(!nullable.get(Expr));
    assert!(nullable.get(Expr_));
    assert!(!nullable.get(Term));
    assert!(nullable.get(Term_));
    assert!(!nullable.get(Factor));
}

#[test]
fn test_first() {
    const add: usize    = 0;
    const sub: usize    = 1;
    const mul: usize    = 2;
    const div: usize    = 3;
    const lparen: usize = 4;
    const name: usize   = 6;
    const num: usize    = 7;

    let grammar = rr_expr_grammar();
    let (first, _) = grammar.first_set();
    assert_eq!(&first.get(Expr), &[lparen, name, num]);
    assert_eq!(&first.get(Expr_), &[add, sub]);
    assert_eq!(&first.get(Term), &[lparen, name, num]);
    assert_eq!(&first.get(Term_), &[mul, div]);
    assert_eq!(&first.get(Factor), &[lparen, name, num]);
    println!("first = {:?}", first);
}

#[test]
fn test_follow() {
    const add: Option<usize>    = Some(0);
    const sub: Option<usize>    = Some(1);
    const mul: Option<usize>    = Some(2);
    const div: Option<usize>    = Some(3);
    const rparen: Option<usize> = Some(5);
    const eof: Option<usize>    = None;

    let grammar = rr_expr_grammar();
    let (follow, _, _) = grammar.follow_set();
    assert_eq!(&follow.get(Expr), &[eof, rparen]);
    assert_eq!(&follow.get(Expr_), &[eof, rparen]);
    assert_eq!(&follow.get(Term), &[eof, add, sub, rparen]);
    assert_eq!(&follow.get(Term_), &[eof, add, sub, rparen]);
    assert_eq!(&follow.get(Factor), &[eof, add, sub, mul, div, rparen]);
}