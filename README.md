# `fqn-estimator`

Rust implementation of the rolling «Fast $`Q_n`$» algorithm for data streams.

[![Documentation](https://img.shields.io/docsrs/fqn-estimator?style=for-the-badge)
](https://docs.rs/fqn-estimator)
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
        257.0, 917.0, 236.0, 271.0, 339.0, 19.0, 994.0, 710.0, 411.0, 922.0,
        516.0, 329.0, 405.0, 112.0, 980.0, 308.0, 918.0, 83.0, 116.0, 122.0,
        329.0, 227.0, 541.0, 774.0, 455.0, 706.0, 151.0, 829.0, 463.0, 763.0,
        453.0, 218.0, 872.0, 326.0, 162.0, 607.0, 689.0, 672.0, 56.0, 997.0, 
        598.0, 920.0, 817.0, 949.0, 155.0, 688.0, 755.0, 721.0, 430.0, 184.0, 
        314.0, 308.0, 709.0, 626.0, 333.0, 307.0, 63.0, 473.0, 594.0, 366.0,
        687.0, 463.0, 46.0, 994.0, 948.0, 392.0, 431.0, 171.0, 413.0, 975.0,
        126.0, 975.0, 337.0, 49.0, 196.0, 463.0, 784.0, 722.0, 522.0, 182.0,
        919.0, 181.0, 120.0, 177.0, 131.0, 612.0, 5.0, 952.0, 663.0, 628.0, 
        648.0, 238.0, 845.0, 354.0, 223.0, 315.0, 985.0, 38.0, 2.0, 34.0,
    ];

    let mut estimator = QnScaleEstimator::new(samples.len());
    estimator.extend(samples);

    let scale = estimator.estimate().unwrap().to_f64();
    assert!(310.31 < scale && scale < 310.32);
    
    let median = estimator.median().unwrap().to_median();
    assert!(430.49 < median && median < 431.51);
}
```

## Features

- `num-traits`: use [`num-traits`](https://crates.io/crates/num-traits) to enable median for even-sized samples
