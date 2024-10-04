use criterion::{criterion_group, criterion_main, Criterion};
use napi::bindgen_prelude::Float32Array;

extern crate rust_walker;
use rust_walker::{roll_walker_table, simulate_chances};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("roll 10", |b| {
    b.iter(|| roll_walker_table(1, Float32Array::new(vec![1.0, 2.0, 3.0, 4.0])))
  });
  c.bench_function("roll 100_000", |b| {
    b.iter(|| roll_walker_table(100_000, Float32Array::new(vec![1.0, 2.0, 3.0, 4.0])))
  });
  c.bench_function("roll 100_000_000", |b| {
    b.iter(|| roll_walker_table(100_000_000, Float32Array::new(vec![1.0, 2.0, 3.0, 4.0])))
  });
}

fn chances(c: &mut Criterion) {
  c.bench_function("roll 1_000_000 of 1/10", |b| {
    b.iter(|| simulate_chances(1_000_000, 10))
  });
  c.bench_function("roll 1_000_000 of 1/100", |b| {
    b.iter(|| simulate_chances(1_000_000, 100))
  });
  c.bench_function("roll 1_000_000 of 1/1000", |b| {
    b.iter(|| simulate_chances(1_000_000, 1000))
  });
}

criterion_group!(benches, chances);
criterion_main!(benches);
