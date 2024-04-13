//! Queue and stack traits.

/// Operation to push new value into collection.
pub trait Push {
    /// Index type
    type Index;

    /// Value type
    type Value;

    /// Pushes new `value` into the collection and returns the element's assigned index.
    fn push(&mut self, value: Self::Value) -> Self::Index;
}

/// Operation to pop value from collection.
pub trait Pop {
    /// Value type
    type Value;

    /// Removes the last element from collection and returns it, or None if it is empty.
    fn pop(&mut self) -> Option<Self::Value>;
}

/// Operation to dequeue value from collection.
pub trait Dequeue {
    /// Value type
    type Value;

    /// Removes the top element from collection and returns it, or None if it is empty.
    fn dequeue(&mut self) -> Option<Self::Value>;
}

/// Returns the natural key for given value.
pub trait Key<K> {
    fn key(&self) -> K;
}

impl<T: Clone> Key<T> for T {
    #[inline]
    fn key(&self) -> T {
        self.clone()
    }
}
