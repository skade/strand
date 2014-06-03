
#[deriving(Show)]
pub enum Errors {
  PreConditionNotMet(String),
  PostConditionNotMet(String),
  ActionFailed(String)
}