pub trait Branchable {
  fn branch(self) -> Self;
}

impl Branchable for int {
  fn branch(self) -> int {
    self
  }
}