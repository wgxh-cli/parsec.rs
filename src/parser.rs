use crate::pipers::*;

pub trait Parse<'a, I, O> {
  fn parse(&self, input: I) -> O;
}
pub trait ParseExt<'a, I: 'a, O: 'a>: Parse<'a, I, O> {
  fn boxed(self) -> BoxedParser<'a, I, O> where Self: Sized + 'a {
    BoxedParser::new(self)
  }
  fn cloned(&'a self) -> ClonedParser<'a, I, O>  where Self: Sized + 'a {
    ClonedParser {
      parser: self,
    }
  }
  fn pipe<O2>(&'a self, piper: impl Piper<'a, I, O, O2> + 'a) -> BoxedParser<'a, I, O2> where Self: Sized + 'a {
    piper.comb(self.cloned())
  }
}
impl<'a, I: 'a, O: 'a, P> ParseExt<'a, I, O> for P where P: Parse<'a, I, O> {}

pub struct BoxedParser<'a, I, O> {
  parser: Box<dyn Parse<'a, I, O> + 'a>,
}
impl<'a, I, O> Parse<'a, I, O> for BoxedParser<'a, I, O> {
  fn parse(&self, input: I) -> O {
    self.parser.parse(input)
  }
}
impl<'a, I, O> BoxedParser<'a, I, O> {
  pub fn new(parser: impl Parse<'a, I, O> + 'a) -> Self {
    BoxedParser {
      parser: Box::new(parser),
    }
  }
}

pub struct ClonedParser<'a, I, O> {
  parser: &'a dyn Parse<'a, I, O>,
}
impl<'a, I, O> Parse<'a, I, O> for ClonedParser<'a, I, O> {
  fn parse(&self, input: I) -> O {
    self.parser.parse(input)
  }
}
impl<'a, I, O> Clone for ClonedParser<'a, I, O> {
  fn clone(&self) -> Self {
    ClonedParser {
      parser: self.parser.clone(),
    }
  }
}

pub struct FnParser<'a, I, O> {
  parser: Box<dyn Fn(I) -> O + 'a>,
}
impl<'a, I, O> Parse<'a, I, O> for FnParser<'a, I, O> {
  fn parse(&self, input: I) -> O {
    (self.parser)(input)
  }
}
impl<'a, I, O> FnParser<'a, I, O> {
  pub fn new(parser: impl Fn(I) -> O + 'a) -> Self {
    FnParser {
      parser: Box::new(parser),
    }
  }
}
pub fn with_fn<'a, I, O>(func: impl Fn(I) -> O + 'a) -> FnParser<'a, I, O> {
  FnParser::new(func)
}

