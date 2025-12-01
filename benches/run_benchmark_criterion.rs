use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn find_max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for &item in arr.iter().skip(1) {
        if item > max {
            max = item;
        }
    }
    max
}

fn benchmark_criterion(c: &mut Criterion) {
    c.bench_function("find_max", |b| {
        b.iter(|| find_max(black_box(&[1, 2, 3, 4, 5])))
    });
}

// cargo bench --bench run_benchmark_criterion
criterion_group!(benches, benchmark_criterion);
criterion_main!(benches);
