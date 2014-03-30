#[cfg(test)]
mod tests {
  use strain::state::State;
  use strain::mutable::Event;
  use strain::mutable::Strain;
  use strain::strain;
  use strain::errors::{Errors, PreConditionNotMet, PostConditionNotMet};

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
    let res = strain.evolve(&Increment).and_then(|_| {
      strain.evolve(&Increment).and_then(|_| {
        strain.evolve(&Decrement)
      })
    });
    assert!(res.is_ok());
    assert_eq!(strain.state().count, 1);
  }

  #[test]
  fn test_unmet_pre_condition() {
    let mut strain : strain::Strain<Counter> = strain::Strain { state: ~Counter { count: -1 } };
    let res = strain.evolve(&Increment);
    assert!(res.is_err());
    assert_eq!(strain.state().count, -1);
  }
}
