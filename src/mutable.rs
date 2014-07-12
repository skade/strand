use errors::Errors;
use state::{State};
use strand::Mutable;
use strand;

pub trait Event<T: State> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &mut T) -> Result<(), Errors>;
}

pub trait Strand<T> {
  fn evolve(&mut self, event: &Event<T>) -> Result<(), Errors>;
}

impl<T: State, A: strand::Strand<T> + Mutable<T>> Strand<T> for A {
  fn evolve(&mut self, event: &Event<T>) -> Result<(), Errors>{
    event.precondition(self.state()).and_then(|_| {
      event.action(self.state()).and_then(|_| {
        event.postcondition(self.state())
      })
    })
  }
}
