pub mod prelude;
pub mod utils;
pub mod combs;

use prelude::*;

/// The `Parse` trait
///
/// **Definition**: A executable trait that take an input and returns a output.
pub trait Parse<'a, I: 'a, O: 'a> {
  fn parse(&'a self, input: I) -> O;

  fn cloned(&'a self) -> wrappers::BoxedParser<'a, I, O> {
    wrappers::with_fn(move |i| self.parse(i)).boxed()
  }
  fn pipe<C>(&'a self, f: impl Parse<'a, O, C> + 'a) -> pipes::Pipe<'a, I, O, C> where Self: Sized + 'a {
    pipes::Pipe::new(f, wrappers::with_fn(move |a| self.parse(a)))
  }
  fn boxed(self) -> wrappers::BoxedParser<'a, I, O> where Self: Sized + 'a {
    wrappers::BoxedParser::new(self)
  }
}

