use crate::{Clear, Cons, Len, Map, MapGet, MapInsert, MapMut, MapRemove};
use alloc::{boxed::Box, sync::Arc};
use core::{
    any::{Any, TypeId},
    marker::PhantomData,
};

/// A type-safe associative array of unique types to values.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct AnyMap<T: ?Sized = dyn Any, M = DefaultBackingMap<AnyMapKey, Box<T>>> {
    map: M,
    marker: PhantomData<Arc<(AnyMapKey, Box<T>)>>,
}

/// Key of a [Registry].
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnyMapKey {
    /// Key for a dynamic type specified by a [usize] ID.
    Id(usize),
    /// Key for a static type.
    TypeId(TypeId),
}

impl AnyMapKey {
    #[inline]
    pub fn with_type<T: 'static>() -> Self {
        AnyMapKey::TypeId(TypeId::of::<T>())
    }

    #[inline]
    pub fn is_type<T: 'static>(&self) -> bool {
        match self {
            AnyMapKey::TypeId(id) => *id == TypeId::of::<T>(),
            _ => true,
        }
    }
}

/// Registry data backing map type.
#[cfg(feature = "std")]
type DefaultBackingMap<K, V> = std::collections::HashMap<K, V>;
/// Registry data backing map type.
#[cfg(not(feature = "std"))]
type DefaultBackingMap<K, V> = alloc::collections::BTreeMap<K, V>;

impl<T: ?Sized, M> AnyMap<T, M> {
    /// Constructs a new, empty [AnyMap].
    ///
    /// # Examples
    /// ```rust
    /// # use adts::AnyMap;
    /// let map = <AnyMap>::new();
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
    /// # use adts::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert(1u32);
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
    /// # use adts::{AnyMap, AnyMapKey};
    /// let mut map = <AnyMap>::new();
    /// map.insert(1u32);
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
}

