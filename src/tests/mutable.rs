#[cfg(test)]
mod tests {
  use strand::mutable::Event;
  use strand::mutable::Strand;
  use strand::branchable::Branchable;
  use strand::strand::{Mutable};
  use strand::strand;
  use strand::errors::{Errors, PreConditionNotMet, PostConditionNotMet};

  #[deriving(Clone)]
  struct Value {
    x: int
  }

  struct Counter {
    count: Value
  }

  impl Branchable for Counter {
    fn branch(self) -> Counter {
      Counter { count: self.count.clone() }
    }
  }

  impl strand::Strand<Value> for Counter {
    fn new(state: Value) -> Counter {
      Counter { count: state }
    }
  }

  impl Mutable<Value> for Counter {
    fn state<'a>(&'a mut self) -> &'a mut Value {
      &mut self.count
    }
  }

  struct Increment;
  struct Decrement;

  impl Event<Value,int> for Increment {
    fn precondition(&self, count: &Value) -> Result<(), Errors> {
      if count.x < 0 {
        Err(PreConditionNotMet("I cannot count to negatives".to_string()))
      } else {
        Ok(())
      }
    }

    fn action(&self, count: &mut Value) -> Result<int, Errors>  {
      count.x = count.x + 1;
      Ok(count.x)
    }

    fn postcondition(&self, count: &Value) -> Result<(), Errors> {
      if count.x < 0 {
        Err(PostConditionNotMet("I shouldn't have counted to negatives".to_string()))
      } else {
        Ok(())
      }
    }
  }

  impl Event<Value, int> for Decrement {
    fn precondition(&self, count: &Value) -> Result<(), Errors> {
      if count.x < 1 {
        Err(PreConditionNotMet("I cannot count to negatives".to_string()))
      } else {
        Ok(())
      }
    }

    fn action(&self, count: &mut Value) -> Result<int, Errors>  {
      count.x = count.x - 1;
      Ok(count.x)
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
