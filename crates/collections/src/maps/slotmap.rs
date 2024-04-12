use alloc::{boxed::Box, vec::Vec};
use core::{array, mem::MaybeUninit};
use genindex::{GenIndex, IndexPair};

static INVALID_INDEX: &str = "invalid index";

/// Paged generational index slot map.
pub struct PagedSlotMap<T, I = IndexPair, const N: usize = 64> {
    indices: Vec<I>,
    values: Vec<Box<[MaybeUninit<T>; N]>>,
    free_list_head: usize,
    free_list_tail: usize,
    free_list_size: usize,
}

impl<T, I, const N: usize> PagedSlotMap<T, I, N> {
    /// Create a new empty [PagedSlotMap].
    ///
    /// # Examples
    /// ```rust
    /// # use collections::PagedSlotMap;
    /// let map = PagedSlotMap::<()>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            values: Vec::new(),
            free_list_head: 0,
            free_list_tail: 0,
            free_list_size: 0,
        }
    }

    /// Returns the number of elements this map can hold without additional allocation.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::PagedSlotMap;
    /// let mut map = PagedSlotMap::<()>::new();
    /// assert_eq!(map.capacity(), 0);
    /// map.push(());
    /// assert!(map.capacity() > 0);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.indices.capacity()
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::PagedSlotMap;
    /// let mut map = PagedSlotMap::<()>::new();
    /// assert_eq!(map.len(), 0);
    /// map.push(());
    /// assert_eq!(map.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.indices.len() - self.free_list_size
    }

    /// Clears the map, removing all values.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::PagedSlotMap;
    /// # let mut map = PagedSlotMap::<()>::new();
    /// map.push(());
    /// map.push(());
    /// map.clear();
    /// assert_eq!(map.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.indices.clear();
        self.values.clear();
        self.free_list_head = 0;
        self.free_list_tail = 0;
        self.free_list_size = 0;
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
    /// # use collections::PagedSlotMap;
    /// let mut map = PagedSlotMap::<()>::new();
    /// map.reserve(10);
    /// assert!(map.capacity() == 64);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        if additional > self.free_list_size {
            let additional_pages = (additional - self.free_list_size).div_ceil(N);
            self.indices.reserve(additional_pages * N);
            self.values.reserve(additional_pages);
        }
    }
}

