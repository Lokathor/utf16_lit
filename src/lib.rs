//! Provides a proc-macro for making utf-16 literals.
//!
//! ```rust
//! use utf16_lit::{utf16, utf16_null};
//!
//! const EXAMPLE: &[u16] = &utf16!("example");
//!
//! const EXAMPLE_NULL: &[u16] = &utf16_null!("example");
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
      #[macro_export]
      macro_rules! $name {
        ($text:expr) => {{
          // Here we pick a name highly unlikely to exist in the scope
          // that $text came from, which prevents a potential const eval cycle error.
          const __SWEIRFOH2387OPC: &str = $text;
          const UTF8: &str = __SWEIRFOH2387OPC;
          const LEN: usize = $crate::internals::length_as_utf16(UTF8) + $n;
          const UTF16: [u16; LEN] = {
            let mut buffer = [0u16; LEN];
            let mut bytes = UTF8.as_bytes();
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
            buffer
          };
          UTF16
        }};
      }
    )*
  }
}

imp! {
  /// Turns a string literal into a `&[u16]` literal.
  ///
  /// If you want to have a "null terminated" string (such as for some parts of
  /// Windows FFI) then you should use [`utf16_null!`](utf16_null!).
  utf16 has 0 trailing zeroes

  /// Turns a string literal into a `&[u16]` literal with a null on the end.
  ///
  /// If you do **not** want to have a null terminator added to the string then
  /// you should use [`utf16!`](utf16!).
  utf16_null has 1 trailing zeroes
}

#[doc(hidden)]
pub mod internals {
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
