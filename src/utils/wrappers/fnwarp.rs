use crate::prelude::*;

pub struct FnWrap<'a, I, O> {
  func: Box<dyn Fn(I) -> O + 'a>,
}
impl<'a, I: 'a, O: 'a> Parse<'a, I, O> for FnWrap<'a, I, O> {
  fn parse(&'a self, input: I) -> O {
    (self.func)(input)
  }
}
impl<'a, I, O> FnWrap<'a, I, O> {
  pub fn new(func: impl Fn(I) -> O + 'a) -> Self {
    FnWrap {
      func: Box::new(func),
    }
  }
}
pub fn with_fn<'a, I, O>(func: impl Fn(I) -> O + 'a) -> FnWrap<'a, I, O> {
  FnWrap::new(func)
}
