use crate::{Clear, Len, MapGet, MapInsert, MapMut, Retain};
use alloc::collections::BTreeMap;
use core::borrow::Borrow;

impl<K: Ord, V> Len for BTreeMap<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K: Ord, V> Clear for BTreeMap<K, V> {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }
}

impl<B: ?Sized + Ord, K: Borrow<B> + Ord, V> MapGet<B> for BTreeMap<K, V> {
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

impl<B: ?Sized + Ord, K: Borrow<B> + Ord, V> MapMut<B> for BTreeMap<K, V> {
    #[inline]
    fn get_mut(&mut self, key: &B) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }

    #[inline]
    fn remove(&mut self, key: &B) -> Option<Self::Value> {
        self.remove(key)
    }
}

impl<K: Ord, V> MapInsert for BTreeMap<K, V> {
    type Key = K;
    type Value = V;

    #[inline]
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.insert(key, value)
    }
}

impl<K: Ord, V> Retain for BTreeMap<K, V> {
    type Key = K;
    type Value = V;

    #[inline]
    fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
        self.retain(f);
    }
}
