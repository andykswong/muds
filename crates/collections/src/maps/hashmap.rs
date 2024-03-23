use crate::{Clear, Len, MapGet, MapInsert, MapMut, Retain};
use core::{
    borrow::Borrow,
    hash::{BuildHasher, Hash},
};
use std::collections::HashMap;

impl<K, V, S> Len for HashMap<K, V, S> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V, S> Clear for HashMap<K, V, S> {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }
}

impl<B: ?Sized + Eq + Hash, K: Borrow<B> + Eq + Hash, V, S: BuildHasher> MapGet<B>
    for HashMap<K, V, S>
{
    type Key = K;
    type Value = V;

    #[inline]
    fn get(&self, key: &B) -> Option<&Self::Value> {
        self.get(key)
    }

    #[inline]
    fn contains_key(&self, key: &B) -> bool {
        self.contains_key(key)
    }
}

impl<B: ?Sized + Eq + Hash, K: Borrow<B> + Eq + Hash, V, S: BuildHasher> MapMut<B>
    for HashMap<K, V, S>
{
    #[inline]
    fn get_mut(&mut self, key: &B) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }

    #[inline]
    fn remove(&mut self, key: &B) -> Option<Self::Value> {
        self.remove(key)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> MapInsert for HashMap<K, V, S> {
    type Key = K;
    type Value = V;

    #[inline]
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.insert(key, value)
    }
}

impl<K, V, S> Retain for HashMap<K, V, S> {
    type Key = K;
    type Value = V;

    #[inline]
    fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
        self.retain(f);
    }
}
