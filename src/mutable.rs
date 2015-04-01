use errors::Errors;
use strand::Mutable;
use strand;

pub trait Event {
  type T;

  fn precondition(&self, state: &Self::T) -> Result<(), Errors>;

  fn postcondition(&self, state: &Self::T) -> Result<(), Errors>;

  fn action(&self, state: &mut Self::T) -> Result<(), Errors>;
}

pub trait Strand<T> {
  fn evolve<'a>(&'a mut self, event: &'a Event<T = T>) -> Result<(), Errors>;
}

pub trait AsEvent {
  fn as_event<T,S>(self) -> Box<Event<T = T>+Send>;
}

impl<T, A: strand::Strand<T> + Mutable<T>> Strand<T> for A {
  fn evolve<'a>(&'a mut self, event: &'a Event<T = T>) -> Result<(), Errors> {
    event.precondition(self.state()).and_then(|_| {
      event.action(self.state()).and_then(|_| {
        match event.postcondition(self.state()) {
          Ok(_) => Ok(()),
          Err(e) => Err(e)
        }
      })
    })
  }
}
