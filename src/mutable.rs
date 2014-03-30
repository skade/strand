use errors::Errors;
use state::State;
use strain;

pub trait Event<T: State> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &mut T) -> Result<(), Errors>;
}

pub trait Strain<T: State> {
  fn evolve(&mut self, event: &Event<T>) -> Result<(), Errors>;
  fn branch(&self) -> ~Self;
}

impl<T: State + Clone> Strain<T> for strain::Strain<T> {
  fn evolve(&mut self, event: &Event<T>) -> Result<(), Errors>{
    event.precondition(self.state).and_then(|_| {
      event.action(self.state).and_then(|_| {
        event.postcondition(self.state)
      })
    })
  }

  fn branch(&self) -> ~strain::Strain<T> {
    ~self.clone()
  }
}