//! Greater-than comparison.

/// Greater-than comparison.
pub trait GreaterThan<R: crate::peano::N>: crate::peano::N {}

impl<Tail: crate::peano::N> GreaterThan<()> for ((), Tail) {}

impl<Tail: crate::peano::N + GreaterThan<RTail>, RTail: crate::peano::N> GreaterThan<((), RTail)>
    for ((), Tail)
{
}
