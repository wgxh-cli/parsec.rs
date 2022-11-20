pub mod boxed;
pub mod with_fn;
pub mod convert;

pub use boxed::*;
pub use with_fn::*;
pub use convert::*;
use crate::prelude::*;

pub trait WrapperExt<'a, I: 'a, O: 'a, E: 'a>: Parse<'a, I, O, E> {
  fn boxed(self) -> BoxedParser<'a, I, O, E> where Self: Sized + 'a {
    BoxedParser::new(self)
  }
  fn caller(&'a self) -> BoxedParser<'a, I, O, E> where Self: Sized + 'a {
    WithFn::new(|i| {
      self.parse(i)
    }).boxed()
  }
}
impl<'a, I: 'a, O: 'a, E: 'a, P> WrapperExt<'a, I, O, E> for P where P: Parse<'a, I, O, E> + 'a {}
