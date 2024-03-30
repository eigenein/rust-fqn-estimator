use std::mem;

/// B-tree multiset (a B-tree set that allows duplicate elements).
///
/// I haven't found a suitable existing implementation, so… helaas pindakaas.
///
/// # Generic parameters
///
/// - `K`: key type
/// - `B`: tree's degree, defaults to `6` in consistency with [`std::collections::BTreeSet`].
#[must_use]
pub struct Multiset<K, const B: usize = 6>(
    /// Root node.
    Node<K, B>,
);

impl<K, const B: usize> Default for Multiset<K, B> {
    fn default() -> Self {
        Self(Node::default())
    }
}

impl<K: Copy + Ord, const B: usize> Multiset<K, B> {
    pub fn insert(&mut self, key: K) {
        if self.0.is_full() {
            // Split the root first:
            let (median, sibling) = self.0.split_off();

            // The former root becomes a child of the new root:;
            self.0 = Node {
                keys: vec![median],
                children: vec![mem::take(&mut self.0), sibling],
            };
        }

        self.0.insert(key);
    }
}

#[must_use]
struct Node<K, const B: usize> {
    keys: Vec<K>,
    children: Vec<Self>,
}

impl<K, const B: usize> Default for Node<K, B> {
    fn default() -> Self {
        Self {
            keys: Vec::with_capacity(2 * B - 1),
            children: Vec::with_capacity(2 * B),
        }
    }
}

impl<K: Copy + Ord, const B: usize> Node<K, B> {
    /// Split the node.
    ///
    /// # Returns
    ///
    /// The new median key and sibling.
    fn split_off(&mut self) -> (K, Self) {
        // The sibling will take half of my keys and children:
        let sibling = Self {
            keys: self.keys.split_off(B),
            children: self.children.split_off(B),
        };

        // Return the median and the newly created sibling, so that we could link them to the parent:
        let median_key = self.keys.pop().unwrap();

        (median_key, sibling)
    }

    #[must_use]
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    #[must_use]
    fn is_full(&self) -> bool {
        self.keys.len() == 2 * B - 1
    }

    /// Insert the key to the node.
    fn insert(&mut self, key: K) {
        // Find out the position into which we need to insert the key:
        let index = self
            .keys
            .iter()
            .position(|k| *k >= key)
            .unwrap_or(self.keys.len());

        if self.is_leaf() {
            // When I'm a leaf, just insert the key (the parent ensured that I'm able to accommodate it):
            debug_assert!(!self.is_full(), "should not insert to a full leaf");
            self.keys.insert(index, key);
        } else {
            // When I'm not a leaf, I need to delegate the insertion to the corresponding child.
            let mut index = index;

            if self.children[index].is_full() {
                // But if the child is already full, I need to split it first:
                let (median, sibling) = self.children[index].split_off();

                // Its median value becomes the key between this child and its new sibling:
                self.keys.insert(index, median);

                // The sibling goes to the right of the child:
                self.children.insert(index + 1, sibling);

                // We may need to delegate the insertion to the sibling because of the new separator key:
                if key >= median {
                    index += 1;
                }
            }

            // Finally, delegate insertion to the respective child:
            self.children[index].insert(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_ok() {
        let mut set = Multiset::<_, 1>::default();

        set.insert(42);
        assert_eq!(set.0.keys, [42]);
        assert!(set.0.children.is_empty());

        set.insert(43);
        assert_eq!(set.0.keys, [42, 43]);
        assert!(set.0.children.is_empty());
    }
}
