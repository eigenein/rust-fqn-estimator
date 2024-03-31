use crate::RawMedian;

#[must_use]
pub struct SortedVec<T>(pub Vec<T>);

impl<T: Copy> SortedVec<T> {
    /// Get the vector median.
    pub fn median(&self) -> Option<RawMedian<T>> {
        if self.0.is_empty() {
            None
        } else if self.0.len() & 1 == 1 {
            Some(RawMedian::Odd(self.0[self.0.len() / 2]))
        } else {
            let i = self.0.len() / 2;
            Some(RawMedian::Even(self.0[i - 1], self.0[i]))
        }
    }
}

impl<T: PartialOrd> SortedVec<T> {
    pub fn insert_sorted(&mut self, value: T) {
        let index = self.0.partition_point(|x| x < &value);
        self.0.insert(index, value);
    }

    pub fn remove_value(&mut self, value: &T) -> Option<T> {
        self.0
            .iter()
            .rposition(|existing_value| existing_value == value)
            .map(|index| self.0.remove(index))
    }
}

#[cfg(test)]
mod tests {
    use crate::{sorted_vec::SortedVec, RawMedian};

    #[test]
    fn empty_ok() {
        assert_eq!(SortedVec::<()>(vec![]).median(), None);
    }

    #[test]
    fn odd_ok() {
        assert_eq!(
            SortedVec(vec![1, 2, 3, 4, 5]).median(),
            Some(RawMedian::Odd(3))
        );
    }

    #[test]
    fn even_ok() {
        assert_eq!(
            SortedVec(vec![1, 2, 3, 4, 5, 6]).median(),
            Some(RawMedian::Even(3, 4))
        );
    }
}
