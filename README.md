[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![crates.io](https://img.shields.io/crates/v/utf16_lit.svg)](https://crates.io/crates/utf16_lit)
[![docs.rs](https://docs.rs/utf16_lit/badge.svg)](https://docs.rs/utf16_lit/)

# utf16_lit

Lets you change a standard Rust string literal (utf8) into a utf16 value at compile time.

This crate doesn't use `syn` or `quote`, so it builds very fast.

## Stability: Not Really

Currently, "function-like" proc-macros are not so great in rust. They must only be used in "item position" (at the top level of a module), not in "statement position" (a line of their own within a block of code) or "expression position" (as a sub-portion of an expression within a statement). This makes the current usage of the proc-macro quite poor.

Once function-like proc-macros are available in expression position, the library will be updated so that the usage produces an expression. This will be a completely breaking change.

```rust
// Currently the usage is like this:
utf16_lit!(EXAMPLE, "example");

// our goal usage is something like this
pub const EXAMPLE: &[u16] = utf16_lit!("example");

// and also things like this should be possible
fn is_foo(input: &[u16]) -> bool {
  input == utf16_lit!("foo")
}
```

Tracking Issues of Note:
* [Tracking issue for procedural macros and "hygiene 2.0"](https://github.com/rust-lang/rust/issues/54727)
* [Stabilize fn-like proc macros in expression, pattern and statement positions](https://github.com/rust-lang/rust/pull/68717)
* [Stabilize `Span::mixed_site`](https://github.com/rust-lang/rust/pull/68716)
