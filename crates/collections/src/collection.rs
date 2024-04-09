//! Collection traits.

/// A collection with length measure.
pub trait Len {
    /// Returns the number of elements in the collection.
    fn len(&self) -> usize;

    /// Returns `true` if the collection contains no elements.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Operation to clear a collection.
pub trait Clear {
    /// Clears self, removing all values.
    fn clear(&mut self);
}

/// Operation to decide which collection elements to retain.
pub trait Retain {
    /// Key type
    type Key;

    /// Value type
    type Value;

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(&index, &mut value)` returns `false`.
    fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool);
}

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

    /// Removes the top element from collection and returns it, or None if it is empty.
    fn pop(&mut self) -> Option<Self::Value>;
}
