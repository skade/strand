use errors::Errors;
use strand::Mutable;
use strand;

pub trait Event<T> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &mut T) -> Result<(), Errors>;
}

pub trait Strand<T> {
  fn evolve(&mut self, event: &Event<T>) -> Result<(), Errors>;
}

impl<T, A: strand::Strand<T> + Mutable<T>> Strand<T> for A {
  fn evolve(&mut self, event: &Event<T>) -> Result<(), Errors>{
    event.precondition(self.state()).and_then(|_| {
      event.action(self.state()).and_then(|_| {
        event.postcondition(self.state())
      })
    })
  }
}
