use crate::{Clear, Len, Map, MapGet, MapInsert, MapMut};
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
        M: MapMut<AnyMapKey>,
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
        M: MapMut<AnyMapKey>,
    {
        self.map.remove(key)?.downcast_into()
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

mod collections_impl {
    use super::AnyMap;
    use crate::{Clear, Len};

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
    use crate::{Clear, Len};
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
}
