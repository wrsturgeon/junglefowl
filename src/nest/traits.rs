//! Nested tuples whose left-hand elements are all a given type `T`.

use core::marker::PhantomData;

use crate::peano;

/// Place N elements into a nested tuple of known shape at compile time.
/// Inspired by [`frunk`'s implementation](https://github.com/lloydmeta/frunk/blob/master/core/src/macros.rs#L31) but with trailing comma support and a different sentinel node.
/// # Use
/// ```rust
/// use junglefowl::*;
///
/// let nested = nest![1, 2, 3, 4, 5];
/// // nested : ({integer}, ({integer}, ({integer}, ({integer}, ({integer}, PhantomData<{determined by use}>)))))
/// # takes_nest(&nested); // see below comment
/// # takes_u8(nested.0); // compilation error if unused, but that's the point
/// # fn takes_nest<T: Nest>(_: &T) {}
///
/// // No need for type annotations; just use it later:
/// # fn takes_u8(_: u8) {}
/// let nested_tbd = nest![1, 2, 3, 4, 5];
/// # takes_nest(&nested_tbd);
/// takes_u8(nested_tbd.0);
/// // nested_tbd : (u8, (u8, (u8, (u8, (u8, PhantomData<u8>)))))
/// ````
#[macro_export]
macro_rules! nest {
    ($(,)?) => { ::core::marker::PhantomData };
    ($head:expr$(,)?) => { ($head,$crate::nest![]) };
    (...$etc:expr)=>{ $etc };
    ($head:expr, $($tail:tt)*) => { ($head, $crate::nest![$($tail)*]) };
}

/// Sealed traits.
mod whitelist {
    use super::PhantomData;

    /// Nested tuples whose left-hand elements are all type `T`.
    pub trait Nest {}
    impl<T> Nest for PhantomData<T> {}
    impl<T, Tail: Nest> Nest for (T, Tail) {} // doesn't matter yet if elements of T and Tail are mismatched
}

/// Nested tuples whose left-hand elements are all type `T`.
pub trait Nest: whitelist::Nest {
    /// Left-hand element type.
    type Element;
    /// Length of the "array" as a Peano type.
    type Length: peano::N;
    /// Length as a value at runtime. Will be made `const` when const traits are stabilized.
    fn length(&self) -> usize;
}
impl<T> Nest for PhantomData<T> {
    type Element = T;
    type Length = ();
    #[inline(always)]
    fn length(&self) -> usize {
        <Self::Length as peano::N>::USIZE
    }
}
impl<T, Tail: Nest<Element = T>> Nest for (T, Tail) {
    type Element = T;
    type Length = ((), Tail::Length);
    #[inline(always)]
    fn length(&self) -> usize {
        <Self::Length as peano::N>::USIZE
    }
}
