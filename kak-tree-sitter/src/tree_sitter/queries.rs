//! Supported queries.

use std::fs;

use super::discovery::DiscoveredLang;

#[derive(Debug)]
pub struct Queries {
  pub highlights: Option<String>,
  pub injections: Option<String>,
  pub locals: Option<String>,
  pub text_objects: Option<String>,
}

impl Queries {
  pub fn load_from_discovered(disc: &DiscoveredLang) -> Self {
    Queries {
      highlights: fs::read_to_string(&disc.highlights).ok(),
      injections: disc
        .injections
        .as_ref()
        .and_then(|p| fs::read_to_string(p).ok()),
      locals: disc
        .locals
        .as_ref()
        .and_then(|p| fs::read_to_string(p).ok()),
      text_objects: disc
        .textobjects
        .as_ref()
        .and_then(|p| fs::read_to_string(p).ok()),
    }
  }
}
