//! Helpers for cons tuple type.

use core::marker::PhantomData;

/// Constructs a [trait@Cons] based on the values or identifiers passed in.
///
/// # Examples
/// ```
/// # use collections::cons;
/// let (c1, (c2, (c3, ()))) = cons!(123f32, "hello", Some(45));
/// assert_eq!((c1, c2, c3), (123f32, "hello", Some(45)));
///
/// let cons!(c1, c2, c3) = cons!(123f32, "hello", Some(45));
/// assert_eq!(c3, Some(45));
///
/// // LHS patterns needs to be escaped with {}
/// let matched = if let cons!(_, _, {Some(c3)}) = cons!(123f32, "hello", Some(45)) { true } else { false };
/// assert!(matched);
///
/// let cons!(c1, c2, c3, ..rest) = cons!(1, 2, 3, 4, 5);
/// assert_eq!(rest, cons!(4, 5));
///
/// // rev[..] captures in reverse order.
/// let cons!(rev[c1, c2, c3, c4, c5]) = cons!(1, 2, 3, 4, 5);
/// assert_eq!([c1, c2, c3, c4, c5], [5, 4, 3, 2, 1]);
/// ```
#[macro_export]
macro_rules! cons {
    () => { () };

    // Base cases for rev, calls cons! normally
    (rev[{$head:pat_param}] $($reversed:tt)*) => {
        cons!($head, $($reversed)*)
    };
    (rev[$head:tt] $($reversed:tt)*) => {
        cons!($head, $($reversed)*)
    };
    (rev[$head:expr] $($reversed:tt)*) => {
        cons!($head, $($reversed)*)
    };

    // Recursively reverses the rev list
    (rev[{$head:pat_param}, $($rest:tt)*] $($reversed:tt)*) => {
        cons!(rev[$($rest)*] $head, $($reversed)*)
    };
    (rev[$head:tt, $($rest:tt)*] $($reversed:tt)*) => {
        cons!(rev[$($rest)*] $head, $($reversed)*)
    };
    (rev[$head:expr, $($rest:tt)*] $($reversed:tt)*) => {
        cons!(rev[$($rest)*] $head, $($reversed)*)
    };

    // Matches rest params
    (..$rest:tt) => { $rest };
    (..$rest:expr) => { $rest };

    // Base cases, returns single element cons
    ({$head:pat_param}) => { ($head, ()) };
    ($head:tt) => { ($head, ()) };
    ($head:expr) => { ($head, ()) };

    // Recursively builds the cons
    ({$head:pat_param}, $($tail:tt)*) => {
        ($head, cons!($($tail)*))
    };
    ($head:tt, $($tail:tt)*) => {
        ($head, cons!($($tail)*))
    };
    ($head:expr, $($tail:tt)*) => {
        ($head, cons!($($tail)*))
    };
}

/// Returns the concrete [trait@Cons] type signature for the provided types.
///
/// # Examples
/// ```
/// # use collections::{cons, Cons};
/// let c: Cons!(f32, &str, Option<i32>) = cons![123f32, "hello", Some(45)];
/// let c: Cons!(f32, ..Cons!(&str, Option<i32>)) = cons![123f32, "hello", Some(45)];
/// ```
#[macro_export]
macro_rules! Cons {
    () => { () };
    (..$Rest:ty) => { $Rest };
    ($A:ty) => { ($A, ()) };
    ($A:ty, $($Tail:tt)*) => {
        ($A, Cons!($($Tail)*))
    };
}

/// Trait for a [Cons](https://en.wikipedia.org/wiki/Cons).
pub trait Cons: Sized {
    const LEN: usize;

    /// Returns the length of cons.
    ///
    /// # Examples
    /// ```
    /// # use collections::{cons, Cons};
    /// assert_eq!(cons!(1, 2, 3, 4, 5).len(), 5);
    /// ```
    #[inline]
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Returns if the cons is empty.
    ///
    /// # Examples
    /// ```
    /// # use collections::{cons, Cons};
    /// assert!(().is_empty());
    /// assert!(!cons!(1, 2, 3, 4, 5).is_empty());
    /// ```
    #[inline]
    fn is_empty(&self) -> bool {
        Self::LEN == 0
    }

    /// Gets an element by type from this cons.
    ///
    /// # Examples
    /// ```
    /// # use collections::{cons, Cons};
    /// assert_eq!(*cons!(1f32, 1i32, 1u32).get::<i32, _>(), 1i32);
    /// ```
    #[inline]
    fn get<T, Index>(&self) -> &T
    where
        Self: ConsGetter<T, Index>,
    {
        ConsGetter::get(self)
    }

