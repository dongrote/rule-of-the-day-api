use chrono::NaiveDate;
use crate::types::RuleForLife;

pub trait RuleOfTheDay {
  fn get_rule_of_the_day(&self, date: NaiveDate) -> Result<RuleForLife, String>;
}
