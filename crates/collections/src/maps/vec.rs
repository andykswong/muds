use crate::{Clear, Len, MapGet, MapInsert, MapMut, Pop, Push, Reserve, Retain};
use alloc::vec::Vec;
use core::mem::replace;

impl<T> Len for Vec<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Clear for Vec<T> {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T> Push for Vec<T> {
    type Index = usize;
    type Value = T;

    #[inline]
    fn push(&mut self, value: Self::Value) -> Self::Index {
        self.push(value);
        self.len() - 1
    }
}

impl<T> Pop for Vec<T> {
    type Value = T;

    #[inline]
    fn pop(&mut self) -> Option<Self::Value> {
        self.pop()
    }
}

impl<T> MapGet<usize> for Vec<T> {
    type Key = usize;
    type Value = T;

    #[inline]
    fn get(&self, key: &usize) -> Option<&Self::Value> {
        self.as_slice().get(*key)
    }

    #[inline]
    fn contains_key(&self, key: &usize) -> bool {
        self.len() > *key
    }
}

impl<T> MapMut<usize> for Vec<T> {
    #[inline]
    fn get_mut(&mut self, key: &usize) -> Option<&mut Self::Value> {
        self.as_mut_slice().get_mut(*key)
    }

    /// Removes and returns the element at given index, shifting all elements after it to the left.
    #[inline]
    fn remove(&mut self, key: &usize) -> Option<Self::Value> {
        if self.contains_key(key) {
            Some(self.remove(*key))
        } else {
            None
        }
    }
}

impl<T: Default> MapInsert for Vec<T> {
    type Key = usize;
    type Value = T;

    /// Inserts or replaces an element at given index.
    #[inline]
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        if self.len() > key {
            Some(replace(&mut self[key], value))
        } else {
            self.reserve(key + 1 - self.len());
            self.resize_with(key, Default::default);
            self.push(value);
            None
        }
    }
}

impl<T> Reserve for Vec<T> {
    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }
}

impl<T> Retain for Vec<T> {
    type Key = usize;
    type Value = T;

    #[inline]
    fn retain(&mut self, mut f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
        let mut i: usize = 0;
        self.retain_mut(|value| {
            let result = f(&i, value);
            i += 1;
            result
        });
    }
}
