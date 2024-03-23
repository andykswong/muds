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

impl From<IndexU64> for u64 {
    #[inline]
    fn from(idx: IndexU64) -> Self {
        idx.0
    }
}

impl From<u64> for IndexU64 {
    #[inline]
    fn from(value: u64) -> Self {
        IndexU64(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{GenIndex, IndexU64};

    #[test]
    fn test_create() {
        let index: IndexU64 = IndexU64::from_raw_parts(0, 0);
        assert_eq!(index, IndexU64::default());

        let index: IndexU64 = (2, 3).into();
        assert_eq!((index.index(), index.generation()), index.into());

        assert_eq!((3 << 32) | 2, Into::<u64>::into(index));
        assert_eq!(Into::<IndexU64>::into((3 << 32) | 2), index);
    }

    #[test]
    fn test_cmp() {
        assert!(IndexU64::from_raw_parts(1, 1) < IndexU64::from_raw_parts(2, 1));
        assert!(IndexU64::from_raw_parts(1, 3) < IndexU64::from_raw_parts(2, 1));

        assert_eq!(IndexU64::from_raw_parts(1, 3), IndexU64::from_raw_parts(1, 3));
        assert_ne!(IndexU64::from_raw_parts(1, 3), IndexU64::from_raw_parts(1, 2));

        assert!(!(IndexU64::from_raw_parts(1, 3) < IndexU64::from_raw_parts(1, 4)));
        assert!(!(IndexU64::from_raw_parts(1, 4) < IndexU64::from_raw_parts(1, 3)));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        use serde_json::{json, Value};

        let expected_index = IndexU64::from_raw_parts(123, 456);
        let json: Value = json!(456u64 << 32 | 123);

        let index: IndexU64 = serde_json::from_value(json).unwrap();

        assert_eq!(index, expected_index);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        use serde_json::{json, Value};

        let index = IndexU64::from_raw_parts(123, 456);
        let expected_json: Value = json!(456u64 << 32 | 123);

        let json: Value = serde_json::to_value(index).unwrap();

        assert_eq!(json, expected_json);
    }
}
