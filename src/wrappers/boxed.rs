use crate::prelude::*;

pub struct BoxedParser<'a, I, O, E> {
  parser: Box<dyn Parse<'a, I, O, E> + 'a>,
}
impl<'a, I, O, E> Parse<'a, I, O, E> for BoxedParser<'a, I, O, E> {
  fn parse(&self, context: I) -> ParseResult<I, O, E> {
    self.parser.parse(context)
  }
}
impl<'a, I, O, E> BoxedParser<'a, I, O, E> {
  pub fn new(parser: impl Parse<'a, I, O, E> + 'a) -> Self {
    BoxedParser {
      parser: Box::new(parser),
    }
  }
}
