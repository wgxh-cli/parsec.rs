//! The `general` module defined some context-free combinators.
//!
//! In rust, that is, will not specialize the generics of `Parse`.
//!
//! There are the defined combinator module below:
//! * map - A -> B -> C => A -> C

pub mod maps;
pub mod inspect;

pub use maps::*;
pub use inspect::*;
