use crate::{Clear, Len, MapGet, MapInsert, MapMut, Retain, VecMap};
use alloc::collections::BTreeMap;
use core::{borrow::Borrow, marker::PhantomData};
use genindex::{GenIndex, IndexPair};

static INVALID_INDEX: &str = "invalid index";

/// An associative array that uses [GenIndex] as keys to elements and stores data in a backing map.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct GenIndexMap<T, I = IndexPair, M = VecMap<(I, T)>> {
    map: M,
    marker: PhantomData<(I, T)>,
}

/// A [GenIndexMap] backed by a [VecMap].
pub type GenIndexVecMap<T, I = IndexPair> = GenIndexMap<T, I>;

/// A [GenIndexMap] backed by a [BTreeMap].
pub type GenIndexBTreeMap<T, I = IndexPair> =
    GenIndexMap<T, I, BTreeMap<<I as GenIndex>::Index, (I, T)>>;

#[cfg(feature = "std")]
/// A [GenIndexMap] backed by a [std::collections::HashMap].
pub type GenIndexHashMap<T, I = IndexPair> =
    GenIndexMap<T, I, std::collections::HashMap<<I as GenIndex>::Index, (I, T)>>;

/// IntoIterator for a [GenIndexMap].
type GenIndexMapIntoIter<T, I, M> =
    core::iter::Map<<M as IntoIterator>::IntoIter, fn(<M as IntoIterator>::Item) -> (I, T)>;

/// Iterator for a [GenIndexMap].
type GenIndexMapIter<'a, T, I, M> = core::iter::Map<
    <&'a M as IntoIterator>::IntoIter,
    fn(<&'a M as IntoIterator>::Item) -> (&'a I, &'a T),
>;

/// Mutable iterator for a [GenIndexMap].
type GenIndexMapIterMut<'a, T, I, M> = core::iter::Map<
    <&'a mut M as IntoIterator>::IntoIter,
    fn(<&'a mut M as IntoIterator>::Item) -> (&'a I, &'a mut T),
>;

impl<T, I, M> GenIndexMap<T, I, M> {
    /// Constructs a new, empty [GenIndexMap].
    ///
    /// # Examples
    /// ```rust
    /// # use collections::GenIndexMap;
    /// let map = GenIndexMap::<()>::new();
    /// ```
    #[inline]
    pub fn new() -> Self
    where
        M: Default,
    {
        Self {
            map: Default::default(),
            marker: PhantomData,
        }
    }

    /// Returns the number of elements in the map, also referred to as its ‘length’.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::GenIndexMap;
    /// # use genindex::IndexU64;
    /// let mut map = GenIndexMap::<i32, IndexU64>::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert(1.into(), 123);
    /// assert_eq!(map.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize
    where
        M: Len,
    {
        self.map.len()
    }

    /// Clears the map, removing all values.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::GenIndexMap;
    /// # use genindex::IndexU64;
    /// let mut map = GenIndexMap::<i32, IndexU64>::new();
    /// map.insert(1.into(), 123);
    /// map.clear();
    /// assert!(map.len() == 0);
    /// ```
    #[inline]
    pub fn clear(&mut self)
    where
        M: Clear,
    {
        self.map.clear()
    }

    /// Returns an iterator over this [GenIndexMap].
    ///
    /// # Examples
    /// ```rust
    /// # use collections::GenIndexMap;
    /// # use genindex::{GenIndex, IndexU64};
    /// let mut map = GenIndexMap::<u64, IndexU64>::new();
    /// for i in 1..10 {
    ///     map.insert(i.into(), i * i);
    /// }
    ///
    /// let mut count = 0;
    /// for (idx, value) in map.iter() {
    ///     assert_eq!((idx.index() * idx.index()) as u64, *value);
    ///     count += 1;
    /// }
    /// assert_eq!(count, 9);
    /// ```
    #[inline]
    pub fn iter<'a, K>(&'a self) -> GenIndexMapIter<'a, T, I, M>
    where
        &'a M: IntoIterator<Item = (K, &'a (I, T))>,
    {
        fn map<'a, K, I, T>((_, (i, t)): (K, &'a (I, T))) -> (&'a I, &'a T) {
            (i, t)
        }
        (&self.map).into_iter().map(map)
    }

    /// Returns an iterator that allows modifying each value over this [GenIndexMap].
    ///
    /// # Examples
    /// ```
    /// # use collections::GenIndexMap;
    /// # use genindex::{GenIndex, IndexU64};
    /// let mut map = GenIndexMap::<u64, IndexU64>::new();
    /// for i in 1..10 {
    ///     map.insert(i.into(), i * i);
    /// }
    ///
    /// let mut count = 0;
    /// for (_, value) in map.iter_mut() {
    ///     *value += 5;
    ///     count += 1;
    /// }
    /// assert_eq!(count, 9);
    /// ```
    #[inline]
    pub fn iter_mut<'a, K>(&'a mut self) -> GenIndexMapIterMut<'a, T, I, M>
    where
        &'a mut M: IntoIterator<Item = (K, &'a mut (I, T))>,
    {
        fn map<'a, K, I, T>((_, (i, t)): (K, &'a mut (I, T))) -> (&'a I, &'a mut T) {
            (i, t)
        }
        (&mut self.map).into_iter().map(map)
    }
}

