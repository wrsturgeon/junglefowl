//! Split an array into two parts at a compile-time known boundary. Calculate the length of each segment at compile time as well.

use core::marker::PhantomData;

use crate::peano;

use super::Nest;

/// Syntactic sugar for `Split::<peano!($n)>>::split($nest)`.
#[macro_export]
macro_rules! split {
    ($nest:expr,$n:expr) => {
        $crate::nest::Split::<$crate::peano!($n)>::split($nest)
    };
}

/// List that can be split after as many elements as its generic argument's value (interpret as a Peano type).
pub trait Split<N: peano::N>: Nest {
    /// Array/nest type of the left/first segment after splitting.
    type LeftSegment: Nest<Element = Self::Element>;
    /// Array/nest type of the right/second segment after splitting.
    type RightSegment: Nest<Element = Self::Element>;
    /// Split the list, consuming it.
    fn split(self) -> (Self::LeftSegment, Self::RightSegment);
}

impl<T> Split<()> for PhantomData<T> {
    type LeftSegment = Self;
    type RightSegment = Self;
    #[inline(always)]
    fn split(self) -> (Self::LeftSegment, Self::RightSegment) {
        (Self, Self)
    }
}

impl<T, Tail: Nest<Element = T>, N: Splitter<Self>> Split<N> for (T, Tail)
where
    Self::Length: peano::ops::GreaterThan<N>,
{
    type LeftSegment = N::LeftSegment;
    type RightSegment = N::RightSegment;
    #[inline(always)]
    fn split(self) -> (Self::LeftSegment, Self::RightSegment) {
        N::split(self)
    }
}

/// A Peano type smaller than a list length that can split that list after as many elements as this number's value.
pub trait Splitter<N: Nest>: peano::N {
    /// Array/nest type of the left/first segment after splitting.
    type LeftSegment: Nest<Element = N::Element>;
    /// Array/nest type of the right/second segment after splitting.
    type RightSegment: Nest<Element = N::Element>;
    /// Split the list, consuming it.
    fn split(nest: N) -> (Self::LeftSegment, Self::RightSegment);
}

impl<T, Tail: Nest<Element = T>> Splitter<(T, Tail)> for () {
    type LeftSegment = PhantomData<T>;
    type RightSegment = (T, Tail);
    #[inline(always)]
    fn split(nest: (T, Tail)) -> (Self::LeftSegment, Self::RightSegment) {
        (PhantomData::<T>, nest)
    }
}

impl<T, Tail: Nest<Element = T>, SelfTail: Splitter<Tail>> Splitter<(T, Tail)> for ((), SelfTail) {
    type LeftSegment = (T, SelfTail::LeftSegment);
    type RightSegment = SelfTail::RightSegment;
    #[inline(always)]
    fn split(nest: (T, Tail)) -> (Self::LeftSegment, Self::RightSegment) {
        let rec = SelfTail::split(nest.1);
        ((nest.0, rec.0), rec.1)
    }
}

#[test]
fn split_12345() {
    use crate::nest;
    let n12345: (u8, _) = nest![1, 2, 3, 4, 5];
    let (left, right) = split!(n12345, 3);
    assert_eq!(3, left.length());
    assert_eq!(2, right.length());
    assert_eq!(nest![1, 2, 3], left);
    assert_eq!(nest![4, 5], right);
}
