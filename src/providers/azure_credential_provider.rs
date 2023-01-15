use std::env;
use crate::traits::credential_provider::CredentialProvider;

static AZURE_ACCOUNT_ENV: &str = "AZURE_ACCOUNTNAME";
static AZURE_SECRET_ENV: &str = "AZURE_ACCOUNTSECRET";

#[derive(Debug)]
pub enum AzureCredentialProviderError<'a> {
  EnvironmentVariableNotDefined(&'a str),
}

pub struct AzureCredentialProvider {
  account_name: String,
  auth_secret: String,
}

impl AzureCredentialProvider {
  pub fn empty() -> AzureCredentialProvider {
    AzureCredentialProvider {
      account_name: AzureCredentialProvider::empty_string(),
      auth_secret: AzureCredentialProvider::empty_string(),
    }
  }

  pub fn environment() -> Result<AzureCredentialProvider, AzureCredentialProviderError<'static>> {
    match env::var(AZURE_ACCOUNT_ENV) {
      Ok(account_name) => {
        match env::var(AZURE_SECRET_ENV) {
          Ok(auth_secret) => Ok(AzureCredentialProvider { account_name, auth_secret, }),
          _ => Err(AzureCredentialProviderError::EnvironmentVariableNotDefined(AZURE_SECRET_ENV)),
        }
      }
      _ => Err(AzureCredentialProviderError::EnvironmentVariableNotDefined(AZURE_ACCOUNT_ENV)),
    }
  }

  fn empty_string() -> String { String::from("") }
}

impl CredentialProvider for AzureCredentialProvider {
  fn account(&self) -> &str {
    &self.account_name
  }

  fn secret(&self) -> &str {
    &self.auth_secret
  }
}


#[cfg(test)]
mod tests {
  use std::env::set_var;
  use super::*;

  #[test]
  fn test_can_create_empty_credentials() {
    let provider = AzureCredentialProvider::empty();
    assert_eq!("", provider.account());
    assert_eq!("", provider.secret());
  }

  #[test]
  fn test_can_create_from_environment() {
    set_var(AZURE_ACCOUNT_ENV, "foobar");
    set_var(AZURE_SECRET_ENV, "barfoo");
    match AzureCredentialProvider::environment() {
      Ok(provider) => {
        assert_eq!("foobar", provider.account());
        assert_eq!("barfoo", provider.secret());  
      },
      Err(err) => panic!("{:?}", err),
    }
  }
}
