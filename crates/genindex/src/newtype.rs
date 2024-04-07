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
    value: I,
    marker: PhantomData<*const T>,
}

impl<T, I: GenIndex> Clone for NewTypeIndex<T, I> {
    #[inline]
    fn clone(&self) -> Self {
        self.value.into()
    }
}

impl<T, I: GenIndex> Copy for NewTypeIndex<T, I> {}

impl<T, I: GenIndex> Default for NewTypeIndex<T, I> {
    #[inline]
    fn default() -> Self {
        NewTypeIndex {
            value: Default::default(),
            marker: PhantomData,
        }
    }
}

impl<T, I: Debug + GenIndex> Debug for NewTypeIndex<T, I> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T, I: GenIndex + Hash> Hash for NewTypeIndex<T, I> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T, I: GenIndex> PartialOrd for NewTypeIndex<T, I> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T, I: GenIndex> PartialEq for NewTypeIndex<T, I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
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
            value: I::from_raw_parts(index, generation),
            marker: PhantomData,
        }
    }

    #[inline]
    fn index(&self) -> Self::Index {
        self.value.index()
    }

    #[inline]
    fn generation(&self) -> Self::Generation {
        self.value.generation()
    }
}

impl<T, I: GenIndex> From<I> for NewTypeIndex<T, I> {
    #[inline]
    fn from(index: I) -> Self {
        NewTypeIndex {
            value: index,
            marker: PhantomData,
        }
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

#[cfg(test)]
mod tests {
    use crate::{GenIndex, NewTypeIndex};

    struct TestType;

    #[test]
    fn test_create() {
        let index: NewTypeIndex<TestType> = NewTypeIndex::from_raw_parts(0, 0);
        assert_eq!(index, NewTypeIndex::default());

        let index: NewTypeIndex<TestType> = (2, 3).into();
        assert_eq!((index.index(), index.generation()), index.into());
    }

    #[test]
    fn test_cmp() {
        assert!(
            NewTypeIndex::<TestType>::from_raw_parts(1, 1) < NewTypeIndex::from_raw_parts(2, 1)
        );
        assert!(
            NewTypeIndex::<TestType>::from_raw_parts(1, 3) < NewTypeIndex::from_raw_parts(2, 1)
        );

        assert_eq!(
            NewTypeIndex::<TestType>::from_raw_parts(1, 3),
            NewTypeIndex::from_raw_parts(1, 3)
        );
        assert_ne!(
            NewTypeIndex::<TestType>::from_raw_parts(1, 3),
            NewTypeIndex::from_raw_parts(1, 2)
        );

        assert!(
            !(NewTypeIndex::<TestType>::from_raw_parts(1, 3) < NewTypeIndex::from_raw_parts(1, 4))
        );
        assert!(
            !(NewTypeIndex::<TestType>::from_raw_parts(1, 4) < NewTypeIndex::from_raw_parts(1, 3))
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        use crate::IndexPair;
        use alloc::vec;
        use serde_json::{json, Value};

        let expected_index = NewTypeIndex::<TestType, IndexPair>::from_raw_parts(123, 456);
        let json: Value = json!([123, 456]);

        let index: NewTypeIndex<TestType, IndexPair> = serde_json::from_value(json).unwrap();

        assert_eq!(index, expected_index);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        use crate::IndexPair;
        use alloc::vec;
        use serde_json::{json, Value};

        let index = NewTypeIndex::<TestType, IndexPair>::from_raw_parts(123, 456);
        let expected_json: Value = json!([123, 456]);

        let json: Value = serde_json::to_value(index).unwrap();

        assert_eq!(json, expected_json);
    }
}
