use utf16_lit::utf16_lit;

utf16_lit!(EXAMPLE, "example");
#[test]
fn test_example() {
  let normal: Vec<u16> = "example".encode_utf16().collect();
  assert_eq!(normal, EXAMPLE);
}

utf16_lit!(TAB, "\t");
#[test]
fn test_tab() {
  let normal: Vec<u16> = "\t".encode_utf16().collect();
  assert_eq!(normal, TAB);
}

utf16_lit!(NEWLINE, "\n");
#[test]
fn test_newline() {
  let normal: Vec<u16> = "\n".encode_utf16().collect();
  assert_eq!(normal, NEWLINE);
}

utf16_lit!(CARRIAGE_RETURN, "\r");
#[test]
fn test_carriage_return() {
  let normal: Vec<u16> = "\r".encode_utf16().collect();
  assert_eq!(normal, CARRIAGE_RETURN);
}

utf16_lit!(BACKSLASH, "\\");
#[test]
fn test_backslash() {
  let normal: Vec<u16> = "\\".encode_utf16().collect();
  assert_eq!(normal, BACKSLASH);
}

utf16_lit!(NULL, "\0");
#[test]
fn test_null() {
  let normal: Vec<u16> = "\0".encode_utf16().collect();
  assert_eq!(normal, NULL);
}

utf16_lit!(SINGLE_QUOTE, "\'");
#[test]
fn test_single_quote() {
  let normal: Vec<u16> = "\'".encode_utf16().collect();
  assert_eq!(normal, SINGLE_QUOTE);
}

utf16_lit!(DOUBLE_QUOTE, "\"");
#[test]
fn test_double_quote() {
  let normal: Vec<u16> = "\"".encode_utf16().collect();
  assert_eq!(normal, DOUBLE_QUOTE);
}

utf16_lit!(ESCAPED_ASCII, "\x52");
#[test]
fn test_escaped_ascii() {
  let normal: Vec<u16> = "\x52".encode_utf16().collect();
  assert_eq!(normal, ESCAPED_ASCII);
}

utf16_lit!(ESCAPED_UNICODE, "\u{00B6}");
#[test]
fn test_escaped_unicode() {
  let normal: Vec<u16> = "\u{00B6}".encode_utf16().collect();
  assert_eq!(normal, ESCAPED_UNICODE);
}
