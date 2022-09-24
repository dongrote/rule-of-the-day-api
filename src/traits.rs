use chrono::{DateTime, Local};
use crate::types::RuleForLife;

pub trait RuleOfTheDay {
  fn get_rule_of_the_day(&self, date: DateTime<Local>) -> Result<RuleForLife, String>;
}
