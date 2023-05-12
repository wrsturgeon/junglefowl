# Junglefowl
## Brutally murdering Rust's type system one proof at a time.

Junglefowl runs Peano arithmetic on Rust types, verified at compile time. 

## Why?

So we can do theoretically hard stuff, like these const-generic slices:
```rust
use junglefowl::*;

// Accept only `u8` arrays with exactly 3 elements:
fn picky<T: Nest<Element = u8, Length = peano!(3)>>(_: &T) {}

// Create an array with 5 elements:
let n12345 = nest![1, 2, 3, 4, 5];

// Split it after its second element without changing anything in memory:
let (left, right) = split!(n12345, 2);

// And we can prove that the second segment will have exactly two elements:
picky(&right);
// picky(&left); // won't compile!

// And know exactly what its elements are:
assert_eq!(nest![3, 4, 5], right);
```

## How?
Here's our Peano encoding:
```
0 <-->                ()
1 <-->           ((), ())
2 <-->      ((), ((), ()))
3 <--> ((), ((), ((), ())))
```
Note that, thanks to a clever abuse of Rust's syntax, _these are both types and values_.

Next, there's a macro so you can forget what you just read:
```rust
peano!(0);
 --> ()
peano!(42);
 --> ((), ((), ((), ((), ((), ((), ((), ((), ((), ...)))))))))
```
Note that this macro expands to a _type_, so you would use it like this:
```rust
let x: peano!(42) = todo!();
```
instead of like this:
```rust
let x = peano!(42); // bad!
```

And next, there's a _hell_ of a lot of other stuff, but instead of explaining it all, watch this compile:
```rust
static_assert_eq!(peano!(39), sub!(peano!(42), peano!(3)));
```
expands to
```rust
enum False {} // uninstantiable type

//      this part vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv evaluates to zero when the two sides are equal
const _: [False; (peano!(39) != sub!(peano!(42), peano!(3))) as usize] = [];
// ... which makes the list length zero, which matches the right-hand side (and couldn't be nonzero since its members are uninstantiable)
// learned the list length trick from the `static_assertions` crate, so all credit there!
```
Expanding the interesting part above (and inverting so `!=` becomes `==`):
```rust
peano!(39) == <         peano!(42)          as peano::Sub<     peano!(3)     >>::Difference;
peano!(39) == <((), ((), ((), peano!(39)))) as peano::Sub<((), ((), ((), ())))>::Difference;
```
Here's the definition of `peano::Sub`, pretty representative for most operations in this crate:
```rust
pub trait Sub<R: peano::N>: peano::N { type Difference: peano::N; } // sealed trait
impl<T: peano::N> Sub<()> for T { type Difference = Self; } // subtracting zero is our super-simple base case
impl<L: peano::N + Sub<R>, R: peano::N> Sub<((), R)> for ((), L) { type Difference = sub!(L, R); } // otherwise, reduce the problem until it's dividing by zero
```
Begin reduction!
```rust
peano!(39) == <((), ((), ((), peano!(39)))) as peano::Sub<((), ((), ((), ())))>::Difference;
peano!(39) == <     ((), ((), peano!(39)))  as peano::Sub<     ((), ((), ())) >::Difference;
peano!(39) == <          ((), peano!(39))   as peano::Sub<          ((), ())  >::Difference;
peano!(39) == <               peano!(39)    as peano::Sub<               ()   >::Difference;
peano!(39) ==                 peano!(39)                                                   ;
```
_et voila!_

## What's with the name?
A certain well-known theorem prover is named after the French word for ~~cock~~ rooster (_coq_), so I Googled "rooster" and found (to my amazement!) that they belong to the _junglefowl_ species.
This name sounded suitably cool.
