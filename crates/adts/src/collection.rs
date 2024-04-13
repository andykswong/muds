//! Generic collection traits.

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

/// Operation to merge collections.
pub trait Merge<RHS = Self> {
    /// Output type.
    type Output;

    /// Returns self merged with `rhs`.
    fn merge(self, rhs: RHS) -> Self::Output;
}

/// Operation to reverse a collection.
 pub trait Rev {
    /// Output type.
    type Output;

    /// Returns self in reversed order.
    fn rev(self) -> Self::Output;
}
