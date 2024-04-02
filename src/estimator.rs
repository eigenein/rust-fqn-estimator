use std::{collections::VecDeque, fmt::Debug, ops::Sub};

use crate::{
    scale::ScaleEstimate,
    select::select_kth_statistic,
    sorted_vec::SortedVec,
    window::Window,
    RawMedian,
};

/// `Qn` estimator of scale.
#[must_use = "constructing an estimator without using it makes no sense"]
pub struct QnScaleEstimator<T> {
    chronological: Window<T>,
    sorted: SortedVec<T>,
}

impl<T> QnScaleEstimator<T> {
    /// Create a new estimator with the specified window length.
    pub fn new(window_len: usize) -> Self {
        Self {
            chronological: Window(VecDeque::with_capacity(window_len)),
            sorted: SortedVec(Vec::with_capacity(window_len)),
        }
    }

    /// Clear the current sample window.
    pub fn clear(&mut self) {
        self.chronological.0.clear();
        self.sorted.0.clear();
    }
}

impl<T: Copy + PartialEq + PartialOrd> QnScaleEstimator<T> {
    /// Push the upcoming value to the estimator.
    ///
    /// If the current window is already at its maximum length, the oldest value
    /// gets discarded and will no longer affect the estimator parameters.
    ///
    /// This operation is `O(window_len)`.
    #[allow(clippy::missing_panics_doc)]
    pub fn push(&mut self, value: T) {
        if let Some(popped_value) = self.chronological.push(value) {
            // The window was already full, remove the popped value from the sorted vector as well:
            self.sorted
                .remove_value(&popped_value)
                .expect("the popped value should also be present in the sorted vector");
        }

        // And now, insert the upcoming value into the sorted vector:
        self.sorted.insert_sorted(value);

        debug_assert_eq!(self.sorted.0.capacity(), self.chronological.0.capacity());
        debug_assert_eq!(self.sorted.0.len(), self.chronological.0.len());
    }

    /// Push multiple values to the estimator.
    pub fn extend(&mut self, iter: impl IntoIterator<Item = T>) {
        for value in iter {
            self.push(value);
        }
    }
}

impl<T: Copy + Debug + Default + PartialOrd + Sub<T, Output = T>> QnScaleEstimator<T> {
    /// Calculate the estimate of scale.
    ///
    /// This is a linear-time operation.
    ///
    /// # Returns
    ///
    /// Qn estimate of scale, or [`None`] if the sample contains less than 2 samples.
    #[must_use = "calculating the scale without using it makes no sense"]
    pub fn estimate(&self) -> Option<ScaleEstimate<T>> {
        let n = self.sorted.0.len();
        if n == 0 {
            return None;
        }
        let h = n / 2 + 1;
        let k =
            // From original Qn estimator:
            h * (h - 1) / 2
            // Offset to express the original statistic in terms of the `X + (-X)` statistic:
            + n + n * (n - 1) / 2;
        Some(ScaleEstimate {
            n_samples: n,
            statistic: select_kth_statistic(self.sorted.0.iter().copied(), k),
        })
    }
}

impl<T: Copy> QnScaleEstimator<T> {
    /// Obtain the sample median.
    ///
    /// This is a constant-time operation.
    ///
    /// # Returns
    ///
    /// The sample median, or [`None`] – if the sample is empty.
    #[must_use]
    pub fn median(&self) -> Option<RawMedian<T>> {
        self.sorted.median()
    }
}

#[cfg(test)]
mod tests {
    use crate::QnScaleEstimator;

    #[test]
    fn empty_ok() {
        let estimator = QnScaleEstimator::<i32>::new(1);
        assert_eq!(estimator.estimate(), None);
    }

    #[test]
    fn two_samples_ok() {
        let mut estimator = QnScaleEstimator::<i32>::new(2);
        estimator.push(1);
        estimator.push(2);

        let scale = estimator.estimate().unwrap();
        assert_eq!(scale.n_samples, 2);
        assert_eq!(scale.statistic, 1);
    }

    #[test]
    fn window_overflow_ok() {
        let samples = [
            2, 78, 1, 4, 19, 37, 68, 91, 42, 42, 75, 40, 4, 18, 18, 77, 9, 78, 57, 99,
        ];

        let mut estimator = QnScaleEstimator::new(10);
        estimator.extend(samples);

        let scale = estimator.estimate().unwrap();
        assert_eq!(scale.n_samples, 10);
        assert_eq!(scale.statistic, 22);
    }
}
