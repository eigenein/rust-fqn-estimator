use std::ops::Sub;

use either::Either;

/// Lazy re-implementation of `pickL` from the reference implementation.
pub struct LazyList<V, I>(Either<(I, V, V), Vec<V>>);

impl<V, I> LazyList<V, I> {
    pub const fn new(window: I, min: V, max: V) -> Self {
        Self(Either::Left((window, min, max)))
    }
}

impl<V, I> LazyList<V, I>
where
    I: Clone + Iterator<Item = V>,
    V: Copy + PartialOrd + Sub<V, Output = V>,
{
    /// Build the list denoted by `L` in the papers
    /// (the function itself is also known as `pickL()`).
    ///
    /// The most important thing here is to get it done in `O(n)`.
    ///
    /// # Developer's notes
    ///
    /// The biggest problem here is the allocation of the vector that we return.
    pub fn build(&mut self) -> &mut [V] {
        if let Either::Left((window, min, max)) = &mut self.0 {
            let mut l = Vec::new();

            // This is tracking our maximum pointer.
            let mut max_column_iter = window.clone().peekable();

            for lhs in window {
                // Update the maximum pointer: move right until a strictly smaller element is found:
                while max_column_iter.next_if(|rhs| lhs - *rhs >= *max).is_some() {}

                // Okay, now we're holding at our first actual element (you still holding your beer? 🍺)
                // Let's clone the iterator and push everything till the specified minimum.
                let min_column_iter = max_column_iter
                    .clone()
                    .map(|rhs| lhs - rhs)
                    .take_while(|rhs| rhs > min);
                l.extend(min_column_iter);
            }

            self.0 = Either::Right(l);
        }

        self.0.as_mut().right_or_else(|_| unreachable!())
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
    fn lazy_list_3x3_ok() {
        let window = [1, 2, 3].into_iter();

        let mut lazy_l = LazyList::new(window.clone(), -1, 1);
        assert_eq!(lazy_l.build(), [0, 0, 0]);

        let mut lazy_l = LazyList::new(window.clone(), -2, 2);
        assert_eq!(lazy_l.build(), [0, -1, 1, 0, -1, 1, 0]);

        let mut lazy_l = LazyList::new(window, -3, 3);
        assert_eq!(lazy_l.build(), [0, -1, -2, 1, 0, -1, 2, 1, 0]);
    }

    #[test]
    fn lazy_list_2x2_zero_ok() {
        // Zero matrix:
        let mut lazy_l = LazyList::new([1, 1].into_iter(), -1, 1);
        assert_eq!(lazy_l.build(), [0, 0, 0, 0]);

        // Zero matrix, corner case:
        let mut lazy_l = LazyList::new([1, 1].into_iter(), 0, 0);
        assert_eq!(lazy_l.build(), []);
    }
}
