use crate::{
    Clear, Dequeue, Len, Map, MapGet, MapInsert, MapMut, MapRemove, Merge, Pop, Push, Retain, Rev,
};
use alloc::collections::VecDeque;
use core::mem::replace;

impl<T> Len for VecDeque<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Clear for VecDeque<T> {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T> Push for VecDeque<T> {
    type Index = usize;
    type Value = T;

    #[inline]
    fn push(&mut self, value: Self::Value) -> Self::Index {
        self.push_back(value);
        self.len() - 1
    }
}

impl<T> Pop for VecDeque<T> {
    type Value = T;

    #[inline]
    fn pop(&mut self) -> Option<Self::Value> {
        self.pop_back()
    }
}

impl<T> Dequeue for VecDeque<T> {
    type Value = T;

    #[inline]
    fn dequeue(&mut self) -> Option<Self::Value> {
        self.pop_front()
    }
}

impl<T> Merge for VecDeque<T> {
    type Output = Self;

    #[inline]
    fn merge(mut self, mut other: Self) -> Self::Output {
        self.append(&mut other);
        self
    }
}

impl<T> Rev for VecDeque<T> {
    type Output = Self;

    #[inline]
    fn rev(self) -> Self::Output {
        self.into_iter().rev().collect()
    }
}

impl<T> Map for VecDeque<T> {
    type Key = usize;
    type Value = T;
}

impl<T> MapGet<usize> for VecDeque<T> {
    #[inline]
    fn get(&self, key: &usize) -> Option<&Self::Value> {
        self.get(*key)
    }

    #[inline]
    fn contains_key(&self, key: &usize) -> bool {
        self.len() > *key
    }
}

impl<T> MapMut<usize> for VecDeque<T> {
    #[inline]
    fn get_mut(&mut self, key: &usize) -> Option<&mut Self::Value> {
        self.get_mut(*key)
    }
}

impl<T> MapRemove<usize> for VecDeque<T> {
    /// Removes and returns the element at given index.
    /// Whichever end is closer to the removal point will be moved to make room.
    #[inline]
    fn remove(&mut self, key: &usize) -> Option<(Self::Key, Self::Value)> {
        if self.contains_key(key) {
            Some((*key, self.remove(*key)?))
        } else {
            None
        }
    }
}

impl<T: Default> MapInsert for VecDeque<T> {
    /// Replaces an element at given index.
    #[inline]
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        if self.len() > key {
            Some(replace(&mut self[key], value))
        } else {
            None
        }
    }
}

impl<T> Retain for VecDeque<T> {
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

#[cfg(test)]
mod tests {
    use crate::{
        Clear, Dequeue, Len, MapGet, MapInsert, MapMut, MapRemove, Merge, Pop, Push, Retain, Rev,
    };
    use alloc::{collections::VecDeque, vec};

    #[test]
    fn test_clear_len() {
        let mut vec: VecDeque<i32> = vec![0, 1, 2].into();
        assert_eq!(Len::len(&vec), 3);
        Clear::clear(&mut vec);
        assert!(Len::is_empty(&vec));
    }

    #[test]
    fn test_map_get() {
        let vec: VecDeque<i32> = vec![0, 1, 2].into();
        assert!(MapGet::contains_key(&vec, &0));
        assert_eq!(MapGet::get(&vec, &1), Some(&1));
        assert_eq!(MapGet::get(&vec, &3), None);
    }

    #[test]
    fn test_push() {
        let mut vec: VecDeque<i32> = vec![0, 1, 2].into();
        assert_eq!(Push::push(&mut vec, 3), 3);
        assert_eq!(vec, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_pop() {
        let mut vec: VecDeque<i32> = vec![0, 1, 2].into();
        assert_eq!(Pop::pop(&mut vec), Some(2));
        assert_eq!(vec, vec![0, 1]);
    }

    #[test]
    fn test_dequeue() {
        let mut vec: VecDeque<i32> = vec![0, 1, 2].into();
        assert_eq!(Dequeue::dequeue(&mut vec), Some(0));
        assert_eq!(vec, vec![1, 2]);
    }

    #[test]
    fn test_concat() {
        let vec: VecDeque<i32> = vec![0, 1, 2].into();
        let vec2 = vec![3, 4].into();
        assert_eq!(Merge::merge(vec, vec2), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_rev() {
        let vec: VecDeque<i32> = vec![0, 1, 2].into();
        assert_eq!(Rev::rev(vec), vec![2, 1, 0]);
    }

    #[test]
    fn test_map_mut() {
        let mut vec: VecDeque<i32> = vec![0, 1, 2].into();

        let new_value = 123;
        *MapMut::get_mut(&mut vec, &1).unwrap() = new_value;
        assert_eq!(vec, vec![0, new_value, 2]);
    }

    #[test]
    fn test_map_remove() {
        let mut vec: VecDeque<i32> = vec![0, 1, 2].into();
        assert_eq!(MapRemove::remove(&mut vec, &1), Some((1, 1)));
        assert_eq!(vec, vec![0, 2]);
    }

    #[test]
    fn test_map_insert() {
        let mut vec: VecDeque<i32> = vec![0, 1, 2].into();

        let new_value = 123;
        assert_eq!(MapInsert::insert(&mut vec, 1, new_value), Some(1));
        assert_eq!(vec, vec![0, new_value, 2]);

        assert_eq!(MapInsert::insert(&mut vec, 999, new_value), None);
        assert_eq!(vec, vec![0, new_value, 2]);
    }

    #[test]
    fn test_retain() {
        let mut vec: VecDeque<i32> = vec![0, 1, 2].into();

        Retain::retain(&mut vec, |_, val| {
            if *val == 1 {
                *val = 3;
                true
            } else {
                false
            }
        });
        assert_eq!(vec, vec![3]);
    }
}
