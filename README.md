# `fqn-estimator`

Rust implementation of the rolling «Fast _Qn_» algorithm for data streams.

[![Check](https://github.com/eigenein/rust-fqn-estimator/actions/workflows/check.yaml/badge.svg)](https://github.com/eigenein/rust-fqn-estimator/actions/workflows/check.yaml)

The _kth_-statistic retrieval from the pairwise differences is based on the paper[^1] of A. Mirzaian and E. Arjomandi, adapting the reference implementation from M. Cafaro and others[^2]. However, instead of using the insertion sort, the `std::collections::BTreeMap` is used to maintain the sorted window.

Scaling coefficients are taken from the Andrey Akinshin's paper[^3] on finite-sample scale estimators.

[^1]: Selection in `X + Y` and matrices with sorted rows and columns (A. Mirzaian, E. Arjomandi): <https://doi.org/10.1016/0020-0190(85)90123-1>.
[^2]: Fast Detection of Outliers in Data Streams with the `Qn` Estimator (Massimo Cafaro, Catiuscia Melle, Marco Pulimeno, Italo Epicoco): [arXiv:1910.02459](https://doi.org/10.48550/arXiv.1910.02459), [reference implementation](https://github.com/cafaro/FQN).
[^3]: Finite-sample Rousseeuw-Croux scale estimators (Andrey Akinshin): [arXiv:2209.12268](https://doi.org/10.48550/arXiv.2209.12268).
