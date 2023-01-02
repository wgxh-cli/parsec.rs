use crate::prelude::*;

pub struct Pipe<'a, A, B, C> {
  f: Box<dyn Parse<'a, B, C> + 'a>,
  g: Box<dyn Parse<'a, A, B> + 'a>,
}
impl<'a, A: 'a, B: 'a, C: 'a> Parse<'a, A, C> for Pipe<'a, A, B, C> {
  fn parse(&'a self, input: A) -> C {
    self.f.parse(self.g.parse(input))
  }
}
impl<'a, A, B, C> Pipe<'a, A, B, C> {
  pub fn new(
    f: impl Parse<'a, B, C> + 'a,
    g: impl Parse<'a, A, B> + 'a,
  ) -> Self {
    Pipe {
      f: Box::new(f),
      g: Box::new(g),
    }
  }
}