impl<T, I: GenIndex, const N: usize> PagedSlotMap<T, I, N>
where
    I::Index: TryInto<usize>,
{
    /// Returns a reference to the value at `key`.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::PagedSlotMap;
    /// let mut map = PagedSlotMap::<i32>::new();
    /// let idx = map.push(123);
    ///
    /// assert_eq!(map.get(&idx), Some(&123));
    /// map.remove(&idx);
    /// assert!(map.get(&idx).is_none());
    /// ```
    pub fn get(&self, key: &I) -> Option<&T> {
        let idx = key.index().try_into().ok()?;
        if *self.indices.get(idx)? == *key {
            Some(unsafe { get_value_unchecked(&self.values, idx).assume_init_ref() })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value at `key`.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::PagedSlotMap;
    /// let mut map = PagedSlotMap::<i32>::new();
    /// let idx = map.push(123);
    ///
    /// *map.get_mut(&idx).unwrap() += 1;
    /// assert_eq!(map.remove(&idx), Some(124));
    /// assert!(map.get_mut(&idx).is_none());
    /// ```
    pub fn get_mut(&mut self, key: &I) -> Option<&mut T> {
        let idx = key.index().try_into().ok()?;
        if *self.indices.get(idx)? == *key {
            Some(unsafe { get_value_unchecked_mut(&mut self.values, idx).assume_init_mut() })
        } else {
            None
        }
    }

    /// Returns an iterator over the map.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::PagedSlotMap;
    /// # use genindex::GenIndex;
    /// let mut map = PagedSlotMap::<u32>::new();
    /// for i in 0..10 {
    ///     map.push(i);
    /// }
    ///
    /// for (idx, value) in map.iter() {
    ///     assert_eq!(idx.index(), *value);
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> iter::Iter<T, I, N> {
        iter::Iter {
            index: (&self.indices).into_iter(),
            values: &self.values,
            start: 0,
            end: self.len(),
        }
    }

    /// Returns an iterator that allows modifying each value over this map.
    ///
    /// # Examples
    /// ```
    /// # use collections::PagedSlotMap;
    /// # use genindex::GenIndex;
    /// let mut map = PagedSlotMap::<u32>::new();
    /// for i in 0..10 {
    ///     map.push(i);
    /// }
    ///
    /// for (idx, value) in map.iter_mut() {
    ///     *value += 5;
    ///     assert_eq!(idx.index() + 5, *value);
    /// }
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> iter::IterMut<T, I, N> {
        iter::IterMut {
            start: 0,
            end: self.len(),
            index: (&self.indices).into_iter(),
            values: &mut self.values,
        }
    }
}

impl<T, I: GenIndex, const N: usize> PagedSlotMap<T, I, N>
where
    I::Index: TryFrom<usize> + TryInto<usize>,
{
    /// Pushes `value` into the map, allocating more capacity if necessary.
    /// The `value`'s assigned index in the map is returned.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::PagedSlotMap;
    /// let mut map = PagedSlotMap::<i32>::new();
    /// let idx = map.push(123);
    /// assert_eq!(map.get(&idx), Some(&123));
    /// ```
    pub fn push(&mut self, value: T) -> I {
        let (idx, index) = if self.free_list_size == 0 {
            let idx = self.indices.len();
            if self.values.len() * N <= idx {
                self.values.push(new_page());
            }
            let index = I::from_index(into_index(idx));
            self.indices.push(index);
            (idx, index)
        } else {
            let idx = self.free_list_head;
            let index = unsafe { self.indices.get_unchecked_mut(idx) };
            self.free_list_head = into_usize(index.index());
            self.free_list_size -= 1;
            *index = I::from_raw_parts(into_index(idx), index.next_generation().generation());
            (idx, *index)
        };
        unsafe { get_value_unchecked_mut(&mut self.values, idx) }.write(value);
        index
    }

    /// Removes and returns the element at `key` from the map if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use collections::PagedSlotMap;
    /// let mut map = PagedSlotMap::<i32>::new();
    /// let i = map.push(123);
    /// assert_eq!(map.remove(&i), Some(123));
    /// assert_eq!(map.remove(&i), None);
    /// ```
    pub fn remove(&mut self, key: &I) -> Option<T> {
        let idx = key.index().try_into().ok()?;
        let index = self.indices.get_mut(idx)?;
        if *index != *key {
            return None;
        }
        *index = I::from_raw_parts(into_index(if idx == 0 { 1 } else { 0 }), index.generation());
        self.push_free_idx(idx);
        Some(unsafe { get_value_unchecked(&self.values, idx).assume_init_read() })
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(key, &value)` returns `false`.
    ///
    /// # Examples
    /// ```
    /// # use collections::PagedSlotMap;
    /// let mut map = PagedSlotMap::<i32>::new();
    /// let idx1 = map.push(1);
    /// let idx2 = map.push(2);
    /// map.retain(|_, val| { if *val == 1 { *val = 3; true } else { false } });
    /// assert_eq!(map.get(&idx1), Some(&3));
    /// assert!(map.get(&idx2).is_none());
    /// ```
    pub fn retain(&mut self, mut f: impl FnMut(&I, &mut T) -> bool) {
        let mut free_list_head = self.len() + 1;
        let mut remove_count = 0;
        for (i, index) in self
            .indices
            .iter_mut()
            .enumerate()
            .filter(|(i, index)| usize_eq(*i, index.index()))
        {
            let value = unsafe { get_value_unchecked_mut(&mut self.values, i) };
            if !f(index, unsafe { value.assume_init_mut() }) {
                unsafe { value.assume_init_drop() }
                *index = I::from_raw_parts(into_index(free_list_head), index.generation());
                free_list_head = i;
                remove_count += 1;
            }
        }

        if remove_count > 0 {
            self.push_free_idx(free_list_head);
            self.free_list_size += remove_count - 1;
        }
    }

    /// Pushes given index to the tail of free list.
    fn push_free_idx(&mut self, idx: usize) {
        if self.free_list_size > 0 {
            // INVARIANT: when free_list_size > 0, free_list_tail should be valid
            let tail_index = unsafe { self.indices.get_unchecked_mut(self.free_list_tail) };
            *tail_index = I::from_raw_parts(into_index(idx), tail_index.generation());
        } else {
            self.free_list_head = idx;
        }
        self.free_list_tail = idx;
        self.free_list_size += 1;
    }
}

#[inline]
fn usize_eq<I: TryInto<usize>>(lhs: usize, rhs: I) -> bool {
    rhs.try_into().is_ok_and(|rhs| lhs == rhs)
}

#[inline]
fn into_index<I: TryFrom<usize>>(i: usize) -> I {
    i.try_into().ok().expect(INVALID_INDEX)
}

#[inline]
fn into_usize<I: TryInto<usize>>(i: I) -> usize {
    i.try_into().ok().expect(INVALID_INDEX)
}

#[inline]
fn new_page<T, const N: usize>() -> Box<[MaybeUninit<T>; N]> {
    Box::new(array::from_fn(|_| MaybeUninit::uninit()))
}

#[inline]
unsafe fn get_value_unchecked<T, const N: usize>(
    values: &Vec<Box<[MaybeUninit<T>; N]>>,
    idx: usize,
) -> &MaybeUninit<T> {
    values.get_unchecked(idx / N).get_unchecked(idx % N)
}

#[inline]
unsafe fn get_value_unchecked_mut<T, const N: usize>(
    values: &mut Vec<Box<[MaybeUninit<T>; N]>>,
    idx: usize,
) -> &mut MaybeUninit<T> {
    values.get_unchecked_mut(idx / N).get_unchecked_mut(idx % N)
}

mod core_impl {
    use super::{PagedSlotMap, INVALID_INDEX};
    use alloc::{boxed::Box, vec::Vec};
    use core::{
        fmt,
        hash::{Hash, Hasher},
        mem::MaybeUninit,
        ops::{Index, IndexMut},
    };
    use genindex::GenIndex;

    impl<T: Clone, I: GenIndex, const N: usize> Clone for PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        fn clone(&self) -> Self {
            let mut values = Vec::new();
            let mut page: Box<[MaybeUninit<T>; N]> = super::new_page();
            for (i, index) in self.indices.iter().enumerate() {
                if i > 0 && i % N == 0 {
                    values.push(page);
                    page = super::new_page();
                }

                if super::usize_eq(i, index.index()) {
                    let value =
                        unsafe { super::get_value_unchecked(&self.values, i).assume_init_ref() };
                    unsafe { page.get_unchecked_mut(i % N) }.write(value.clone());
                }
            }

            if self.indices.len() > 0 {
                values.push(page);
            }

            Self {
                indices: self.indices.clone(),
                values,
                free_list_head: self.free_list_head.clone(),
                free_list_tail: self.free_list_tail.clone(),
                free_list_size: self.free_list_size.clone(),
            }
        }
    }

    impl<T: fmt::Debug, I: fmt::Debug + GenIndex, const N: usize> fmt::Debug for PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.debug_map().entries(self.iter()).finish()
        }
    }

    impl<T, I, const N: usize> Default for PagedSlotMap<T, I, N> {
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: PartialEq, I: GenIndex, const N: usize> Eq for PagedSlotMap<T, I, N> where
        I::Index: TryInto<usize>
    {
    }

    impl<T, I: GenIndex, const N: usize> Extend<T> for PagedSlotMap<T, I, N>
    where
        I::Index: TryFrom<usize> + TryInto<usize>,
    {
        fn extend<It: IntoIterator<Item = T>>(&mut self, iter: It) {
            for t in iter {
                self.push(t);
            }
        }
    }

    impl<'a, T, I: GenIndex, const N: usize> Extend<&'a T> for PagedSlotMap<T, I, N>
    where
        I::Index: TryFrom<usize> + TryInto<usize>,
        T: Clone,
    {
        fn extend<It: IntoIterator<Item = &'a T>>(&mut self, iter: It) {
            for t in iter {
                self.push(t.clone());
            }
        }
    }

    impl<T, I: GenIndex, const N: usize> FromIterator<T> for PagedSlotMap<T, I, N>
    where
        I::Index: TryFrom<usize> + TryInto<usize>,
    {
        fn from_iter<It: IntoIterator<Item = T>>(iter: It) -> Self {
            let iter = iter.into_iter();
            let mut map = PagedSlotMap::new();
            let (lower, upper) = iter.size_hint();
            map.reserve(upper.unwrap_or(lower));
            map.extend(iter);
            map
        }
    }

    impl<'a, T: Clone, I: GenIndex, const N: usize> FromIterator<&'a T> for PagedSlotMap<T, I, N>
    where
        I::Index: TryFrom<usize> + TryInto<usize>,
    {
        fn from_iter<It: IntoIterator<Item = &'a T>>(iter: It) -> Self {
            let iter = iter.into_iter();
            let mut map = PagedSlotMap::new();
            let (lower, upper) = iter.size_hint();
            map.reserve(upper.unwrap_or(lower));
            map.extend(iter);
            map
        }
    }

    impl<T: Hash, I: GenIndex + Hash, const N: usize> Hash for PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.indices.hash(state);
            for (i, index) in self.indices.iter().enumerate() {
                let value = if super::usize_eq(i, index.index()) {
                    Some(unsafe { super::get_value_unchecked(&self.values, i).assume_init_ref() })
                } else {
                    None
                };
                value.hash(state);
            }
            self.free_list_head.hash(state);
            self.free_list_tail.hash(state);
            self.free_list_size.hash(state);
        }
    }

    impl<T, I: GenIndex, const N: usize> Index<I> for PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        type Output = T;

        fn index(&self, index: I) -> &Self::Output {
            let idx = index
                .index()
                .try_into()
                .ok()
                .filter(|idx| self.indices.get(*idx).is_some_and(|i| *i == index))
                .expect(INVALID_INDEX);
            unsafe { super::get_value_unchecked(&self.values, idx).assume_init_ref() }
        }
    }

    impl<T, I: GenIndex, const N: usize> IndexMut<I> for PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        fn index_mut(&mut self, index: I) -> &mut Self::Output {
            let idx = index
                .index()
                .try_into()
                .ok()
                .filter(|idx| self.indices.get(*idx).is_some_and(|i| *i == index))
                .expect(INVALID_INDEX);
            unsafe { super::get_value_unchecked_mut(&mut self.values, idx).assume_init_mut() }
        }
    }

    impl<T: PartialEq, I: GenIndex, const N: usize> PartialEq for PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        fn eq(&self, other: &Self) -> bool {
            if !(self.indices == other.indices
                && self.free_list_head == other.free_list_head
                && self.free_list_tail == other.free_list_tail
                && self.free_list_size == other.free_list_size)
            {
                return false;
            }
            for (i, index) in self.indices.iter().enumerate() {
                if super::usize_eq(i, index.index()) {
                    let lhs =
                        unsafe { super::get_value_unchecked(&self.values, i).assume_init_ref() };
                    let rhs =
                        unsafe { super::get_value_unchecked(&other.values, i).assume_init_ref() };
                    if !lhs.eq(rhs) {
                        return false;
                    }
                }
            }
            true
        }
    }
}

