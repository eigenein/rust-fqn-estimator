use std::ops::Sub;

/// Build the list denoted by `L` in the papers
/// (the function itself is also known as `pickL()`).
///
/// The most important thing here is to get it done in `O(n)`.
pub fn pick_list<V, I>(window: I, min: V, max: V, into_buffer: &mut Vec<V>)
where
    I: Clone + Iterator<Item = V>,
    V: Copy + PartialOrd + Sub<V, Output = V>,
{
    into_buffer.clear();

    // This is tracking our maximum pointer.
    let mut max_column_iter = window.clone().peekable();

    for lhs in window {
        // Update the maximum pointer: move right until a strictly smaller element is found:
        while max_column_iter.next_if(|rhs| lhs - *rhs >= max).is_some() {}

        // Okay, now we're holding at our first actual element (you still holding your beer? 🍺)
        // Let's clone the iterator and push everything till the specified minimum.
        let min_column_iter = max_column_iter
            .clone()
            .map(|rhs| lhs - rhs)
            .take_while(|rhs| *rhs > min);
        into_buffer.extend(min_column_iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// # Matrix
    ///
    /// ```text
    /// 0, -1, -2
    /// 1,  0, -1
    /// 2,  1,  0
    /// ```
    #[test]
    fn pick_list_3x3_ok() {
        let window = [1, 2, 3].into_iter();
        let mut buffer = Vec::new();

        pick_list(window.clone(), -1, 1, &mut buffer);
        assert_eq!(buffer, [0, 0, 0]);

        pick_list(window.clone(), -2, 2, &mut buffer);
        assert_eq!(buffer, [0, -1, 1, 0, -1, 1, 0]);

        pick_list(window, -3, 3, &mut buffer);
        assert_eq!(buffer, [0, -1, -2, 1, 0, -1, 2, 1, 0]);
    }

    #[test]
    fn pick_list_2x2_zero_ok() {
        let mut buffer = Vec::new();

        // Zero matrix:
        pick_list([1, 1].into_iter(), -1, 1, &mut buffer);
        assert_eq!(buffer, [0, 0, 0, 0]);

        // Zero matrix, corner case:
        pick_list([1, 1].into_iter(), 0, 0, &mut buffer);
        assert_eq!(buffer, []);
    }
}
