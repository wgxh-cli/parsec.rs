pub mod combs;
pub mod prelude;

pub type ParseResult<I, O, E> = Result<(I, O), E>;
pub trait Parse<'a, I, O, E = String> {
  fn parse(&mut self, context: I) -> ParseResult<I, O, E>;
  fn boxed(self) -> BoxedParser<'a, I, O, E> where Self: Sized + 'a {
    BoxedParser::new(self)
  }
}
impl<'a, I, O, E, RP> Parse<'a, I, O, E> for &'a mut RP where RP: Parse<'a, I, O, E> {
  fn parse(&mut self, context: I) -> ParseResult<I, O, E> {
    (*self).parse(context)
  }
}

pub struct BoxedParser<'a, I, O, E> {
  parser: Box<dyn Parse<'a, I, O, E> + 'a>,
}
impl<'a, I, O, E> Parse<'a, I, O, E> for BoxedParser<'a, I, O, E> {
  fn parse(&mut self, context: I) -> ParseResult<I, O, E> {
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
