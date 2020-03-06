

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

pub fn perform_the_escaping(line: &str) -> Vec<char> {
  let mut out = vec![];

  let mut it = CharEscapeIterator::new(line.chars());

  while let Some(esc_ch) = it.next() {
    match esc_ch {
      CharEscape::Escaped(ch) | CharEscape::Literal(ch) => out.push(ch),
      other => panic!("escape sequence error: {:?}", other),
    }
  }

  out
}
