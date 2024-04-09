use alloc::vec::Vec;
use core::{cmp::Ordering, mem::replace};
use genindex::{GenIndex, IndexPair};

static INVALID_INDEX: &str = "invalid index";

/// [SparseSet] is a type of associative array that uses a dense and a sparse vector to map keys to elements.
#[derive(Clone, Debug, Default, Eq)]
pub struct SparseSet<T, I = IndexPair> {
    sparse: Vec<usize>,
    entries: Vec<(I, T)>,
}

/// Iterator for a [SparseSet].
type SparseSetIter<'a, T, I> =
    core::iter::Map<core::slice::Iter<'a, (I, T)>, fn(&(I, T)) -> (&I, &T)>;

/// Mutable iterator for a [SparseSet].
type SparseSetIterMut<'a, T, I> =
    core::iter::Map<core::slice::IterMut<'a, (I, T)>, fn(&mut (I, T)) -> (&I, &mut T)>;

impl<T, I> SparseSet<T, I> {
    /// Create a new empty [SparseSet].
    ///
    /// # Examples
    /// ```rust
    /// # use collections::SparseSet;
    /// let map = SparseSet::<()>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            entries: Vec::new(),
        }
    }

    /// Returns the number of elements this [SparseSet] can hold without additional allocation.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::SparseSet;
    /// let mut map = SparseSet::<()>::new();
    /// assert_eq!(map.capacity(), 0);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    /// Returns the number of elements in this [SparseSet].
    ///
    /// # Examples
    /// ```rust
    /// # use collections::SparseSet;
    /// let mut map = SparseSet::<()>::new();
    /// assert_eq!(map.len(), 0);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Clears the [SparseSet], removing all values.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::SparseSet;
    /// # let mut map = SparseSet::<()>::new();
    /// map.clear();
    /// assert_eq!(map.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.sparse.clear();
        self.entries.clear();
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in this [SparseSet].
    /// The collection may reserve more space to avoid frequent reallocations. After calling reserve, capacity
    /// will be greater than or equal to self.len() + additional. Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::SparseSet;
    /// let mut map = SparseSet::<()>::new();
    /// map.reserve(10);
    /// assert!(map.capacity() >= 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.entries.reserve(additional);
        let min_sparse = self.entries.len() + additional;
        if min_sparse > self.sparse.len() {
            self.sparse.reserve(min_sparse - self.sparse.len());
        }
    }

    /// Returns an iterator over this [SparseSet].
    ///
    /// # Examples
    /// ```rust
    /// # use collections::SparseSet;
    /// # use genindex::{GenIndex, IndexU64};
    /// let mut set = SparseSet::<u64, IndexU64>::new();
    /// for i in 1..10 {
    ///     set.insert(i.into(), i * i);
    /// }
    ///
    /// let mut count = 0;
    /// for (idx, value) in set.iter() {
    ///     assert_eq!((idx.index() * idx.index()) as u64, *value);
    ///     count += 1;
    /// }
    /// assert_eq!(count, 9);
    #[inline]
    pub fn iter(&self) -> SparseSetIter<'_, T, I> {
        fn map<I, T>((i, t): &(I, T)) -> (&I, &T) {
            (i, t)
        }
        self.entries.iter().map(map)
    }

    /// Returns an iterator that allows modifying each value over this [SparseSet].
    ///
    /// # Examples
    /// ```
    /// # use collections::SparseSet;
    /// # use genindex::{GenIndex, IndexU64};
    /// let mut set = SparseSet::<u64, IndexU64>::new();
    /// for i in 1..10 {
    ///     set.insert(i.into(), i * i);
    /// }
    ///
    /// let mut count = 0;
    /// for (_, value) in set.iter_mut() {
    ///     *value += 5;
    ///     count += 1;
    /// }
    /// assert_eq!(count, 9);
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> SparseSetIterMut<'_, T, I> {
        fn map_mut<I, T>((i, t): &mut (I, T)) -> (&I, &mut T) {
            (&*i, t)
        }
        self.entries.iter_mut().map(map_mut)
    }
}

