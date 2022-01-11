#![forbid(unsafe_code)]

use utf16_lit::utf16_null;

#[test]
fn test_example() {
  let normal: Vec<u16> = "example".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!("example"));
}

#[test]
fn test_tab() {
  let normal: Vec<u16> = "\t".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!("\t"));
}

#[test]
fn test_newline() {
  let normal: Vec<u16> = "\n".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!("\n"));
}

#[test]
fn test_carriage_return() {
  let normal: Vec<u16> = "\r".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!("\r"));
}

#[test]
fn test_backslash() {
  let normal: Vec<u16> = "\\".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!("\\"));
}

#[test]
fn test_null() {
  let normal: Vec<u16> = "\0".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!("\0"));
}

#[test]
fn test_single_quote() {
  let normal: Vec<u16> = "\'".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!("\'"));
}

#[test]
fn test_double_quote() {
  let normal: Vec<u16> = "\"".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!("\""));
}

#[test]
fn test_escaped_ascii() {
  let normal: Vec<u16> = "\x52".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!("\x52"));
}

#[test]
fn test_escaped_unicode() {
  let normal: Vec<u16> = "\u{00B6}".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!("\u{00B6}"));
}

#[test]
fn test_raw_string() {
  let normal: Vec<u16> = r"\".encode_utf16().chain(Some(0)).collect();
  assert_eq!(normal, utf16_null!(r"\"));
  assert_eq!(normal, utf16_null!(r#"\"#));
  assert_eq!(normal, utf16_null!(r##"\"##));
}
