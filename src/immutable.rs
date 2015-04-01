use errors::Errors;
use strand::Immutable;
use strand;

pub trait Event {
  type T;

  fn precondition(&self, state: Self::T) -> Result<(), Errors>;

  fn postcondition(&self, state: Self::T) -> Result<(), Errors>;

  fn action(&self, state: Self::T) -> Result<Self::T, Errors>;
}

pub trait Strand<T, A: strand::Strand<T> + Immutable<T>> {
  fn evolve(self, event: &Event<T = T>) -> Result<A, Errors>;
}

pub trait AsEvent {
  fn as_event<T>(self) -> Box<Event<T = T>+Send>;
}

impl<T, A: strand::Strand<T> + Immutable<T>> Strand<T, A> for A {
  fn evolve(self, event: &Event<T = T>) -> Result<A, Errors>{
    event.precondition(self.state()).and_then(|_| {
      event.action(self.state()).and_then(|state| {
        match event.postcondition(self.state()) {
          Ok(_) => {
            let new_strand: A = strand::Strand::new(state);
            Ok( new_strand )
          },
          Err(errval) => { Err(errval) }
        }
      })
    })
  }
}
