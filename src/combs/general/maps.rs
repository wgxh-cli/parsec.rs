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
pub fn map<'a, A, B, C>(mapper: impl Parse<'a, B, C> + 'a) -> Map<'a, A, B, C> {
  Map::new(mapper)
}

/// `MapFn` is actually a shortcut for `Map::new(with_fn(...))`
///
/// See also `Map`
pub fn map_fn<'a, A: 'a, B: 'a, C: 'a>(mapper: impl Fn(B) -> C + 'a) -> Map<'a, A, B, C> {
  Map::new(with_fn(mapper))
}

// NOTICE! This is just a test to find possible lifetime problems in use.
// It will be changed soon.
#[cfg(test)]
mod test {
  use crate::prelude::*;
  use super::map_fn;

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
        .pipe(map_fn(|c: char| {
          c.to_lowercase().to_string()
        }))
        .parse(test_suit),
      's'.to_string()
    );
  }
}

