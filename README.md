# `fqn-estimator`

Rust implementation of the rolling «Fast $`Q_n`$» algorithm for data streams.

[![Check](https://github.com/eigenein/rust-fqn-estimator/actions/workflows/check.yaml/badge.svg)](https://github.com/eigenein/rust-fqn-estimator/actions/workflows/check.yaml)

The _kth_-statistic retrieval from the pairwise differences is based on the paper[^1] of A. Mirzaian and E. Arjomandi, adapting the implementation[^2] from M. Cafaro and others[^3].

[^1]: DOI: [Selection in `X + Y` and matrices with sorted rows and columns](https://doi.org/10.1016/0020-0190(85)90123-1) (A. Mirzaian and E. Arjomandi)
[^2]: GitHub: [cafaro/FQN](https://github.com/cafaro/FQN) (Massimo Cafaro)
[^3]: DOI: [Fast Detection of Outliers in Data Streams with the `Qn` Estimator](https://doi.org/10.48550/arXiv.1910.02459) (Massimo Cafaro, Catiuscia Melle, Marco Pulimeno, and Italo Epicoco)

Instead of using the insertion sort[^2][^3], I use a «2-level rotated array»[^4] to maintain the sorted window, that costs $`O(n + sqrt{n})`$ space, $`O(sqrt{n})`$ time for inserts and deletes, and $`O(\log n)`$ time for accessing the median.

$`Q_n`$ scaling coefficients are taken from the Andrey Akinshin's paper[^5] on finite-sample scale estimators.

[^4]: GitHub: [senderista/rotated-array-set](https://github.com/senderista/rotated-array-set) (Tobin Baker)
[^5]: DOI: [Finite-sample Rousseeuw-Croux scale estimators](https://doi.org/10.48550/arXiv.2209.12268) (Andrey Akinshin)
