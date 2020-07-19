#![allow(bad_style)]

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

extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

use core::str::FromStr;

use std::fmt::Write;

/// Turns a string literal into a `&[u16]` literal.
///
/// If you want to have a "null terminated" string (such as for some parts of
/// Windows FFI) then you should use [`utf16_null!`](utf16_null!).
#[proc_macro]
pub fn utf16(stream: TokenStream) -> TokenStream {
  const USAGE: &str = "Usage: utf16!(string_lit)";
  
  let mut tt_iter = stream.into_iter();
  let lit = match tt_iter.next().expect(USAGE) {
    TokenTree::Literal(lit) => lit,
    _ => panic!(USAGE),
  };
  // we expect only one string literal per invocation.
  assert!(tt_iter.next().is_none(), USAGE);

  let lit_string = format!("{}", lit);
  // right now we only support double quoted strings
  assert!(lit_string.as_bytes().first() == Some(&b'"'), USAGE);
  assert!(lit_string.as_bytes().last() == Some(&b'"'), USAGE);
  let lit_str = &lit_string[1..lit_string.len()-1];
  
  str_to_utf16_units_tokenstream(lit_str)
}

/// Turns a string literal into a `&[u16]` literal with a null on the end.
///
/// If you do **not** want to have a null terminator added to the string then
/// you should use [`utf16!`](utf16!).
#[proc_macro]
pub fn utf16_null(stream: TokenStream) -> TokenStream {
  const USAGE: &str = "Usage: utf16!(string_lit)";
  
  let mut tt_iter = stream.into_iter();
  let lit = match tt_iter.next().expect(USAGE) {
    TokenTree::Literal(lit) => lit,
    _ => panic!(USAGE),
  };
  // we expect only one string literal per invocation.
  assert!(tt_iter.next().is_none(), USAGE);

  let mut lit_string = format!("{}", lit);
  // right now we only support double quoted strings
  assert!(lit_string.as_bytes().first() == Some(&b'"'), USAGE);
  assert!(lit_string.as_bytes().last() == Some(&b'"'), USAGE);
  // we need a null on the end, so we just reuse the end of this string.
  lit_string.pop();
  lit_string.push('\0');
  let lit_str = &lit_string[1..];
  
  str_to_utf16_units_tokenstream(lit_str)
}

fn str_to_utf16_units_tokenstream(s: &str) -> TokenStream {
  let mut encode_buf = [0_u16; 2];
  let mut buf = String::with_capacity(s.as_bytes().len() * 8 + 10);
  //
  buf.push_str("&[");
  for char_escape in CharEscapeIterator::new(s.chars()) {
    match char_escape {
      CharEscape::Escaped(ch) | CharEscape::Literal(ch) => {
        for unit in ch.encode_utf16(&mut encode_buf) {
          let _cant_fail = write!(buf, "{},", unit);
        }
      },
      other => panic!("Illegal character escape sequence: {:?}", other),
    }
  }
  buf.push_str("]");
  //
  TokenStream::from_str(&buf).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharEscape {
  Literal(char),
  Escaped(char),
  Improper(char),
  DanglingBackslash,
}

#[derive(Debug, Clone, Copy)]
struct CharEscapeIterator<I> {
  it: I,
}

impl<I> CharEscapeIterator<I>
where
  I: Iterator<Item = char>,
{
  fn new(it: I) -> Self {
    Self { it }
  }
}

impl<I> Iterator for CharEscapeIterator<I>
where
  I: Iterator<Item = char>,
{
  type Item = CharEscape;
  fn next(&mut self) -> Option<CharEscape> {
    if let Some(ch) = self.it.next() {
      match ch {
        '\\' => {
          if let Some(follow) = self.it.next() {
            match follow {
              '0' => Some(CharEscape::Escaped('\0')),
              'n' => Some(CharEscape::Escaped('\n')),
              'r' => Some(CharEscape::Escaped('\r')),
              't' => Some(CharEscape::Escaped('\t')),
              '\\' => Some(CharEscape::Escaped('\\')),
              '\'' => Some(CharEscape::Escaped('\'')),
              '"' => Some(CharEscape::Escaped('"')),
              'x' => {
                let mut inner = || {
                  let d1 = self.it.next()?;
                  let d2 = self.it.next()?;
                  let mut temp = [0; 4];
                  let a = u8::from_str_radix(d1.encode_utf8(&mut temp), 16).ok()?;
                  let b = u8::from_str_radix(d2.encode_utf8(&mut temp), 16).ok()?;
                  let c = a << 4 | b;
                  if c < 128 {
                    Some(CharEscape::Escaped(c as char))
                  } else {
                    None
                  }
                };
                inner().or(Some(CharEscape::Improper('x')))
              }
              'u' => {
                let mut inner = || {
                  let open_brace = self.it.next();
                  if open_brace != Some('{') {
                    return None;
                  }
                  let mut buffer = [0_u8; 6];
                  let mut buffer_index = 0;
                  loop {
                    let next_ch = self.it.next()?;
                    if next_ch == '}' {
                      break;
                    } else if buffer_index >= buffer.len() {
                      // we have to keep eating until we see '}', so for now
                      // just signal failure and we check after the loop.
                      buffer_index = usize::max_value();
                    } else {
                      buffer[buffer_index] = next_ch as u8;
                      buffer_index += 1;
                    }
                  }
                  if buffer_index == usize::max_value() {
                    return None;
                  }
                  let s = core::str::from_utf8(&buffer[..buffer_index]).ok()?;
                  let u = u32::from_str_radix(s, 16).ok()?;
                  core::char::from_u32(u).map(CharEscape::Escaped)
                };
                inner().or(Some(CharEscape::Improper('u')))
              }
              imp => Some(CharEscape::Improper(imp)),
            }
          } else {
            Some(CharEscape::DanglingBackslash)
          }
        }
        other => Some(CharEscape::Literal(other)),
      }
    } else {
      None
    }
  }
}
