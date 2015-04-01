extern crate strand;

#[cfg(test)]
mod tests {
  use strand::mutable::Event;
  use strand::mutable::Strand;
  use strand::branchable::Branchable;
  use strand::strand::{Mutable};
  use strand::strand;
  use strand::errors::{Errors};

  #[derive(Copy,Clone)]
  struct Value {
    x: i32
  }

  #[derive(Copy,Clone)]
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

  impl Event for Increment {
    type T = Value;

    fn precondition(&self, count: &Value) -> Result<(), Errors> {
      if count.x < 0 {
        Err(Errors::PreConditionNotMet("I cannot count to negatives".to_string()))
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
        Err(Errors::PostConditionNotMet("I shouldn't have counted to negatives".to_string()))
      } else {
        Ok(())
      }
    }
  }

  impl Event for Decrement {
    type T = Value;

    fn precondition(&self, count: &Value) -> Result<(), Errors> {
      if count.x < 1 {
        Err(Errors::PreConditionNotMet("I cannot count to negatives".to_string()))
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
        Err(Errors::PostConditionNotMet("I shouldn't have counted to negatives".to_string()))
      } else {
        Ok(())
      }
    }

  }

  #[test]
  fn test_state_changes() {
    let mut strand: Counter = Counter { count: Value { x: 0 } };
    let res = strand.evolve(&Increment).and_then(|_| {
      strand.evolve(&Increment).and_then(|_| {
        strand.evolve(&Decrement)
      })
    });

    assert!(res.is_ok());
    assert_eq!(strand.count.x, 1);
  }

  #[test]
  fn test_unmet_pre_condition() {
    let mut strand: Counter = Counter { count: Value { x: -1 } };
    let res = strand.evolve(&Increment);
    assert!(res.is_err());
    assert_eq!(strand.count.x, -1);
  }

  #[test]
  fn test_branch() {
    let mut strand: Counter = Counter { count: Value { x: 0 } };
    let res = strand.evolve(&Increment);
    assert!(res.is_ok());
    let mut branch = strand.branch();
    let res1 = strand.evolve(&Increment);
    let res2 = branch.evolve(&Decrement);
    assert!(res1.is_ok());
    assert!(res2.is_ok());
    assert_eq!(strand.count.x, 2);
    assert_eq!(branch.count.x, 0);
  }
}
