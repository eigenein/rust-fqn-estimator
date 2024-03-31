use std::ops::{Add, Div};

/// Intermediary median value which isn't converted to a single value yet.
#[must_use = "obtaining a median without using it makes no sense"]
#[derive(Debug, Eq, PartialEq)]
pub enum RawMedian<T> {
    Odd(T),
    Even(T, T),
}

#[cfg(feature = "num-traits")]
impl<T> RawMedian<T>
where
    T: Add<Output = T> + Div<Output = T> + num_traits::One,
{
    /// Get the actual median that is itself for odd-sized samples, and average of the middle items
    /// for even-sized samples.
    pub fn to_median(self) -> T {
        match self {
            Self::Odd(median) => median,

            #[allow(clippy::suspicious_operation_groupings)]
            Self::Even(left, right) => (left + right) / (T::one() + T::one()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_ok() {
        assert_eq!(RawMedian::Odd(42).to_median(), 42);
    }

    #[test]
    fn even_ok() {
        assert_eq!(RawMedian::Even(42, 44).to_median(), 43);
    }
}
