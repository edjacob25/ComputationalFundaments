use criterion::{criterion_group, criterion_main, Criterion};
use homework_5::{heapsort, mergesort, quicksort, read_file};

fn benchmark(c: &mut Criterion) {
    let mut random: [Vec<u32>; 5] = Default::default();
    let mut sorted: [Vec<u32>; 5] = Default::default();
    let mut reverse: [Vec<u32>; 5] = Default::default();
    for i in 2..7 {
        random[i - 2] = read_file(format!("random_{}", i).to_string()).unwrap();
        sorted[i - 2] = read_file(format!("sorted_{}", i).to_string()).unwrap();
        reverse[i - 2] = read_file(format!("reverse_{}", i).to_string()).unwrap();
    }

    let mut group = c.benchmark_group("quicksort");
    for i in 2..7 {
        group.bench_function(format!("random {}", i), |b| {
            b.iter(|| quicksort(random[i - 2].clone()))
        });
        group.bench_function(format!("sorted {}", i), |b| {
            b.iter(|| quicksort(sorted[i - 2].clone()))
        });
        group.bench_function(format!("reverse {}", i), |b| {
            b.iter(|| quicksort(reverse[i - 2].clone()))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("mergesort");
    for i in 2..7 {
        group.bench_function(format!("random {}", i), |b| {
            b.iter(|| mergesort(&mut random[i - 2].clone()))
        });
        group.bench_function(format!("sorted {}", i), |b| {
            b.iter(|| mergesort(&mut sorted[i - 2].clone()))
        });
        group.bench_function(format!("reverse {}", i), |b| {
            b.iter(|| mergesort(&mut reverse[i - 2].clone()))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("heapsort");
    for i in 2..7 {
        group.bench_function(format!("random {}", i), |b| {
            b.iter(|| heapsort(&mut random[i - 2].clone()))
        });
        group.bench_function(format!("sorted {}", i), |b| {
            b.iter(|| heapsort(&mut sorted[i - 2].clone()))
        });
        group.bench_function(format!("reverse {}", i), |b| {
            b.iter(|| heapsort(&mut reverse[i - 2].clone()))
        });
    }
    group.finish();
}

criterion_group! {
    name=benches;
    config=Criterion::default().sample_size(10);
    targets=benchmark
}

criterion_main!(benches);