impl<T: ?Sized, M> AnyMap<T, M>
where
    M: Map<Key = AnyMapKey, Value = Box<T>>,
    M::Value: Downcast,
{
    /// Gets a value by type.
    ///
    /// # Examples
    /// ```rust
    /// # use adts::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// map.insert(1u32);
    /// assert_eq!(map.get::<u32>(), Some(&1));
    /// ```
    #[inline]
    pub fn get<V: 'static>(&self) -> Option<&V>
    where
        M: MapGet<AnyMapKey>,
    {
        self.get_by_key(&AnyMapKey::with_type::<V>())
    }

    /// Mutably gets a value by type.
    ///
    /// # Examples
    /// ```rust
    /// # use adts::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// map.insert(1u32);
    /// *map.get_mut::<u32>().unwrap() = 2;
    /// assert_eq!(map.get::<u32>(), Some(&2));
    /// ```
    #[inline]
    pub fn get_mut<V: 'static>(&mut self) -> Option<&mut V>
    where
        M: MapMut<AnyMapKey>,
    {
        self.get_by_key_mut(&AnyMapKey::with_type::<V>())
    }

    /// Gets multiple values by type.
    ///
    /// # Examples
    /// ```rust
    /// # use adts::{AnyMap, Cons, cons};
    /// let mut map = <AnyMap>::new();
    /// map.insert(1u32);
    /// map.insert(2i16);
    /// assert_eq!(map.multi_get::<Cons!(&i16, &u32)>(), Some(cons!(&2, &1)));
    /// ```
    #[inline]
    pub fn multi_get<'a, C: Cons>(&'a self) -> Option<C::Value>
    where
        C: AnyMapMultiGet<'a, Self>,
    {
        C::multi_get(&self)
    }

    /// Mutably gets multiple values by type.
    ///
    /// # Examples
    /// ```rust
    /// # use adts::{AnyMap, Cons, cons};
    /// let mut map = <AnyMap>::new();
    /// map.insert(1u32);
    /// map.insert(2i16);
    /// map.insert(3u8);
    /// if let Some(cons!(second, third, first)) = map.multi_get_mut::<Cons!(&mut i16, &u8, &mut u32)>() {
    ///     assert_eq!(*third, 3);
    ///     *second = 3;
    ///     *first = 0;
    /// }
    /// assert_eq!(map.get::<u32>(), Some(&0));
    /// assert_eq!(map.get::<i16>(), Some(&3));
    /// ```
    #[inline]
    pub fn multi_get_mut<'a, C: Cons>(&'a mut self) -> Option<C::Value>
    where
        C: AnyMapMultiMut<'a, Self>,
    {
        C::multi_get_mut(self)
    }

    /// Removes and returns a type value from the map if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use adts::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// assert_eq!(map.insert(1u32), None);
    /// assert_eq!(map.remove::<u32>(), Some(1));
    /// ```
    #[inline]
    pub fn remove<V: 'static>(&mut self) -> Option<V>
    where
        M: MapRemove<AnyMapKey>,
    {
        self.remove_by_key(&AnyMapKey::with_type::<V>())
    }

    /// Inserts a type value to this map and returns existing value.
    ///
    /// # Examples
    /// ```rust
    /// # use adts::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// assert_eq!(map.insert(1u32), None);
    /// assert_eq!(map.insert(2u32), Some(1));
    /// ```
    #[inline]
    pub fn insert<V: 'static>(&mut self, value: V) -> Option<V>
    where
        M: MapInsert,
        V: IntoDowncast<Box<T>>,
    {
        self.insert_by_key(AnyMapKey::with_type::<V>(), value)
    }

    /// Gets a value for given key.
    ///
    /// # Examples
    /// ```rust
    /// # use adts::{AnyMap, AnyMapKey};
    /// let mut map = <AnyMap>::new();
    /// let key = AnyMapKey::with_type::<u32>();
    /// map.insert(1u32);
    /// assert_eq!(map.get_by_key::<u32>(&key), Some(&1));
    /// ```
    #[inline]
    pub fn get_by_key<V: 'static>(&self, key: &AnyMapKey) -> Option<&V>
    where
        M: MapGet<AnyMapKey>,
    {
        self.map.get(key)?.downcast_as_ref()
    }

    /// Mutably gets a value for given key.
    ///
    /// # Examples
    /// ```rust
    /// # use adts::{AnyMap, AnyMapKey};
    /// let mut map = <AnyMap>::new();
    /// let key = AnyMapKey::with_type::<u32>();
    /// map.insert(1u32);
    /// *map.get_by_key_mut::<u32>(&key).unwrap() = 2;
    /// assert_eq!(map.get_by_key::<u32>(&key), Some(&2));
    /// ```
    #[inline]
    pub fn get_by_key_mut<V: 'static>(&mut self, key: &AnyMapKey) -> Option<&mut V>
    where
        M: MapMut<AnyMapKey>,
    {
        self.map.get_mut(key)?.downcast_as_mut()
    }

    /// Removes and returns a type value from the map if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use adts::{AnyMap, AnyMapKey};
    /// let mut map = <AnyMap>::new();
    /// assert_eq!(map.insert(1u32), None);
    /// assert_eq!(map.remove_by_key(&AnyMapKey::with_type::<u32>()), Some(1u32));
    /// ```
    #[inline]
    pub fn remove_by_key<V: 'static>(&mut self, key: &AnyMapKey) -> Option<V>
    where
        M: MapRemove<AnyMapKey>,
    {
        self.map.remove(key)?.1.downcast_into()
    }

    /// Inserts a value to this map for given key and returns existing value.
    /// Does nothing if `key` is a TypeId and it does not match the TypeId of `value`.
    ///
    /// # Examples
    /// ```rust
    /// # use adts::{AnyMap, AnyMapKey};
    /// let mut map = <AnyMap>::new();
    /// let key = AnyMapKey::with_type::<u32>();
    /// assert_eq!(map.insert_by_key(key, 1u32), None);
    /// assert_eq!(map.insert_by_key(key, 2u32), Some(1));
    /// ```
    #[inline]
    pub fn insert_by_key<V: 'static>(&mut self, key: AnyMapKey, value: V) -> Option<V>
    where
        M: MapInsert,
        V: IntoDowncast<Box<T>>,
    {
        if let AnyMapKey::TypeId(id) = key {
            if id != TypeId::of::<V>() {
                return None;
            }
        }
        self.map.insert(key, value.into())?.downcast_into()
    }
}

