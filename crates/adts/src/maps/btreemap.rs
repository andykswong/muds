use crate::{Clear, Len, Map, MapGet, MapInsert, MapMut, Retain};
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

impl<K, V> Map for BTreeMap<K, V> {
    type Key = K;
    type Value = V;
}

impl<B: ?Sized + Ord, K: Borrow<B> + Ord, V> MapGet<B> for BTreeMap<K, V> {
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

#[cfg(test)]
mod tests {
    use crate::{Clear, Len, MapGet, MapInsert, MapMut, Retain};
    use alloc::{collections::BTreeMap, format, string::String};

    fn create_map() -> BTreeMap<String, u32> {
        let mut map = BTreeMap::new();
        for i in 0..10 {
            map.insert(format!("{i}"), i);
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
        assert!(MapGet::contains_key(&map, "0"));
        assert_eq!(MapGet::get(&map, "1"), Some(&1));
        assert_eq!(MapGet::get(&map, "11"), None);
    }

    #[test]
    fn test_map_mut() {
        let mut map = create_map();

        let new_value = 123;
        *MapMut::get_mut(&mut map, "1").unwrap() = new_value;
        assert_eq!(map["1"], new_value);

        assert_eq!(MapMut::remove(&mut map, "1"), Some(new_value));
        assert_eq!(MapGet::get(&map, "1"), None);
    }

    #[test]
    fn test_map_insert() {
        let mut map = create_map();

        let new_value = 123;
        assert_eq!(MapInsert::insert(&mut map, "1".into(), new_value), Some(1));
        assert_eq!(map["1"], new_value);

        assert_eq!(MapInsert::insert(&mut map, "999".into(), new_value), None);
        assert_eq!(map["999"], new_value);
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
        assert_eq!(map["1"], 3);
    }
}
