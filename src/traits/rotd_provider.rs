use crate::types::RuleForLife;

pub trait RuleOfTheDayProvider {
  fn rule_of_the_day(&self) -> RuleForLife;
}
