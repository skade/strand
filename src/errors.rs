
#[deriving(Show)]
pub enum Errors {
  PreConditionNotMet(~str),
  PostConditionNotMet(~str),
  ActionFailed(~str)
}