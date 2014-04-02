use state::State;

pub trait Strain<T: State> {
  fn new(state: T) -> Self;
}
pub trait Mutable<T: State> : Strain<T> {
  fn state<'a>(&'a mut self) -> &'a mut T;
}
pub trait Immutable<T: State> : Strain<T> {
  fn state(&self) -> T;
}

impl Strain<~int> for ~int {
  fn new(state: ~int) -> ~int {
    state
  }
}

impl Strain<int> for int {
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