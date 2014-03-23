#[crate_type = "lib"];
#[crate_id = "strain#0.0.1"];
#[feature(globs,phase)];
#[phase(syntax, link)] extern crate log;

extern crate leveldb;

pub trait State {
}

pub trait Event<T: State> {
  fn precondition(&self, state: &T) -> Result<(), ~str>;

  fn postcondition(&self, state: &T) -> Result<(), ~str>;

  fn action(&self, state: &mut T) -> Result<(), ~str>;
}

pub struct Strain<T> {
  state: ~T,
}

impl<T: State> Strain<T> {
  pub fn new<T: State>(state: ~T) -> Strain<T> {
    Strain { state: state }
  }

  pub fn feed(&mut self, event: &Event<T>) {
    event.precondition(self.state);
    event.action(self.state);
    event.postcondition(self.state);
  }

  pub fn state(self) -> ~T {
    self.state
  }
}

#[cfg(test)]
mod tests {
  use super::{State, Event, Strain};

  struct Counter {
    count: int,
  }
  impl State for Counter {}

  struct Increment;
  struct Decrement;

  impl Event<Counter> for Increment {
    fn precondition(&self, state: &Counter) -> Result<(), ~str> {
      if state.count < 0 {
        Err(~"I cannot count to negatives")
      } else {
        Ok(())
      }
    }

    fn action(&self, state: &mut Counter) -> Result<(), ~str>  {
      state.count = state.count + 1;
      Ok(())
    }

    fn postcondition(&self, state: &Counter) -> Result<(), ~str> {
      if state.count < 0 {
        Err(~"I shouldn't have counted to negatives")
      } else {
        Ok(())
      }
    }
  }

  impl Event<Counter> for Decrement {
    fn precondition(&self, state: &Counter) -> Result<(), ~str> {
      if state.count > 0 {
        Err(~"I cannot count to negatives")
      } else {
        Ok(())
      }
    }

    fn action(&self, state: &mut Counter) -> Result<(), ~str>  {
      state.count = state.count - 1;
      Ok(())
    }

    fn postcondition(&self, state: &Counter) -> Result<(), ~str> {
      if state.count < 0 {
        Err(~"I shouldn't have counted to negatives")
      } else {
        Ok(())
      }
    }

  }

  #[test]
  fn test_state_changes() {
    let mut strain : Strain<Counter> = Strain { state: ~Counter { count: 0 } };
    strain.feed(&Increment);
    strain.feed(&Increment);
    strain.feed(&Decrement);
    assert_eq!(strain.state().count, 1);
  }
}
