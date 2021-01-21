/// Positional arguments.
#[derive(Debug, Clone)]
pub struct Arg {
  pub(crate) name: String,
  pub(crate) description: Option<String>,
  pub(crate) required: bool,
}

impl Arg {
  /// Create a new instance.
  pub fn new(name: &str, required: bool) -> Self {
    Self {
        name: name.into(),
        description: None,
        required,
    }
  }

  /// Set the description value.
  pub fn description(mut self, description: &str) -> Self {
    self.description = Some(description.into());
    self
  }
}
