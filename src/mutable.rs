use errors::Errors;
use state::State;
use strain;
use branchable::Branchable;

pub trait Event<T: State> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &mut T) -> Result<(), Errors>;
}

pub trait Strain<T: State> {
  fn evolve(&mut self, event: &Event<T>) -> Result<(), Errors>;
}

impl<T: State> Strain<T> for strain::Strain<T> {
  fn evolve(&mut self, event: &Event<T>) -> Result<(), Errors>{
    event.precondition(self.state).and_then(|_| {
      event.action(self.state).and_then(|_| {
        event.postcondition(self.state)
      })
    })
  }
}