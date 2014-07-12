use state::State;

pub trait Strand<T: State> {
  fn new(state: T) -> Self;
}
pub trait Mutable<T: State> : Strand<T> {
  fn state<'a>(&'a mut self) -> &'a mut T;
}
pub trait Immutable<T: State> : Strand<T> {
  fn state(&self) -> T;
}

impl Strand<Box<int>> for Box<int> {
  fn new(state: Box<int>) -> Box<int> {
    state
  }
}

impl Strand<int> for int {
  fn new(state: int) -> int {
    state
  }
}

impl Immutable<int> for int {
  fn state(&self) -> int {
    self.clone()
  }
}

//impl<T: Clone> Immutable<T> for T {
//  fn state(&self) -> ~T {
//    ~self.clone()
//  }
//}
