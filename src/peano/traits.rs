//! Provides a trait, `N` (for "natural number"), which maps from a unique type representing a number to a Rust `usize` value.
//! ## How
//! `0` -> `               ()` (it even looks like a zero!)
//! `1` -> `          ((), ())`
//! `2` -> `     ((), ((), ()))`
//! `3` -> `((), ((), ((), ())))`
//! . . .
//! ad infinitum.
//! Since exactly one value inhabits the type `()`, each one of these types also has exactly one possible instantiation.

/// Sealed traits to prevent end-users from cracking open the API to treat their types as Peano numbers.
mod whitelist {
    /// Inductive "type" (trait) representing a natural number in Peano arithmetic.
    pub trait N {}
    impl N for () {}
    impl<Tail: super::N> N for ((), Tail) {}
}

/// Inductive "type" (trait) representing a natural number in Peano arithmetic.
pub trait N: whitelist::N {
    /// The value this unique type represents, as a Rust `usize`. Technically breaks at 2^64 on a 64b machine, but reaching the limit would be ridiculously impractical.
    const USIZE: usize;
}

impl N for () {
    const USIZE: usize = 0;
}

impl<Tail: N> N for ((), Tail) {
    const USIZE: usize = 1 + Tail::USIZE;
}
