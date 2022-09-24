#![allow(non_upper_case_globals)]

use lr_parsing_tools::grammar::{Symbol, Grammar, GrammarBuilder};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

pub fn follow_benchmark(c: &mut Criterion) {
    c.bench_function("follow_set", |b| b.iter(|| rr_expr_grammar().follow_set()));
}

criterion_group!(benches, follow_benchmark);
criterion_main!(benches);