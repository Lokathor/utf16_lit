#[test]
fn collision_with_internal_names() {
  const UTF16: &str = "";
  utf16_lit::utf16!(UTF16);
}
#[test]
fn collision_with_previous_invocation() {
  const STUFF: &str = "";
  utf16_lit::utf16!(STUFF);
  utf16_lit::utf16!(STUFF);
}
