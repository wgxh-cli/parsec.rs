use crate::prelude::*;

/// A parser that will do nothing.
/// 
/// `Empty` will not change any of the input and just return `()` as result.
///
/// This is usually useful when build new parser with the others.
pub struct Empty;
impl<'a, I> Parse<'a, I, ()> for Empty {
  fn parse(&self, context: I) -> ParseResult<I, (), String> {
    Ok((context, ()))
  }
}
