//! Ranking functions re-written to accept any exact-sized iterators
//! to avoid additional allocations for odd rows and columns.
//!
//! # Developer notes
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

/// Calculate `rank+(A, a)` from the original papers.
///
/// Also known as `rank+` in the original papers, and `rankRightV` in the reference implementation.
///
/// Note that `rank-(A, a)` is simply `n * n - n_greater(A, a)`.
fn n_greater<'v, V, I>(window: I, guard: &'v V) -> usize
where
    V: PartialOrd<V>,
    &'v V: Sub<&'v V, Output = V>,
    I: Clone + ExactSizeIterator<Item = &'v V>,
{
    let window_size = window.len();
    let mut column_iter = window.clone().enumerate().peekable();

    window
        .map(|lhs| {
            // Move right until a smaller element is found:
            while column_iter
                .next_if(|(_, rhs)| &(lhs - rhs) > guard)
                .is_some()
            {}

            // Count the elements on the left, excluding the current one
            // (which is no longer less than the guard):
            column_iter
                .peek()
                .map_or(window_size, |(column_index, _)| *column_index)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_greater_ok() {
        // Matrix:
        // 0, -1
        // 1,  0
        assert_eq!(n_greater([1, 2].iter(), &-2), 4);
        assert_eq!(n_greater([1, 2].iter(), &-1), 3);
        assert_eq!(n_greater([1, 2].iter(), &0), 1);
        assert_eq!(n_greater([1, 2].iter(), &1), 0);

        // Matrix:
        // 0, -1, -1
        // 1,  0,  0
        // 1,  0,  0
        assert_eq!(n_greater([1, 2, 2].iter(), &-2), 9);
        assert_eq!(n_greater([1, 2, 2].iter(), &-1), 7);
        assert_eq!(n_greater([1, 2, 2].iter(), &0), 2);
        assert_eq!(n_greater([1, 2, 2].iter(), &1), 0);

        // Matrix:
        // 0, -1, -2
        // 1,  0, -1
        // 2,  1,  0
        assert_eq!(n_greater([1, 2, 3].iter(), &-3), 9);
        assert_eq!(n_greater([1, 2, 3].iter(), &-2), 8);
        assert_eq!(n_greater([1, 2, 3].iter(), &-1), 6);
        assert_eq!(n_greater([1, 2, 3].iter(), &0), 3);
        assert_eq!(n_greater([1, 2, 3].iter(), &1), 1);
        assert_eq!(n_greater([1, 2, 3].iter(), &2), 0);
    }
}
