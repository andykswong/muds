use crate::UnsignedNum;
use core::{fmt::Debug, hash::Hash};
use num::{One, Zero};

/// An index with generation that can be used as a weak reference to array values.
/// The generation part allows indices to be reused without suffering from [ABA problem](https://en.wikipedia.org/wiki/ABA_problem),
/// so that data can be safely stored in a packed array.
pub trait GenIndex: Copy + Debug + Default + Hash + PartialEq + PartialOrd {
    /// The type of index value.
    type Index: UnsignedNum;

    /// The type of generation value.
    type Generation: UnsignedNum;

    /// Returns the maximum generation value.
    fn max_generation() -> Self::Generation;

    /// Create a new `GenIndex` from its raw parts.
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self;

    /// Returns the index value of this `GenIndex`.
    fn index(&self) -> Self::Index;

    /// Returns the generation value of this `GenIndex`.
    fn generation(&self) -> Self::Generation;

    /// Returns a null value.
    #[inline]
    fn null() -> Self {
        Default::default()
    }

    /// Checks if the value represents null.
    #[inline]
    fn is_null(&self) -> bool {
        *self == Self::null()
    }

    /// Returns the next generation value.
    #[inline]
    fn next_generation(&self) -> Self {
        let next_gen = self.generation();
        let next_gen = if next_gen == Self::max_generation() {
            Self::Generation::zero()
        } else {
            next_gen + Self::Generation::one()
        };
        Self::from_raw_parts(self.index(), next_gen)
    }
}
