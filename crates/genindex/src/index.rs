use crate::GenIndex;
use core::{cmp::Ordering, fmt::Debug, hash::Hash};
use num::{Bounded, Unsigned, Zero};

/// A standard [GenIndex] with usize index and usize generation
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Index<I = usize, G = usize>(I, G);

impl<I: Copy + PartialOrd + Unsigned, G: Bounded + Copy + PartialOrd + Unsigned> GenIndex
    for Index<I, G>
{
    type Index = I;
    type Generation = G;

    #[inline]
    fn max_generation() -> Self::Generation {
        G::max_value()
    }

    #[inline]
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self {
        (index, generation).into()
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

impl<I, G> From<Index<I, G>> for (I, G) {
    #[inline]
    fn from(idx: Index<I, G>) -> Self {
        (idx.0, idx.1)
    }
}

impl<I, G> From<(I, G)> for Index<I, G> {
    #[inline]
    fn from((index, generation): (I, G)) -> Self {
        Self(index, generation)
    }
}

impl<I: Zero, G: Zero> Default for Index<I, G> {
    #[inline]
    fn default() -> Self {
        Self(I::zero(), G::zero())
    }
}

impl<I: PartialOrd, G: PartialOrd> PartialOrd for Index<I, G> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0)? {
            Ordering::Equal => {
                if self.1 == other.1 {
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
    use crate::{GenIndex, Index};

    #[test]
    fn test_create() {
        let index: Index<u32, u16> = Index::from_raw_parts(0, 0);
        assert_eq!(index, Index::default());

        let index: Index<u32, u16> = (2, 3).into();
        assert_eq!((index.index(), index.generation()), index.into());
    }

    #[test]
    fn test_cmp() {
        assert!(<Index>::from_raw_parts(1, 1) < Index::from_raw_parts(2, 1));
        assert!(<Index>::from_raw_parts(1, 3) < Index::from_raw_parts(2, 1));

        assert_eq!(<Index>::from_raw_parts(1, 3), Index::from_raw_parts(1, 3));
        assert_ne!(<Index>::from_raw_parts(1, 3), Index::from_raw_parts(1, 2));

        assert!(!(<Index>::from_raw_parts(1, 3) < Index::from_raw_parts(1, 4)));
        assert!(!(<Index>::from_raw_parts(1, 4) < Index::from_raw_parts(1, 3)));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        use alloc::vec;
        use serde_json::{json, Value};

        let expected_index = Index::from_raw_parts(123, 456);
        let json: Value = json!([123, 456]);

        let index: Index = serde_json::from_value(json).unwrap();

        assert_eq!(index, expected_index);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        use alloc::vec;
        use serde_json::{json, Value};

        let index: Index = Index::from_raw_parts(123, 456);
        let expected_json: Value = json!([123, 456]);

        let json: Value = serde_json::to_value(index).unwrap();

        assert_eq!(json, expected_json);
    }
}
