use crate::prelude::*;
use std::marker::PhantomData;

pub struct Map<'a, A, B, C> {
  mapper: BoxedParser<'a, B, C>,
  _marker: PhantomData<A>,
}
impl<'a, A: 'a, B: 'a, C: 'a> Comb<'a, A, B, C> for Map<'a, A, B, C> {
  fn comb(
    &'a self,
    parser: &'a (impl Parse<'a, A, B> + 'a)
  ) -> BoxedParser<'a, A, C> {
    with_fn::<'a, A, C>(move |a| {
      self.mapper.parse(parser.parse(a))
    }).boxed()
  }
}
impl<'a, A, B, C> Map<'a, A, B, C> {
  pub fn new(mapper: impl Parse<'a, B, C> + 'a) -> Self {
    Map {
      mapper: mapper.boxed(),
      _marker: PhantomData,
    }
  }
}