/// Trait for getting multiple values by type from [AnyMap].
pub trait AnyMapMultiGet<'a, M> {
    /// Value type.
    type Value: Cons;

    /// Gets multiple values from [AnyMap].
    fn multi_get(map: &'a M) -> Option<Self::Value>;
}

impl<'a, M> AnyMapMultiGet<'a, M> for () {
    type Value = ();

    #[inline]
    fn multi_get(_: &'a M) -> Option<Self::Value> {
        Some(())
    }
}

impl<'a, T: ?Sized, M, V: 'static, Tail> AnyMapMultiGet<'a, AnyMap<T, M>> for (&'a V, Tail)
where
    M: MapGet<AnyMapKey, Key = AnyMapKey, Value = Box<T>>,
    M::Value: Downcast,
    Tail: AnyMapMultiGet<'a, AnyMap<T, M>>,
{
    type Value = (&'a V, Tail::Value);

    #[inline]
    fn multi_get(map: &'a AnyMap<T, M>) -> Option<Self::Value> {
        Some((map.get()?, Tail::multi_get(map)?))
    }
}

/// Trait for mutably getting multiple values by type from [AnyMap].
pub trait AnyMapMultiMut<'a, M> {
    /// Value type.
    type Value: Cons;

    /// [Cons] of mut types within `Value`.
    type Mut: Cons + 'static;

    /// Mutably gets multiple values from [AnyMap].
    fn multi_get_mut(map: &'a mut M) -> Option<Self::Value>;
}

impl<'a, M> AnyMapMultiMut<'a, M> for () {
    type Value = ();
    type Mut = ();

    #[inline]
    fn multi_get_mut(_: &'a mut M) -> Option<Self::Value> {
        Some(())
    }
}

impl<'a, T: ?Sized, M, V: 'static, Tail> AnyMapMultiMut<'a, AnyMap<T, M>> for (&'a V, Tail)
where
    M: MapGet<AnyMapKey, Key = AnyMapKey, Value = Box<T>>,
    M::Value: Downcast,
    Tail: AnyMapMultiMut<'a, AnyMap<T, M>>,
{
    type Value = (&'a V, Tail::Value);
    type Mut = Tail::Mut;

    #[inline]
    fn multi_get_mut(map: &'a mut AnyMap<T, M>) -> Option<Self::Value> {
        if contains_type_id::<Tail::Mut>(TypeId::of::<V>()) {
            None
        } else {
            let v: *const V = &*map.get()?;
            Some((unsafe { v.as_ref() }?, Tail::multi_get_mut(map)?))
        }
    }
}

impl<'a, T: ?Sized, M, V: 'static, Tail> AnyMapMultiMut<'a, AnyMap<T, M>> for (&'a mut V, Tail)
where
    M: MapMut<AnyMapKey, Key = AnyMapKey, Value = Box<T>>,
    M::Value: Downcast,
    Tail: AnyMapMultiMut<'a, AnyMap<T, M>>,
{
    type Value = (&'a mut V, Tail::Value);
    type Mut = (V, Tail::Mut);

    #[inline]
    fn multi_get_mut(map: &'a mut AnyMap<T, M>) -> Option<Self::Value> {
        if contains_type_id::<Tail::Mut>(TypeId::of::<V>()) {
            None
        } else {
            let v: *mut V = &mut *map.get_mut()?;
            Some((unsafe { v.as_mut() }?, Tail::multi_get_mut(map)?))
        }
    }
}

#[inline(always)]
fn contains_type_id<C: Cons + 'static>(type_id: TypeId) -> bool {
    if C::LEN > 0 {
        contains_type_id::<C::Tail>(type_id)
    } else if type_id == TypeId::of::<C::Head>() {
        true
    } else {
        false
    }
}

