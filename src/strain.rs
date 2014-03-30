use state::State;

pub trait Strain<T: State> {
  fn new(state: T) -> ~Strain<T>;
}

//impl Strain<~int> for ~int {
//  fn new(state: ~int) -> ~Strain<~int> {
//    ~state as ~Strain<~int>
//  }
//  fn state(&self) -> &~int {
//    &self.clone()
//  }
//}
//
//impl Strain<int> for int {
//  fn new(state: int) -> ~Strain<int> {
//    ~state as ~Strain<int>
//  }
//  fn state(&self) -> &int {
//    self
//  }
//}

//#[deriving(Clone)]
//pub struct Strain<T> {
//  state: ~T,
//}
//
//impl<T: State + Clone> Strain<T> {
//  pub fn new<T: State>(state: ~T) -> Strain<T> {
//    Strain { state: state }
//  }
//
//  pub fn state(self) -> ~T {
//    self.state
//  }
//}