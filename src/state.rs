pub trait State {}
pub trait Mutable<T> {
  fn state<'a>(&'a mut self) -> &'a mut T;
}
pub trait Immutable<T> {
  fn state(&self) -> ~T;
}

impl State for int {}
impl State for ~int {}