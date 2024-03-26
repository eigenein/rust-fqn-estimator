use std::ops::Sub;

/// # Returns
///
/// Tuple of the `k1`-th and `k2`-th elements of the matrix derived from `window` and `negated` window.
///
/// P.S. Abandon hope all ye who enter here 💀
fn biselect<V>(window: &[V], k1: usize, k2: usize) -> (&V, &V) {
    let n = window.len();

    if n <= 2 {
        return (&window[k1], &window[k2]);
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

    let (a, b) = biselect::<V>(todo!(), k1, k2);
    todo!()
}

/// Return list of the matrix's values denoted by `L` in the papers.
///
/// The most important thing here is to get it done in `O(n)`.
///
/// # Developer's notes
///
/// - I know, 1-letter names are bad, but I seriously do not know how to name this function.
///
/// - I am pretty sure this could be done without additional temporary vector, but I'm too
///   silly to come up with a good solution. However, I try to avoid additional allocations by
///   reusing the vector.
fn l<'v, V, I>(window: I, min: &'v V, max: &'v V, out: &mut Vec<V>)
where
    V: PartialOrd<V>,
    &'v V: Sub<&'v V, Output = V>,
    I: Clone + ExactSizeIterator<Item = &'v V>,
{
    out.clear();

    // This is tracking our maximum pointer.
    let mut max_column_iter = window.clone().peekable();

    for lhs in window {
        // Update the maximum pointer: move right until a strictly smaller element is found:
        while max_column_iter.next_if(|rhs| &(lhs - rhs) >= max).is_some() {}

        // Okay, now we're holding at our first actual element (you still holding your beer? 🍺)
        // Let's clone and push everything till the specified minimum.
        out.extend(max_column_iter.clone().map(|rhs| lhs - rhs).take_while(|rhs| rhs > &min));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l_ok() {
        let mut buffer = Vec::new();

        // Matrix:
        // 0, -1, -2
        // 1,  0, -1
        // 2,  1,  0
        l([1, 2, 3].iter(), &-2, &2, &mut buffer);
        assert_eq!(buffer, [0, -1, 1, 0, -1, 1, 0]);

        // Zero matrix:
        l([1, 1].iter(), &-1, &1, &mut buffer);
        assert_eq!(buffer, [0, 0, 0, 0]);

        // Zero matrix, corner case:
        l([1, 1].iter(), &0, &0, &mut buffer);
        assert_eq!(buffer, []);
    }
}
