use crate::prelude::*;
use std::marker::PhantomData;

pub struct Inspect<'a, I, O> {
  inspector: Box<dyn for<'b> Fn(&'b O) + 'a>,
  _marker: PhantomData<I>,
}
impl<'a, I, O> Comb<'a, I, O, O> for Inspect<'a, I, O> {
  fn comb(
    self,
    parser: &'a (impl Parse<'a, I, O> + 'a)
  ) -> BoxedParser<'a, I, O> where Self: 'a {
    with_fn(move |i| {
      let result = parser.parse(i);
      (self.inspector)(&result);

      result
    }).boxed()
  }
}
impl<'a, I, O> Inspect<'a, I, O> {
  pub fn new(inspector: impl for<'b> Fn(&'b O) + 'a) -> Self {
    Inspect {
      inspector: Box::new(inspector),
      _marker: PhantomData,
    }
  }
}
pub fn inspect<'a, I, O>(inspector: impl for<'b> Fn(&'b O) + 'a) -> Inspect<'a, I, O> {
  Inspect::new(inspector)
}

#[cfg(test)]
mod test {
  use crate::prelude::*;
  use super::inspect;

  struct Next;
  impl<'a> Parse<'a, String, char> for Next {
    fn parse(&self, input: String) -> char {
      input.chars().next().unwrap()
    }
  }

  #[test]
  fn test() {
    let test_suit = "wgxh".to_string();
    assert_eq!(
      Next
        .pipe(inspect(|char| { dbg!(char); }))
        .parse(test_suit),
      'w'
    );
  }
}
