use crate::prelude::*;

pub struct Next;
impl<'a> Parse<'a, String, char> for Next {
  fn parse(&self, context: String) -> ParseResult<String, char, String> {
    let mut chars = context.chars();

    chars
      .next()
      .map(|char| (chars.collect(), char))
      .ok_or_else(|| "empty string".to_string())
  }
}
