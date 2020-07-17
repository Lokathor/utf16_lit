//! Provides a proc-macro for making utf-16 literals.
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

/// Turns a string literal into a `&[u16]` literal.
///
/// If you want to have a "null terminated" string (such as for some parts of
/// Windows FFI) then you should use [`utf16_null`](utf16_null).
#[macro_export]
macro_rules! utf16 {
    ($utf8:pat) => {{
        mod imp {
            ::utf16_lit_impl::utf16!($utf8);
        }
        imp::UTF16_ENCODED
    }}
}

/// Turns a string literal into a `&[u16]` literal with a null on the end.
///
/// If you do **not** want to have a null terminator added to the string then
/// you should use [`utf16`](utf16).
#[macro_export]
macro_rules! utf16_null {
    ($utf8:pat) => {{
        mod imp {
            ::utf16_lit_impl::utf16_null!($utf8);
        }
        imp::UTF16_ENCODED
    }}
}