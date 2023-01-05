use crate::prelude::*;
use std::marker::PhantomData;

pub struct MapOk<'a, I, T, E, O> {
  mapper: Box<dyn Fn(T) -> O + 'a>,
  _marker: PhantomData<(I, E)>,
}
impl<'a, I, T, E, O, S1, S2> Comb<'a, I, S1, S2> for MapOk<'a, I, T, E, O>
where
  S1: Res<T, E> + 'a,
  S2: Res<O, E> + 'a,
{
  fn comb(
    self,
    parser: &'a (impl Parse<'a, I, S1> + 'a)
  ) -> BoxedParser<'a, I, S2> where Self: 'a {
    with_fn(move |i| {
      match parser.parse(i).to_state() {
        ResStates::Succeed(result) => S2::from_state(ResStates::Succeed((self.mapper)(result))),
        ResStates::Failed(err) => S2::from_state(ResStates::Failed(err))
      }
    }).boxed()
  }
}
impl<'a, I, T, E, O> MapOk<'a, I, T, E, O> {
  pub fn new(mapper: impl Fn(T) -> O + 'a) -> Self {
    MapOk {
      mapper: Box::new(mapper),
      _marker: PhantomData,
    }
  }
}
pub fn map_ok<'a, I, T, E, O>(mapper: impl Fn(T) -> O + 'a) -> MapOk<'a, I, T, E, O> {
  MapOk::new(mapper)
}

pub struct MapErr<'a, I, T, E, O> {
  mapper: Box<dyn Fn(E) -> O + 'a>,
  _marker: PhantomData<(I, T)>
}
impl<'a, I, T, E, O, S1, S2> Comb<'a, I, S1, S2> for MapErr<'a, I, T, E, O>
where
  S1: Res<T, E> + 'a,
  S2: Res<T, O> + 'a,
{
  fn comb(
    self,
    parser: &'a (impl Parse<'a, I, S1> + 'a)
  ) -> BoxedParser<'a, I, S2> where Self: 'a {
    with_fn(move |i| {
      match parser.parse(i).to_state() {
        ResStates::Failed(err) => S2::from_state(ResStates::Failed((self.mapper)(err))),
        ResStates::Succeed(result) => S2::from_state(ResStates::Succeed(result)),
      }
    }).boxed()
  }
}
impl<'a, I, T, E, O> MapErr<'a, I, T, E, O> {
  pub fn new(mapper: impl Fn(E) -> O + 'a) -> Self {
    MapErr {
      mapper: Box::new(mapper),
      _marker: PhantomData,
    }
  }
}
pub fn map_err<'a, I, T, E, O>(mapper: impl Fn(E) -> O + 'a) -> MapErr<'a, I, T, E, O> {
  MapErr::new(mapper)
}
