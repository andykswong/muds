use alloc::vec::Vec;
use core::mem::replace;

/// An associative array that uses a [Vec] of [Option]s to map usize keys to elements.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecMap<T> {
    items: Vec<Option<T>>,
    len: usize,
}

/// IntoIterator for a [VecMap].
type VecMapIntoIter<T> = core::iter::FilterMap<
    core::iter::Enumerate<alloc::vec::IntoIter<Option<T>>>,
    fn((usize, Option<T>)) -> Option<(usize, T)>,
>;

/// Iterator for a [VecMap].
type VecMapIter<'a, T> = core::iter::FilterMap<
    core::iter::Enumerate<core::slice::Iter<'a, Option<T>>>,
    fn((usize, &Option<T>)) -> Option<(usize, &T)>,
>;

/// Mutable iterator for a [VecMap].
type VecMapIterMut<'a, T> = core::iter::FilterMap<
    core::iter::Enumerate<core::slice::IterMut<'a, Option<T>>>,
    fn((usize, &mut Option<T>)) -> Option<(usize, &mut T)>,
>;

impl<T> VecMap<T> {
    /// Constructs a new, empty [VecMap].
    /// It will not allocate until elements are pushed onto it.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::VecMap;
    /// let map = VecMap::<()>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            len: 0,
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::VecMap;
    /// let mut map = VecMap::<()>::new();
    /// assert_eq!(map.capacity(), 0);
    /// map.reserve(10);
    /// assert!(map.capacity() >= 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Returns the number of elements in the map, also referred to as its ‘length’.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::VecMap;
    /// let mut map = VecMap::<()>::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert(1, ());
    /// assert_eq!(map.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Clears the map, removing all values.
    /// Note that this method has no effect on the allocated capacity of the map.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::VecMap;
    /// let mut map = VecMap::<()>::new();
    /// map.insert(1, ());
    /// map.clear();
    /// assert_eq!(map.len(), 0);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.items.clear();
        self.len = 0;
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in this map.
    /// The collection may reserve more space to avoid frequent reallocations. After calling reserve, capacity
    /// will be greater than or equal to self.len() + additional. Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::VecMap;
    /// let mut map = VecMap::<()>::new();
    /// map.reserve(10);
    /// assert!(map.capacity() >= 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional);
    }

    /// Returns a reference to the value corresponding to the index `i` .
    ///
    /// # Examples
    /// ```rust
    /// # use collections::VecMap;
    /// let mut map = VecMap::<i32>::new();
    /// let idx = 2;
    /// map.insert(idx, 123);
    /// assert_eq!(map.get(idx), Some(&123));
    /// assert!(map.get(3).is_none());
    /// ```
    #[inline]
    pub fn get(&self, i: usize) -> Option<&T> {
        self.items.get(i)?.as_ref()
    }

    /// Returns a mutable reference to the value corresponding to the index `i` .
    ///
    /// # Examples
    /// ```rust
    /// # use collections::VecMap;
    /// let mut map = VecMap::<i32>::new();
    /// let idx = 1;
    /// map.insert(idx, 123);
    /// *map.get_mut(idx).unwrap() += 1;
    /// assert_eq!(map.get(idx), Some(&124));
    /// ```
    #[inline]
    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        self.items.get_mut(i)?.as_mut()
    }

    /// Inserts `value` into the map, allocating more capacity if necessary.
    /// The existing key-value in the map is returned.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::VecMap;
    /// let mut map = VecMap::<i32>::new();
    /// let idx = 1;
    /// assert!(map.insert(idx, 123).is_none());
    /// assert_eq!(map.insert(idx, 456), Some(123));
    /// assert!(map.insert(0, 123).is_none());
    /// assert_eq!(map.get(idx), Some(&456));
    /// ```
    pub fn insert(&mut self, i: usize, v: T) -> Option<T> {
        self.len += 1;
        match self.items.get_mut(i) {
            Some(Some(old_value)) => Some(replace(old_value, v)),
            _ => {
                if i >= self.items.len() {
                    self.items.resize_with(i + 1, || None);
                }
                *self.items.get_mut(i)? = Some(v);
                None
            }
        }
    }

    /// Removes and returns the element at index `i` from the map if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::VecMap;
    /// let mut map = VecMap::<i32>::new();
    /// map.insert(1, 123);
    /// assert_eq!(map.remove(1), Some(123));
    /// assert_eq!(map.remove(1), None);
    /// ```
    pub fn remove(&mut self, i: usize) -> Option<T> {
        let item = self.items.get_mut(i)?;
        if item.is_some() {
            self.len -= 1;
            item.take()
        } else {
            None
        }
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(index, &value)` returns `false`.
    ///
    /// # Examples
    /// ```
    /// # use collections::VecMap;
    /// let mut map = VecMap::<i32>::new();
    /// map.insert(1, 1);
    /// map.insert(0, 2);
    /// map.retain(|_, val| { if *val == 1 { *val = 3; true } else { false } });
    /// assert_eq!(map.get(1), Some(&3));
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn retain(&mut self, mut f: impl FnMut(usize, &mut T) -> bool) {
        for (i, item) in self.items.iter_mut().enumerate() {
            if item.as_mut().is_some_and(|v| !f(i, v)) {
                *item = None;
                self.len -= 1;
            }
        }
    }

    /// Returns an iterator over this map.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::VecMap;
    /// let mut map = VecMap::<usize>::new();
    /// for i in 0..10 {
    ///     map.insert(i * 2, i * 2);
    /// }
    ///
    /// let mut count = 0;
    /// for (i, value) in map.iter() {
    ///     assert_eq!(i, count * 2);
    ///     count += 1;
    /// }
    /// assert_eq!(count, 10);
    #[inline]
    pub fn iter(&self) -> VecMapIter<'_, T> {
        fn map<T>((i, t): (usize, &Option<T>)) -> Option<(usize, &T)> {
            Some((i, t.as_ref()?))
        }
        self.items.iter().enumerate().filter_map(map)
    }

    /// Returns an iterator that allows modifying each value over this map.
    ///
    /// # Examples
    /// ```
    /// # use collections::VecMap;
    /// let mut map = VecMap::<usize>::new();
    /// for i in 0..10 {
    ///     map.insert(i * 2, i * 2);
    /// }
    ///
    /// let mut count = 0;
    /// for (i, value) in map.iter_mut() {
    ///     *value += 1;
    ///     assert_eq!(i, count * 2);
    ///     assert_eq!(*value, count * 2 + 1);
    ///     count += 1;
    /// }
    /// assert_eq!(count, 10);
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> VecMapIterMut<'_, T> {
        fn map_mut<T>((i, t): (usize, &mut Option<T>)) -> Option<(usize, &mut T)> {
            Some((i, t.as_mut()?))
        }
        self.items.iter_mut().enumerate().filter_map(map_mut)
    }
}

