use chrono::{DateTime, Local};
use crate::models::RulesForLifeCollection;
use crate::types::RuleForLife;
use crate::traits::RuleOfTheDay;

pub fn handler(timestamp: DateTime<Local>) -> Result<RuleForLife, warp::http::StatusCode> {
    let collection = RulesForLifeCollection::new();
    match collection.get_rule_of_the_day(timestamp) {
        Ok(rule) => Ok(rule),
        _ => Err(warp::http::StatusCode::IM_A_TEAPOT),
    }
}
