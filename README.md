# `fqn-estimator`

Rust implementation of the «Fast Qn» algorithm for data streams.

[![Check](https://github.com/eigenein/rust-fqn-estimator/actions/workflows/check.yaml/badge.svg)](https://github.com/eigenein/rust-fqn-estimator/actions/workflows/check.yaml)

## Credits

- Selection in X + Y and matrices with sorted rows and columns (A. Mirzaian, E. Arjomandi): <https://doi.org/10.1016/0020-0190(85)90123-1>.
- Fast Detection of Outliers in Data Streams with the Qn Estimator (Massimo Cafaro, Catiuscia Melle, Marco Pulimeno, Italo Epicoco): [arXiv:1910.02459](https://doi.org/10.48550/arXiv.1910.02459), [reference implementation](https://github.com/cafaro/FQN).
- Finite-sample Rousseeuw-Croux scale estimators (Andrey Akinshin): [arXiv:2209.12268](https://doi.org/10.48550/arXiv.2209.12268).
