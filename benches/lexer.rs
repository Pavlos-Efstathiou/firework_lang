use criterion::{criterion_group, criterion_main, Criterion};
use firework_lang::lexer::Lexer;
use std::fs;

fn lexer_speed() {
    let source_code = fs::read_to_string("firework_tests/bench.firework").unwrap();
    let mut lexer = Lexer::new(&source_code);

    lexer.lex();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lex", |b| b.iter(|| lexer_speed()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
