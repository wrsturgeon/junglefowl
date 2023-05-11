//! Arithmetic "operations" on types.
//! Pitfalls along the way:
//! - Structs like `Add<L, R>` don't work since you can't (yet) specialize.
//! - For e.g. an `impl<R> Add<R> for L` approach where `L` is `S<Tail>`, Rust can't prove by two-option "brute force" that `Tail` implements `Add`.

mod add;
mod gt;
mod gte;
mod sub;

pub use add::*;
pub use gt::*;
pub use gte::*;
pub use sub::*;
