/// A-dash array iterator: iterates over even-indexed elements **and** the last one – if the total
/// number of items is even. For that purpose, it keeps track of the last odd-indexed element.
///
/// Unfortunately, [`std::iter::Chain`] does not implement [`ExactSizeIterator`]. Bruh 😐
#[derive(Copy, Clone)]
pub struct DashIter<V, I>(I, Option<V>);

impl<V, I> DashIter<V, I> {
    pub const fn new(inner: I) -> Self {
        Self(inner, None)
    }
}

impl<V, I: Iterator<Item = V>> Iterator for DashIter<V, I> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(next) => {
                // Remember the next odd-indexed element, we may need it later.
                self.1 = self.0.next();
                Some(next)
            }

            // There is no next even-indexed element. Return the last odd-indexed one, if any.
            None => self.1.take(),
        }
    }
}

impl<V, I: ExactSizeIterator<Item = V>> ExactSizeIterator for DashIter<V, I> {
    fn len(&self) -> usize {
        self.0.len() / 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dash_iter_even_ok() {
        let dash_iter = DashIter::new([1, 2, 3, 4].into_iter());
        assert_eq!(dash_iter.len(), 3);
        assert_eq!(dash_iter.collect::<Vec<_>>(), vec![1, 3, 4]);
    }

    #[test]
    fn dash_iter_odd_ok() {
        let dash_iter = DashIter::new([1, 2, 3, 4, 5, 6, 7].into_iter());
        assert_eq!(dash_iter.len(), 4);
        assert_eq!(dash_iter.collect::<Vec<_>>(), vec![1, 3, 5, 7]);
    }
}
