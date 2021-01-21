/// Add a examples section
#[derive(Debug, Clone, Default)]
pub struct Example {
  pub(crate) prompt: String,
  pub(crate) text: Option<String>,
  pub(crate) command: Option<String>,
  pub(crate) output: Option<String>,
}

impl Example {
  pub fn new() -> Self {
    Self {
      prompt: "$".into(),
      text: None,
      command: None,
      output: None,
    }
  }

  pub fn prompt(mut self, prompt: &str) -> Self {
    self.prompt = prompt.into();
    self
  }

  pub fn text(mut self, text: &str) -> Self {
    self.text = Some(text.into());
    self
  }

  pub fn command(mut self, command: &str) -> Self {
    self.command = Some(command.into());
    self
  }

  pub fn output(mut self, output: &str) -> Self {
    self.output = Some(output.into());
    self
  }
}
