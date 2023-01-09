//! # `errproc`
//!
//! The series of `errproc` combinators are used to adapt Rust's builtin types.
//! Such as, `Result` or `Option`, and there's some traits to generalize these combinators.
//!
//! They are usually useful when work with return types with both success and failure states.
//! But these traits enable you to use your own types instead of builtin types.

pub mod maps;
pub mod inspect;