impl<T, I: GenIndex, M> GenIndexMap<T, I, M> {
    /// Returns a reference to the value corresponding to the `key` if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::GenIndexMap;
    /// # use genindex::IndexU64;
    /// let mut map = GenIndexMap::<i32, IndexU64>::new();
    /// map.insert(1.into(), 123);
    /// assert_eq!(map.get(&1.into()), Some(&123));
    /// assert!(map.get(&2.into()).is_none());
    /// ```
    pub fn get<K>(&self, key: &I) -> Option<&T>
    where
        I::Index: TryInto<K>,
        M: MapGet<K, Value = (I, T)>,
        M::Key: Borrow<K>,
    {
        let (i, v) = self.map.get(&index_of(key)?)?;
        if i == key {
            Some(v)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value corresponding to the `key` if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::GenIndexMap;
    /// # use genindex::IndexU64;
    /// let mut map = GenIndexMap::<i32, IndexU64>::new();
    /// map.insert(1.into(), 123);
    /// *map.get_mut(&1.into()).unwrap() += 1;
    /// assert_eq!(map.get(&1.into()), Some(&124));
    /// assert!(map.get_mut(&2.into()).is_none());
    /// ```
    pub fn get_mut<K>(&mut self, key: &I) -> Option<&mut T>
    where
        I::Index: TryInto<K>,
        M: MapMut<K, Value = (I, T)>,
        M::Key: Borrow<K>,
    {
        let (i, v) = self.map.get_mut(&index_of(key)?)?;
        if i == key {
            Some(v)
        } else {
            None
        }
    }

    /// Removes and returns the element at `key` from the map if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::GenIndexMap;
    /// # use genindex::IndexU64;
    /// let mut map = GenIndexMap::<i32, IndexU64>::new();
    /// map.insert(1.into(), 123);
    /// assert_eq!(map.remove(&1.into()), Some(123));
    /// assert_eq!(map.remove(&1.into()), None);
    /// ```
    #[inline]
    pub fn remove<K>(&mut self, key: &I) -> Option<T>
    where
        I::Index: TryInto<K>,
        M: MapMut<K, Value = (I, T)>,
        M::Key: Borrow<K>,
    {
        if self.get(key).is_some() {
            Some(self.map.remove(&index_of(key)?)?.1)
        } else {
            None
        }
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(&index, &mut value)` returns `false`.
    ///
    /// # Examples
    /// ```
    /// # use collections::GenIndexMap;
    /// # use genindex::IndexU64;
    /// let mut map = GenIndexMap::<i32, IndexU64>::new();
    /// map.insert(1.into(), 1);
    /// map.insert(2.into(), 2);
    /// map.retain(|_, val| { if *val == 1 { *val = 3; true } else { false } });
    /// assert_eq!(map.get(&1.into()), Some(&3));
    /// assert!(map.get(&2.into()).is_none());
    /// ```
    #[inline]
    pub fn retain(&mut self, mut f: impl FnMut(&I, &mut T) -> bool)
    where
        M: Retain<Value = (I, T)>,
    {
        self.map.retain(|_, (i, t)| f(i, t))
    }

    /// Inserts `value` into the map. The existing key-value in the map is returned.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::GenIndexMap;
    /// # use genindex::IndexU64;
    /// let mut map = GenIndexMap::<i32, IndexU64>::new();
    /// assert!(map.insert(1.into(), 123).is_none());
    /// assert_eq!(map.insert((1, 1).into(), 456), Some(123));
    /// assert!(map.insert(2.into(), 123).is_none());
    /// assert_eq!(map.get(&(1, 1).into()), Some(&456));
    /// ```
    #[inline]
    pub fn insert(&mut self, key: I, value: T) -> Option<T>
    where
        M: MapInsert<Value = (I, T)>,
        I::Index: TryInto<M::Key>,
    {
        Some(
            self.map
                .insert(index_of(&key).expect(INVALID_INDEX), (key, value))?
                .1,
        )
    }
}

#[inline]
fn index_of<I: GenIndex, K>(i: &I) -> Option<K>
where
    I::Index: TryInto<K>,
{
    i.index().try_into().ok()
}

mod core_impl {
    use super::{GenIndexMap, INVALID_INDEX};
    use crate::{MapGet, MapInsert, MapMut};
    use core::ops::{Index, IndexMut};
    use genindex::GenIndex;

    impl<T, I: GenIndex, M> Extend<(I, T)> for GenIndexMap<T, I, M>
    where
        M: MapInsert<Value = (I, T)>,
        I::Index: TryInto<M::Key>,
    {
        fn extend<It: IntoIterator<Item = (I, T)>>(&mut self, iter: It) {
            for (i, v) in iter {
                self.insert(i, v);
            }
        }
    }

    impl<'a, T: Clone + 'a, I: GenIndex + 'a, M> Extend<(&'a I, &'a T)> for GenIndexMap<T, I, M>
    where
        M: MapInsert<Value = (I, T)>,
        I::Index: TryInto<M::Key>,
    {
        fn extend<It: IntoIterator<Item = (&'a I, &'a T)>>(&mut self, iter: It) {
            for (i, v) in iter {
                self.insert(*i, v.clone());
            }
        }
    }

    impl<T, I: GenIndex, M> FromIterator<(I, T)> for GenIndexMap<T, I, M>
    where
        M: Default + MapInsert<Value = (I, T)>,
        I::Index: TryInto<M::Key>,
    {
        fn from_iter<It: IntoIterator<Item = (I, T)>>(iter: It) -> Self {
            let iter = iter.into_iter();
            let mut map = GenIndexMap::new();
            map.extend(iter);
            map
        }
    }

    impl<'a, T: Clone + 'a, I: GenIndex + 'a, M> FromIterator<(&'a I, &'a T)> for GenIndexMap<T, I, M>
    where
        M: Default + MapInsert<Value = (I, T)>,
        I::Index: TryInto<M::Key>,
    {
        fn from_iter<It: IntoIterator<Item = (&'a I, &'a T)>>(iter: It) -> Self {
            let iter = iter.into_iter();
            let mut set = GenIndexMap::new();
            set.extend(iter);
            set
        }
    }

    impl<T, I: GenIndex, K, M> Index<I> for GenIndexMap<T, I, M>
    where
        M: MapGet<K, Key = K, Value = (I, T)>,
        I::Index: TryInto<K>,
    {
        type Output = T;

        fn index(&self, index: I) -> &Self::Output {
            self.get(&index).expect(INVALID_INDEX)
        }
    }

    impl<T, I: GenIndex, K, M> IndexMut<I> for GenIndexMap<T, I, M>
    where
        M: MapMut<K, Key = K, Value = (I, T)>,
        I::Index: TryInto<K>,
    {
        fn index_mut(&mut self, index: I) -> &mut Self::Output {
            self.get_mut(&index).expect(INVALID_INDEX)
        }
    }
}

mod collections_impl {
    use super::GenIndexMap;
    use crate::{Clear, Len, Map, MapGet, MapInsert, MapMut, Retain};
    use genindex::GenIndex;

    impl<T, I, M: Clear> Clear for GenIndexMap<T, I, M> {
        #[inline]
        fn clear(&mut self) {
            self.clear();
        }
    }

    impl<T, I, M: Len> Len for GenIndexMap<T, I, M> {
        #[inline]
        fn len(&self) -> usize {
            self.len()
        }
    }

    impl<T, I, M> Map for GenIndexMap<T, I, M> {
        type Key = I;
        type Value = T;
    }

    impl<T, I: GenIndex, M> MapGet<I> for GenIndexMap<T, I, M>
    where
        I::Index: TryInto<M::Key>,
        M: MapGet<<M as Map>::Key, Value = (I, T)>,
    {
        #[inline]
        fn get(&self, key: &I) -> Option<&Self::Value> {
            self.get(key)
        }
    }

    impl<T, I: GenIndex, M> MapMut<I> for GenIndexMap<T, I, M>
    where
        I::Index: TryInto<M::Key>,
        M: MapMut<<M as Map>::Key, Value = (I, T)>,
    {
        #[inline]
        fn get_mut(&mut self, key: &I) -> Option<&mut Self::Value> {
            self.get_mut(key)
        }

        #[inline]
        fn remove(&mut self, key: &I) -> Option<Self::Value> {
            self.remove(key)
        }
    }

    impl<T, I: GenIndex, M> MapInsert for GenIndexMap<T, I, M>
    where
        M: MapInsert<Value = (I, T)>,
        I::Index: TryInto<M::Key>,
    {
        #[inline]
        fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
            self.insert(key, value)
        }
    }

    impl<T, I: GenIndex, M: Retain<Value = (I, T)>> Retain for GenIndexMap<T, I, M> {
        type Key = I;
        type Value = T;

        #[inline]
        fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
            self.retain(f);
        }
    }
}

mod iter {
    use super::{GenIndexMap, GenIndexMapIntoIter, GenIndexMapIter, GenIndexMapIterMut};

