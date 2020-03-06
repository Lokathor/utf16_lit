#![allow(bad_style)]

//! Provides a proc-macro for making utf-16 literals.
//! 
//! ```rust
//! use utf16_lit::{utf16, utf16_null};
//! 
//! // must be an item!
//! utf16!(EXAMPLE, "example");
//! 
//! utf16_null!(EXAMPLE_NULL, "example");
//! 
//! fn main() {
//!   let v: Vec<u16> = "example".encode_utf16().collect();
//!   assert_eq!(v, EXAMPLE);
//! 
//!   let v: Vec<u16> = "example".encode_utf16().chain(Some(0)).collect();
//!   assert_eq!(v, EXAMPLE_NULL);
//! }
//! ```
//!
//! Currently "function-like" proc macros can't be used in expression or
//! statement position, only item position. This means that the ergonomics of
//! this proc-macro are quite poor at the moment. Once the proc-macro situation
//! improves we can make this proc-macro more "natural" to use.
//!
//! In the future I hope to slim it down so that it's just a string literal to
//! `&[u16]` conversion, without needing to pass in idents to make constants or
//! any of that.

extern crate proc_macro;
use core::str::FromStr;
use proc_macro::{TokenStream, TokenTree};

mod char_escape;
use char_escape::perform_the_escaping;

/// Makes a `&[u16]` const.
///
/// * **Usage:**
///   * Statement: `utf16!(IDENT, string_literal);`
///   * Expands to: `pub const IDENT: &[u16] = utf16_string_literal;`
///
/// If you want to have a "null terminated" string (such as for some parts of
/// Windows FFI) then you should use [`utf16_null`](utf16_null).
#[proc_macro]
pub fn utf16(stream: TokenStream) -> TokenStream {
  const USAGE: &str = "Usage: utf16!(ident, string_lit)";

  // This "parsing" system is janky as hell, but it doesn't depend on the
  // `quote` or `syn` crates, so we save a lot on compile time at the expense of
  // having slightly worse errors. However, since the user usually calls the
  // macro correctly and doesn't have an error, it's probably a good trade.
  let mut tt_iter = stream.into_iter();
  let ident = match tt_iter.next().expect(USAGE) {
    TokenTree::Ident(i) => i,
    _ => panic!(USAGE),
  };
  match tt_iter.next().expect(USAGE) {
    TokenTree::Punct(p) if p.as_char() == ',' => (),
    _ => panic!(USAGE),
  };
  let lit = match tt_iter.next().expect(USAGE) {
    TokenTree::Literal(lit) => lit,
    _ => panic!(USAGE),
  };
  assert!(tt_iter.next().is_none(), USAGE);

  // The literal, if it's a string literal, will have the double quotes on the
  // edges, which we want to strip off of the value we're going to encode as
  // utf16. The end we can cheaply strip off with `pop`, but for the start it
  // would be expensive to delete the 0th char and move all the rest. Instead,
  // we want to simply start the encoding just after the double quote position.
  let mut lit_string = lit.to_string();
  assert!(lit_string.pop() == Some('"'), USAGE);
  assert!(lit_string.chars().nth(0) == Some('"'), USAGE);

  // Also we have to convert any escape sequences within the string into the
  // correct characters ourselves because the `proc_macro` crate doesn't do that
  // for us.
  let all_the_chars = perform_the_escaping(&lit_string[1..]);
  let mut units: Vec<u16> = Vec::with_capacity(all_the_chars.len() * 2);
  let mut encode_buf = [0_u16; 2];
  for ch in all_the_chars {
    for u in ch.encode_utf16(&mut encode_buf) {
      units.push(*u);
    }
  }

  // Finally, instead of trying to fiddle a TokenStream with extend and all
  // that, we just write down the text of the code we wanted to have had the
  // whole time and then we use `from_str` and let the system handle it for us.
  let buf = format!(
    "pub const {ident}: &[u16] = &{units:?};",
    ident = ident,
    units = &units[..],
  );
  TokenStream::from_str(&buf).unwrap()
}

/// Makes a `&[u16]` const with a null terminator.
///
/// * **Usage:**
///   * Statement: `utf16_null!(IDENT, string_literal);`
///   * Expands to: `pub const IDENT: &[u16] = utf16_string_with_a_null;`
///
/// If you do **not** want to have a null terminator added to the string then
/// you should use [`utf16`](utf16).
#[proc_macro]
pub fn utf16_null(stream: TokenStream) -> TokenStream {
  const USAGE: &str = "Usage: utf16_null!(ident, string_lit)";

  // This "parsing" system is janky as hell, but it doesn't depend on the
  // `quote` or `syn` crates, so we save a lot on compile time at the expense of
  // having slightly worse errors. However, since the user usually calls the
  // macro correctly and doesn't have an error, it's probably a good trade.
  let mut tt_iter = stream.into_iter();
  let ident = match tt_iter.next().expect(USAGE) {
    TokenTree::Ident(i) => i,
    _ => panic!(USAGE),
  };
  match tt_iter.next().expect(USAGE) {
    TokenTree::Punct(p) if p.as_char() == ',' => (),
    _ => panic!(USAGE),
  };
  let lit = match tt_iter.next().expect(USAGE) {
    TokenTree::Literal(lit) => lit,
    _ => panic!(USAGE),
  };
  assert!(tt_iter.next().is_none(), USAGE);

  // The literal, if it's a string literal, will have the double quotes on the
  // edges, which we want to strip off of the value we're going to encode as
  // utf16. The end we can cheaply strip off with `pop`, but for the start it
  // would be expensive to delete the 0th char and move all the rest. Instead,
  // we want to simply start the encoding just after the double quote position.
  let mut lit_string = lit.to_string();
  assert!(lit_string.pop() == Some('"'), USAGE);
  assert!(lit_string.chars().nth(0) == Some('"'), USAGE);

  // Also we have to convert any escape sequences within the string into the
  // correct characters ourselves because the `proc_macro` crate doesn't do that
  // for us.
  let all_the_chars = perform_the_escaping(&lit_string[1..]);
  let mut units: Vec<u16> = Vec::with_capacity(all_the_chars.len() * 2);
  let mut encode_buf = [0_u16; 2];
  for ch in all_the_chars {
    for u in ch.encode_utf16(&mut encode_buf) {
      units.push(*u);
    }
  }
  units.push(0);

  // Finally, instead of trying to fiddle a TokenStream with extend and all
  // that, we just write down the text of the code we wanted to have had the
  // whole time and then we use `from_str` and let the system handle it for us.
  let buf = format!(
    "pub const {ident}: &[u16] = &{units:?};",
    ident = ident,
    units = &units[..],
  );
  TokenStream::from_str(&buf).unwrap()
}
