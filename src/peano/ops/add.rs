//! Addition.

use junglefowl_macros::peano;

use crate::static_assert_eq;

/// Add two Peano types.
#[macro_export]
macro_rules! add {
    ($a:ty, $b:ty $(,)?) => {
        <$a as $crate::peano::ops::Add<$b>>::Sum
    };
}

/// Addition: e.g. `()` + `()` = `()`, or more intuitively, `peano!(42) + peano!(1) = peano!(43)`.
pub trait Add<R: crate::peano::N>: crate::peano::N {
    /// The result, once again as a type. Use ...`::USIZE` to obtain a value.
    type Sum: crate::peano::N;
}

impl<R: crate::peano::N> Add<R> for () {
    type Sum = R;
}

impl<R: crate::peano::N, Tail: crate::peano::N + crate::peano::ops::Add<R>> Add<R> for ((), Tail) {
    type Sum = ((), add!(Tail, R));
}

static_assert_eq!(peano!(45), add!(peano!(42), peano!(3)));
