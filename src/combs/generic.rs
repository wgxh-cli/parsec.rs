pub mod empty;
pub mod map;
pub mod and;

use crate::prelude::*;

pub trait GenericExt<'a, I, O, E>: Parse<'a, I, O, E> {
  fn map<O2>(
    self,
    mapper: impl Fn(O) -> O2 + 'a
  ) -> map::Map<'a, I, O, O2, E> where Self: Sized + 'a {
    map::Map::new(self, mapper)
  }

  fn and<O2>(
    self,
    second: impl Parse<'a, I, O2, E> + 'a
  ) -> and::And<'a, I, O, O2, E> where Self: Sized + 'a {
    and::And::new(self, second)
  }
}
// Implement `GenericExt` to every parser.
impl<'a, I, O, E, P> GenericExt<'a, I, O, E> for P where P: Parse<'a, I, O, E> {}
