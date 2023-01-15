pub trait CredentialProvider {
  fn account(&self) -> &str;
  fn secret(&self) -> &str;
}
