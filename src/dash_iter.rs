/// A-dash array iterator of the specified step.
///
/// Step is a multiple of `2` and corresponds to the [`crate::bi_select::bi_select`] call depth.
#[derive(Copy, Clone)]
pub struct DashIter<V, I> {
    inner: I,
    step: usize,
    last: Option<V>,
}

impl<V, I> DashIter<V, I> {
    pub const fn new(inner: I, step: usize) -> Self {
        Self {
            inner,
            step,
            last: None,
        }
    }
}

impl<V, I: Iterator<Item = V>> Iterator for DashIter<V, I> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(next) => {
                // Remember the last indexed element before the full step, we may need it later:
                self.last = self.inner.by_ref().take(self.step - 1).last();
                Some(next)
            }

            // There is no next element. Return the last one, if any.
            None => self.last.take(),
        }
    }
}

impl<V, I: ExactSizeIterator<Item = V>> ExactSizeIterator for DashIter<V, I> {
    fn len(&self) -> usize {
        let count = self.inner.len();
        assert_ne!(count, 0, "the inner length should not be zero");

        // First element is always included:
        1 + (
            // Count subsequent elements every `step`:
            count - 1
            // Round up to the closest integer to account for the possible last element:
            + self.step - 1
        ) / self.step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the corner case with just one element.
    #[test]
    fn dash_iter_one_ok() {
        let dash_iter = DashIter::new([1].into_iter(), 2);
        assert_eq!(dash_iter.len(), 1);
        assert_eq!(dash_iter.collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    fn dash_iter_multiple_ok() {
        let mut window = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        // 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
        // ^   ^   ^   ^   ^    ^     ^     ^  ^
        let dash_iter = DashIter::new(window.into_iter(), 2);
        window = dash_iter.clone().collect();
        assert_eq!(dash_iter.len(), window.len());
        assert_eq!(window, vec![1, 3, 5, 7, 9, 11, 13, 15, 16]);

        // 1 3 5 7 9 11 13 15 16
        // ^     ^      ^     ^
        let dash_iter = DashIter::new(window.into_iter(), 3);
        window = dash_iter.clone().collect();
        assert_eq!(dash_iter.len(), window.len());
        assert_eq!(window, vec![1, 7, 13, 16]);

        // 1 7 13 16
        // ^      ^
        let dash_iter = DashIter::new(window.into_iter(), 4);
        window = dash_iter.clone().collect();
        assert_eq!(dash_iter.len(), window.len());
        assert_eq!(window, vec![1, 16]);

        // 1 16
        // ^ ^
        let dash_iter = DashIter::new(window.into_iter(), 2);
        window = dash_iter.clone().collect();
        assert_eq!(dash_iter.len(), window.len());
        assert_eq!(window, vec![1, 16]);
    }

    #[test]
    fn dash_iter_non_multiple_ok() {
        let mut window = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        // 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
        // ^     ^     ^     ^        ^        ^
        let dash_iter = DashIter::new(window.into_iter(), 3);
        window = dash_iter.clone().collect();
        assert_eq!(dash_iter.len(), window.len());
        assert_eq!(window, vec![1, 4, 7, 10, 13, 16]);

        // 1 4 7 10 13 16
        // ^           ^
        let dash_iter = DashIter::new(window.into_iter(), 100);
        window = dash_iter.clone().collect();
        assert_eq!(dash_iter.len(), window.len());
        assert_eq!(window, vec![1, 16]);
    }
}
