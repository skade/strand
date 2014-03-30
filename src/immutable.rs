use errors::Errors;
use state::State;
use strain;

pub trait Event<T: State + Clone> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &T) -> Result<T, Errors>;
}

pub trait Strain<T: State + Clone> {
  fn evolve(self, event: &Event<T>) -> Result<strain::Strain<T>, Errors>;
  fn branch(&self) -> Self;
}

impl<T: State + Clone> Strain<T> for strain::Strain<T> {
  fn evolve(self, event: &Event<T>) -> Result<strain::Strain<T>, Errors>{
    event.precondition(self.state).and_then(|_| {
      event.action(self.state).and_then(|state| {
        match event.postcondition(self.state) {
          Ok(_) => { Ok(strain::Strain { state: ~state }) },
          Err(errval) => { Err(errval) }
        }
      })
    })
  }

  fn branch(&self) -> strain::Strain<T> {
    self.clone()
  }
}