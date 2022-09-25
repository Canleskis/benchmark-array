use benchmark_array::Array;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};

fn random_arrays<const N: usize>(size: usize) -> Vec<Array<N>> {
    let mut rng = thread_rng();
    let mut gen = || rng.gen_range(-100.0..100.0);
    let mut v = Vec::new();
    for _ in 0..size {
        v.push(Array([0.0; N].map(|_| gen())));
    }
    v
}

fn fold<const N: usize>(v: Vec<Array<N>>) -> Vec<Array<N>> {
    let result = v.iter().map(|a1| {
        v.iter().fold(Array::default(), |acc, a2| {
            let d = *a2 - *a1;

            acc + d
        })
    });

    result.collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Array fold");

    for i in (100..=500).step_by(10) {
        group.bench_with_input(BenchmarkId::new("Length 1", i), &i, |b, i| {
            b.iter(|| fold::<1>(black_box(random_arrays(*i))))
        });
        group.bench_with_input(BenchmarkId::new("Length 2", i), &i, |b, i| {
            b.iter(|| fold::<2>(black_box(random_arrays(*i))))
        });
        group.bench_with_input(BenchmarkId::new("Length 3", i), &i, |b, i| {
            b.iter(|| fold::<3>(black_box(random_arrays(*i))))
        });
        group.bench_with_input(BenchmarkId::new("Length 4", i), &i, |b, i| {
            b.iter(|| fold::<4>(black_box(random_arrays(*i))))
        });
        group.bench_with_input(BenchmarkId::new("Length 5", i), &i, |b, i| {
            b.iter(|| fold::<5>(black_box(random_arrays(*i))))
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
