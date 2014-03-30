use errors::Errors;
use state::State;
use strain;
use branchable::Branchable;

pub trait Event<T: State> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &T) -> Result<T, Errors>;
}

pub trait Strain<T: State> {
  fn evolve(self, event: &Event<T>) -> Result<~strain::Strain<T>, Errors>;
}

impl<T: State, A: strain::Strain<T>> Strain<T> for A {
  fn evolve(self, event: &Event<T>) -> Result<~A, Errors>{
    event.precondition(self.state()).and_then(|_| {
      event.action(self.state()).and_then(|state| {
        match event.postcondition(self.state()) {
          Ok(_) => { Ok( strain::Strain::new<A>(~state)) },
          Err(errval) => { Err(errval) }
        }
      })
    })
  }
}