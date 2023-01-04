use crate::prelude::*;
use std::marker::PhantomData;

pub struct Map<'a, A, B, C> {
  mapper: BoxedParser<'a, B, C>,
  _marker: PhantomData<A>,
}
impl<'a, A: 'a, B: 'a, C: 'a> Comb<'a, A, B, C> for Map<'a, A, B, C> {
  fn comb(
    self,
    parser: &'a (impl Parse<'a, A, B> + 'a)
  ) -> BoxedParser<'a, A, C> where Self: 'a {
    with_fn(move |a| self.mapper.parse(parser.parse(a))).boxed()
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

/// `MapFn` is actually a shortcut for `Map::new(with_fn(...))`
///
/// See also `Map`
pub struct MapFn<'a, A, B, C> {
  mapper: Box<dyn Fn(B) -> C + 'a>,
  _marker: PhantomData<&'a A>,
}
impl<'a, A, B, C> Comb<'a, A, B, C> for MapFn<'a, A, B, C> {
  fn comb(
    self,
    parser: &'a (impl Parse<'a, A, B> + 'a)
  ) -> BoxedParser<'a, A, C> where Self: 'a {
    Map::new(with_fn(move |b| (self.mapper)(b))).comb(parser)
  }
}
impl<'a, A, B, C> MapFn<'a, A, B, C> {
  pub fn new(mapper: impl Fn(B) -> C + 'a) -> Self {
    MapFn {
      mapper: Box::new(mapper),
      _marker: PhantomData,
    }
  }
}

// NOTICE! This is just a test to find possible lifetime problems in use.
// It will be changed soon.
#[cfg(test)]
mod test {
  use crate::prelude::*;
  use super::MapFn;

  pub struct Next;
  impl<'a> Parse<'a, String, char> for Next {
    fn parse(&self, mut input: String) -> char {
      input.pop().unwrap()
    }
  }

  #[test]
  fn test() {
    let test_suit = "ASS".to_string();

    assert_eq!(Next.parse(test_suit.clone()), 'S');
    assert_eq!(
      Next
        .pipe(MapFn::new(|c: char| {
          c.to_lowercase().to_string()
        }))
        .parse(test_suit),
      's'.to_string()
    );
  }
}

