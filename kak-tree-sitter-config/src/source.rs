use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::ConfigError;

/// Tree-sitter runtime resources sources.
///
/// Sources can be local or remote. In the case of remote sources, we only support git repositories for now.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
  Local { path: PathBuf },
  Git { url: String },
}

impl Source {
  pub fn local(path: impl Into<PathBuf>) -> Self {
    let path = path.into();
    Self::Local { path }
  }

  pub fn git(url: impl Into<String>) -> Self {
    let url = url.into();
    Self::Git { url }
  }
}

impl Source {
  pub fn merge_user_config(&mut self, user_source: UserSource) {
    match (self, user_source) {
      (self_, UserSource::Local { path }) => *self_ = Source::Local { path },

      (self_ @ Source::Local { .. }, UserSource::Git { url: Some(url) }) => {
        *self_ = Source::Git { url };
      }

      (Source::Git { url: self_url }, UserSource::Git { url }) => {
        if let Some(url) = url {
          *self_url = url;
        }
      }

      _ => (),
    }
  }
}

impl TryFrom<UserSource> for Source {
  type Error = ConfigError;

  fn try_from(source: UserSource) -> Result<Self, Self::Error> {
    match source {
      UserSource::Local { path } => Ok(Self::Local { path }),

      UserSource::Git { url } => {
        let url = url.ok_or_else(|| ConfigError::missing_opt("url"))?;
        Ok(Self::Git { url })
      }
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserSource {
  Local {
    path: PathBuf,
  },

  Git {
    url: Option<String>,
  },
}

impl UserSource {
  pub fn local(path: impl Into<PathBuf>) -> Self {
    let path = path.into();
    Self::Local { path }
  }

  pub fn git(url: impl Into<Option<String>>) -> Self {
    let url = url.into();
    Self::Git { url }
  }
}