impl<T, I: GenIndex> SparseSet<T, I>
where
    I::Index: TryInto<usize>,
{
    /// Returns a reference to the value corresponding to the index `i` .
    ///
    /// # Examples
    /// ```rust
    /// # use collections::SparseSet;
    /// # use genindex::IndexU64;
    /// let mut set = SparseSet::<i32, IndexU64>::new();
    /// let idx = 1.into();
    /// set.insert(idx, 123);
    ///
    /// assert_eq!(set.get(&idx), Some(&123));
    /// assert!(set.get(&2.into()).is_none());
    /// ```
    pub fn get(&self, i: &I) -> Option<&T> {
        let dense_index = self.get_sparse_dense_indices(i)?.1?;
        let (index, value) = self.entries.get(dense_index)?;
        if i == index {
            Some(value)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value corresponding to the index `i` .
    ///
    /// # Examples
    /// ```rust
    /// # use collections::SparseSet;
    /// # use genindex::IndexU64;
    /// let mut set = SparseSet::<i32, IndexU64>::new();
    /// let idx = 1.into();
    /// set.insert(idx, 123);
    ///
    /// *set.get_mut(&idx).unwrap() += 1;
    /// assert_eq!(set.get(&idx), Some(&124));
    /// ```
    pub fn get_mut(&mut self, i: &I) -> Option<&mut T> {
        let dense_index = self.get_sparse_dense_indices(i)?.1?;
        let (index, value) = self.entries.get_mut(dense_index)?;
        if i == index {
            Some(value)
        } else {
            None
        }
    }

    /// Inserts `value` into the set, allocating more capacity if necessary.
    /// The existing key-value in the set is returned.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::SparseSet;
    /// # use genindex::IndexU64;
    /// let mut set = SparseSet::<i32, IndexU64>::new();
    /// let idx = 1.into();
    /// assert!(set.insert(idx, 123).is_none());
    /// assert_eq!(set.insert(idx, 456), Some(123));
    /// assert_eq!(set.get(&idx), Some(&456));
    /// ```
    pub fn insert(&mut self, i: I, v: T) -> Option<T> {
        let (sparse_index, dense_index) = self.get_sparse_dense_indices(&i)?;

        if let Some((index, value)) =
            dense_index.and_then(|dense_index| self.entries.get_mut(dense_index))
        {
            if i.index() == index.index() {
                return Some(replace(value, v));
            }
        }

        self.reserve_sparse_index(sparse_index);
        *self.sparse.get_mut(sparse_index)? = self.entries.len();
        self.entries.push((i, v));
        None
    }

    /// Removes and returns the element at index `i` from the set if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::SparseSet;
    /// # use genindex::IndexU64;
    /// let mut set = SparseSet::<i32, IndexU64>::new();
    /// set.insert(1.into(), 123);
    /// set.insert(3.into(), 456);
    ///
    /// assert_eq!(set.remove(&2.into()), None);
    /// assert_eq!(set.remove(&1.into()), Some(123));
    /// assert_eq!(set.remove(&1.into()), None);
    /// ```
    pub fn remove(&mut self, i: &I) -> Option<T> {
        let (sparse_index, dense_index) = self.get_sparse_dense_indices(i)?;
        let dense_index = dense_index?;
        if self.entries.get(dense_index)?.0 != *i {
            return None;
        }

        if dense_index < self.entries.len() - 1 {
            let swapped_index = self.entries.last()?.0.index().try_into().ok()?;
            *self.sparse.get_mut(swapped_index)? = dense_index;
        }
        *self.sparse.get_mut(sparse_index)? = usize::MAX;

        Some(self.entries.swap_remove(dense_index).1)
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(index, value)` returns `false`.
    ///
    /// # Examples
    /// ```
    /// # use collections::SparseSet;
    /// # use genindex::IndexU64;
    /// let mut set = SparseSet::<i32, IndexU64>::new();
    /// set.insert(1.into(), 1);
    /// set.insert(2.into(), 2);
    /// set.retain(|_, val| { if *val == 1 { *val = 3; true } else { false } });
    /// assert_eq!(*set.get(&1.into()).unwrap(), 3);
    /// assert!(set.get(&2.into()).is_none());
    /// ```
    pub fn retain(&mut self, mut f: impl FnMut(&I, &mut T) -> bool) {
        let mut i = 0;
        while i < self.entries.len() {
            let (index, ref mut value) = self.entries[i];
            if !f(&index, value) {
                // Item is swap-removed. Do not increment i here so that we can process the swapped item next
                self.remove(&index);
            } else {
                i += 1;
            }
        }
    }

    /// Sorts the set data with a comparator function.
    /// This sort is stable (i.e., does not reorder equal elements) and O(n * log(n)) worst-case.
    ///
    /// # Examples
    /// ```
    /// # use collections::SparseSet;
    /// # use genindex::IndexU64;
    /// let mut set = SparseSet::<i32, IndexU64>::new();
    /// let (idx1, idx2, idx3) = (0.into(), 5.into(), 1.into());
    /// set.insert(idx1, 2);
    /// set.insert(idx2, 3);
    /// set.insert(idx3, 1);
    /// set.sort_by(|(_, v1), (_, v2)| v1.cmp(v2));
    ///
    /// assert_eq!(set.get(&idx1), Some(&2));
    /// assert_eq!(set.get(&idx2), Some(&3));
    /// assert_eq!(set.get(&idx3), Some(&1));
    ///
    /// let mut iter = set.iter();
    /// let (_, value) = iter.next().unwrap();
    /// assert_eq!(*value, 1);
    /// let (_, value) = iter.next().unwrap();
    /// assert_eq!(*value, 2);
    /// let (_, value) = iter.next().unwrap();
    /// assert_eq!(*value, 3);
    /// ```
    pub fn sort_by(&mut self, mut compare: impl FnMut((&I, &T), (&I, &T)) -> Ordering) {
        self.entries
            .sort_by(|lhs: &(I, T), rhs: &(I, T)| compare((&lhs.0, &lhs.1), (&rhs.0, &rhs.1)));

        // Fix sparse array
        for (item_index, (i, _)) in self.entries.iter().enumerate() {
            if let Some(sparse_entry) = i
                .index()
                .try_into()
                .ok()
                .and_then(|sparse_index: usize| self.sparse.get_mut(sparse_index))
            {
                *sparse_entry = item_index;
            }
        }
    }

    fn get_sparse_dense_indices(&self, i: &I) -> Option<(usize, Option<usize>)> {
        let sparse_index = i.index().try_into().ok()?;
        Some((
            sparse_index,
            self.sparse
                .get(sparse_index)
                .map(|dense_index| *dense_index),
        ))
    }

    fn reserve_sparse_index(&mut self, index: usize) {
        if index >= self.sparse.len() {
            let additional = index - self.sparse.len() + 1;
            self.sparse.reserve(additional);
            unsafe { self.sparse.set_len(self.sparse.capacity()) }
        }
    }
}

mod iter {
    use super::{SparseSet, SparseSetIter, SparseSetIterMut};
    use genindex::GenIndex;

    impl<T, I: GenIndex> IntoIterator for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        type Item = (I, T);
        type IntoIter = alloc::vec::IntoIter<(I, T)>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.entries.into_iter()
        }
    }

    impl<'a, T, I: GenIndex> IntoIterator for &'a SparseSet<T, I> {
        type Item = (&'a I, &'a T);
        type IntoIter = SparseSetIter<'a, T, I>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T, I: GenIndex> IntoIterator for &'a mut SparseSet<T, I> {
        type Item = (&'a I, &'a mut T);
        type IntoIter = SparseSetIterMut<'a, T, I>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }
}

mod core_impl {
    use super::{SparseSet, INVALID_INDEX};
    use core::{
        hash::{Hash, Hasher},
        ops::{Index, IndexMut},
    };
    use genindex::GenIndex;

    impl<T, I: GenIndex> Extend<(I, T)> for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        fn extend<It: IntoIterator<Item = (I, T)>>(&mut self, iter: It) {
            for (i, v) in iter {
                self.insert(i, v);
            }
        }
    }

    impl<'a, T: Clone + 'a, I: GenIndex + 'a> Extend<(&'a I, &'a T)> for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        fn extend<It: IntoIterator<Item = (&'a I, &'a T)>>(&mut self, iter: It) {
            for (i, v) in iter {
                self.insert(*i, v.clone());
            }
        }
    }

    impl<T, I: GenIndex, const N: usize> From<[(I, T); N]> for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        fn from(values: [(I, T); N]) -> Self {
            let mut set = SparseSet::new();
            set.reserve(N);
            set.extend(values.into_iter());
            set
        }
    }

    impl<T, I: GenIndex> FromIterator<(I, T)> for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        fn from_iter<It: IntoIterator<Item = (I, T)>>(iter: It) -> Self {
            let iter = iter.into_iter();
            let mut set = SparseSet::new();
            let (lower, upper) = iter.size_hint();
            set.reserve(upper.unwrap_or(lower));
            set.extend(iter);
            set
        }
    }

    impl<'a, T: Clone + 'a, I: GenIndex + 'a> FromIterator<(&'a I, &'a T)> for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        fn from_iter<It: IntoIterator<Item = (&'a I, &'a T)>>(iter: It) -> Self {
            let iter = iter.into_iter();
            let mut set = SparseSet::new();
            let (lower, upper) = iter.size_hint();
            set.reserve(upper.unwrap_or(lower));
            set.extend(iter);
            set
        }
    }

    impl<T: Hash, I: Hash> Hash for SparseSet<T, I> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.entries.hash(state);
        }
    }

    impl<T, I: GenIndex> Index<I> for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        type Output = T;

        fn index(&self, index: I) -> &Self::Output {
            self.get(&index).expect(INVALID_INDEX)
        }
    }

    impl<T, I: GenIndex> IndexMut<I> for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        fn index_mut(&mut self, index: I) -> &mut Self::Output {
            self.get_mut(&index).expect(INVALID_INDEX)
        }
    }

    impl<T: PartialEq, I: PartialEq> PartialEq for SparseSet<T, I> {
        fn eq(&self, other: &Self) -> bool {
            self.entries == other.entries
        }
    }
}

