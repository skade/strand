pub trait Strand<T> {
  fn new(state: T) -> Self;
}
pub trait Mutable<T> : Strand<T> {
  fn state<'a>(&'a mut self) -> &'a mut T;
}
pub trait Immutable<T> : Strand<T> {
  fn state(&self) -> T;
}
