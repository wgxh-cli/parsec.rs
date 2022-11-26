use crate::prelude::*;
use std::marker::PhantomData;

pub struct WithFn<'a, I, O, E> {
  func: Box<dyn Fn(I) -> ParseResult<I, O, E> + 'a>,
  marker: PhantomData<(I, O)>,
}
impl<'a, I, O, E> Parse<'a, I, O, E> for WithFn<'a, I, O, E> {
  fn parse(&self, context: I) -> ParseResult<I, O, E> {
    (self.func)(context)
  }
}
impl<'a, I, O, E> WithFn<'a, I, O, E> {
  pub fn new(func: impl Fn(I) -> ParseResult<I, O, E> + 'a) -> Self {
    WithFn {
      func: Box::new(func),
      marker: PhantomData
    }
  }
}
