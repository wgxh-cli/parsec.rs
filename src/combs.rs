pub mod general;

pub use general::*;

use crate::prelude::*;

pub trait Comb<'a, A, B, C> {
  fn comb(
    &'a self,
    parser: &'a (impl Parse<'a, A, B> + 'a)
  ) -> BoxedParser<'a, A, C>;
}

