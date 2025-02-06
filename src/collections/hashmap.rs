// Stub definitions for various types.
pub struct DefaultHasher; // default hasher type
pub struct RandomState; // default state for HashMap

pub struct HashMap<K, V, S = RandomState> {
    // ...internal representation...
    _marker: core::marker::PhantomData<(K, V, S)>,
}

impl<K, V, S> HashMap<K, V, S> {
    // Constructors.
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn with_capacity(cap: usize) -> Self {
        unimplemented!()
    }
    pub fn with_hasher(hasher: S) -> Self {
        unimplemented!()
    }
    pub fn with_capacity_and_hasher(cap: usize, hasher: S) -> Self {
        unimplemented!()
    }

    // Capacity and state methods.
    pub fn capacity(&self) -> usize {
        unimplemented!()
    }
    pub fn len(&self) -> usize {
        unimplemented!()
    }
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }
    pub fn reserve(&mut self, additional: usize) {
        unimplemented!()
    }
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), ()> {
        unimplemented!()
    }
    pub fn shrink_to(&mut self, min_capacity: usize) {
        unimplemented!()
    }
    pub fn shrink_to_fit(&mut self) {
        unimplemented!()
    }
    pub fn clear(&mut self) {
        unimplemented!()
    }

    // Entry API.
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        unimplemented!()
    }
    pub fn raw_entry(&self) -> () {
        unimplemented!()
    }
    pub fn raw_entry_mut(&mut self) -> () {
        unimplemented!()
    }

    // Accessors.
    pub fn get(&self, key: &K) -> Option<&V> {
        unimplemented!()
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        unimplemented!()
    }
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        unimplemented!()
    }
    pub fn hasher(&self) -> &S {
        unimplemented!()
    }

    // Iterators.
    pub fn iter(&self) -> Iter<K, V> {
        unimplemented!()
    }
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        unimplemented!()
    }
    pub fn keys(&self) -> Keys<K, V> {
        unimplemented!()
    }
    pub fn values(&self) -> Values<K, V> {
        unimplemented!()
    }
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        unimplemented!()
    }
    pub fn into_keys(self) -> IntoKeys<K, V> {
        unimplemented!()
    }
    pub fn into_values(self) -> IntoValues<K, V> {
        unimplemented!()
    }
    pub fn into_iter(self) -> IntoIter<K, V> {
        unimplemented!()
    }

    // Modifiers.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        unimplemented!()
    }
    pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, ()> {
        unimplemented!()
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        unimplemented!()
    }
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        unimplemented!()
    }
    pub fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, f: F) {
        unimplemented!()
    }
    pub fn extract_if<F: FnMut(&K, &V) -> bool>(&mut self, f: F) -> () {
        unimplemented!()
    }
    pub fn drain(&mut self) -> Drain<K, V> {
        unimplemented!()
    }
}

// Entry enum for use with the entry API.
pub enum Entry<K, V> {
    Occupied(OccupiedEntry<K, V>),
    Vacant(VacantEntry<K, V>),
}

// Associated entry views.
pub struct OccupiedEntry<'a, K, V> {
    _marker: core::marker::PhantomData<(&'a K, &'a mut V)>,
}
pub struct VacantEntry<'a, K, V> {
    _marker: core::marker::PhantomData<&'a mut K>,
}

// Iterators.
pub struct Iter<'a, K, V> {
    _marker: core::marker::PhantomData<(&'a K, &'a V)>,
}
pub struct IterMut<'a, K, V> {
    _marker: core::marker::PhantomData<(&'a K, &'a mut V)>,
}
pub struct Keys<'a, K, V> {
    _marker: core::marker::PhantomData<(&'a K, &'a V)>,
}
pub struct Values<'a, K, V> {
    _marker: core::marker::PhantomData<&'a V>,
}
pub struct ValuesMut<'a, K, V> {
    _marker: core::marker::PhantomData<&'a mut V>,
}

pub struct IntoIter<K, V> {
    _marker: core::marker::PhantomData<(K, V)>,
}
pub struct IntoKeys<K, V> {
    _marker: core::marker::PhantomData<K>,
}
pub struct IntoValues<K, V> {
    _marker: core::marker::PhantomData<V>,
}
