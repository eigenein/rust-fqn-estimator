use std::collections::VecDeque;

#[must_use]
pub struct Window<T>(VecDeque<T>);

impl<T> Window<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(VecDeque::with_capacity(capacity))
    }

    pub fn push(&mut self, item: T) -> Option<T> {
        let popped_item = if self.0.len() == self.0.capacity() {
            self.0.pop_front()
        } else {
            None
        };
        self.0.push_back(item);
        popped_item
    }
}
