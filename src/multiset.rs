use std::collections::{btree_map::Entry, BTreeMap};

/// You won't believe it, but <crates.io> lacks proper B-tree multi-sets.
/// B-tree implementations there are mostly focused on storing unique elements 😢
#[must_use]
struct Multiset<V> {
    len: usize,
    map: BTreeMap<V, usize>,
}

impl<V> Multiset<V> {
    pub const fn new() -> Self {
        Self {
            len: 0,
            map: BTreeMap::new(),
        }
    }
}

impl<V: Ord> Multiset<V> {
    pub fn push(&mut self, key: V) {
        self.len += 1;
        match self.map.entry(key) {
            Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
    }

    pub fn pop(&mut self, key: V) {
        self.len -= 1;
        if let Entry::Occupied(mut entry) = self.map.entry(key) {
            let counter = entry.get_mut();
            *counter -= 1;
            if *counter == 0 {
                entry.remove();
            }
        }
    }
}
