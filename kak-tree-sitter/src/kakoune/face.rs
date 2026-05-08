//! Face definition.

use std::fmt::Display;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Face {
  name: String,
}

impl Display for Face {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.name.fmt(f)
  }
}

impl Face {
  /// Create a [`Face`] from a capture group; e.g. constant.character.escape.
  pub fn from_capture_group(name: impl AsRef<str>) -> Self {
    let name = name.as_ref().replace('.', "_");
    Self { name }
  }
}
