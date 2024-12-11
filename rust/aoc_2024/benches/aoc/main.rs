use aoc_lib::days::day11;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_day11(c: &mut Criterion) {
    let stones = day11::input();
    c.bench_function("parse_input", |b| b.iter(|| day11::input()));
    c.bench_function("part_a", |b| b.iter(|| day11::parta(&stones)));
    c.bench_function("part_b", |b| b.iter(|| day11::partb(&stones)));
}
criterion_group!(benches, bench_day11);
criterion_main!(benches);
