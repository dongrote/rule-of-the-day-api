use crate::traits::credential_provider::CredentialProvider;
use crate::traits::rotd_provider::RuleOfTheDayProvider;
use crate::types::RuleForLife;

pub struct AzureTablesRuleOfTheDayProvider<'a> {
  credentials: &'a dyn CredentialProvider,
}

impl<'a> AzureTablesRuleOfTheDayProvider<'a> {
  pub fn create_with_credential_provider(credential_provider: &'a impl CredentialProvider) -> Self {
    AzureTablesRuleOfTheDayProvider {
      credentials: credential_provider,
    }
  }

  fn create_azure_credential(&self) {
    self.credentials.account();
    self.credentials.secret();
  }
}

impl<'a> RuleOfTheDayProvider for AzureTablesRuleOfTheDayProvider<'a> {
  fn rule_of_the_day(&self) -> RuleForLife {
      self.create_azure_credential();
      RuleForLife::default()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_with_credential_provider() {
    let mock_credential_provider = create_mock_credential_provider();
    let provider = AzureTablesRuleOfTheDayProvider::create_with_credential_provider(&mock_credential_provider);
    provider.rule_of_the_day();
  }

  struct MockCredentialProvider {}

  impl CredentialProvider for MockCredentialProvider {
    fn account(&self) -> &str {
        "account"
    }

    fn secret(&self) -> &str {
        "secret"
    }
  }

  fn create_mock_credential_provider() -> MockCredentialProvider {
    MockCredentialProvider {  }
  }
}
