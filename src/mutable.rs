use errors::Errors;
use state::{State,Mutable};
use strain;

pub trait Event<T: State> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &mut T) -> Result<(), Errors>;
}

pub trait Strain<T> {
  fn evolve(&mut self, event: &Event<T>) -> Result<(), Errors>;
}

impl<T: State, A: strain::Strain<T> + Mutable<T>> Strain<T> for A {
  fn evolve(&mut self, event: &Event<T>) -> Result<(), Errors>{
    event.precondition(self.state()).and_then(|_| {
      event.action(self.state()).and_then(|_| {
        event.postcondition(self.state())
      })
    })
  }
}