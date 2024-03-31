# `fqn-estimator`

Rust implementation of the rolling «Fast $`Q_n`$» algorithm for data streams.

[![Check status](https://img.shields.io/github/actions/workflow/status/eigenein/rust-fqn-estimator/check.yaml?style=for-the-badge)]((https://github.com/eigenein/rust-fqn-estimator/actions/workflows/check.yaml))
[![Code coverage](https://img.shields.io/codecov/c/github/eigenein/rust-fqn-estimator?style=for-the-badge)
](https://app.codecov.io/gh/eigenein/rust-fqn-estimator)

The $`k`$th order statistic retrieval from the pairwise differences is based on the paper[^1] of A. Mirzaian and E. Arjomandi, adapting the implementation[^2] from M. Cafaro and others[^3].

[^1]: DOI: [Selection in `X + Y` and matrices with sorted rows and columns](https://doi.org/10.1016/0020-0190(85)90123-1) (A. Mirzaian, E. Arjomandi)
[^2]: GitHub: [cafaro/FQN](https://github.com/cafaro/FQN) (Massimo Cafaro)
[^3]: DOI: [Fast Detection of Outliers in Data Streams with the `Qn` Estimator](https://doi.org/10.48550/arXiv.1910.02459) (Massimo Cafaro, Catiuscia Melle, Marco Pulimeno, Italo Epicoco)

$`Q_n`$ scaling coefficients are taken from the paper[^4] on finite-sample scale estimators.

[^4]: DOI: [Finite-sample Rousseeuw-Croux scale estimators](https://doi.org/10.48550/arXiv.2209.12268) (Andrey Akinshin)

## Example

```rust
use fqn_estimator::QnScaleEstimator;

fn main() {
    let samples = [
        257, 917, 236, 271, 339, 19, 994, 710, 411, 922, 516, 329, 405, 112,
        980, 308, 918, 83, 116, 122, 329, 227, 541, 774, 455, 706, 151, 829,
        463, 763, 453, 218, 872, 326, 162, 607, 689, 672, 56, 997, 598, 920,
        817, 949, 155, 688, 755, 721, 430, 184, 314, 308, 709, 626, 333, 307,
        63, 473, 594, 366, 687, 463, 46, 994, 948, 392, 431, 171, 413, 975,
        126, 975, 337, 49, 196, 463, 784, 722, 522, 182, 919, 181, 120, 177,
        131, 612, 5, 952, 663, 628, 648, 238, 845, 354, 223, 315, 985, 38, 2, 34,
    ];

    let mut estimator = QnScaleEstimator::new(samples.len());
    estimator.extend(samples);

    let scale = estimator.scale().unwrap();
    assert_eq!(scale.n_samples, samples.len());
    assert_eq!(scale.statistic, 145);
}
```

## Features

- `num-traits`: use [`num-traits`](https://crates.io/crates/num-traits) to enable median for even-sized samples and $`Q_n`$ scale consistency coefficients[^4]
