use chrono::{DateTime, Local, Datelike};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use crate::types::RuleForLife;
use crate::traits::RuleOfTheDay;

pub struct RulesForLifeCollection {
    rules_for_life: Vec<RuleForLife>,
}

impl RulesForLifeCollection {
    pub fn new() -> Self {
        RulesForLifeCollection {
            rules_for_life: vec![
                RuleForLife {
                    number: 1,
                    text: "Stand up straight with your shoulders back.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 2,
                    text: "Treat yourself like you are someone you are responsible for helping.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 3,
                    text: "Make friends with people who want the best for you.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 4,
                    text: "Compare yourself with who you were yesterday, not with who someone else is today.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 5,
                    text: "Do not let your children do anything that makes you dislike them.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 6,
                    text: "Set your house in perfect order before you criticize the world.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 7,
                    text: "Pursue what is meaningful (not what is expedient).".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 8,
                    text: "Tell the truth--or, at least, don't lie.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 9,
                    text: "Assume that the person you are listening to might know something you don't.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 10,
                    text: "Be precise in your speech.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 11,
                    text: "Do not bother children when they are skate-boarding.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 12,
                    text: "Pet a cat when you encounter one on the street.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 13,
                    text: "Do not carelessly denigrate social institutions or creative achievement.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 14,
                    text: "Imagine who you could be and then aim single-mindedly at that.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 15,
                    text: "Do not hide unwanted things in the fog.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 16,
                    text: "Notice that opportunity lurks where responsibility has been abdicated.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 17,
                    text: "Do not do what you hate.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 18,
                    text: "Abandon ideology.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 19,
                    text: "Work as hard as you possibly can on at least one thing and see what happens.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 20,
                    text: "Try to make one room in your home as beautiful as possible.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 21,
                    text: "If old memories still upset you, write them down carefully and completely.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 22,
                    text: "Plan and work diligently to maintain the romance in your relationship.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 23,
                    text: "Do not allow yourself to become resentful, deceitful, or arrogant.".into(),
                    quotes: vec![],
                },
                RuleForLife {
                    number: 24,
                    text: "Be grateful in spite of your suffering.".into(),
                    quotes: vec![],
                },
            ],
        }
    }

    fn rule_index_from_ts(&self, timestamp: DateTime<Local>) -> usize {
        let seed: u64 = timestamp.year().try_into().unwrap();
        let mut prng = ChaCha8Rng::seed_from_u64(seed);
        let mut random_bytes: Vec<u8> = vec![0; 366];
        prng.fill_bytes(&mut random_bytes);
        let day_of_year: usize = match timestamp.ordinal0().try_into() {
            Ok(value) => value,
            _ => 0,
        };
        let random_value: usize = random_bytes[day_of_year].into();
        random_value % self.rules_for_life.len()
    }

}

impl RuleOfTheDay for RulesForLifeCollection {
    fn get_rule_of_the_day(&self, timestamp: DateTime<Local>) -> Result<RuleForLife, String> {
        Ok(self.rules_for_life[self.rule_index_from_ts(timestamp)].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rule_of_the_day_works() {
        let collection = RulesForLifeCollection::new();
        let rule = collection.get_rule_of_the_day(Local::now());
        assert!(rule.is_ok());
        let rule = rule.unwrap();
        println!("{}", rule.text);
    }
}
