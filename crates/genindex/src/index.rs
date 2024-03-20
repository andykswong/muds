use crate::{GenIndex, UnsignedNum};
use core::{cmp::Ordering, fmt::Debug, hash::Hash};
use num::Bounded;

/// A standard [GenIndex] with usize index and usize generation
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Index<I: UnsignedNum = usize, G: Bounded + UnsignedNum = usize>(I, G);

impl<I: UnsignedNum, G: Bounded + UnsignedNum> Default for Index<I, G> {
    fn default() -> Self {
        Self::from_raw_parts(I::zero(), G::zero())
    }
}

impl<I: UnsignedNum, G: Bounded + UnsignedNum> GenIndex for Index<I, G> {
    type Index = I;
    type Generation = G;

    /// Returns the maximum generation value.
    #[inline]
    fn max_generation() -> Self::Generation {
        G::max_value()
    }

    #[inline]
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self {
        Self(index, generation)
    }

    #[inline]
    fn index(&self) -> Self::Index {
        self.0
    }

    #[inline]
    fn generation(&self) -> Self::Generation {
        self.1
    }
}

impl<I: UnsignedNum, G: Bounded + UnsignedNum> From<Index<I, G>> for (I, G) {
    #[inline]
    fn from(idx: Index<I, G>) -> Self {
        (idx.0, idx.1)
    }
}

impl<I: UnsignedNum, G: Bounded + UnsignedNum> From<(I, G)> for Index<I, G> {
    #[inline]
    fn from((index, generation): (I, G)) -> Self {
        Index::from_raw_parts(index, generation)
    }
}

impl<I: UnsignedNum, G: Bounded + UnsignedNum> PartialOrd for Index<I, G> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.index().cmp(&other.index()) {
            Ordering::Equal => {
                if self.generation() == other.generation() {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            ordering => Some(ordering),
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn test_index_deserialize() {
        use crate::{GenIndex, Index};
        use alloc::vec;
        use serde_json::{json, Value};

        let expected_index = Index::from_raw_parts(123, 456);
        let json: Value = json!([123, 456]);

        let index: Index = serde_json::from_value(json).unwrap();

        assert_eq!(index, expected_index);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_index_serialize() {
        use crate::{GenIndex, Index};
        use alloc::vec;
        use serde_json::{json, Value};

        let index: Index = Index::from_raw_parts(123, 456);
        let expected_json: Value = json!([123, 456]);

        let json: Value = serde_json::to_value(index).unwrap();

        assert_eq!(json, expected_json);
    }
}