mod collections_impl {
    use super::AnyMap;
    use crate::{Clear, Len, Merge};

    impl<T: ?Sized, M: Clear> Clear for AnyMap<T, M> {
        #[inline]
        fn clear(&mut self) {
            self.clear();
        }
    }

    impl<T: ?Sized, M: Len> Len for AnyMap<T, M> {
        #[inline]
        fn len(&self) -> usize {
            self.len()
        }
    }

    impl<T: ?Sized, M: Merge<Output = M>> Merge for AnyMap<T, M> {
        type Output = Self;

        #[inline]
        fn merge(mut self, other: Self) -> Self::Output {
            self.map = self.map.merge(other.map);
            self
        }
    }
}

/// [Any]-like trait object pointer type that can be downcasted to underlying types.
pub trait Downcast {
    /// Downcasts to underlying type.
    fn downcast_into<T: 'static>(self) -> Option<T>;

    /// Returns mutable reference to downcasted type.
    fn downcast_as_mut<T: 'static>(&mut self) -> Option<&mut T>;

    /// Returns reference to downcasted type.
    fn downcast_as_ref<T: 'static>(&self) -> Option<&T>;
}

/// Trait for conversion of self into a [Downcast] type.
pub trait IntoDowncast<T: ?Sized + Downcast> {
    /// Converts self into [Downcast] type.
    fn into(self) -> T;
}

macro_rules! impl_box_downcast {
    ($any_trait:ident $(+ $auto_traits:ident)*) => {
        impl Downcast for Box<dyn $any_trait $(+ $auto_traits)*> {
            #[inline]
            fn downcast_into<T: 'static>(self) -> Option<T> {
                Some(*self.downcast().ok()?)
            }

            #[inline]
            fn downcast_as_mut<T: 'static>(&mut self) -> Option<&mut T> {
                self.as_mut().downcast_mut()
            }

            #[inline]
            fn downcast_as_ref<T: 'static>(&self) -> Option<&T> {
                self.as_ref().downcast_ref()
            }
        }

        impl<T: $any_trait $(+ $auto_traits)*> IntoDowncast<Box<dyn $any_trait $(+ $auto_traits)*>> for T {
            #[inline]
            fn into(self) -> Box<dyn $any_trait $(+ $auto_traits)*> {
                Box::new(self)
            }
        }
    }
}

impl_box_downcast!(Any);
impl_box_downcast!(Any + Send);
impl_box_downcast!(Any + Send + Sync);

#[cfg(test)]
mod tests {
    use super::AnyMap;
    use crate::{Clear, Len, Merge};
    use core::any::Any;

    #[test]
    fn test_new_sync_send() {
        let _map = <AnyMap>::new();
        let _map = AnyMap::<dyn Any + Send>::new();
        let _map = AnyMap::<dyn Any + Send + Sync>::new();
    }

    #[test]
    fn test_clear_len() {
        let mut map = <AnyMap>::new();
        map.insert(1usize);
        assert_eq!(Len::len(&map), 1);
        Clear::clear(&mut map);
        assert!(Len::is_empty(&map));
    }

    #[test]
    fn test_merge() {
        let mut map = <AnyMap>::new();
        map.insert(1usize);
        let mut map2 = <AnyMap>::new();
        map2.insert(2i32);

        let map = Merge::merge(map, map2);
        assert_eq!(Len::len(&map), 2);
        assert_eq!(map.get::<usize>(), Some(&1));
        assert_eq!(map.get::<i32>(), Some(&2));
    }
}
