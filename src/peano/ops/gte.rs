//! Greater-than-or-equal-to comparison.

/// Greater-than-or-equal-to comparison.
pub trait GreaterThanOrEqualTo<R: crate::peano::N>: crate::peano::N {}

impl GreaterThanOrEqualTo<()> for () {}

impl<Tail: crate::peano::N> GreaterThanOrEqualTo<()> for ((), Tail) {}

impl<L: crate::peano::N + GreaterThanOrEqualTo<R>, R: crate::peano::N> GreaterThanOrEqualTo<((), R)>
    for ((), L)
{
}
