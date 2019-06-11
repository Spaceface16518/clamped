# Clamped

Simple clamping of numbers in Rust

# Usage

`clamped` relies heavily on conditional compilation with Cargo's feature system.
You need to enable all the elements of `clamped` you need via features.
You can enable each feature independently (they won't conflict with each other).

The current features are

- `clamp_trait`: the `Clamp` trait and it's corresponding `clamped` member function
```rust
use clamped::Clamp;

// Clamp numbers between 20 and 30
assert_eq!(10.clamped(20, 30), 20); // 10 -> 20
assert_eq!(15.clamped(20, 30), 20); // 15 -> 20
assert_eq!(20.clamped(20, 30), 20); // 20 -> 20
assert_eq!(25.clamped(20, 30), 25); // 25 -> 25
assert_eq!(30.clamped(20, 30), 30); // 30 -> 30
assert_eq!(35.clamped(20, 30), 30); // 35 -> 30
```
- `clamp_fn`: The `clamp` function, which is the same as the `Clamp` + `clamped` method, but does not use a trait method
- `clamp_macro`: A `clamp!` macro, which contains the signature of all clamp functions.
You can use this if you don't want to include any more traits, functions, or other external code.

- `inline_always`: This feature modifies the signature of all functions in the crate, turning on the `#[inline(always)]` attribute.
This will force the compiler to inline all calls, essentially allowing functions and trait methods to act like macros with type checking.

**Note:** Only the `clamp_trait` feature is enabled by default (This may change in the future as more features are added)

All features are currently `#[no_std]` compatible, but **this may change in the future!!!**