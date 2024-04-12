//! Helper traits for joining with `Map`s.

use crate::{cons, Concat, Cons};

use super::{MapGet, MapMut};
use core::{borrow::Borrow, iter::FusedIterator};

/// Iterator trait for joining with [MapGet]s and [MapMut]s.
pub trait MapJoin<'a, K: 'a, V>: Iterator<Item = (&'a K, V)> + Sized {
    /// Returns an iterator adaptor that wraps items as `Cons`, i.e. `(key, (value, ())`.
    /// Useful for chaining with multiple joins where the resulting item will be a valid `Cons`.
    ///
    /// # Examples
    /// ```rust
    /// # use std::collections::BTreeMap;
    /// # use adts::{cons, MapJoin};
    /// let mut map = BTreeMap::new();
    /// map.insert(1, 2);
    /// map.insert(3, 4);
    /// let mut iter = map.iter().cons();
    /// assert_eq!(iter.next(), Some(cons!(&1, &2)));
    /// assert_eq!(iter.next(), Some(cons!(&3, &4)));
    /// ```
    #[inline(always)]
    fn cons(self) -> core::iter::Map<Self, fn((&'a K, V)) -> (&'a K, (V, ()))> {
        self.map(|(k, v)| (k, (v, ())))
    }

    /// Returns an iterator adaptor that inner joins this iterator with a [MapGet].
    ///
    /// # Examples
    /// ```rust
    /// # use std::collections::BTreeMap;
    /// # use adts::{cons, MapJoin};
    /// let mut map = BTreeMap::new();
    /// let mut map2 = BTreeMap::new();
    /// map.insert(1, 1);
    /// map.insert(2, 2);
    /// map2.insert(1, 3);
    /// map2.insert(2, 4);
    /// let mut iter = map.iter().cons().map_join(&map2);
    /// assert_eq!(iter.next(), Some(cons!(&1, &1, &3)));
    /// assert_eq!(iter.next(), Some(cons!(&2, &2, &4)));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline(always)]
    fn map_join<M>(self, rhs: &'a M) -> MapJoinIter<Self, &'a M>
    where
        M: MapGet<K>,
        M::Key: Borrow<K>,
        Self::Item: Cons,
    {
        MapJoinIter {
            iter: self,
            map: rhs,
        }
    }

    /// Returns an iterator adaptor that left joins this iterator with a [MapGet].
    ///
    /// # Examples
    /// ```rust
    /// # use std::collections::BTreeMap;
    /// # use adts::{cons, MapJoin};
    /// let mut map = BTreeMap::new();
    /// let mut map2 = BTreeMap::new();
    /// map.insert(1, 1);
    /// map.insert(2, 2);
    /// map2.insert(1, 3);
    /// let mut iter = map.iter().cons().map_join_left(&map2);
    /// assert_eq!(iter.next(), Some(cons!(&1, &1, Some(&3))));
    /// assert_eq!(iter.next(), Some(cons!(&2, &2, None)));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline(always)]
    fn map_join_left<M>(self, rhs: &'a M) -> MapJoinLeftIter<Self, &'a M>
    where
        M: MapGet<K>,
        M::Key: Borrow<K>,
        Self::Item: Cons,
    {
        MapJoinLeftIter {
            iter: self,
            map: rhs,
        }
    }

    /// Returns an iterator adaptor that left exclusive joins this iterator with a `Map`.
    /// The returned iterator will yield only the elements with keys not in the RHS map.
    ///
    /// # Examples
    /// ```rust
    /// # use std::collections::BTreeMap;
    /// # use adts::{cons, MapJoin};
    /// let mut map = BTreeMap::new();
    /// let mut map2 = BTreeMap::new();
    /// map.insert(1, 1);
    /// map.insert(2, 2);
    /// map2.insert(1, 3);
    /// let mut iter = map.iter().cons().map_join_left_excl(&map2);
    /// assert_eq!(iter.next(), Some(cons!(&2, &2)));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline(always)]
    fn map_join_left_excl<M>(self, rhs: &'a M) -> MapJoinLeftExclIter<Self, &'a M>
    where
        M: MapGet<K>,
        M::Key: Borrow<K>,
    {
        MapJoinLeftExclIter {
            iter: self,
            map: rhs,
        }
    }

    /// Inner joins with a [MapMut].
    ///
    /// # Safety
    /// Self must be a map iterator that never returns duplicate keys.
    /// Otherwise, this method may potentially hand out multiple mutable references to the same RHS value!
    ///
    /// # Examples
    /// ```rust
    /// # use std::collections::BTreeMap;
    /// # use adts::{cons, MapJoin};
    /// let mut map = BTreeMap::new();
    /// let mut map2 = BTreeMap::new();
    /// map.insert(1, 1);
    /// map2.insert(1, 3);
    /// let mut iter = map.iter().cons().map_join_mut(&mut map2);
    /// let cons!(_, _, v) = iter.next().unwrap();
    /// *v *= 2;
    /// assert_eq!(map2.get(&1), Some(&6));
    /// ```
    #[inline(always)]
    fn map_join_mut<M>(self, rhs: &'a mut M) -> MapJoinIter<Self, &'a mut M>
    where
        M: MapMut<K>,
        M::Key: Borrow<K>,
    {
        MapJoinIter {
            iter: self,
            map: rhs,
        }
    }

    /// Left joins with a [MapMut].
    ///
    /// # Safety
    /// Self must be a map iterator that never returns duplicate keys.
    /// Otherwise, this method may potentially hand out multiple mutable references to the same RHS value!
    ///
    /// # Examples
    /// ```rust
    /// # use std::collections::BTreeMap;
    /// # use adts::{cons, MapJoin};
    /// let mut map = BTreeMap::new();
    /// let mut map2 = BTreeMap::new();
    /// map.insert(1, 1);
    /// map2.insert(1, 3);
    /// let mut iter = map.iter().cons().map_join_left_mut(&mut map2);
    /// if let Some(cons!(_, _, {Some(v)})) = iter.next() {
    ///     *v *= 2;
    /// }
    /// assert_eq!(map2.get(&1), Some(&6));
    /// ```
    #[inline(always)]
    fn map_join_left_mut<M>(self, rhs: &'a mut M) -> MapJoinLeftIter<Self, &'a mut M>
    where
        M: MapMut<K>,
        M::Key: Borrow<K>,
    {
        MapJoinLeftIter {
            iter: self,
            map: rhs,
        }
    }
}

impl<'a, T, K: 'a, V> MapJoin<'a, K, V> for T where T: Iterator<Item = (&'a K, V)> {}

/// Iterator adaptor that inner joins 2 maps.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MapJoinIter<LHS: Iterator, RHS> {
    iter: LHS,
    map: RHS,
}

impl<'a, K: 'a, V, LHS, RHS> Iterator for MapJoinIter<LHS, &'a RHS>
where
    LHS: Iterator<Item = (&'a K, V)>,
    RHS: MapGet<K>,
    RHS::Key: Borrow<K>,
    V: Concat<Cons!(&'a RHS::Value)>,
{
    type Item = <(&'a K, V) as Concat<Cons!(&'a RHS::Value)>>::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((key, lval)) = self.iter.next() {
            if let Some(rval) = self.map.get(key) {
                return Some((key, lval).concat(cons!(rval)));
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<'a, K: 'a, V, LHS, RHS> Iterator for MapJoinIter<LHS, &'a mut RHS>
where
    LHS: Iterator<Item = (&'a K, V)>,
    RHS: MapMut<K>,
    RHS::Key: Borrow<K>,
    V: Concat<Cons!(&'a mut RHS::Value)>,
{
    type Item = <(&'a K, V) as Concat<Cons!(&'a mut RHS::Value)>>::Output;

    /// Advances the iterator and returns the next value.
    ///
    /// # Safety
    /// LHS must be a map iterator that does not return duplicate keys.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((key, lval)) = self.iter.next() {
            if let Some(rval) = self.map.get_mut(key) {
                // Safety: there must be no duplicate key so that we do not hand out
                // multiple mutable references to the same value within RHS
                let rval = unsafe { &mut *(rval as *mut _) };
                return Some((key, lval).concat(cons!(rval)));
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<LHS, RHS> FusedIterator for MapJoinIter<LHS, RHS>
where
    Self: Iterator,
    LHS: FusedIterator,
{
}

/// Iterator adaptor that left joins 2 maps.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MapJoinLeftIter<LHS: Iterator, RHS> {
    iter: LHS,
    map: RHS,
}

impl<'a, K: 'a, V, LHS, RHS> Iterator for MapJoinLeftIter<LHS, &'a RHS>
where
    LHS: Iterator<Item = (&'a K, V)>,
    RHS: MapGet<K>,
    RHS::Key: Borrow<K>,
    V: Concat<Cons!(Option<&'a RHS::Value>)>,
{
    type Item = <(&'a K, V) as Concat<Cons!(Option<&'a RHS::Value>)>>::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(key, lval)| (key, lval).concat(cons!(self.map.get(key))))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: 'a, V, LHS, RHS> Iterator for MapJoinLeftIter<LHS, &'a mut RHS>
where
    LHS: Iterator<Item = (&'a K, V)>,
    RHS: MapMut<K>,
    RHS::Key: Borrow<K>,
    V: Concat<Cons!(Option<&'a mut RHS::Value>)>,
{
    type Item = <(&'a K, V) as Concat<Cons!(Option<&'a mut RHS::Value>)>>::Output;

    /// Advances the iterator and returns the next value.
    ///
    /// # Safety
    /// LHS must be a map iterator that does not return duplicate keys.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, lval)| {
            let rval = self
                .map
                .get_mut(key)
                // Safety: there must be no duplicate key so that we do not hand out
                // multiple mutable references to the same value within RHS
                .map(|rval| unsafe { &mut *(rval as *mut _) });
            (key, lval).concat(cons!(rval))
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<LHS, RHS> FusedIterator for MapJoinLeftIter<LHS, RHS>
where
    Self: Iterator,
    LHS: FusedIterator,
{
}

/// Iterator adaptor that left exclusive joins 2 maps.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MapJoinLeftExclIter<LHS: Iterator, RHS> {
    iter: LHS,
    map: RHS,
}

impl<'a, K: 'a, V, LHS, RHS> Iterator for MapJoinLeftExclIter<LHS, &'a RHS>
where
    LHS: Iterator<Item = (&'a K, V)>,
    RHS: MapGet<K>,
    RHS::Key: Borrow<K>,
{
    type Item = LHS::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((key, val)) = self.iter.next() {
            if !self.map.contains_key(key) {
                return Some((key, val));
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<LHS, RHS> FusedIterator for MapJoinLeftExclIter<LHS, RHS>
where
    Self: Iterator,
    LHS: FusedIterator,
{
}
