use errors::Errors;
use strand::Mutable;
use strand;

pub trait Event<T, S> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &mut T) -> Result<S, Errors>;
}

pub trait Strand<T> {
  fn evolve<'a, S>(&'a mut self, event: &'a Event<T, S>) -> Result<S, Errors>;
}

impl<T, A: strand::Strand<T> + Mutable<T>> Strand<T> for A {
  fn evolve<'a, S>(&'a mut self, event: &'a Event<T,S>) -> Result<S, Errors> {
    event.precondition(self.state()).and_then(|_| {
      event.action(self.state()).and_then(|result| {
        match event.postcondition(self.state()) {
          Ok(_) => Ok(result),
          Err(e) => Err(e)
        }
      })
    })
  }
}
