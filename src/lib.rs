//! Brutally murdering Rust's type system one proof at a time.

#![deny(warnings)]
#![warn(
    missing_docs,
    rustdoc::all,
    clippy::missing_docs_in_private_items,
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::pub_use,
    clippy::question_mark_used,
    clippy::implicit_return,
    clippy::inline_always,
    clippy::separated_literal_suffix,
    clippy::mod_module_files
)]

#[macro_use]
pub mod nest;
pub mod peano;

pub use junglefowl_macros::*;
static_assert_zero!(peano!(0));
static_assert_eq!(peano!(0), peano!(0));
static_assert_eq!(peano!(42), peano!(42));
static_assert_nonzero!(peano!(42));

pub use nest::Nest;

/// A type that can never be instantiated (inhabited by zero values).
/// Corresponds to a false statement under the Curry-Howard correspondence.
#[allow(clippy::exhaustive_enums)]
pub enum False {}
