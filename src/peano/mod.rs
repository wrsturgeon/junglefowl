//! Peano arithmetic: essentially unary encoding in types rather than values.

pub mod ops;
mod traits;

pub use traits::*;

/// Assert at compile time that two Peano types represent the same number.
#[macro_export]
macro_rules! static_assert_eq {
    ($a:ty, $b:ty $(,)?) => {
        // inspired by https://docs.rs/static_static_assertions/latest/src/static_static_assertions/const_static_assert.rs.html#55
        #[allow(clippy::as_conversions)]
        const _: [$crate::False; // <-- Literally an uninstantiable type: even if we somehow fucked up, you can't produce a value of type `False`
            (<$a as $crate::peano::N>::USIZE != <$b as $crate::peano::N>::USIZE) as usize] =
            [];
    };
}

/// Assert at compile time that two Peano types represent different numbers.
#[macro_export]
macro_rules! static_assert_ne {
    ($a:ty, $b:ty $(,)?) => {
        #[allow(clippy::as_conversions)]
        const _: [$crate::False;
            (<$a as $crate::peano::N>::USIZE == <$b as $crate::peano::N>::USIZE) as usize] = [];
    };
}

/// Assert at compile time that a Peano type represents zero.
#[macro_export]
macro_rules! static_assert_zero {
    ($t:ty $(,)?) => {
        static_assert_eq!((), $t);
    };
}

/// Assert at compile time that a Peano type does not represent zero (and thus must be positive).
#[macro_export]
macro_rules! static_assert_nonzero {
    ($t:ty $(,)?) => {
        static_assert_ne!((), $t);
    };
}

static_assert_zero!(());
static_assert_nonzero!(((), ()));

static_assert_eq!((), ());
static_assert_eq!(((), ()), ((), ()));

static_assert_ne!((), ((), ()));