    /// Mutably gets an element by type from this cons.
    /// # Examples
    /// ```
    /// # use collections::{cons, Cons};
    /// let mut c = cons!(1f32, 1i32, 1u32);
    /// *c.get_mut::<i32, _>() = 10;
    /// assert_eq!(c, cons!(1f32, 10i32, 1u32));
    /// ```
    #[inline]
    fn get_mut<T, Index>(&mut self) -> &mut T
    where
        Self: ConsGetter<T, Index>,
    {
        ConsGetter::get_mut(self)
    }

    /// Concats this cons with RHS.
    ///
    /// # Examples
    /// ```
    /// # use collections::{cons, Cons};
    /// let cons!(c1, c2, c3, c4, c5) = cons!(1, 2).concat(cons!(3, 4, 5));
    /// assert_eq!([c1, c2, c3, c4, c5], [1, 2, 3, 4, 5]);
    /// ```
    #[inline]
    fn concat<RHS: Cons>(self, rhs: RHS) -> <Self as Concat<RHS>>::Output
    where
        Self: Concat<RHS>,
    {
        Concat::concat(self, rhs)
    }

    /// Reverse this cons.
    //
    /// # Examples
    /// ```
    /// # use collections::{cons, Cons};
    /// let cons!(c1, c2, c3, c4, c5) = cons!(1, 2, 3, 4, 5).rev();
    /// assert_eq!([c1, c2, c3, c4, c5], [5, 4, 3, 2, 1]);
    /// ```
    #[inline]
    fn rev(self) -> <Self as IntoRev>::Output
    where
        Self: IntoRev,
    {
        IntoRev::rev(self)
    }
}

impl Cons for () {
    const LEN: usize = 0;
}

impl<H, T: Cons> Cons for (H, T) {
    const LEN: usize = 1 + T::LEN;
}

/// Trait for object concatenation.
pub trait Concat<RHS> {
    /// Output type.
    type Output;

    /// Concats with RHS.
    fn concat(self, rhs: RHS) -> Self::Output;
}

impl<RHS> Concat<RHS> for ()
where
    RHS: Cons,
{
    type Output = RHS;

    #[inline(always)]
    fn concat(self, rhs: RHS) -> RHS {
        rhs
    }
}

impl<H, Tail, RHS> Concat<RHS> for (H, Tail)
where
    Tail: Concat<RHS>,
    RHS: Cons,
{
    type Output = (H, <Tail as Concat<RHS>>::Output);

    #[inline(always)]
    fn concat(self, rhs: RHS) -> Self::Output {
        (self.0, self.1.concat(rhs))
    }
}

/// Trait for reversing self.
pub trait IntoRev {
    /// Output type.
    type Output;

    /// Revert self.
    fn rev(self) -> Self::Output;
}

impl IntoRev for () {
    type Output = ();

    #[inline(always)]
    fn rev(self) -> Self::Output {
        self
    }
}

impl<T, Tail> IntoRev for (T, Tail)
where
    Tail: IntoRev,
    <Tail as IntoRev>::Output: Concat<(T, ())>,
{
    type Output = <<Tail as IntoRev>::Output as Concat<(T, ())>>::Output;

    #[inline(always)]
    fn rev(self) -> Self::Output {
        self.1.rev().concat((self.0, ()))
    }
}

/// Trait for getting a [trait@Cons] element by type.
pub trait ConsGetter<T, I> {
    /// Gets an element by type from cons.
    fn get(&self) -> &T;

    /// Mutably gets an element by type from cons.
    fn get_mut(&mut self) -> &mut T;
}

impl<T, Tail> ConsGetter<T, Here> for (T, Tail) {
    #[inline(always)]
    fn get(&self) -> &T {
        &self.0
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<Head, Tail, FromTail, TailIndex> ConsGetter<FromTail, There<TailIndex>> for (Head, Tail)
where
    Tail: ConsGetter<FromTail, TailIndex>,
{
    #[inline(always)]
    fn get(&self) -> &FromTail {
        self.1.get()
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut FromTail {
        self.1.get_mut()
    }
}

/// Used as an index into a [trait@Cons].
pub struct Here;

/// Used as an index into a [trait@Cons].
pub struct There<T> {
    marker: PhantomData<T>,
}
