/// Calculated Qn estimate of scale.
#[derive(Eq, PartialEq)]
pub struct ScaleEstimate<T> {
    /// Number of samples.
    pub n_samples: usize,

    /// The original statistic.
    ///
    /// It needs to be multiplied by the scaling coefficients before it may be considered
    /// a «real scale». See the implementations for concrete [`T`].
    pub statistic: T,
}
