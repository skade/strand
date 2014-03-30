#![crate_type = "lib"]
#![crate_id = "strain#0.0.1"]
#![feature(globs,phase)]
#![phase(syntax, link)] extern crate log;

extern crate leveldb;

pub mod state;
pub mod mutable;
pub mod immutable;
pub mod errors;
pub mod strain;

#[cfg(test)]
mod tests {
  use super::state::State;
  use super::mutable::Event;
  use super::mutable::Strain;
  use super::strain;
  use super::errors::{Errors, PreConditionNotMet, PostConditionNotMet};

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
    let mut strain : strain::Strain<Counter> = strain::Strain { state: ~Counter { count: 0 } };
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
    let mut strain : strain::Strain<Counter> = strain::Strain { state: ~Counter { count: -1 } };
    let res = strain.feed(&Increment);
    assert!(res.is_err());
    assert_eq!(strain.state().count, -1);
  }
}


#[cfg(test)]
mod immutable_tests {
  use super::state::State;
  use super::immutable::Event;
  use super::immutable::Strain;
  use super::immutable;
  use super::strain;
  use super::errors::{Errors, PreConditionNotMet, PostConditionNotMet};

  #[deriving(Clone)]
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

    fn action(&self, state: &Counter) -> Result<Counter, Errors>  {
      Ok(Counter { count: state.count + 1})
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

    fn action(&self, state: &Counter) -> Result<Counter, Errors>  {
      Ok(Counter { count: state.count - 1})
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
    let strain : strain::Strain<Counter> = strain::Strain { state: ~Counter { count: 0 } };
    let res = strain.feed(&Increment).and_then(|state| {
      state.feed(&Increment).and_then(|state2| {
        state2.feed(&Decrement)
      })
    });
    assert!(res.is_ok());
    assert_eq!(res.unwrap().state().count, 1);
  }

  #[test]
  fn test_unmet_pre_condition() {
    let strain : strain::Strain<Counter> = strain::Strain { state: ~Counter { count: -1 } };
    let res = strain.clone().feed(&Increment);
    assert!(res.is_err());
    assert_eq!(strain.state().count, -1);
  }
}
