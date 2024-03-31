pub struct SortedVec<T>(pub Vec<T>);

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