mod collections_impl {
    use super::PagedSlotMap;
    use crate::{Clear, Len, Map, MapGet, MapInsert, MapMut, Push, Retain};
    use core::mem::replace;
    use genindex::GenIndex;

    impl<T, I, const N: usize> Clear for PagedSlotMap<T, I, N> {
        #[inline]
        fn clear(&mut self) {
            self.clear();
        }
    }

    impl<T, I, const N: usize> Len for PagedSlotMap<T, I, N> {
        #[inline]
        fn len(&self) -> usize {
            self.len()
        }
    }

    impl<T, I: GenIndex, const N: usize> Push for PagedSlotMap<T, I, N>
    where
        I::Index: TryFrom<usize> + TryInto<usize>,
    {
        type Index = I;
        type Value = T;

        #[inline]
        fn push(&mut self, value: Self::Value) -> Self::Index {
            self.push(value)
        }
    }

    impl<T, I, const N: usize> Map for PagedSlotMap<T, I, N> {
        type Key = I;
        type Value = T;
    }

    impl<T, I: GenIndex, const N: usize> MapGet<I> for PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        #[inline]
        fn get(&self, key: &I) -> Option<&Self::Value> {
            self.get(key)
        }
    }

    impl<T, I: GenIndex, const N: usize> MapMut<I> for PagedSlotMap<T, I, N>
    where
        I::Index: TryFrom<usize> + TryInto<usize>,
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

    impl<T, I: GenIndex, const N: usize> MapInsert for PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        #[inline]
        fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
            let dest = self.get_mut(&key)?;
            Some(replace(dest, value))
        }
    }

    impl<T, I: GenIndex, const N: usize> Retain for PagedSlotMap<T, I, N>
    where
        I::Index: TryFrom<usize> + TryInto<usize>,
    {
        type Key = I;
        type Value = T;

        #[inline]
        fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
            self.retain(f);
        }
    }
}