mod iter {
    use super::{VecMap, VecMapIntoIter, VecMapIter, VecMapIterMut};

    impl<T> IntoIterator for VecMap<T> {
        type Item = (usize, T);
        type IntoIter = VecMapIntoIter<T>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            fn map<T>((i, t): (usize, Option<T>)) -> Option<(usize, T)> {
                Some((i, t?))
            }
            self.items.into_iter().enumerate().filter_map(map)
        }
    }

    impl<'a, T> IntoIterator for &'a VecMap<T> {
        type Item = (usize, &'a T);
        type IntoIter = VecMapIter<'a, T>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T> IntoIterator for &'a mut VecMap<T> {
        type Item = (usize, &'a mut T);
        type IntoIter = VecMapIterMut<'a, T>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }
}

mod core_impl {
    use super::VecMap;
    use core::{
        borrow::Borrow,
        ops::{Index, IndexMut},
    };

    impl<T> Extend<(usize, T)> for VecMap<T> {
        fn extend<It: IntoIterator<Item = (usize, T)>>(&mut self, iter: It) {
            for (i, v) in iter {
                self.insert(i, v);
            }
        }
    }

    impl<'a, T: Clone + 'a, I: Borrow<usize> + 'a> Extend<(I, &'a T)> for VecMap<T> {
        fn extend<It: IntoIterator<Item = (I, &'a T)>>(&mut self, iter: It) {
            for (i, v) in iter {
                self.insert(*i.borrow(), v.clone());
            }
        }
    }

    impl<T> FromIterator<(usize, T)> for VecMap<T> {
        fn from_iter<It: IntoIterator<Item = (usize, T)>>(iter: It) -> Self {
            let iter = iter.into_iter();
            let mut set = VecMap::new();
            let (lower, upper) = iter.size_hint();
            set.reserve(upper.unwrap_or(lower));
            set.extend(iter);
            set
        }
    }

    impl<'a, T: Clone + 'a, I: Borrow<usize> + 'a> FromIterator<(I, &'a T)> for VecMap<T> {
        fn from_iter<It: IntoIterator<Item = (I, &'a T)>>(iter: It) -> Self {
            let iter = iter.into_iter();
            let mut set = VecMap::new();
            let (lower, upper) = iter.size_hint();
            set.reserve(upper.unwrap_or(lower));
            set.extend(iter);
            set
        }
    }

    impl<T> Index<usize> for VecMap<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            self.items[index].as_ref().unwrap()
        }
    }

    impl<T> IndexMut<usize> for VecMap<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.items[index].as_mut().unwrap()
        }
    }
}

mod collections_impl {
    use super::VecMap;
    use crate::{Clear, Len, Map, MapGet, MapInsert, MapMut, Retain};

    impl<T> Clear for VecMap<T> {
        #[inline]
        fn clear(&mut self) {
            self.clear();
        }
    }

    impl<T> Len for VecMap<T> {
        #[inline]
        fn len(&self) -> usize {
            self.len()
        }
    }

    impl<T> Map for VecMap<T> {
        type Key = usize;
        type Value = T;
    }

