use std::{fmt::Debug, ops::Sub};

use crate::{
    dash_iter::DashIter,
    lazy_list::LazyList,
    rank::{n_greater, n_smaller},
};

fn select<V, I>(window: I, k: usize) -> V
where
    V: Copy + Debug + Default + PartialOrd + Sub<V, Output = V>,
    I: Clone + ExactSizeIterator<Item = V>,
{
    debug_assert!(k >= 1, "Here, Kth statistic starts at 1");
    // Starting with unit step, meaning the full window.
    binary_select::<V, I>(window, k, k, 1).0
}

/// # Returns
///
/// Tuple of the `k1`-th and `k2`-th elements of the matrix derived from `window` and negated `window`.
///
/// P.S. Abandon hope all ye who enter here 💀
fn binary_select<V, I>(full_window: I, k1: usize, k2: usize, step: usize) -> (V, V)
where
    V: Copy + Debug + Default + PartialOrd + Sub<V, Output = V>,
    I: Clone + ExactSizeIterator<Item = V>,
{
    // Current `A` matrix:
    let window = DashIter::new(full_window.clone(), step);
    let n = window.len();

    debug_assert!(
        (n * n >= k1) && (k1 >= k2) && (k2 >= 1) && (k1 - k2 <= 4 * n - 4),
        "lemma 5.1 (Mirzaian & Arjomandi) should hold, but: n = {n}, k1 = {k1}, k2 = {k2}"
    );

    debug_assert!(n >= 2);
    if n == 2 {
        return (
            select_trivial(window.clone(), k1),
            select_trivial(window, k2),
        );
    }

    // Define k1-dash and k2-dash from the papers:
    let k1_dash = if n & 1 == 0 {
        // Original paper mentions `ceil(k1 / 4)`, which is simply `floor((k1 + 3) / 4)`
        // (just consider all the possible remainders of 4).
        n + 1 + (k1 + 3) / 4
    } else {
        // `ceil(1 / 4 * (k1 + 2n + 1))` which is `floor(1 / 4 * (k1 + 2n + 1 + 3))` which is
        // `floor(1 / 4 * (k1 + 2n)) + 1`.
        (2 * n + k1) / 4 + 1
    };

    // Surprisingly, here they used the same very trick they did NOT use for `k1`. Okay 🤔
    let k2_dash = (k2 + 3) / 4;

    // Bi-select in the `A-dash` matrix and rank the candidates:
    let (max_candidate, min_candidate) = binary_select(full_window, k1_dash, k2_dash, step * 2);
    debug_assert!(min_candidate <= max_candidate); // b <= a
    let rank_max = n_smaller(window.clone(), max_candidate); // ra-
    let rank_min = n_greater(window.clone(), min_candidate); // rb+

    // We may not need the `L`, hence the lazy wrapper.
    let mut lazy_l = LazyList::new(window, min_candidate, max_candidate);

    #[allow(clippy::suspicious_operation_groupings)]
    (
        if rank_max < k1 {
            max_candidate
        } else if k1 + rank_min <= n * n {
            min_candidate
        } else {
            select_nth(lazy_l.build(), k1 + rank_min - n * n - 1)
        },
        if rank_max < k2 {
            max_candidate
        } else if k2 + rank_min <= n * n {
            min_candidate
        } else {
            select_nth(lazy_l.build(), k2 + rank_min - n * n - 1)
        },
    )
}

/// Handle the trivial case of a 2-window (recursion basis).
fn select_trivial<V, I>(mut window: I, k: usize) -> V
where
    I: Clone + ExactSizeIterator<Item = V>,
    V: Copy + Default + Sub<V, Output = V>,
{
    debug_assert_eq!(window.len(), 2);
    let item_1 = window.next().unwrap();
    let item_2 = window.next().unwrap();
    match k {
        1 => item_1 - item_2, // non-positive
        2 | 3 => V::default(),
        4 => item_2 - item_1, // non-negative
        _ => panic!("`k` should be in `1..=4` but it is `{k}`"),
    }
}

/// Select the Nth largest element from the `L` ([`PartialOrd`] adapter).
///
/// # Panics
///
/// The two elements in `L` cannot be ordered.
fn select_nth<V>(l: &mut [V], index: usize) -> V
where
    V: Copy + Debug + PartialOrd,
{
    debug_assert!(index < l.len(), "out of range: l = {l:?}, index = {index}");
    *l.select_nth_unstable_by(index, |lhs, rhs| {
        lhs.partial_cmp(rhs)
            .unwrap_or_else(|| panic!("`{lhs:?}` and `{rhs:?}` cannot be ordered"))
    })
    .1
}

#[cfg(test)]
mod tests {
    use super::*;

    /// # Matrix
    ///
    /// ```text
    /// 0 -1
    /// 1  0
    /// ```
    #[test]
    fn select_2x2_ok() {
        let window = [1, 2].into_iter();
        let statistics: Vec<_> = (1..=4).map(|k| select(window.clone(), k)).collect();
        assert_eq!(statistics, [-1, 0, 0, 1]);
    }

    /// # Matrix
    ///
    /// ```text
    /// 0, -1, -2
    /// 1,  0, -1
    /// 2,  1,  0
    /// ```
    #[test]
    fn select_3x3_ok() {
        let window = [1, 2, 3].into_iter();
        let statistics: Vec<_> = (1..=9).map(|k| select(window.clone(), k)).collect();
        assert_eq!(statistics, [-2, -1, -1, 0, 0, 0, 1, 1, 2]);
    }

    /// # Matrix
    ///
    /// ```text
    /// 0, -1, -2, -3
    /// 1,  0, -1, -2
    /// 2,  1,  0, -1
    /// 3,  2,  1,  0
    /// ```
    #[test]
    fn select_4x4_ok() {
        let window = [1, 2, 3, 4].into_iter();
        let statistics: Vec<_> = (1..=16).map(|k| select(window.clone(), k)).collect();
        assert_eq!(
            statistics,
            [-3, -2, -2, -1, -1, -1, 0, 0, 0, 0, 1, 1, 1, 2, 2, 3]
        );
    }

    /// # Matrix
    ///
    /// ```text
    /// 0, -1, -2, -3, -4
    /// 1,  0, -1, -2, -3
    /// 2,  1,  0, -1, -2
    /// 3,  2,  1,  0, -1
    /// 4,  3,  2,  1,  0
    /// ```
    #[test]
    fn select_5x5_ok() {
        let window = [1, 2, 3, 4, 5].into_iter();
        let statistics: Vec<_> = (1..=25).map(|k| select(window.clone(), k)).collect();
        assert_eq!(
            statistics,
            [
                -4, -3, -3, -2, -2, -2, -1, -1, -1, -1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 4
            ]
        );
    }
}
