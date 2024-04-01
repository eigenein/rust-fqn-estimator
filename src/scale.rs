/// Calculated Qn estimate of scale.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ScaleEstimate<T> {
    /// Number of samples.
    pub n_samples: usize,

    /// The original statistic.
    ///
    /// It needs to be multiplied by the scaling coefficients before it may be considered
    /// an «actual scale». See the implementations for concrete `T`.
    pub statistic: T,
}

impl<T: TryInto<f64>> ScaleEstimate<T> {
    fn normalization_constant(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let n = self.n_samples as f64;

        let dn = if self.n_samples & 1 == 1 {
            1.0 - 1.594 / n + 3.22 / n.powi(2)
        } else {
            1.0 - 3.672 / n + 11.087 / n.powi(2)
        };

        2.219_144_465_985_08 * dn
    }
}

impl<T: Into<Self>> From<ScaleEstimate<T>> for f64 {
    /// Calculate the actual scale estimate, that is the statistic multiplied by
    /// the normalization constant.
    fn from(estimate: ScaleEstimate<T>) -> Self {
        estimate.normalization_constant() * estimate.statistic.into()
    }
}
