use crate::prelude::*;

pub struct Map<'a, A, B, C> {
  parser: BoxedParser<'a, A, B>,
  mapper: Box<dyn Fn(B) -> C + 'a>,
}
impl<'a, A: 'a, B: 'a, C: 'a> Parse<'a, A, C> for Map<'a, A, B, C> {
  fn parse(&'a self, input: A) -> C {
    (self.mapper)(self.parser.parse(input))
  }
}
impl<'a, A, B, C> Map<'a, A, B, C> {
  pub fn new(parser: impl Parse<'a, A, B> + 'a, mapper: impl Fn(B) -> C + 'a) -> Self {
    Map {
      parser: parser.boxed(),
      mapper: Box::new(mapper),
    }
  }
}

