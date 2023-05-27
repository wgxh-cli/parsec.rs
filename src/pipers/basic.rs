use super::*;

pub struct Mapper<'a, I, O1, O2> {
  source: BoxedParser<'a, I, O1>,
  mapper: BoxedParser<'a, O1, O2>,
}
impl<'a, I, O1, O2> Parse<'a, I, O2> for Mapper<'a, I, O1, O2> {
  fn parse(&self, input: I) -> O2 {
    self.mapper.parse(self.source.parse(input))
  }
}
pub struct Map<'a, I, O1, O2> {
  mapper: BoxedParser<'a, O1, O2>,
  _marker: std::marker::PhantomData<I>,
}
impl<'a, I: 'a, O1: 'a, O2: 'a> Piper<'a, I, O1, O2> for Map<'a, I, O1, O2> {
  fn comb(self, parser: impl Parse<'a, I, O1> + 'a) -> BoxedParser<'a, I, O2> {
    Mapper {
      source: parser.boxed(),
      mapper: self.mapper,
    }.boxed()
  }
}
pub fn map<'a, I, O1: 'a, O2: 'a>(mapper: impl Parse<'a, O1, O2> + 'a) -> Map<'a, I, O1, O2> {
  Map {
    mapper: mapper.boxed(),
    _marker: std::marker::PhantomData,
  }
}
pub fn map_fn<'a, I, O1: 'a, O2: 'a>(func: impl Fn(O1) -> O2 + 'a) -> Map<'a, I, O1, O2> {
  map(with_fn(func))
}

