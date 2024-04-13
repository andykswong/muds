use crate::{Clear, Dequeue, Len, Merge, Push};
use alloc::collections::BinaryHeap;

impl<V> Len for BinaryHeap<V> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<V> Clear for BinaryHeap<V> {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }
}

impl<V: Ord> Merge for BinaryHeap<V> {
    type Output = Self;

    #[inline]
    fn merge(mut self, mut rhs: Self) -> Self {
        self.append(&mut rhs);
        self
    }
}

impl<V: Ord> Push for BinaryHeap<V> {
    type Index = ();
    type Value = V;

    #[inline]
    fn push(&mut self, value: Self::Value) -> Self::Index {
        self.push(value);
        ()
    }
}

impl<V: Ord> Dequeue for BinaryHeap<V> {
    type Value = V;

    #[inline]
    fn dequeue(&mut self) -> Option<Self::Value> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Clear, Dequeue, Len, Merge, Push};
    use alloc::collections::BinaryHeap;

    fn create_map() -> BinaryHeap<u32> {
        let mut map = BinaryHeap::new();
        for i in 0..10 {
            map.push(i);
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
    fn test_push() {
        let mut map = BinaryHeap::new();
        map.push(1);
        Push::push(&mut map, 2);
        assert_eq!(map.peek(), Some(&2));
    }

    #[test]
    fn test_dequeue() {
        let mut map = create_map();
        assert_eq!(Dequeue::dequeue(&mut map), Some(9));
    }

    #[test]
    fn test_merge() {
        let mut map = BinaryHeap::new();
        map.push(1);
        let mut map2 = BinaryHeap::new();
        map2.push(2);

        let map = Merge::merge(map, map2);
        assert_eq!(map.len(), 2);
    }
}
