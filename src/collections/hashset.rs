use super::hashmap::HashMap;
use crate::hash::Hash;

// A simple fixed-capacity hashset using open addressing.
pub struct HashSet<K: Hash + PartialEq> {
    map: HashMap<K, (), ()>, // using a unit type for values and a placeholder hasher type
}

impl<K: Hash + PartialEq> HashSet<K> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }
    // Forwarding common hashmap methods.
    pub fn insert(&mut self, key: K) -> bool {
        match self.map.insert(key, ()) {
            None => true,
            Some(_) => false,
        }
    }
    pub fn contains(&self, key: &K) -> bool {
        self.map.get(key).is_some()
    }
    pub fn remove(&mut self, key: &K) -> bool {
        self.map.remove(key).is_some()
    }
    pub fn clear(&mut self) {
        self.map.clear()
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    // ...other methods mirroring those in HashMap...
}