    impl<T, I, K, M: IntoIterator<Item = (K, (I, T))>> IntoIterator for GenIndexMap<T, I, M> {
        type Item = (I, T);
        type IntoIter = GenIndexMapIntoIter<T, I, M>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            fn map<K, I, T>((_, (i, t)): (K, (I, T))) -> (I, T) {
                (i, t)
            }
            self.map.into_iter().map(map)
        }
    }

    impl<'a, T: 'a, I: 'a, K: 'a, M> IntoIterator for &'a GenIndexMap<T, I, M>
    where
        &'a M: IntoIterator<Item = (K, &'a (I, T))>,
    {
        type Item = (&'a I, &'a T);
        type IntoIter = GenIndexMapIter<'a, T, I, M>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T: 'a, I: 'a, K: 'a, M> IntoIterator for &'a mut GenIndexMap<T, I, M>
    where
        &'a mut M: IntoIterator<Item = (K, &'a mut (I, T))>,
    {
        type Item = (&'a I, &'a mut T);
        type IntoIter = GenIndexMapIterMut<'a, T, I, M>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::GenIndexMap;
    use core::marker::PhantomData;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T, I, M: Serialize> Serialize for GenIndexMap<T, I, M> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.map.serialize(serializer)
        }
    }

    impl<'de, T, I, M: Deserialize<'de>> Deserialize<'de> for GenIndexMap<T, I, M> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let map: M = Deserialize::deserialize(deserializer)?;
            Ok(Self {
                map,
                marker: PhantomData,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Clear, Len, MapGet, MapInsert, MapMut, Retain};

    use super::GenIndexMap;
    use genindex::{GenIndex, IndexU64};

    fn create_map() -> GenIndexMap<u32, IndexU64> {
        let mut map = GenIndexMap::new();
        for i in 0..10 {
            map.insert(IndexU64::from_index(i), i);
        }
        map
    }

    #[test]
    fn test_from_iter() {
        let map = create_map();

        let map2 = GenIndexMap::from_iter(map.iter());
        assert!(map == map2);

        let map2 = GenIndexMap::from_iter(map.clone().into_iter());
        assert!(map == map2);
    }

    #[test]
    fn test_iter() {
        let map = create_map();
        let mut i = 0;
        for (idx, value) in &map {
            assert_eq!(idx.index(), *value);
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
            assert_eq!(i, idx.index());
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
            assert_eq!(idx.index(), value);
            assert_eq!(i, value);
            i += 1;
        }
        assert_eq!(i, 10);
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
        let (&first, &value) = map.iter().next().unwrap();
        assert!(MapGet::contains_key(&map, &first));
        assert_eq!(MapGet::get(&map, &first), Some(&value));
        assert_eq!(MapGet::get(&map, &IndexU64::from_index(123)), None);
    }

    #[test]
    fn test_map_mut() {
        let mut map = create_map();
        let first = *map.iter().next().unwrap().0;

        let new_value = 1234;
        map[first] = new_value;
        assert_eq!(map[first], new_value);

        let new_value = 123;
        *MapMut::get_mut(&mut map, &first).unwrap() = new_value;
        assert_eq!(MapGet::get(&map, &first), Some(&new_value));

        assert_eq!(MapMut::remove(&mut map, &first), Some(new_value));
        assert_eq!(MapGet::get(&map, &first), None);
    }

    #[test]
    fn test_map_insert() {
        let mut map = create_map();
        let (&first, &value) = map.iter().next().unwrap();

        let new_value = 123;
        assert_eq!(MapInsert::insert(&mut map, first, new_value), Some(value));
        assert_eq!(MapGet::get(&map, &first), Some(&new_value));

        let unknown_idx = IndexU64::from_index(123);
        assert_eq!(MapInsert::insert(&mut map, unknown_idx, new_value), None);
        assert_eq!(MapGet::get(&map, &unknown_idx), Some(&new_value));
    }

    #[test]
    fn test_retain() {
        let mut map = create_map();
        let mut iter = map.iter();
        iter.next();
        let idx1 = *iter.next().unwrap().0;

        Retain::retain(&mut map, |_, val| {
            if *val == 1 {
                *val = 3;
                true
            } else {
                false
            }
        });
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&idx1), Some(&3));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_genindex_btreemap_serialize() {
        use super::GenIndexBTreeMap;
        use alloc::collections::BTreeMap;
        use genindex::{GenIndex, IndexPair};
        use serde_json::Value;

        let mut map = GenIndexBTreeMap::default();
        map.insert(<IndexPair>::from_raw_parts(1, 2), "a");
        map.insert(IndexPair::from_raw_parts(0, 3), "b");
        map.insert(IndexPair::from_raw_parts(4, 5), "c");

        let mut btree = BTreeMap::default();
        btree.insert(1, (<IndexPair>::from_raw_parts(1, 2), "a"));
        btree.insert(0, (IndexPair::from_raw_parts(0, 3), "b"));
        btree.insert(4, (IndexPair::from_raw_parts(4, 5), "c"));

        let expected_json: Value = serde_json::to_value(btree).unwrap();
        let json: Value = serde_json::to_value(map).unwrap();

        assert_eq!(json, expected_json);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_genindex_btreemap_deserialize() {
        use super::GenIndexBTreeMap;
        use alloc::collections::BTreeMap;
        use alloc::string::String;
        use genindex::{GenIndex, IndexPair};
        use serde_json::Value;

        let mut btree = BTreeMap::default();
        btree.insert(1usize, (<IndexPair>::from_raw_parts(1, 2), "a"));
        btree.insert(3, (IndexPair::from_raw_parts(3, 4), "c"));

        let json: Value = serde_json::to_value(btree).unwrap();

        let map: GenIndexBTreeMap<String> = serde_json::from_value(json).unwrap();

        assert_eq!(map.len(), 2);
        assert_eq!(map[IndexPair::from_raw_parts(1, 2)], "a");
        assert_eq!(map[IndexPair::from_raw_parts(3, 4)], "c");
    }

    #[cfg(all(feature = "serde", feature = "std"))]
    #[test]
    fn test_genindex_hashmap_serialize() {
        use super::GenIndexHashMap;
        use genindex::{GenIndex, IndexPair};
        use serde_json::Value;
        use std::collections::HashMap;

        let mut map = GenIndexHashMap::default();
        map.insert(<IndexPair>::from_raw_parts(1, 2), "a");
        map.insert(IndexPair::from_raw_parts(0, 3), "b");
        map.insert(IndexPair::from_raw_parts(4, 5), "c");

        let mut hashmap = HashMap::<i32, (IndexPair, &str)>::default();
        hashmap.insert(1, (<IndexPair>::from_raw_parts(1, 2), "a"));
        hashmap.insert(0, (IndexPair::from_raw_parts(0, 3), "b"));
        hashmap.insert(4, (IndexPair::from_raw_parts(4, 5), "c"));

        let expected_json: Value = serde_json::to_value(hashmap).unwrap();
        let json: Value = serde_json::to_value(map).unwrap();

        assert_eq!(json, expected_json);
    }

    #[cfg(all(feature = "serde", feature = "std"))]
    #[test]
    fn test_genindex_hashmap_deserialize() {
        use super::GenIndexHashMap;
        use alloc::string::String;
        use genindex::{GenIndex, IndexPair};
        use serde_json::Value;
        use std::collections::HashMap;

        let mut btree = HashMap::<usize, (IndexPair, &str)>::default();
        btree.insert(1usize, (<IndexPair>::from_raw_parts(1, 2), "a"));
        btree.insert(3, (IndexPair::from_raw_parts(3, 4), "c"));

        let json: Value = serde_json::to_value(btree).unwrap();

        let map: GenIndexHashMap<String, IndexPair> = serde_json::from_value(json).unwrap();

        assert_eq!(map.len(), 2);
        assert_eq!(map[IndexPair::from_raw_parts(1, 2)], "a");
        assert_eq!(map[IndexPair::from_raw_parts(3, 4)], "c");
    }
}
