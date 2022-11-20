use crate::prelude::*;

/// Maps a result returned by a parser into another.
///
/// When working with `Map`, you should make sure that the `mapper` will never fail.
/// That is, `mapper` never takes side effects, it's completely pure.
///
/// # Arguments
///
/// * `parser` - The parser whose result is going to be mapped
/// * `mapper` - A mapper that maps the result into desired one
pub struct Map<'a, I, O1, O2, E> {
  parser: BoxedParser<'a, I, O1, E>,
  mapper: Box<dyn Fn(O1) -> O2 + 'a>,
}
impl<'a, I, O1, O2, E> Parse<'a, I, O2, E> for Map<'a, I, O1, O2, E> {
  fn parse(&self, context: I) -> ParseResult<I, O2, E> {
    self.parser.parse(context).map(|o1| {
      (o1.0, (self.mapper)(o1.1))
    })
  }
}
impl<'a, I: 'a, O1: 'a, O2: 'a, E: 'a> Map<'a, I, O1, O2, E> {
  pub fn new(
    parser: impl Parse<'a, I, O1, E> + 'a,
    mapper: impl Fn(O1) -> O2 + 'a
  ) -> Self {
    Map {
      parser: parser.boxed(),
      mapper: Box::new(mapper),
    }
  }
}

/// Maps a result returned by a parser into another with consideration of the possible errors.
///
/// If `Map` might be failed, then use `AndThen` instead.
///
/// # Arguments
///
/// * `parser` - The parser whose result is going to be mapped
/// * `mapper` - A mapper that maps the result into desired one
pub struct AndThen<'a, I, O1, O2, E> {
  parser: BoxedParser<'a, I, O1, E>,
  mapper: Box<dyn Fn(O1) -> Result<O2, E> + 'a>,
}
impl<'a, I, O1, O2, E> Parse<'a, I, O2, E> for AndThen<'a, I, O1, O2, E> {
  fn parse(&self, context: I) -> ParseResult<I, O2, E> {
    self.parser.parse(context).and_then(|(remain, o1)| {
      (self.mapper)(o1).map(|output| (remain, output))
    })
  }
}
impl<'a, I: 'a, O1: 'a, O2: 'a, E: 'a> AndThen<'a, I, O1, O2, E> {
  pub fn new(
    parser: impl Parse<'a, I, O1, E> + 'a,
    mapper: impl Fn(O1) -> Result<O2, E> + 'a
  ) -> Self {
    AndThen {
      parser: parser.boxed(),
      mapper: Box::new(mapper),
    }
  }
}
