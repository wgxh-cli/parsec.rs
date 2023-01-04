pub mod prelude;
pub mod utils;
pub mod combs;

use prelude::*;

/// The `Parse` trait
///
/// **Definition**: A executable trait that take an input and returns a output.
pub trait Parse<'a, I, O> {
  fn parse(&'a self, input: I) -> O;

  fn pipe<AO>(
    &'a self,
    mapper: &'a (impl Comb<'a, I, O, AO> + 'a)
  ) -> BoxedParser<'a, I, AO> where Self: Sized + 'a {
    mapper.comb(self)
  }
  fn boxed(self) -> wrappers::BoxedParser<'a, I, O> where Self: Sized + 'a {
    wrappers::BoxedParser::new(self)
  }
}

