use errors::Errors;
use strand::Immutable;
use strand;

pub trait Event<T> {
  fn precondition(&self, state: T) -> Result<(), Errors>;

  fn postcondition(&self, state: T) -> Result<(), Errors>;

  fn action(&self, state: T) -> Result<T, Errors>;
}

pub trait Strand<T, A: strand::Strand<T> + Immutable<T>> {
  fn evolve(self, event: &Event<T>) -> Result<A, Errors>;
}

impl<T, A: strand::Strand<T> + Immutable<T>> Strand<T, A> for A {
  fn evolve(self, event: &Event<T>) -> Result<A, Errors>{
    event.precondition(self.state()).and_then(|_| {
      event.action(self.state()).and_then(|state| {
        match event.postcondition(self.state()) {
          Ok(_) => {
            let new_strain: A = strand::Strand::new(state);
            Ok( new_strain )
          },
          Err(errval) => { Err(errval) }
        }
      })
    })
  }
}
