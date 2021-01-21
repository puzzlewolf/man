/// Subcommands
#[derive(Debug, Clone, Default)]
pub struct Subcommand {
  pub(crate) name: String,
  pub(crate) description: Option<String>,
}

impl Subcommand {
  /// Create a new instance.
  pub fn new(name: &str) -> Self {
    Self {
        name: name.into(),
        description: None
    }
  }

  /// Set the description value.
  pub fn description(mut self, description: &str) -> Self {
    self.description = Some(description.into());
    self
  }
}
