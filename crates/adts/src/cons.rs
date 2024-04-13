//! Helpers for cons tuple type.

use crate::{Merge, Rev};
use core::marker::PhantomData;

/// Constructs a [trait@Cons] based on the values or identifiers passed in.
///
/// # Examples
/// ```
/// # use adts::cons;
/// let (c1, (c2, (c3, ()))) = cons!(123f32, "hello", Some(45));
/// assert_eq!((c1, c2, c3), (123f32, "hello", Some(45)));
///
/// let cons!(c1, c2, c3) = cons!(123f32, "hello", Some(45));
/// assert_eq!(c3, Some(45));
///
/// // LHS patterns needs to be escaped with {}
/// let cons!(_, _, {Some(c3)}) = cons!(123f32, "hello", Some(45)) else { panic!(); };
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
/// # use adts::{cons, Cons};
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
    /// The type of the head of this [Cons].
    type Head;

    /// The type of the tail of this [Cons].
    type Tail: Cons;

    /// Length of this [Cons].
    const LEN: usize;

    /// Returns the length of cons.
    ///
    /// # Examples
    /// ```
    /// # use adts::{cons, Cons};
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
    /// # use adts::{cons, Cons};
    /// assert!(().is_empty());
    /// assert!(!cons!(1, 2, 3, 4, 5).is_empty());
    /// ```
    #[inline]
    fn is_empty(&self) -> bool {
        Self::LEN == 0
    }

    /// Gets the first element of given type from this [Cons].
    ///
    /// # Examples
    /// ```
    /// # use adts::{cons, Cons};
    /// let v: i32 = *cons!(1f32, 1i32, 1u32).get();
    /// assert_eq!(v, 1i32);
    /// ```
    #[inline]
    fn get<T, Index>(&self) -> &T
    where
        Self: Get<T, Index>,
    {
        Get::get(self)
    }

    /// Mutably gets the first element of given type from this [Cons].
    /// # Examples
    /// ```
    /// # use adts::{cons, Cons};
    /// let mut c = cons!(1f32, 1i32, 1u32);
    /// *c.get_mut::<i32, _>() = 10;
    /// assert_eq!(c, cons!(1f32, 10i32, 1u32));
    /// ```
    #[inline]
    fn get_mut<T, Index>(&mut self) -> &mut T
    where
        Self: Get<T, Index>,
    {
        Get::get_mut(self)
    }

    /// Concats this [Cons] with RHS.
    ///
    /// # Examples
    /// ```
    /// # use adts::{cons, Cons};
    /// let cons!(c1, c2, c3, c4, c5) = cons!(1, 2).concat(cons!(3, 4, 5));
    /// assert_eq!([c1, c2, c3, c4, c5], [1, 2, 3, 4, 5]);
    /// ```
    #[inline]
    fn concat<RHS: Cons>(self, rhs: RHS) -> <Self as Merge<RHS>>::Output
    where
        Self: Merge<RHS>,
    {
        Merge::merge(self, rhs)
    }

    /// Reverse this [Cons].
    //
    /// # Examples
    /// ```
    /// # use adts::{cons, Cons};
    /// let cons!(c1, c2, c3, c4, c5) = cons!(1, 2, 3, 4, 5).rev();
    /// assert_eq!([c1, c2, c3, c4, c5], [5, 4, 3, 2, 1]);
    /// ```
    #[inline]
    fn rev(self) -> <Self as Rev>::Output
    where
        Self: Rev,
    {
        Rev::rev(self)
    }
}

impl Cons for () {
    type Head = ();
    type Tail = ();
    const LEN: usize = 0;
}

impl<H, T: Cons> Cons for (H, T) {
    type Head = H;
    type Tail = T;
    const LEN: usize = 1 + T::LEN;
}

impl<RHS> Merge<RHS> for ()
where
    RHS: Cons,
{
    type Output = RHS;

    #[inline(always)]
    fn merge(self, rhs: RHS) -> RHS {
        rhs
    }
}

impl<H, Tail, RHS> Merge<RHS> for (H, Tail)
where
    Tail: Merge<RHS>,
    RHS: Cons,
{
    type Output = (H, <Tail as Merge<RHS>>::Output);

    #[inline(always)]
    fn merge(self, rhs: RHS) -> Self::Output {
        (self.0, self.1.merge(rhs))
    }
}

impl Rev for () {
    type Output = ();

    #[inline(always)]
    fn rev(self) -> Self::Output {
        self
    }
}

impl<T, Tail> Rev for (T, Tail)
where
    Tail: Rev,
    <Tail as Rev>::Output: Merge<(T, ())>,
{
    type Output = <<Tail as Rev>::Output as Merge<(T, ())>>::Output;

    #[inline(always)]
    fn rev(self) -> Self::Output {
        self.1.rev().merge((self.0, ()))
    }
}

/// Trait for getting a [trait@Cons] element by type.
pub trait Get<T, I> {
    /// Gets an element by type from cons.
    fn get(&self) -> &T;

    /// Mutably gets an element by type from cons.
    fn get_mut(&mut self) -> &mut T;
}

impl<T, Tail> Get<T, Here> for (T, Tail) {
    #[inline(always)]
    fn get(&self) -> &T {
        &self.0
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, Head, Tail, TailIndex> Get<T, There<TailIndex>> for (Head, Tail)
where
    Tail: Get<T, TailIndex>,
{
    #[inline(always)]
    fn get(&self) -> &T {
        self.1.get()
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut T {
        self.1.get_mut()
    }
}

/// Used as a matching index indicator in a [trait@Cons].
pub struct Here;

/// Used as an non-matching index indicator in a [trait@Cons].
pub struct There<T> {
    marker: PhantomData<T>,
}
