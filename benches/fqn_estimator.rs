use criterion::{criterion_group, criterion_main, Criterion};
use fastrand::Rng;
use fqn_estimator::QnScaleEstimator;

fn bench_1000_samples(criterion: &mut Criterion) {
    const LEN: usize = 1000;

    let mut rng = Rng::with_seed(0);
    let mut estimator = QnScaleEstimator::new(LEN);
    estimator.extend((0..LEN).map(|_| rng.i32(0..100)));
    criterion.bench_function("scale 1000", |bencher| {
        bencher.iter(|| estimator.estimate())
    });
}

fn bench_10_000_samples(criterion: &mut Criterion) {
    const LEN: usize = 10_000;

    let mut rng = Rng::with_seed(0);
    let mut estimator = QnScaleEstimator::new(LEN);
    estimator.extend((0..LEN).map(|_| rng.i32(0..1000)));
    criterion.bench_function("scale 10000", |bencher| {
        bencher.iter(|| estimator.estimate())
    });
}

fn bench_1_000_000_samples(criterion: &mut Criterion) {
    const LEN: usize = 1_000_000;

    let mut rng = Rng::with_seed(0);
    let mut estimator = QnScaleEstimator::new(LEN);
    estimator.extend((0..LEN).map(|_| rng.i32(0..10000)));
    criterion.bench_function("scale 1000000", |bencher| {
        bencher.iter(|| estimator.estimate())
    });
}

criterion_group!(benches, bench_1000_samples, bench_10_000_samples, bench_1_000_000_samples);
criterion_main!(benches);
