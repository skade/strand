use errors::Errors;
use state::State;
use strain;

pub trait Event<T: State> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &T) -> Result<T, Errors>;
}

pub trait Strain<T: State> {
  fn feed(self, event: &Event<T>) -> Result<strain::Strain<T>, Errors>;
}

impl<T: State> Strain<T> for strain::Strain<T> {
  fn feed(self, event: &Event<T>) -> Result<strain::Strain<T>, Errors>{
    event.precondition(self.state).and_then(|_| {
      event.action(self.state).and_then(|state| {
        match event.postcondition(self.state) {
          Ok(_) => { Ok(strain::Strain { state: ~state }) },
          Err(errval) => { Err(errval) }
        }
      })
    })
  }
}