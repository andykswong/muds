use num::{One, Unsigned, Zero};

/// An index with generation that can be used as a weak reference to array values.
/// The generation part allows indices to be reused without suffering from [ABA problem](https://en.wikipedia.org/wiki/ABA_problem),
/// so that data can be safely stored in a packed array.
pub trait GenIndex: Copy + Default + PartialEq + PartialOrd {
    /// The type of index value.
    type Index: Unsigned;

    /// The type of generation value.
    type Generation: Unsigned;

    /// Returns the maximum generation value.
    fn max_generation() -> Self::Generation;

    /// Create a new `GenIndex` from its raw parts.
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self;

    /// Returns the index value of this `GenIndex`.
    fn index(&self) -> Self::Index;

    /// Returns the generation value of this `GenIndex`.
    fn generation(&self) -> Self::Generation;

    /// Returns a null value.
    ///
    /// # Examples
    /// ```
    /// # use genindex::{GenIndex, IndexU64};
    /// assert_eq!(IndexU64::null(), IndexU64::default());
    /// ```
    #[inline]
    fn null() -> Self {
        Default::default()
    }

    /// Checks if the value represents null.
    ///
    /// # Examples
    /// ```
    /// # use genindex::{GenIndex, IndexU64};
    /// assert!(IndexU64::null().is_null());
    /// ```
    #[inline]
    fn is_null(&self) -> bool {
        *self == Self::null()
    }

    /// Returns the next generation value.
    ///
    /// # Examples
    /// ```
    /// # use genindex::{GenIndex, IndexU64};
    /// assert_eq!(IndexU64::from_raw_parts(10, 11).next_generation(), IndexU64::from_raw_parts(10, 12));
    /// ```
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
