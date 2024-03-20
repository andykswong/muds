use crate::{GenIndex, IndexF64};
use core::{
    cmp::Ordering,
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

/// A [GenIndex] newtype.
#[derive(Eq, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct NewTypeIndex<T, I: GenIndex = IndexF64> {
    index: I,
    marker: PhantomData<*const T>,
}

impl<T, I: GenIndex> NewTypeIndex<T, I> {
    #[inline]
    pub fn from_index(index: I) -> Self {
        Self {
            index,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn to_index(&self) -> I {
        self.index
    }
}

impl<T, I: GenIndex> Clone for NewTypeIndex<T, I> {
    #[inline]
    fn clone(&self) -> Self {
        Self::from_index(self.index.clone())
    }
}

impl<T, I: GenIndex> Copy for NewTypeIndex<T, I> {}

impl<T, I: GenIndex> Default for NewTypeIndex<T, I> {
    #[inline]
    fn default() -> Self {
        Self::from_index(Default::default())
    }
}

impl<T, I: GenIndex> Debug for NewTypeIndex<T, I> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.index.fmt(f)
    }
}

impl<T, I: GenIndex> Hash for NewTypeIndex<T, I> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl<T, I: GenIndex> PartialOrd for NewTypeIndex<T, I> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<T, I: GenIndex> PartialEq for NewTypeIndex<T, I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl<T, I: GenIndex> GenIndex for NewTypeIndex<T, I> {
    type Index = I::Index;

    type Generation = I::Generation;

    #[inline]
    fn max_generation() -> Self::Generation {
        I::max_generation()
    }

    #[inline]
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self {
        Self {
            index: I::from_raw_parts(index, generation),
            marker: PhantomData,
        }
    }

    #[inline]
    fn index(&self) -> Self::Index {
        self.index.index()
    }

    #[inline]
    fn generation(&self) -> Self::Generation {
        self.index.generation()
    }
}

impl<T, I: GenIndex> From<NewTypeIndex<T, I>> for (I::Index, I::Generation) {
    #[inline]
    fn from(idx: NewTypeIndex<T, I>) -> Self {
        (idx.index(), idx.generation())
    }
}

impl<T, I: GenIndex> From<(I::Index, I::Generation)> for NewTypeIndex<T, I> {
    #[inline]
    fn from((index, generation): (I::Index, I::Generation)) -> Self {
        NewTypeIndex::from_raw_parts(index, generation)
    }
}

// endregion: TypedIndex

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        use crate::{GenIndex, Index, NewTypeIndex};
        use alloc::vec;
        use serde_json::{json, Value};

        struct TestType;

        let expected_index = NewTypeIndex::<TestType, Index>::from_raw_parts(123, 456);
        let json: Value = json!([123, 456]);

        let index: NewTypeIndex<TestType, Index> = serde_json::from_value(json).unwrap();

        assert_eq!(index, expected_index);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        use crate::{GenIndex, Index, NewTypeIndex};
        use alloc::vec;
        use serde_json::{json, Value};

        struct TestType;

        let index = NewTypeIndex::<TestType, Index>::from_raw_parts(123, 456);
        let expected_json: Value = json!([123, 456]);

        let json: Value = serde_json::to_value(index).unwrap();

        assert_eq!(json, expected_json);
    }
}
