//! Ranking functions re-written to accept any exact-sized iterators
//! to avoid additional allocations for odd rows and columns.
//!
//! # Developer's notes
//!
//! - **The window must be sorted.** This ensures the correctness of the fast algorithm,
//!   and eliminates the need to take absolute differences
//!   (`row_index > column_index` implies `lhs >= rhs`).
//!
//! - Although, the ranking functions are defined on a matrix, there is no need to store
//!   the matrix itself. **The matrix's elements are computed on-the-fly**.
//!   Rows are «sorted» in descending order, and columns are «sorted» in ascending order
//!   (consider pair-wise sums of the window's elements and the same window's negated elements).
//!
//! - The reference implementation raises [the concern][1], so I'm using the definition
//!   rather than blindly re-writing the reference implementation.
//!
//! [1]: https://github.com/cafaro/FQN/issues/1

use std::ops::Sub;

/// Calculate `rank+(A, a)` from the original papers, that is number of elements greater than `guard`.
///
/// Also known as `rank+` in the original papers, and `rankRightV` in the reference implementation.
pub fn n_greater<V, I>(window: I, guard: V) -> usize
where
    V: Copy + PartialOrd<V> + Sub<V, Output = V>,
    I: Clone + ExactSizeIterator<Item = V>,
{
    let window_size = window.len();
    let mut column_iter = window.clone().enumerate().peekable();

    window
        .map(|lhs| {
            // Move right until a smaller element is found:
            while column_iter.next_if(|(_, rhs)| lhs - *rhs > guard).is_some() {}

            // Count the elements on the left, excluding the current one
            // (which is no longer less than the guard):
            column_iter
                .peek()
                .map_or(window_size, |(column_index, _)| *column_index)
        })
        .sum()
}

/// Calculate `rank-(A, a)`, that is number of elements less than `guard`.
pub fn n_smaller<V, I>(window: I, guard: V) -> usize
where
    V: Copy + PartialOrd<V> + Sub<V, Output = V>,
    I: Clone + ExactSizeIterator<Item = V>,
{
    let window_size = window.len();
    let mut column_iter = window.clone().enumerate().peekable();

    window
        .map(|lhs| {
            // Note that here I use `>=`, it will stop at the first strictly smaller element,
            // and it must be counted.
            while column_iter
                .next_if(|(_, rhs)| lhs - *rhs >= guard)
                .is_some()
            {}

            column_iter
                .peek()
                .map_or(0, |(column_index, _)| window_size - *column_index)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// # Matrix
    ///
    /// ```text
    /// 0, -1
    /// 1,  0
    /// ```
    #[test]
    fn rank_2x2_ok() {
        let window = [1, 2].into_iter();

        assert_eq!(n_greater(window.clone(), -2), 4);
        assert_eq!(n_greater(window.clone(), -1), 3);
        assert_eq!(n_greater(window.clone(), 0), 1);
        assert_eq!(n_greater(window.clone(), 1), 0);

        assert_eq!(n_smaller(window.clone(), 2), 4);
        assert_eq!(n_smaller(window.clone(), 1), 3);
        assert_eq!(n_smaller(window.clone(), 0), 1);
        assert_eq!(n_smaller(window, -1), 0);
    }

    /// # Matrix
    ///
    /// ```text
    /// 0, -1, -1
    /// 1,  0,  0
    /// 1,  0,  0
    /// ```
    #[test]
    fn rank_3x3_with_repetitions_ok() {
        let window = [1, 2, 2].into_iter();

        assert_eq!(n_greater(window.clone(), -2), 9);
        assert_eq!(n_greater(window.clone(), -1), 7);
        assert_eq!(n_greater(window.clone(), 0), 2);
        assert_eq!(n_greater(window.clone(), 1), 0);

        assert_eq!(n_smaller(window.clone(), 2), 9);
        assert_eq!(n_smaller(window.clone(), 1), 7);
        assert_eq!(n_smaller(window.clone(), 0), 2);
        assert_eq!(n_smaller(window, -1), 0);
    }

    /// # Matrix
    ///
    /// ```text
    /// 0, -1, -2
    /// 1,  0, -1
    /// 2,  1,  0
    /// ```
    #[test]
    fn rank_3x3_ok() {
        let window = [1, 2, 3].into_iter();

        assert_eq!(n_greater(window.clone(), -3), 9);
        assert_eq!(n_greater(window.clone(), -2), 8);
        assert_eq!(n_greater(window.clone(), -1), 6);
        assert_eq!(n_greater(window.clone(), 0), 3);
        assert_eq!(n_greater(window.clone(), 1), 1);
        assert_eq!(n_greater(window.clone(), 2), 0);

        assert_eq!(n_smaller(window.clone(), 3), 9);
        assert_eq!(n_smaller(window.clone(), 2), 8);
        assert_eq!(n_smaller(window.clone(), 1), 6);
        assert_eq!(n_smaller(window.clone(), 0), 3);
        assert_eq!(n_smaller(window.clone(), -1), 1);
        assert_eq!(n_smaller(window, -2), 0);
    }
}
