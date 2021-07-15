use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
  Apply,
  Save,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Filter {
  All,
  Match(Criteria),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Criteria {
  pub names: HashSet<String>,
  pub groups: HashSet<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Options {
  pub action: Option<Action>,
  pub filter: Option<Filter>,
}
