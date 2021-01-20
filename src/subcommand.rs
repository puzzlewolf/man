/// Subcommands
#[derive(Debug, Clone, Default)]
pub struct Subcommand {
  pub(crate) name: Option<String>,
  pub(crate) help: Option<String>,
}

impl Subcommand {
  /// Create a new instance.
  pub fn new() -> Self {
    Self::default()
  }

  /// Set the name value.
  pub fn name(mut self, name: &str) -> Self {
    self.name = Some(name.into());
    self
  }

  /// Set the help value.
  pub fn help(mut self, help: &str) -> Self {
    self.help = Some(help.into());
    self
  }
}
