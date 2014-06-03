#[cfg(test)]
mod tests {
  use strain::mutable::Event;
  use strain::mutable::Strain;
  use strain::branchable::Branchable;
  use strain::state::{State};
  use strain::strain::{Mutable};
  use strain::strain;
  use strain::errors::{Errors, PreConditionNotMet, PostConditionNotMet};

  #[deriving(Clone)]
  struct Value {
    x: int
  }
  impl State for Value {}

  struct Counter {
    count: Value
  }

  impl Branchable for Counter {
    fn branch(self) -> Counter {
      Counter { count: self.count.clone() }
    }
  }

  impl strain::Strain<Value> for Counter {
    fn new(state: Value) -> Counter {
      Counter { count: state }
    }
  }

  impl Mutable<Value> for Counter {
    fn state<'a>(&'a mut self) -> &'a mut Value {
      &'a mut self.count
    }
  }

  struct Increment;
  struct Decrement;

  impl Event<Value> for Increment {
    fn precondition(&self, count: &Value) -> Result<(), Errors> {
      if count.x < 0 {
        Err(PreConditionNotMet("I cannot count to negatives".to_string()))
      } else {
        Ok(())
      }
    }

    fn action(&self, count: &mut Value) -> Result<(), Errors>  {
      count.x = count.x + 1;
      Ok(())
    }

    fn postcondition(&self, count: &Value) -> Result<(), Errors> {
      if count.x < 0 {
        Err(PostConditionNotMet("I shouldn't have counted to negatives".to_string()))
      } else {
        Ok(())
      }
    }
  }

  impl Event<Value> for Decrement {
    fn precondition(&self, count: &Value) -> Result<(), Errors> {
      if count.x < 1 {
        Err(PreConditionNotMet("I cannot count to negatives".to_string()))
      } else {
        Ok(())
      }
    }

    fn action(&self, count: &mut Value) -> Result<(), Errors>  {
      count.x = count.x - 1;
      Ok(())
    }

    fn postcondition(&self, count: &Value) -> Result<(), Errors> {
      if count.x < 0 {
        Err(PostConditionNotMet("I shouldn't have counted to negatives".to_string()))
      } else {
        Ok(())
      }
    }

  }

  #[test]
  fn test_state_changes() {
    let mut strain: Counter = Counter { count: Value { x: 0 } };
    let res = strain.evolve(&Increment).and_then(|_| {
      strain.evolve(&Increment).and_then(|_| {
        strain.evolve(&Decrement)
      })
    });

    assert!(res.is_ok());
    assert_eq!(strain.count.x, 1);
  }

  #[test]
  fn test_unmet_pre_condition() {
    let mut strain: Counter = Counter { count: Value { x: -1 } };
    let res = strain.evolve(&Increment);
    assert!(res.is_err());
    assert_eq!(strain.count.x, -1);
  }

  #[test]
  fn test_branch() {
    let mut strain: Counter = Counter { count: Value { x: 0 } };
    let res = strain.evolve(&Increment);
    assert!(res.is_ok());
    let mut branch = strain.branch();
    let res1 = strain.evolve(&Increment);
    let res2 = branch.evolve(&Decrement);
    assert!(res1.is_ok());
    assert!(res2.is_ok());
    assert_eq!(strain.count.x, 2);
    assert_eq!(branch.count.x, 0);
  }
}