mod collections_impl {
    use super::SparseSet;
    use crate::{Clear, Len, MapGet, MapInsert, MapMut, Reserve, Retain};
    use core::mem::replace;
    use genindex::GenIndex;

    impl<T, I> Clear for SparseSet<T, I> {
        #[inline]
        fn clear(&mut self) {
            self.clear();
        }
    }

    impl<T, I> Len for SparseSet<T, I> {
        #[inline]
        fn len(&self) -> usize {
            self.len()
        }
    }

    impl<T, I> Reserve for SparseSet<T, I> {
        #[inline]
        fn reserve(&mut self, additional: usize) {
            self.reserve(additional);
        }
    }

    impl<T, I: GenIndex> MapGet<I> for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        type Key = I;
        type Value = T;

        #[inline]
        fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
            self.get(key)
        }
    }

    impl<T, I: GenIndex> MapMut<I> for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        #[inline]
        fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
            self.get_mut(key)
        }

        #[inline]
        fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
            self.remove(key)
        }
    }

    impl<T, I: GenIndex> MapInsert for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        type Key = I;
        type Value = T;

        #[inline]
        fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
            let dest = self.get_mut(&key)?;
            Some(replace(dest, value))
        }
    }

    impl<T, I: GenIndex> Retain for SparseSet<T, I>
    where
        I::Index: TryInto<usize>,
    {
        type Key = I;
        type Value = T;

        #[inline]
        fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
            self.retain(f);
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::SparseSet;
    use alloc::vec::Vec;
    use genindex::GenIndex;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T, I> Serialize for SparseSet<T, I>
    where
        T: Serialize,
        I: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.entries.serialize(serializer)
        }
    }

    impl<'de, T, I> Deserialize<'de> for SparseSet<T, I>
    where
        T: Deserialize<'de>,
        I: Deserialize<'de> + GenIndex,
        I::Index: TryInto<usize>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let entries: Vec<(I, T)> = Deserialize::deserialize(deserializer)?;
            let iter_entries = || {
                entries
                    .iter()
                    .map(|(i, _)| i.index().try_into().ok().expect(super::INVALID_INDEX))
            };

            let mut sparse = Vec::new();
            sparse.reserve(iter_entries().max().unwrap_or(0));
            unsafe { sparse.set_len(sparse.capacity()) }
            for (i, sparse_index) in iter_entries().enumerate() {
                unsafe { *sparse.get_unchecked_mut(sparse_index) = i };
            }

            Ok(SparseSet { entries, sparse })
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        use super::SparseSet;
        use alloc::vec;
        use genindex::IndexU64;
        use serde_json::{json, Value};

        let mut set = SparseSet::<&str, IndexU64>::new();
        set.insert(1.into(), "a");
        set.insert(0.into(), "b");
        set.insert(4.into(), "c");

        let expected_json: Value = json!([[1, "a"], [0, "b"], [4, "c"]]);

        let json: Value = serde_json::to_value(set).unwrap();

        assert_eq!(json, expected_json);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        use super::SparseSet;
        use alloc::{string::String, vec};
        use genindex::IndexU64;
        use serde_json::{json, Value};

        let json: Value = json!([[1, "a"], [3, "c"]]);

        let set: SparseSet<String, IndexU64> = serde_json::from_value(json).unwrap();

        assert_eq!(set.len(), 2);
        assert_eq!(set.get(&1.into()), Some(&"a".into()));
        assert_eq!(set.get(&3.into()), Some(&"c".into()));
    }
}
