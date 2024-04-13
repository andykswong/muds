use crate::{Clear, Len, Map, MapGet, MapInsert, MapMut, MapRemove, Merge, Retain};
use core::{
    borrow::Borrow,
    hash::{BuildHasher, Hash},
};
use std::collections::HashMap;

impl<K, V, S> Len for HashMap<K, V, S> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V, S> Clear for HashMap<K, V, S> {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }
}

impl<K, V, S> Map for HashMap<K, V, S> {
    type Key = K;
    type Value = V;
}

impl<B: ?Sized + Eq + Hash, K: Borrow<B> + Eq + Hash, V, S: BuildHasher> MapGet<B>
    for HashMap<K, V, S>
{
    #[inline]
    fn get(&self, key: &B) -> Option<&Self::Value> {
        self.get(key)
    }

    #[inline]
    fn contains_key(&self, key: &B) -> bool {
        self.contains_key(key)
    }
}

impl<B: ?Sized + Eq + Hash, K: Borrow<B> + Eq + Hash, V, S: BuildHasher> MapMut<B>
    for HashMap<K, V, S>
{
    #[inline]
    fn get_mut(&mut self, key: &B) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }
}

impl<B: ?Sized + Eq + Hash, K: Borrow<B> + Eq + Hash, V, S: BuildHasher> MapRemove<B>
    for HashMap<K, V, S>
{
    #[inline]
    fn remove(&mut self, key: &B) -> Option<(Self::Key, Self::Value)> {
        self.remove_entry(key)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> MapInsert for HashMap<K, V, S> {
    #[inline]
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.insert(key, value)
    }
}

impl<K, V, S> Retain for HashMap<K, V, S> {
    type Key = K;
    type Value = V;

    #[inline]
    fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
        self.retain(f);
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Merge for HashMap<K, V, S> {
    type Output = Self;

    #[inline]
    fn merge(mut self, other: Self) -> Self::Output {
        self.extend(other.into_iter());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{Clear, Len, MapGet, MapInsert, MapMut, MapRemove, Merge, Retain};
    use alloc::{format, string::String};
    use std::collections::HashMap;

    fn create_map() -> HashMap<String, u32> {
        let mut map = HashMap::new();
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
    }

    #[test]
    fn test_map_remove() {
        let mut map = create_map();
        assert_eq!(MapRemove::remove(&mut map, "1"), Some(("1".into(), 1)));
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

    #[test]
    fn test_merge() {
        let mut map = HashMap::new();
        map.insert("1", 1);
        let mut map2 = HashMap::new();
        map2.insert("2", 2);

        let map = Merge::merge(map, map2);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&1));
        assert_eq!(map.get("2"), Some(&2));
    }
}
