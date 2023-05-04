use criterion::{criterion_group, criterion_main, Criterion};
use criterion::{AxisScale, PlotConfiguration};
use criterion::{BenchmarkId, Throughput};
use std::time::Duration;

use smooth_numbers::*;

fn bench_3_smooth(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("3-smooth");
    group.plot_config(plot_config);

    for n in [1, 10, 100, 1000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("pratt", n), &n, |b, &n| {
            b.iter(|| pratt(n))
        });
        group.bench_with_input(
            BenchmarkId::new("with_primes", n),
            &([2, 3], &n),
            |b, (primes, &n)| b.iter(|| with_primes(primes, n)),
        );
        // group.bench_with_input(BenchmarkId::new("smooth", n), &(2, n), |b, &(k, n)| {
        //     b.iter(|| smooth(k, n))
        // });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs_f64(0.25))
        .measurement_time(Duration::from_secs_f64(1.0));
    targets = bench_3_smooth
}
criterion_main!(benches);
