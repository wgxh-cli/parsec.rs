use crate::prelude::*;

pub trait ToParser<'a, I, O, E> {
  fn to_parser(self) -> BoxedParser<'a, I, O, E>;
}
pub trait AsParser<'a, I, O, E> {
  fn as_parser(&self) -> BoxedParser<'a, I, O, E>;
}

impl<'a, I: 'a, O: 'a, E: 'a, F> ToParser<'a, I, O, E> for F where F: Fn(I) -> ParseResult<I, O, E> + 'a {
  fn to_parser(self) -> BoxedParser<'a, I, O, E> where Self: Sized {
    WithFn::new(self).boxed()
  }
}
