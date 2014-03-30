pub trait State {}
pub trait Mutable<T> {
  fn state<'a>(&'a mut self) -> &'a mut T;
}

impl State for int {}
impl State for ~int {}