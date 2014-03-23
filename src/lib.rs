#[crate_type = "lib"];
#[crate_id = "strain#0.0.1"];
#[feature(globs,phase)];
#[phase(syntax, link)] extern crate log;

extern crate leveldb;

#[deriving(Show)]
pub enum Errors {
  PreConditionNotMet(~str),
  PostConditionNotMet(~str),
  ActionFailed(~str)
}

pub trait State {
}

pub trait Event<T: State> {
  fn precondition(&self, state: &T) -> Result<(), Errors>;

  fn postcondition(&self, state: &T) -> Result<(), Errors>;

  fn action(&self, state: &mut T) -> Result<(), Errors>;
}

pub struct Strain<T> {
  state: ~T,
}

impl<T: State> Strain<T> {
  pub fn new<T: State>(state: ~T) -> Strain<T> {
    Strain { state: state }
  }

  pub fn feed(&mut self, event: &Event<T>) -> Result<(), Errors>{
    event.precondition(self.state).and_then(|_| {
      event.action(self.state).and_then(|_| {
        event.postcondition(self.state)
      })
    })
  }

  pub fn state(self) -> ~T {
    self.state
  }
}

#[cfg(test)]
mod tests {
  use super::{State, Event, Strain, Errors, PreConditionNotMet, PostConditionNotMet};

  struct Counter {
    count: int,
  }
  impl State for Counter {}

  struct Increment;
  struct Decrement;

  impl Event<Counter> for Increment {
    fn precondition(&self, state: &Counter) -> Result<(), Errors> {
      if state.count < 0 {
        Err(PreConditionNotMet(~"I cannot count to negatives"))
      } else {
        Ok(())
      }
    }

    fn action(&self, state: &mut Counter) -> Result<(), Errors>  {
      state.count = state.count + 1;
      Ok(())
    }

    fn postcondition(&self, state: &Counter) -> Result<(), Errors> {
      if state.count < 0 {
        Err(PostConditionNotMet(~"I shouldn't have counted to negatives"))
      } else {
        Ok(())
      }
    }
  }

  impl Event<Counter> for Decrement {
    fn precondition(&self, state: &Counter) -> Result<(), Errors> {
      if state.count < 1 {
        Err(PreConditionNotMet(~"I cannot count to negatives"))
      } else {
        Ok(())
      }
    }

    fn action(&self, state: &mut Counter) -> Result<(), Errors>  {
      state.count = state.count - 1;
      Ok(())
    }

    fn postcondition(&self, state: &Counter) -> Result<(), Errors> {
      if state.count < 0 {
        Err(PostConditionNotMet(~"I shouldn't have counted to negatives"))
      } else {
        Ok(())
      }
    }

  }

  #[test]
  fn test_state_changes() {
    let mut strain : Strain<Counter> = Strain { state: ~Counter { count: 0 } };
    let res = strain.feed(&Increment).and_then(|_| {
      strain.feed(&Increment).and_then(|_| {
        strain.feed(&Decrement)
      })
    });
    assert!(res.is_ok());
    assert_eq!(strain.state().count, 1);
  }

  #[test]
  fn test_unmet_pre_condition() {
    let mut strain : Strain<Counter> = Strain { state: ~Counter { count: -1 } };
    let res = strain.feed(&Increment);
    assert!(res.is_err());
    assert_eq!(strain.state().count, -1);
  }
}