    impl<T> MapGet<usize> for VecMap<T> {
        #[inline]
        fn get(&self, key: &usize) -> Option<&Self::Value> {
            self.get(*key)
        }
    }

    impl<T> MapMut<usize> for VecMap<T> {
        #[inline]
        fn get_mut(&mut self, key: &usize) -> Option<&mut Self::Value> {
            self.get_mut(*key)
        }

        #[inline]
        fn remove(&mut self, key: &usize) -> Option<Self::Value> {
            self.remove(*key)
        }
    }

    impl<T> MapInsert for VecMap<T> {
        #[inline]
        fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
            self.insert(key, value)
        }
    }

    impl<T> Retain for VecMap<T> {
        type Key = usize;
        type Value = T;

        #[inline]
        fn retain(&mut self, mut f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
            self.retain(|k, v| f(&k, v));
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::VecMap;
    use alloc::vec::Vec;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T: Serialize> Serialize for VecMap<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.items.serialize(serializer)
        }
    }

    impl<'de, T: Deserialize<'de>> Deserialize<'de> for VecMap<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let items: Vec<Option<T>> = Deserialize::deserialize(deserializer)?;
            let len = items.iter().filter(|item| item.is_some()).count();
            Ok(VecMap { items, len })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VecMap;
    use crate::{Clear, Len, MapGet, MapInsert, MapMut, Retain};

    fn create_map() -> VecMap<usize> {
        let mut map = VecMap::new();
        for i in 0..10 {
            map.insert(i * 2, i);
        }
        map
    }

    #[test]
    fn test_clear_len() {
        let mut map = create_map();
        assert_eq!(Len::len(&map), 10);
        Clear::clear(&mut map);
        assert!(Len::is_empty(&map));
    }

    #[test]
    fn test_map_get() {
        let map = create_map();
        assert!(MapGet::contains_key(&map, &2));
        assert_eq!(MapGet::get(&map, &4), Some(&2));
        assert_eq!(MapGet::get(&map, &1), None);
    }

    #[test]
    fn test_map_mut() {
        let mut map = create_map();

        let new_value = 1234;
        map[2] = new_value;
        assert_eq!(map[2], new_value);

        let new_value = 123;
        *MapMut::get_mut(&mut map, &2).unwrap() = new_value;
        assert_eq!(MapGet::get(&map, &2), Some(&new_value));

        assert_eq!(MapMut::remove(&mut map, &2), Some(new_value));
        assert_eq!(MapGet::get(&map, &2), None);
    }

    #[test]
    fn test_map_insert() {
        let mut map = create_map();

        let new_value = 123;
        assert_eq!(MapInsert::insert(&mut map, 2, new_value), Some(1));
        assert_eq!(MapGet::get(&map, &2), Some(&new_value));

        assert_eq!(MapInsert::insert(&mut map, 111, new_value), None);
        assert_eq!(MapGet::get(&map, &111), Some(&new_value));
    }

    #[test]
    fn test_retain() {
        let mut map = create_map();
        Retain::retain(&mut map, |_, val| {
            if *val == 1 {
                *val = 3;
                true
            } else {
                false
            }
        });
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(2), Some(&3));
    }

    #[test]
    fn test_from_iter() {
        let map = create_map();

        let map2 = VecMap::from_iter(map.iter());
        assert!(map == map2);

        let map2 = VecMap::from_iter(map.clone().into_iter());
        assert!(map == map2);
    }

    #[test]
    fn test_iter() {
        let map = create_map();
        let mut i = 0;
        for (idx, value) in &map {
            assert_eq!(i * 2, idx);
            assert_eq!(i, *value);
            i += 1;
        }
        assert_eq!(i, 10);
    }

    #[test]
    fn test_iter_mut() {
        let mut map = create_map();
        let mut i = 0;
        for (idx, value) in &mut map {
            *value += 1;
            assert_eq!(i * 2, idx);
            assert_eq!(i + 1, *value);
            i += 1;
        }
        assert_eq!(i, 10);
    }

    #[test]
    fn test_into_iter() {
        let map = create_map();
        let mut i = 0;
        for (idx, value) in map {
            assert_eq!(i * 2, idx);
            assert_eq!(i, value);
            i += 1;
        }
        assert_eq!(i, 10);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        use alloc::vec;
        use serde_json::{json, Value};

        let mut map = VecMap::new();
        map.insert(1, "a");
        map.insert(3, "c");
        let expected_json = json!([null, "a", null, "c"]);
        let json: Value = serde_json::to_value(map).unwrap();
        assert_eq!(json, expected_json);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        use alloc::{string::String, vec};
        use serde_json::{json, Value};

        let json: Value = json!([null, "a", "b", null, null]);
        let set: VecMap<String> = serde_json::from_value(json).unwrap();

        assert_eq!(set.len(), 2);
        assert_eq!(set.get(1), Some(&"a".into()));
        assert_eq!(set.get(2), Some(&"b".into()));
    }
}
