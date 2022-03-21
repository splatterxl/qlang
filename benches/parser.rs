use std::fs;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use qlang::parser::Parser;

fn test(input: &str) {
    Parser::new(input).parse();
}

fn bench_all(c: &mut Criterion) {
    let input = fs::read_to_string("example.q").unwrap();

    let mut bmg = c.benchmark_group("all_features");
    bmg.throughput(Throughput::Bytes(input.len() as u64));
    bmg.bench_function("test", |b| b.iter(|| test(&input)));

    drop(bmg);

    // --

    let input = "import * from \"std\";\n".repeat(1_000_000);

    let mut bmg = c.benchmark_group("1m_lines");
    bmg.throughput(Throughput::Bytes(input.len() as u64));
    bmg.bench_function("test", |b| b.iter(|| test(&input)));

    drop(bmg);

    // --

    let input = "import * from \"std\";\n".repeat(6_000_000);

    let mut bmg = c.benchmark_group("6m_lines");
    bmg.throughput(Throughput::Bytes(input.len() as u64));
    bmg.bench_function("test", |b| b.iter(|| test(&input)));
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
