use crate::prelude::*;

pub struct BoxedParser<'a, I, O> {
  parser: Box<dyn Parse<'a, I, O> + 'a>
}
impl<'a, I, O> Parse<'a, I, O> for BoxedParser<'a, I, O> {
  fn parse(&self, input: I) -> O {
    self.parser.parse(input)
  }
}
impl<'a, I, O> BoxedParser<'a, I, O> {
  pub fn new(parser: impl Parse<'a, I, O> + 'a) -> Self {
    BoxedParser {
      parser: Box::new(parser),
    }
  }
}
