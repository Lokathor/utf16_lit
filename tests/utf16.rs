use utf16_lit::utf16;

#[test]
fn test_example() {
  let normal: Vec<u16> = "example".encode_utf16().collect();
  assert_eq!(normal, utf16!("example"));
}

#[test]
fn test_tab() {
  let normal: Vec<u16> = "\t".encode_utf16().collect();
  assert_eq!(normal, utf16!("\t"));
}

#[test]
fn test_newline() {
  let normal: Vec<u16> = "\n".encode_utf16().collect();
  assert_eq!(normal, utf16!("\n"));
}

#[test]
fn test_carriage_return() {
  let normal: Vec<u16> = "\r".encode_utf16().collect();
  assert_eq!(normal, utf16!("\r"));
}

#[test]
fn test_backslash() {
  let normal: Vec<u16> = "\\".encode_utf16().collect();
  assert_eq!(normal, utf16!("\\"));
}

#[test]
fn test_null() {
  let normal: Vec<u16> = "\0".encode_utf16().collect();
  assert_eq!(normal, utf16!("\0"));
}

#[test]
fn test_single_quote() {
  let normal: Vec<u16> = "\'".encode_utf16().collect();
  assert_eq!(normal, utf16!("\'"));
}

#[test]
fn test_double_quote() {
  let normal: Vec<u16> = "\"".encode_utf16().collect();
  assert_eq!(normal, utf16!("\""));
}

#[test]
fn test_escaped_ascii() {
  let normal: Vec<u16> = "\x52".encode_utf16().collect();
  assert_eq!(normal, utf16!("\x52"));
}

#[test]
fn test_escaped_unicode() {
  let normal: Vec<u16> = "\u{00B6}".encode_utf16().collect();
  assert_eq!(normal, utf16!("\u{00B6}"));
}
