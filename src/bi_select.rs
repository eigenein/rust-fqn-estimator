use std::{fmt::Debug, ops::Sub};

use crate::{dash_iter::DashIter, lazy_l::LazyL, rank::n_greater};

fn select<V, I>(window: I, k: usize) -> V
where
    V: Copy + Debug + PartialOrd + Sub<V, Output = V>,
    I: Clone + ExactSizeIterator<Item = V>,
{
    bi_select::<V, I>(window, k, k).0
}

/// # Returns
///
/// Tuple of the `k1`-th and `k2`-th elements of the matrix derived from `window` and `negated` window.
///
/// P.S. Abandon hope all ye who enter here 💀
fn bi_select<V, I>(mut window: I, k1: usize, k2: usize) -> (V, V)
where
    V: Copy + Debug + PartialOrd + Sub<V, Output = V>,
    I: Clone + ExactSizeIterator<Item = V>,
{
    let n = window.len();

    if n <= 2 {
        return (window.clone().nth(k1).unwrap(), window.nth(k2).unwrap());
    }

    // Define k1-dash and k2-dash from the papers:
    let k1 = if k1 & 1 == 1 {
        // Original paper mentions `ceil(k1 / 4)`, which is simply `floor((k1 + 3) / 4)`
        // (just consider all the possible remainders of 4).
        n + 1 + (k1 + 3) / 4
    } else {
        // `ceil(1 / 4 * (k1 + 2n + 1))` which is `floor(1 / 4 * (k1 + 2n + 1 + 3))` which is
        // `floor(1 / 4 * (k1 + 2n)) + 1`.
        (2 * n + k1) / 4 + 1
    };

    // Surprisingly, here they used the same very trick they did NOT use for `k1`. Okay 🤔
    let k2 = (k2 + 3) / 4;

    // Bi-select in the A-dash matrix and rank the candidates:
    let (min_candidate, max_candidate) = bi_select(DashIter::new(window.clone(), 2), k1, k2);
    let rank_min = n * n - n_greater::<V, I>(window.clone(), min_candidate);
    let rank_max = n_greater::<V, I>(window.clone(), max_candidate);

    // We may not need the `L`, hence the lazy wrapper.
    let mut lazy_l = LazyL::new(window, min_candidate, max_candidate);

    #[allow(clippy::suspicious_operation_groupings)]
    (
        if rank_min < k1 {
            min_candidate
        } else if k1 + rank_max <= n * n {
            max_candidate
        } else {
            select_nth(lazy_l.build(), k1 + rank_max - n * n)
        },
        if rank_min < k2 {
            min_candidate
        } else if k2 + rank_max <= n * n {
            max_candidate
        } else {
            select_nth(lazy_l.build(), k2 + rank_max - n * n)
        },
    )
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
    *l.select_nth_unstable_by(index, |lhs, rhs| {
        lhs.partial_cmp(rhs)
            .unwrap_or_else(|| panic!("`{lhs:?}` and `{rhs:?}` cannot be ordered"))
    })
    .1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn select_3x3_ok() {
        // Matrix:
        // 0, -1, -2
        // 1,  0, -1
        // 2,  1,  0
        let window = [1, 2, 3].into_iter();
        assert_eq!(select(window.clone(), 0), -2);
        assert_eq!(select(window.clone(), 1), -1);
        assert_eq!(select(window.clone(), 2), -1);
        assert_eq!(select(window.clone(), 3), 0);
        assert_eq!(select(window.clone(), 4), 0);
        assert_eq!(select(window.clone(), 5), 0);
        assert_eq!(select(window.clone(), 6), 1);
        assert_eq!(select(window.clone(), 7), 1);
        assert_eq!(select(window.clone(), 8), 2);
    }
}
