use errors::Errors;
use state::{State,Immutable};
use strain;

pub trait Event<T: State> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &T) -> Result<T, Errors>;
}

pub trait Strain<T: State, A: strain::Strain<T> + Immutable<T>> {
  fn evolve(self, event: &Event<T>) -> Result<~A, Errors>;
}

impl<T: State, A: strain::Strain<T> + Immutable<T>> Strain<T, A> for A {
  fn evolve(self, event: &Event<T>) -> Result<~A, Errors>{
    event.precondition(self.state()).and_then(|_| {
      event.action(self.state()).and_then(|state| {
        match event.postcondition(self.state()) {
          Ok(_) => {
            let new_strain: ~A = strain::Strain::new(state);
            Ok( new_strain )
          },
          Err(errval) => { Err(errval) }
        }
      })
    })
  }
}