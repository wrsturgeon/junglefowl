//! Compile-time-length arrays that can be spliced up however you'd like while keeping full knowledge of every piece's length.
//! ```rust
//! use junglefowl::*;
//!
//! // Accept only `u8` arrays with exactly 3 elements:
//! fn picky<T: Nest<Element = u8, Length = peano!(3)>>(_: &T) {}
//!
//! // Create an array with 5 elements:
//! let n12345 = nest![1, 2, 3, 4, 5];
//!
//! // Split it after its second element without changing anything in memory:
//! let (left, right) = split!(n12345, 2);
//!
//! // And we can prove that the second segment will have exactly two elements:
//! picky(&right);
//! // picky(&left); // won't compile!
//!
//! // And know exactly what its elements are:
//! assert_eq!(nest![3, 4, 5], right);
//! ```

/// Array type holding N elements of type T.
#[macro_export]
macro_rules! array {
    ($t:ty, $n:expr) => {
        $crate::Array<$t, $crate::peano!($n)>
    };
}

#[macro_use]
mod traits;
#[macro_use]
mod split;

pub use split::*;
pub use traits::*;
