use serde::Serialize;

#[derive(Serialize, Default)]
pub struct RuleForLife {
    pub text: String,
    pub number: u32,
    pub quotes: Vec<String>,
}

impl Clone for RuleForLife {
  fn clone(&self) -> RuleForLife {
      RuleForLife {
          text: self.text.clone(),
          number: self.number,
          quotes: self.quotes.clone(),
      }
  }
}
