use utf16_lit::utf16;

utf16!(EXAMPLE, "example");
#[test]
fn test_example() {
  let normal: Vec<u16> = "example".encode_utf16().collect();
  assert_eq!(normal, EXAMPLE);
}

utf16!(TAB, "\t");
#[test]
fn test_tab() {
  let normal: Vec<u16> = "\t".encode_utf16().collect();
  assert_eq!(normal, TAB);
}

utf16!(NEWLINE, "\n");
#[test]
fn test_newline() {
  let normal: Vec<u16> = "\n".encode_utf16().collect();
  assert_eq!(normal, NEWLINE);
}

utf16!(CARRIAGE_RETURN, "\r");
#[test]
fn test_carriage_return() {
  let normal: Vec<u16> = "\r".encode_utf16().collect();
  assert_eq!(normal, CARRIAGE_RETURN);
}

utf16!(BACKSLASH, "\\");
#[test]
fn test_backslash() {
  let normal: Vec<u16> = "\\".encode_utf16().collect();
  assert_eq!(normal, BACKSLASH);
}

utf16!(NULL, "\0");
#[test]
fn test_null() {
  let normal: Vec<u16> = "\0".encode_utf16().collect();
  assert_eq!(normal, NULL);
}

utf16!(SINGLE_QUOTE, "\'");
#[test]
fn test_single_quote() {
  let normal: Vec<u16> = "\'".encode_utf16().collect();
  assert_eq!(normal, SINGLE_QUOTE);
}

utf16!(DOUBLE_QUOTE, "\"");
#[test]
fn test_double_quote() {
  let normal: Vec<u16> = "\"".encode_utf16().collect();
  assert_eq!(normal, DOUBLE_QUOTE);
}

utf16!(ESCAPED_ASCII, "\x52");
#[test]
fn test_escaped_ascii() {
  let normal: Vec<u16> = "\x52".encode_utf16().collect();
  assert_eq!(normal, ESCAPED_ASCII);
}

utf16!(ESCAPED_UNICODE, "\u{00B6}");
#[test]
fn test_escaped_unicode() {
  let normal: Vec<u16> = "\u{00B6}".encode_utf16().collect();
  assert_eq!(normal, ESCAPED_UNICODE);
}
