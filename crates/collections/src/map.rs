//! Map traits.

use core::borrow::Borrow;

/// A key-value map.
pub trait Map {
    /// Key type
    type Key;

    /// Value type
    type Value;
}

/// Getter for a map.
pub trait MapGet<K: ?Sized>: Map
where
    Self::Key: Borrow<K>,
{
    /// Returns a reference to the value corresponding to the `key` if exists.
    fn get(&self, key: &K) -> Option<&Self::Value>;

    /// Returns `true` if the map contains a value for the `key`.
    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

/// Mutator for a map.
pub trait MapMut<K: ?Sized>: MapGet<K>
where
    Self::Key: Borrow<K>,
{
    /// Returns a mutable reference to the value corresponding to the `key` if exists.
    fn get_mut(&mut self, key: &K) -> Option<&mut Self::Value>;

    /// Removes and returns the element at `key` from the map if exists.
    fn remove(&mut self, key: &K) -> Option<Self::Value>;
}

/// Operation to insert into a map.
pub trait MapInsert: Map {
    /// Inserts `value` into the map. The existing value in the map is returned.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
}
