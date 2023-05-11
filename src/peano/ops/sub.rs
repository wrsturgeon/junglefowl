//! Subtraction.

use junglefowl_macros::peano;

use crate::static_assert_eq;

/// Subtract two Peano types.
#[macro_export]
macro_rules! sub {
    ($a:ty, $b:ty $(,)?) => {
        <$a as $crate::peano::ops::Sub<$b>>::Difference
    };
}

/// Subtraction: e.g. `()` - `()` = `()`, or more intuitively, `peano!(42) - peano!(1) = peano!(41)`.
pub trait Sub<R: crate::peano::N>: crate::peano::N {
    /// The result, once again as a type. Use ...`::USIZE` to obtain a value.
    type Difference: crate::peano::N;
}

impl<T: crate::peano::N> Sub<()> for T {
    type Difference = Self;
}

impl<
        L: crate::peano::N
            // GreaterThanOrEqualTo is logically unnecessary but EXTREMELY helpful for debugging
            + crate::peano::ops::GreaterThanOrEqualTo<R>
            + Sub<R>,
        R: crate::peano::N,
    > Sub<((), R)> for ((), L)
{
    type Difference = sub!(L, R); // TODO
}

static_assert_eq!(peano!(39), sub!(peano!(42), peano!(3)));