mod iter {
    use super::PagedSlotMap;
    use alloc::{boxed::Box, vec::Vec};
    use core::{iter::FusedIterator, mem::MaybeUninit};
    use genindex::GenIndex;

    impl<T, I: GenIndex, const N: usize> IntoIterator for PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        type Item = (I, T);

        type IntoIter = IntoIter<T, I, N>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            IntoIter {
                start: 0,
                end: self.len(),
                index: (self.indices).into_iter(),
                values: self.values,
            }
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a, const N: usize> IntoIterator for &'a PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        type Item = (&'a I, &'a T);

        type IntoIter = Iter<'a, T, I, N>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a, const N: usize> IntoIterator for &'a mut PagedSlotMap<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        type Item = (&'a I, &'a mut T);

        type IntoIter = IterMut<'a, T, I, N>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }

    /// An into iterator over a [super::PagedSlotMap].
    #[derive(Debug)]
    pub struct IntoIter<T, I: GenIndex, const N: usize> {
        pub(super) index: <Vec<I> as IntoIterator>::IntoIter,
        pub(super) values: Vec<Box<[MaybeUninit<T>; N]>>,
        pub(super) start: usize,
        pub(super) end: usize,
    }

    impl<T, I: GenIndex, const N: usize> Iterator for IntoIter<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        type Item = (I, T);

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(index) = self.index.next() {
                let idx = self.start;
                self.start += 1;
                if super::usize_eq(idx, index.index()) {
                    Some((index, unsafe {
                        super::get_value_unchecked(&self.values, idx).assume_init_read()
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.end - self.start;
            (remaining, Some(remaining))
        }
    }

    impl<T, I: GenIndex, const N: usize> DoubleEndedIterator for IntoIter<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            if let Some(index) = self.index.next_back() {
                self.end -= 1;
                let idx = self.end;
                if super::usize_eq(idx, index.index()) {
                    Some((index, unsafe {
                        super::get_value_unchecked(&self.values, idx).assume_init_read()
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    impl<T, I: GenIndex, const N: usize> ExactSizeIterator for IntoIter<T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        #[inline]
        fn len(&self) -> usize {
            self.end - self.start
        }
    }

    impl<T, I: GenIndex, const N: usize> FusedIterator for IntoIter<T, I, N> where
        I::Index: TryInto<usize>
    {
    }

    /// An immutable iterator over a [super::PagedSlotMap].
    #[derive(Clone, Debug)]
    pub struct Iter<'a, T: 'a, I: GenIndex + 'a, const N: usize> {
        pub(super) index: <&'a Vec<I> as IntoIterator>::IntoIter,
        pub(super) values: &'a Vec<Box<[MaybeUninit<T>; N]>>,
        pub(super) start: usize,
        pub(super) end: usize,
    }

    impl<'a, T: 'a, I: GenIndex + 'a, const N: usize> Iterator for Iter<'a, T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        type Item = (&'a I, &'a T);

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(index) = self.index.next() {
                let idx = self.start;
                self.start += 1;
                if super::usize_eq(idx, index.index()) {
                    Some((index, unsafe {
                        super::get_value_unchecked(&self.values, idx).assume_init_ref()
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.end - self.start;
            (remaining, Some(remaining))
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a, const N: usize> DoubleEndedIterator for Iter<'a, T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            if let Some(index) = self.index.next_back() {
                self.end -= 1;
                let idx = self.end;
                if super::usize_eq(idx, index.index()) {
                    Some((index, unsafe {
                        super::get_value_unchecked(&self.values, idx).assume_init_ref()
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a, const N: usize> ExactSizeIterator for Iter<'a, T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        #[inline]
        fn len(&self) -> usize {
            self.end - self.start
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a, const N: usize> FusedIterator for Iter<'a, T, I, N> where
        I::Index: TryInto<usize>
    {
    }

    /// A mutable iterator over a [super::PagedSlotMap].
    #[derive(Debug)]
    pub struct IterMut<'a, T: 'a, I: GenIndex + 'a, const N: usize> {
        pub(super) index: <&'a Vec<I> as IntoIterator>::IntoIter,
        pub(super) values: &'a mut Vec<Box<[MaybeUninit<T>; N]>>,
        pub(super) start: usize,
        pub(super) end: usize,
    }

    impl<'a, T: 'a, I: GenIndex + 'a, const N: usize> Iterator for IterMut<'a, T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        type Item = (&'a I, &'a mut T);

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(index) = self.index.next() {
                let idx = self.start;
                self.start += 1;
                if super::usize_eq(idx, index.index()) {
                    Some((index, unsafe {
                        &mut *super::get_value_unchecked_mut(&mut self.values, idx).as_mut_ptr()
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.end - self.start;
            (remaining, Some(remaining))
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a, const N: usize> DoubleEndedIterator for IterMut<'a, T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            if let Some(index) = self.index.next_back() {
                self.end -= 1;
                let idx = self.end;
                if super::usize_eq(idx, index.index()) {
                    Some((index, unsafe {
                        &mut *super::get_value_unchecked_mut(&mut self.values, idx).as_mut_ptr()
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a, const N: usize> ExactSizeIterator for IterMut<'a, T, I, N>
    where
        I::Index: TryInto<usize>,
    {
        #[inline]
        fn len(&self) -> usize {
            self.end - self.start
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a, const N: usize> FusedIterator for IterMut<'a, T, I, N> where
        I::Index: TryInto<usize>
    {
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::PagedSlotMap;
    use alloc::{boxed::Box, vec::Vec};
    use core::mem::MaybeUninit;
    use genindex::GenIndex;
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    impl<T, I, const N: usize> Serialize for PagedSlotMap<T, I, N>
    where
        T: Serialize,
        I: GenIndex + Serialize,
        I::Index: TryInto<usize>,
    {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let values: Vec<Option<&T>> = self
                .indices
                .iter()
                .enumerate()
                .map(|(idx, index)| {
                    if super::usize_eq(idx, index.index()) {
                        Some(unsafe {
                            super::get_value_unchecked(&self.values, idx).assume_init_ref()
                        })
                    } else {
                        None
                    }
                })
                .collect();
            (&self.indices, &values).serialize(serializer)
        }
    }

    impl<'de, T, I, const N: usize> Deserialize<'de> for PagedSlotMap<T, I, N>
    where
        T: Deserialize<'de>,
        I: GenIndex + Deserialize<'de>,
        I::Index: TryFrom<usize> + TryInto<usize>,
    {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let (mut indices, option_values): (Vec<I>, Vec<Option<T>>) =
                Deserialize::deserialize(deserializer)?;
            let mut values = Vec::with_capacity(indices.len().div_ceil(N));
            let mut free_list_head = indices.len();
            let mut free_list_tail = indices.len();
            let mut free_list_size = 0;

            let mut page: Box<[MaybeUninit<T>; N]> = super::new_page();

            // Rebuild free list and values
            for (i, (gen_index, value)) in indices
                .iter_mut()
                .zip(option_values.into_iter())
                .enumerate()
            {
                let idx = gen_index
                    .index()
                    .try_into()
                    .map_err(|_| D::Error::custom(super::INVALID_INDEX))?;
                let offset = idx % N;

                if idx > 0 && offset == 0 {
                    values.push(page);
                    page = super::new_page();
                }

                if let Some(v) = value.filter(|_| i == idx) {
                    page[offset].write(v);
                } else {
                    // value is None or index not match => free index
                    let index = (if i == free_list_head {
                        0
                    } else {
                        free_list_head
                    })
                    .try_into()
                    .map_err(|_| D::Error::custom(super::INVALID_INDEX))?;
                    *gen_index = I::from_raw_parts(index, gen_index.generation());
                    free_list_head = idx;
                    if free_list_size == 0 {
                        free_list_tail = idx;
                    }
                    free_list_size += 1;
                }
            }

            if indices.len() > 0 {
                values.push(page);
            }

            Ok(PagedSlotMap {
                indices,
                values,
                free_list_head,
                free_list_tail,
                free_list_size,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PagedSlotMap;
    use crate::{Clear, Len, MapGet, MapInsert, MapMut, Push, Retain};
    use alloc::format;
    use core::hash::{Hash, Hasher};
    use genindex::{GenIndex, IndexPair};
    use std::hash::DefaultHasher;

    fn create_map() -> PagedSlotMap<u32> {
        let mut map = PagedSlotMap::new();
        for i in 0..10 {
            let index: IndexPair = Push::push(&mut map, i);
            assert_eq!(index.index(), i);
        }
        map
    }

    #[test]
    fn test_default() {
        let map = PagedSlotMap::<u32>::default();
        assert_eq!(map.len(), 0);
        assert_eq!(map.capacity(), 0);
    }

    #[test]
    fn test_clone_eq() {
        let map = create_map();
        let map2 = map.clone();
        assert!(map == map2);
    }

    #[test]
    fn test_debug() {
        let mut map = PagedSlotMap::<i32>::new();
        let idx1 = map.push(0);
        let idx2 = map.push(1);
        assert_eq!(format!("{map:?}"), format!("{{{idx1:?}: 0, {idx2:?}: 1}}"));
    }

    #[test]
    fn test_from_iter() {
        let map = create_map();

        let map2 = PagedSlotMap::from_iter(map.iter().map(|(_, v)| v));
        assert!(map == map2);

        let map2 = PagedSlotMap::from_iter(map.clone().into_iter().map(|(_, v)| v));
        assert!(map == map2);
    }

    #[test]
    fn test_hash() {
        let map = create_map();
        let map2 = map.clone();
        let mut s = DefaultHasher::new();
        map.hash(&mut s);
        let hash1 = s.finish();
        let mut s = DefaultHasher::new();
        map2.hash(&mut s);
        let hash2 = s.finish();
        assert_eq!(hash1, hash2);
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
        assert_eq!(MapGet::get(&map, &IndexPair::from_index(123)), None);
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

        let unknown_idx = IndexPair::from_index(123);
        assert_eq!(MapInsert::insert(&mut map, unknown_idx, new_value), None);
        assert_eq!(MapGet::get(&map, &unknown_idx), None);
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
    fn test_iter_rev() {
        let map = create_map();
        let mut i = 10;
        for (idx, value) in (&map).into_iter().rev() {
            i -= 1;
            assert_eq!(idx.index(), *value);
            assert_eq!(i, *value);
        }
        assert_eq!(i, 0);
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
    fn test_iter_mut_rev() {
        let mut map = create_map();
        let mut i = 10;
        for (idx, value) in (&mut map).into_iter().rev() {
            i -= 1;
            *value += 1;
            assert_eq!(i, idx.index());
            assert_eq!(i + 1, *value);
        }
        assert_eq!(i, 0);
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
    fn test_into_iter_rev() {
        let map = create_map();
        let mut i = 10;
        for (idx, value) in map.into_iter().rev() {
            i -= 1;
            assert_eq!(idx.index(), value);
            assert_eq!(i, value);
        }
        assert_eq!(i, 0);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        use alloc::vec;
        use serde_json::{json, Value};

        let mut map = PagedSlotMap::<&str>::new();
        let idx1 = map.push("a");
        map.push("b");
        map.push("c");
        map.remove(&idx1);
        map.push("d");

        let expected_json: Value = json!([[[0, 2], [1, 1], [2, 1]], ["d", "b", "c"]]);

        let json: Value = serde_json::to_value(map).unwrap();

        assert_eq!(json, expected_json);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        use alloc::{string::String, vec};
        use serde_json::{json, Value};

        let json: Value = json!([[[0, 2], [1, 3], [5, 3], [3, 4]], ["d", "b", null, "c"]]);

        let map: PagedSlotMap<String> = serde_json::from_value(json).unwrap();

        assert_eq!(map.len(), 3);
        assert_eq!(map[GenIndex::from_raw_parts(1, 3)], "b");
        assert_eq!(map[GenIndex::from_raw_parts(3, 4)], "c");
        assert_eq!(map[GenIndex::from_raw_parts(0, 2)], "d");
        assert_eq!(map.get(&GenIndex::from_raw_parts(2, 3)), None);
    }
}
