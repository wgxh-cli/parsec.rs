pub mod combs;
pub mod wrappers;
pub mod prelude;

pub type ParseResult<I, O, E> = Result<(I, O), E>;
pub trait Parse<'a, I, O, E = String> {
  fn parse(&self, context: I) -> ParseResult<I, O, E>;
}
