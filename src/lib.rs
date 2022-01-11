#![no_std]
#![forbid(unsafe_code)]

//! Provides a macro_rules for making utf-16 literals.
//!
//! Outputs are (static references to) arrays of the correct size:
//! `&'static [u16; LEN]`.
//!
//! ```rust
//! use utf16_lit::{utf16, utf16_null};
//!
//! const EXAMPLE: &[u16] = utf16!("example");
//!
//! const EXAMPLE_NULL: &[u16] = utf16_null!("example");
//!
//! fn main() {
//!   let v: Vec<u16> = "example".encode_utf16().collect();
//!   assert_eq!(v, EXAMPLE);
//!
//!   let v: Vec<u16> = "example".encode_utf16().chain(Some(0)).collect();
//!   assert_eq!(v, EXAMPLE_NULL);
//!   let v: Vec<u16> = "example\0".encode_utf16().collect();
//!   assert_eq!(v, EXAMPLE_NULL);
//!
//!   // You don't even need to assign the output to a const.
//!   assert_eq!(utf16!("This works")[0], 'T' as u8 as u16);
//! }
//! ```

macro_rules! imp {
  ($(
    $(#[$m:meta])*
    $name:ident has $n:literal trailing zeroes
  )*) => {
    $(
      $(#[$m])*
      ///
      /// ## Remarks
      ///
      ///   - This macro is usable in `const` contexts, and accepts any `const`
      ///     input, such as the output of another macro:
      ///
      ///     ```rust
      ///     use utf16_lit::utf16; // Same for `utf16_null!`
      ///
      ///     const UTF16_VERSION: &[u16] = utf16!(env!("CARGO_PKG_VERSION"));
      ///     ```
      ///
      ///   - This macro yields a reference to an array rather than to a slice
      ///     so that it can be `*`-copied when needed.
      ///
      ///   - The obtained `&'static` reference is not guaranteed to be
      ///     unique, even if bound to a `const`. For instance, the following
      ///     assertion may fail:
      ///
      ///     ```rust,no_run
      ///     use utf16_lit::utf16; // Same for `utf16_null!`
      ///
      ///     const S: &[u16] = utf16!("Hey!");
      ///     assert_eq!(S.as_ptr(), S.as_ptr()); // May fail!
      ///     ```
      ///
      ///     To guarantee unicity of such a pointer, bind the `u16` contents to
      ///     a `static` storage.
      ///
      ///   - To bind the `u16` contents to a `static` storage (_e.g._, for
      ///     pointer unicity, or for FFI purposes), simply `*`-copy it:
      ///
      ///     ```rust
      ///     use utf16_lit::utf16; // Same for `utf16_null!`
      ///
      ///     static S: [u16; utf16!("Hey!").len()] = *utf16!("Hey!");
      ///
      ///     assert_eq!(S.as_ptr(), S.as_ptr()); // Guaranteed to pass.
      ///     ```
      ///
      ///   - The output of this macro uses the native endianness of the
      ///     `--target` platform.
      #[macro_export]
      macro_rules! $name {
        ($text:expr) => {{
          // Here we pick a name highly unlikely to exist in the scope
          // that $text came from, which prevents a potential const eval cycle error.
          const ABC678_PREFIX_THAT_SHOULD_NEVER_CLASH_WITH_OUTER_SCOPE_UTF8: &$crate::internals::core::primitive::str = $text;
          {
            use $crate::internals::core::prelude::v1::*;
            const ABC678_PREFIX_THAT_SHOULD_NEVER_CLASH_WITH_OUTER_SCOPE_LEN: usize =
              $crate::internals::length_as_utf16(ABC678_PREFIX_THAT_SHOULD_NEVER_CLASH_WITH_OUTER_SCOPE_UTF8) + $n;
            const ABC678_PREFIX_THAT_SHOULD_NEVER_CLASH_WITH_OUTER_SCOPE_UTF16: &'static [u16; ABC678_PREFIX_THAT_SHOULD_NEVER_CLASH_WITH_OUTER_SCOPE_LEN] = {
              let mut buffer = [0u16; ABC678_PREFIX_THAT_SHOULD_NEVER_CLASH_WITH_OUTER_SCOPE_LEN];
              let mut bytes = ABC678_PREFIX_THAT_SHOULD_NEVER_CLASH_WITH_OUTER_SCOPE_UTF8.as_bytes();
              let mut i = 0;
              while let Some((ch, rest)) = $crate::internals::next_code_point(bytes) {
                bytes = rest;
                // https://doc.rust-lang.org/std/primitive.char.html#method.encode_utf16
                if ch & 0xFFFF == ch {
                  buffer[i] = ch as u16;
                  i += 1;
                } else {
                  let code = ch - 0x1_0000;
                  buffer[i] = 0xD800 | ((code >> 10) as u16);
                  buffer[i + 1] = 0xDC00 | ((code as u16) & 0x3FF);
                  i += 2;
                }
              }
              &{ buffer }
            };
            ABC678_PREFIX_THAT_SHOULD_NEVER_CLASH_WITH_OUTER_SCOPE_UTF16
          }
        }};
      }
    )*
  }
}

imp! {
  /// Turns a string literal into a `u16` array literal static reference (`&'static [u16; N]`).
  ///
  /// If you want to have a "null terminated" string (such as for some parts of
  /// Windows FFI) then you should use [`utf16_null!`](utf16_null!).
  utf16 has 0 trailing zeroes

  /// Turns a string literal into a `u16` array literal static reference (`&'static [u16; N]`) with a trailing `0`.
  ///
  /// If you do **not** want to have a null terminator added to the string then
  /// you should use [`utf16!`](utf16!).
  utf16_null has 1 trailing zeroes
}

/// Not part of the public API.
#[doc(hidden)]
pub mod internals {
  pub use ::core;
  // A const implementation of https://github.com/rust-lang/rust/blob/d902752866cbbdb331e3cf28ff6bba86ab0f6c62/library/core/src/str/mod.rs#L509-L537
  // Assumes `utf8` is a valid &str
  pub const fn next_code_point(utf8: &[u8]) -> Option<(u32, &[u8])> {
    const CONT_MASK: u8 = 0b0011_1111;
    match utf8 {
      [one @ 0..=0b0111_1111, rest @ ..] => Some((*one as u32, rest)),
      [one @ 0b1100_0000..=0b1101_1111, two, rest @ ..] => Some((
        (((*one & 0b0001_1111) as u32) << 6) | ((*two & CONT_MASK) as u32),
        rest,
      )),
      [one @ 0b1110_0000..=0b1110_1111, two, three, rest @ ..] => Some((
        (((*one & 0b0000_1111) as u32) << 12)
          | (((*two & CONT_MASK) as u32) << 6)
          | ((*three & CONT_MASK) as u32),
        rest,
      )),
      [one, two, three, four, rest @ ..] => Some((
        (((*one & 0b0000_0111) as u32) << 18)
          | (((*two & CONT_MASK) as u32) << 12)
          | (((*three & CONT_MASK) as u32) << 6)
          | ((*four & CONT_MASK) as u32),
        rest,
      )),
      [..] => None,
    }
  }
  // A const implementation of `s.chars().map(|ch| ch.len_utf16()).sum()`
  pub const fn length_as_utf16(s: &str) -> usize {
    let mut bytes = s.as_bytes();
    let mut len = 0;
    while let Some((ch, rest)) = next_code_point(bytes) {
      bytes = rest;
      len += if (ch & 0xFFFF) == ch { 1 } else { 2 };
    }
    len
  }
}
