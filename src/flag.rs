use std::cmp::Ordering;

/// Boolean arguments that can be toggled on or off.
#[derive(Debug, Clone, Default, Eq)]
pub struct Flag {
  pub(crate) short: Option<String>,
  pub(crate) long: Option<String>,
  pub(crate) help: Option<String>,
}

impl Flag {
  /// Create a new instance.
  pub fn new() -> Self {
    Self::default()
  }

  /// Set the short value.
  pub fn short(mut self, short: &str) -> Self {
    self.short = Some(short.into());
    self
  }

  /// Set the long value.
  pub fn long(mut self, long: &str) -> Self {
    self.long = Some(long.into());
    self
  }

  /// Set the help value.
  pub fn help(mut self, help: &str) -> Self {
    self.help = Some(help.into());
    self
  }
}

impl PartialEq for Flag {
  fn eq(&self, other: &Self) -> bool {
    self.short == other.short && self.long == other.long
  }
}

impl PartialOrd for Flag {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Flag {
  fn cmp(&self, other: &Self) -> Ordering {
    let s = self
      .short
      .as_ref()
      .or(self.long.as_ref())
      .map(|s| s.trim_start_matches('-').to_ascii_lowercase());
    let o = other
      .short
      .as_ref()
      .or(other.long.as_ref())
      .map(|s| s.trim_start_matches('-').to_ascii_lowercase());
    s.cmp(&o)
  }
}
