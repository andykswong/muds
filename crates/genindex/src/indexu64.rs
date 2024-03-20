use crate::GenIndex;
use core::cmp::Ordering;

/// A [GenIndex] that is stored as u64, which 32bit index and 32bit generation.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct IndexU64(u64);

impl GenIndex for IndexU64 {
    type Index = u32;
    type Generation = u32;

    #[inline]
    fn max_generation() -> Self::Generation {
        u32::MAX
    }

    #[inline]
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self {
        Self(index as u64 + ((generation as u64) << 32))
    }

    #[inline]
    fn index(&self) -> Self::Index {
        (self.0 & (u32::MAX as u64)) as u32
    }

    #[inline]
    fn generation(&self) -> Self::Generation {
        (self.0 >> 32) as u32
    }
}

impl PartialOrd for IndexU64 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else {
            match self.index().cmp(&other.index()) {
                Ordering::Equal => None,
                ordering => Some(ordering),
            }
        }
    }
}

impl From<IndexU64> for (u32, u32) {
    #[inline]
    fn from(idx: IndexU64) -> Self {
        (idx.index(), idx.generation())
    }
}

impl From<(u32, u32)> for IndexU64 {
    #[inline]
    fn from((index, generation): (u32, u32)) -> Self {
        IndexU64::from_raw_parts(index, generation)
    }
}
