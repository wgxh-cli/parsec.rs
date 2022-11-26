use crate::prelude::*;

pub struct And<'a, I, O1, O2, E> {
  first: BoxedParser<'a, I, O1, E>,
  second: BoxedParser<'a, I, O2, E>,
}
impl<'a, I, O1, O2, E> Parse<'a, I, (O1, O2), E> for And<'a, I, O1, O2, E> {
  fn parse(&self, context: I) -> ParseResult<I, (O1, O2), E> {
    self.first.parse(context).and_then(|(remain, o1)| {
      self.second.parse(remain).map(|(remain, o2)| (remain, (o1, o2)))
    })
  }
}
impl<'a, I, O1, O2, E> And<'a, I, O1, O2, E> {
  pub fn new(
    first: impl Parse<'a, I, O1, E> + 'a,
    second: impl Parse<'a, I, O2, E> + 'a,
  ) -> Self {
    And {
      first: first.boxed(),
      second: second.boxed(),
    }
  }
}

pub struct AndFirst<'a, I, O1, O2, E> {
  first: BoxedParser<'a, I, O1, E>,
  second: BoxedParser<'a, I, O2, E>,
}
impl<'a, I, O1, O2, E> Parse<'a, I, O1, E> for AndFirst<'a, I, O1, O2, E> {
  fn parse(&self, context: I) -> ParseResult<I, O1, E> {
    And::new(&self.first, &self.second)
      .map(|(first, _)| first)
      .parse(context)
  }
}
