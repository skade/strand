use state::State;
use strain::Strain;

pub trait Branchable {
  fn branch(self) -> Self;
}

impl<T, S: Clone + Strain<T>> Branchable for S {
  fn branch(self) -> S {
    self.clone()
  }
}