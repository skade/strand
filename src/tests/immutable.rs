#[cfg(test)]
mod tests {
  use strand::immutable::Event;
  use strand::immutable::Strand;
  use strand::branchable::Branchable;
  use strand::errors::{Errors, PreConditionNotMet, PostConditionNotMet};

  #[deriving(Clone)]
  struct Counter {
    count: int,
  }

  struct Increment;
  struct Decrement;

  impl Event<int> for Increment {
    fn precondition(&self, state: int) -> Result<(), Errors> {
      if state < 0 {
        Err(PreConditionNotMet("I cannot count to negatives".to_string()))
      } else {
        Ok(())
      }
    }

    fn action(&self, state: int) -> Result<int, Errors>  {
      Ok(state + 1)
    }

    fn postcondition(&self, state: int) -> Result<(), Errors> {
      if state < 0 {
        Err(PostConditionNotMet("I shouldn't have counted to negatives".to_string()))
      } else {
        Ok(())
      }
    }
  }

  impl Event<int> for Decrement {
    fn precondition(&self, state: int) -> Result<(), Errors> {
      if state < 1 {
        Err(PreConditionNotMet("I cannot count to negatives".to_string()))
      } else {
        Ok(())
      }
    }

    fn action(&self, state: int) -> Result<int, Errors>  {
      Ok(state - 1)
    }

    fn postcondition(&self, state: int) -> Result<(), Errors> {
      if state < 0 {
        Err(PostConditionNotMet("I shouldn't have counted to negatives".to_string()))
      } else {
        Ok(())
      }
    }

  }

  #[test]
  fn test_state_changes() {
    let strand = 0i;
    let res = strand.evolve(&Increment).and_then(|state| {
      state.evolve(&Increment).and_then(|state2| {
        state2.evolve(&Decrement)
      })
    });
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 1);
  }

  #[test]
  fn test_unmet_pre_condition() {
    let strand = -1i;
    let res = strand.evolve(&Increment);
    assert!(res.is_err());
    assert_eq!(strand, -1i);
  }

  #[test]
  fn test_branch() {
    let strand = 0i;
    let res = strand.evolve(&Increment);
    assert!(res.is_ok());
    let branch_point = res.unwrap();
    let branch = branch_point.branch();
    let end_state_1 = branch_point.evolve(&Increment);
    let end_state_2 = branch.evolve(&Decrement);
    assert!(end_state_1.is_ok());
    assert!(end_state_2.is_ok());
    assert_eq!(end_state_1.unwrap(), 2i);
    assert_eq!(end_state_2.unwrap(), 0i);
  }
}
