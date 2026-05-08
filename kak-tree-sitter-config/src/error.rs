use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error("no configuration directory known for your system; please adjust XDG_CONFIG_HOME")]
  NoConfigDir,

  #[error("no user configuration")]
  NoUserConfig,

  #[error("cannot read configuration at {path}: {err}")]
  CannotReadConfig { path: PathBuf, err: io::Error },

  #[error("cannot parse configuration: {err}")]
  CannotParseConfig { err: String },
}
