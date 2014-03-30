use state::State;

#[deriving(Clone)]
pub struct Strain<T> {
  state: ~T,
}

impl<T: State> Strain<T> {
  pub fn new<T: State>(state: ~T) -> Strain<T> {
    Strain { state: state }
  }

  pub fn state(self) -> ~T {
    self.state
  }
}