use state::State;
use strain::Strain;

pub trait Branchable<T: State + Clone> {
  fn branch(&self) -> Self;
}

impl<T: State + Clone> Branchable<T> for Strain<T> {
  fn branch(&self) -> Strain<T> {
    self.clone()
  }
}