use crate::prelude::*;
use std::marker::PhantomData;

pub struct MapOk<'a, I, O1, O2, E> {
  mapper: Box<dyn Fn(O1) -> O2 + 'a>,
  _marker: PhantomData<(I, E)>,
}
impl<'a, I, O1, O2, E> Comb<'a, I, Result<O1, E>, Result<O2, E>> for MapOk<'a, I, O1, O2, E> {
  fn comb(
    self,
    parser: &'a (impl Parse<'a, I, Result<O1, E>> + 'a)
  ) -> BoxedParser<'a, I, Result<O2, E>> where Self: 'a {
    parser
      .pipe(map_fn(move |res: Result<O1, E>| {
        res.map(|res| {
          (self.mapper)(res)
        })
      }))
      .boxed()
  }
}
impl<'a, I, O1, O2, E> MapOk<'a, I, O1, O2, E> {
  pub fn new(mapper: impl Fn(O1) -> O2 + 'a) -> Self {
    MapOk {
      mapper: Box::new(mapper),
      _marker: PhantomData,
    }
  }
}
pub fn map_ok<'a, I, O1, O2, E>(mapper: impl Fn(O1) -> O2 + 'a) -> MapOk<'a, I, O1, O2, E> {
  MapOk::new(mapper)
}

pub struct MapErr<'a, I, O, E1, E2> {
  mapper: Box<dyn Fn(E1) -> E2 + 'a>,
  _marker: PhantomData<(I, O)>,
}
impl<'a, I, O, E1, E2> Comb<'a, I, Result<O, E1>, Result<O, E2>> for MapErr<'a, I, O, E1, E2> {
  fn comb(
    self,
    parser: &'a (impl Parse<'a, I, Result<O, E1>> + 'a)
  ) -> BoxedParser<'a, I, Result<O, E2>> where Self: 'a {
    parser
      .pipe(map_fn(move |res: Result<O, E1>| {
        res.map_err(|err| {
          (self.mapper)(err)
        })
      }))
      .boxed()
  }
}
impl<'a, I, O, E1, E2> MapErr<'a, I, O, E1, E2> {
  pub fn new(mapper: impl Fn(E1) -> E2 + 'a) -> Self {
    MapErr {
      mapper: Box::new(mapper),
      _marker: PhantomData,
    }
  }
}
pub fn map_err<'a, I, O, E1, E2>(mapper: impl Fn(E1) -> E2 + 'a) -> MapErr<'a, I, O, E1, E2> {
  MapErr::new(mapper)
}
