use crate::prelude::*;

pub trait Comb<'a, A, B, C> {
  fn comb(
    self,
    parser: &'a (impl Parse<'a, A, B> + 'a)
  ) -> BoxedParser<'a, A, C> where Self: 'a;
}

