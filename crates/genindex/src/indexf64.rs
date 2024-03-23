use crate::GenIndex;
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

/// A [GenIndex] that is stored as f64, which 32bit index and 21bit generation.
/// Useful for interfacing with Javascript
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct IndexF64(f64);

/// Equals 2^21 - 1. f64 can safely store integer up to 2^53.
/// We used 32bits for the index, leaving 21bits for generation.
const MAX_SAFE_GENERATION: u32 = (1 << 21) - 1;

/// f64 can safely store integer up to 2^53.
const MAX_SAFE_VALUE: u64 = (1 << 53) - 1;

impl GenIndex for IndexF64 {
    type Index = u32;
    type Generation = u32;

    #[inline]
    fn max_generation() -> Self::Generation {
        MAX_SAFE_GENERATION
    }

    #[inline]
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self {
        Self(index as f64 + (((generation & MAX_SAFE_GENERATION) as u64) << 32) as f64)
    }

    #[inline]
    fn index(&self) -> Self::Index {
        (self.0 as u64 & (u32::MAX as u64)) as u32
    }

    #[inline]
    fn generation(&self) -> Self::Generation {
        ((self.0 as u64) >> 32) as u32
    }
}

impl Hash for IndexF64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0 as u64).hash(state);
    }
}

impl From<IndexF64> for (u32, u32) {
    #[inline]
    fn from(idx: IndexF64) -> Self {
        (idx.index(), idx.generation())
    }
}

impl From<(u32, u32)> for IndexF64 {
    #[inline]
    fn from((index, generation): (u32, u32)) -> Self {
        IndexF64::from_raw_parts(index, generation)
    }
}

impl From<IndexF64> for f64 {
    #[inline]
    fn from(idx: IndexF64) -> Self {
        idx.0
    }
}

impl From<f64> for IndexF64 {
    #[inline]
    fn from(value: f64) -> Self {
        let value = (value as u64) & MAX_SAFE_VALUE;
        IndexF64(value as f64)
    }
}

impl PartialOrd for IndexF64 {
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

#[cfg(test)]
mod tests {
    use crate::{GenIndex, IndexF64};

    #[test]
    fn test_create() {
        let index: IndexF64 = IndexF64::from_raw_parts(0, 0);
        assert_eq!(index, IndexF64::default());

        let index: IndexF64 = (2, 3).into();
        assert_eq!((index.index(), index.generation()), index.into());

        assert_eq!(((3u64 << 32) | 2) as f64, Into::<f64>::into(index));
        assert_eq!(Into::<IndexF64>::into(((3u64 << 32) | 2) as f64 + 0.5), index);
    }

    #[test]
    fn test_cmp() {
        assert!(IndexF64::from_raw_parts(1, 1) < IndexF64::from_raw_parts(2, 1));
        assert!(IndexF64::from_raw_parts(1, 3) < IndexF64::from_raw_parts(2, 1));

        assert_eq!(IndexF64::from_raw_parts(1, 3), IndexF64::from_raw_parts(1, 3));
        assert_ne!(IndexF64::from_raw_parts(1, 3), IndexF64::from_raw_parts(1, 2));

        assert!(!(IndexF64::from_raw_parts(1, 3) < IndexF64::from_raw_parts(1, 4)));
        assert!(!(IndexF64::from_raw_parts(1, 4) < IndexF64::from_raw_parts(1, 3)));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        use serde_json::{json, Value};

        let expected_index = IndexF64::from_raw_parts(123, 456);
        let json: Value = json!((456u64 << 32 | 123) as f64);

        let index: IndexF64 = serde_json::from_value(json).unwrap();

        assert_eq!(index, expected_index);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        use crate::{GenIndex, IndexF64};
        use serde_json::{json, Value};

        let index = IndexF64::from_raw_parts(123, 456);
        let expected_json: Value = json!((456u64 << 32 | 123) as f64);

        let json: Value = serde_json::to_value(index).unwrap();

        assert_eq!(json, expected_json);
    }
}
