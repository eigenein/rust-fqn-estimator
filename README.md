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
