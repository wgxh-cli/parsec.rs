use parsec_core::prelude::*;
use crate::*;
use std::marker::PhantomData;

pub struct InspectOk<'a, I, O, E> {
  inspector: Box<dyn for<'b> Fn(&'b O) + 'a>,
  _marker: PhantomData<(I, E)>,
}
impl<'a, I, O, E> Comb<'a, I, Result<O, E>, Result<O, E>> for InspectOk<'a, I, O, E> {
  fn comb(
    self,
    parser: &'a (impl Parse<'a, I, Result<O, E>> + 'a)
  ) -> BoxedParser<'a, I, Result<O, E>> where Self: 'a {
    parser
      .pipe(inspect(move |res: &Result<O, E>| {
        if let Ok(ref r) = res {
          (self.inspector)(r);
        }
      }))
      .boxed()
  }
}
impl<'a, I, O, E> InspectOk<'a, I, O, E> {
  pub fn new(inspector: impl for<'b> Fn(&'b O) + 'a) -> Self {
    InspectOk {
      inspector: Box::new(inspector),
      _marker: PhantomData,
    }
  }
}
pub fn inspect_ok<'a, I, O, E>(inspector: impl for<'b> Fn(&'b O) + 'a) -> InspectOk<'a, I, O, E> {
  InspectOk::new(inspector)
}

pub struct InspectErr<'a, I, O, E> {
  inspector: Box<dyn for<'b> Fn(&'b E) + 'a>,
  _marker: PhantomData<(I, O)>,
}
impl<'a, I, O, E> Comb<'a, I, Result<O, E>, Result<O, E>> for InspectErr<'a, I, O, E> {
  fn comb(
    self,
    parser: &'a (impl Parse<'a, I, Result<O, E>> + 'a)
  ) -> BoxedParser<'a, I, Result<O, E>> where Self: 'a {
    parser
      .pipe(inspect(move |res| {
        if let Err(err) = res {
          (self.inspector)(err)
        }
      }))
  }
}
impl<'a, I, O, E> InspectErr<'a, I, O, E> {
  pub fn new(inspector: impl for<'b> Fn(&'b E) + 'a) -> Self {
    InspectErr {
      inspector: Box::new(inspector),
      _marker: PhantomData,
    }
  }
}
pub fn inspect_err<'a, I, O, E>(inspector: impl for<'b> Fn(&'b E) + 'a) -> InspectErr<'a, I, O, E> {
  InspectErr::new(inspector)
}
