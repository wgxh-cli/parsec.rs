use std::process::Output;

use crate::prelude::*;

pub struct Repeat<'a, I, O, E> {
  parser: BoxedParser<'a, I, O, E>,
}
impl<'a, I, O, E> Parse<'a, I, Vec<O>, E> for Repeat<'a, I, O, E> {
  fn parse(&self, context: I) -> ParseResult<I, Vec<O>, E> {
    let mut next_input = context;
    let mut results: Vec<O> = Vec::new();
    while let Ok((input, output)) = self.parser.parse(next_input) {
      next_input = input;
      results.push(output)
    }

    Ok((next_input, results))
  }
}
