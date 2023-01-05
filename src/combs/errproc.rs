//! # `errproc`
//!
//! The series of `errproc` combinators are used to adapt Rust's builtin types.
//! Such as, `Result` and `Option`, and there's some traits to generalize these combinators.
//!
//! They are usually useful when work with return types with both success and failure states.
//! But these traits enable you to use your own types instead of builtin types.

pub mod maps;

pub enum ResStates<T, E> {
  Succeed(T),
  Failed(E),
}

pub trait Res<T, E> {
  fn to_state(&self) -> ResStates<T, E>;
  fn from_state(state: ResStates<T, E>) -> Self;
  #[inline]
  fn succeed(&self) -> bool {
    if let ResStates::<T, E>::Succeed(_) = self.to_state() {
      true
    } else {
      false
    }
  }
  #[inline]
  fn failed(&self) -> bool {
    !self.succeed()
  }
}
impl<T: Clone, E: Clone> Res<T, E> for Result<T, E> {
  fn to_state(&self) -> ResStates<T, E> {
    match self {
      Ok(result) => ResStates::Succeed(result.clone()),
      Err(err) => ResStates::Failed(err.clone()),
    }
  }
  fn from_state(state: ResStates<T, E>) -> Self {
    match state {
      ResStates::Succeed(result) => Ok(result),
      ResStates::Failed(err) => Err(err),
    }
  }
}
impl<T: Clone> Res<T, ()> for Option<T> {
  fn to_state(&self) -> ResStates<T, ()> {
    match self {
      Some(result) => ResStates::Succeed(result.clone()),
      None => ResStates::Failed(()),
    }
  }
  fn from_state(state: ResStates<T, ()>) -> Self {
    match state {
      ResStates::Succeed(result) => Some(result),
      ResStates::Failed(_) => None,
    }
  }
}
