/// Calculated Qn estimate of scale.
#[derive(Debug, Eq, PartialEq)]
pub struct ScaleEstimate<T> {
    /// Number of samples.
    pub n_samples: usize,

    /// The original statistic.
    ///
    /// It needs to be multiplied by the scaling coefficients before it may be considered
    /// a «real scale». See the implementations for concrete [`T`].
    pub statistic: T,
}

impl ScaleEstimate<f64> {
    const DN: f64 = 2.219_144_465_985_08;

    /// Calculate the corrected scale estimate.
    #[must_use]
    pub fn scale(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let n = self.n_samples as f64;

        let dn = if self.n_samples & 1 == 1 {
            1.0 - 1.594 / n + 3.22 / n.powi(2)
        } else {
            1.0 - 3.672 / n + 11.087 / n.powi(2)
        };
        self.statistic * Self::DN * dn
    }
}
