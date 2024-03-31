use std::{fmt::Debug, ops::Sub};

use crate::{
    scale::ScaleEstimate,
    select::select_kth_statistic,
    sorted_vec::SortedVec,
    window::Window,
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
            chronological: Window::with_capacity(window_len),
            sorted: SortedVec(Vec::with_capacity(window_len)),
        }
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
    #[must_use = "calculating the scale without using it makes no sense"]
    pub fn scale(&self) -> ScaleEstimate<T> {
        let n = self.sorted.0.len();
        let h = n / 2 + 1;
        let k =
            // From original Qn estimator:
            h * (h - 1) / 2
            // Offset to express the original statistic in terms of the `X + (-X)` statistic:
            + n + n * (n - 1) / 2;
        ScaleEstimate {
            n_samples: n,
            statistic: select_kth_statistic(self.sorted.0.iter().copied(), k),
        }
    }
}
