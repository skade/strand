#![allow(unstable)]
extern crate strand;

#[cfg(test)]
mod tests {
  use strand::immutable::Event;
  use strand::immutable::Strand;
  use strand::branchable::Branchable;
  use strand::errors::{Errors};
  use strand;

  #[derive(Copy,Clone)]
  struct Counter {
    state: i32
  }

  impl strand::strand::Strand<Counter> for Counter {
    fn new(state: Counter) -> Counter {
      state
    }
  }

  impl strand::strand::Immutable<Counter> for Counter {
    fn state(&self) -> Counter {
      self.clone()
    }
  }

  impl Branchable for Counter {
    fn branch(self) -> Self {
      self.clone()
    }
  }

  struct Increment;
  struct Decrement;

  impl Event<Counter> for Increment {
    fn precondition(&self, state: Counter) -> Result<(), Errors> {
      if state.state < 0 {
        Err(Errors::PreConditionNotMet("I cannot count to negatives".to_string()))
      } else {
        Ok(())
      }
    }

    fn action(&self, state: Counter) -> Result<Counter, Errors>  {
      Ok(Counter { state: state.state + 1 })
    }

    fn postcondition(&self, state: Counter) -> Result<(), Errors> {
      if state.state < 0 {
        Err(Errors::PostConditionNotMet("I shouldn't have counted to negatives".to_string()))
      } else {
        Ok(())
      }
    }
  }

  impl Event<Counter> for Decrement {
    fn precondition(&self, state: Counter) -> Result<(), Errors> {
      if state.state < 1 {
        Err(Errors::PreConditionNotMet("I cannot count to negatives".to_string()))
      } else {
        Ok(())
      }
    }

    fn action(&self, state: Counter) -> Result<Counter, Errors>  {
      Ok(Counter { state: state.state - 1 })
    }

    fn postcondition(&self, state: Counter) -> Result<(), Errors> {
      if state.state < 0 {
        Err(Errors::PostConditionNotMet("I shouldn't have counted to negatives".to_string()))
      } else {
        Ok(())
      }
    }

  }

  #[test]
  fn test_state_changes() {
    let strand = Counter { state: 0 };
    let res = strand.evolve(&Increment).and_then(|state| {
      state.evolve(&Increment).and_then(|state2| {
        state2.evolve(&Decrement)
      })
    });
    assert!(res.is_ok());
    assert_eq!(res.unwrap().state, 1);
  }

  #[test]
  fn test_unmet_pre_condition() {
    let strand = Counter { state: -1 };
    let res = strand.evolve(&Increment);
    assert!(res.is_err());
    assert_eq!(strand.state, -1i32);
  }

  #[test]
  fn test_branch() {
    let strand = Counter { state: 0 };
    let res = strand.evolve(&Increment);
    assert!(res.is_ok());
    let branch_point = res.unwrap();
    let branch = branch_point.branch();
    let end_state_1 = branch_point.evolve(&Increment);
    let end_state_2 = branch.evolve(&Decrement);
    assert!(end_state_1.is_ok());
    assert!(end_state_2.is_ok());
    assert_eq!(end_state_1.unwrap().state, 2);
    assert_eq!(end_state_2.unwrap().state, 0);
  }
}
