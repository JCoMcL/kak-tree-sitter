//! Configuration for both the daemon and client.

pub mod error;
pub mod source;

use std::{
  collections::{HashMap, HashSet},
  fs,
  path::{Path, PathBuf},
};

use error::ConfigError;
use serde::{Deserialize, Serialize};
use source::{Source, UserSource};

/// Configuration object used in the server and controller.
///
/// User configuration being opt-in for every option, a different type is used, [`UserConfig`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
  pub features: FeaturesConfig,

  pub highlight: HighlightConfig,

  #[serde(flatten)]
  pub grammars: GrammarsConfig,

  #[serde(flatten)]
  pub languages: LanguagesConfig,
}

impl Config {
  /// Default configuration using the `default-config.toml` file.
  pub const DEFAULT_CONFIG_CONTENT: &'static str = include_str!("../default-config.toml");

  /// Default configuration.
  pub fn default_config() -> Result<Self, ConfigError> {
    log::debug!("loading default configuration");

    toml::from_str(Self::DEFAULT_CONFIG_CONTENT).map_err(|err| ConfigError::CannotParseConfig {
      err: err.to_string(),
    })
  }

  /// Load a user configuration at a given path, and merge with the default configuration.
  pub fn load_user(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
    let mut config = Self::default_config()?;
    match UserConfig::load(path) {
      Ok(user_config) => {
        config.merge_user_config(user_config)?;
      }

      // no user config is fine; simply use the default config
      Err(ConfigError::NoUserConfig) => return Ok(config),

      Err(err) => {
        log::warn!("cannot load user config: {err}");
      }
    }

    Ok(config)
  }

  /// Load the default configuration, the user configuration, and merge both.
  pub fn load_from_xdg() -> Result<Self, ConfigError> {
    let dir = dirs::config_dir().ok_or(ConfigError::NoConfigDir)?;
    let path = dir.join("kak-tree-sitter/config.toml");
    Self::load_user(path)
  }

  /// Merge the config with a user-provided one.
  pub fn merge_user_config(&mut self, user_config: UserConfig) -> Result<(), ConfigError> {
    if let Some(features) = user_config.features {
      self.features.merge_user_config(features);
    }

    if let Some(user_highlight) = user_config.highlight {
      self.highlight.merge_user_config(user_highlight);
    }

    if let Some(user_config) = user_config.grammar {
      self.grammars.merge_user_config(user_config)?;
    }

    if let Some(language) = user_config.language {
      self.languages.merge_user_config(language)?;
    }

    Ok(())
  }
}

/// Feature configuration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeaturesConfig {
  pub highlighting: bool,
  pub text_objects: bool,
}

impl FeaturesConfig {
  fn merge_user_config(&mut self, user_config: UserFeaturesConfig) {
    self.highlighting = user_config.highlighting.unwrap_or(self.highlighting);
    self.text_objects = user_config.text_objects.unwrap_or(self.text_objects);
  }
}

/// Highlight configuration.
///
/// This is a set of capture groups that can be found in various queries.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HighlightConfig {
  pub groups: HashSet<String>,
}

impl HighlightConfig {
  fn merge_user_config(&mut self, user_config: UserHighlightConfig) {
    self.groups.extend(user_config.groups);
  }
}

/// Grammars configuration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GrammarsConfig {
  pub grammar: HashMap<String, GrammarConfig>,
}

impl GrammarsConfig {
  fn merge_user_config(
    &mut self,
    user_config: HashMap<String, UserGrammarConfig>,
  ) -> Result<(), ConfigError> {
    for (lang, user_config) in user_config {
      if let Some(config) = self.grammar.get_mut(&lang) {
        // if we already have a config, everything is optional so we can merge
        config.merge_user_config(user_config)?;
      } else {
        // if we do not have a config, we take it from the user configuration, which can fail
        self
          .grammar
          .insert(lang, GrammarConfig::try_from(user_config)?);
      }
    }
    Ok(())
  }

  /// Get the grammar configuration for `lang`.
  pub fn get_grammar_config(&self, name: impl AsRef<str>) -> Result<&GrammarConfig, ConfigError> {
    let name = name.as_ref();
    self
      .grammar
      .get(name)
      .ok_or_else(|| ConfigError::MissingGrammar {
        name: name.to_owned(),
      })
  }

  /// Get the directory where all grammars live in.
  pub fn get_grammars_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|dir| dir.join("kak-tree-sitter/grammars"))
  }

  /// Get the grammar path for a given language.
  pub fn get_grammar_path(config: &GrammarConfig, name: impl AsRef<str>) -> Option<PathBuf> {
    match config.source {
      Source::Local { ref path } => Some(path.clone()),

      Source::Git { ref pin, .. } => {
        let lang = name.as_ref();
        dirs::data_dir().map(|dir| dir.join(format!("kak-tree-sitter/grammars/{lang}/{pin}.so")))
      }
    }
  }
}

/// Languages configuration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LanguagesConfig {
  pub language: HashMap<String, LanguageConfig>,
}

impl LanguagesConfig {
  fn merge_user_config(
    &mut self,
    user_config: HashMap<String, UserLanguageConfig>,
  ) -> Result<(), ConfigError> {
    for (lang, user_config) in user_config {
      if let Some(config) = self.language.get_mut(&lang) {
        // if we already have a config, everything is optional so we can merge
        config.merge_user_config(user_config)?;
      } else {
        // if we do not have a config, we take it from the user configuration, which can fail
        self
          .language
          .insert(lang, LanguageConfig::try_from(user_config)?);
      }
    }

    Ok(())
  }

  /// Get the configuration for `lang`.
  pub fn get_lang_config(&self, name: impl AsRef<str>) -> Result<&LanguageConfig, ConfigError> {
    let name = name.as_ref();
    self
      .language
      .get(name)
      .ok_or_else(|| ConfigError::MissingLang {
        name: name.to_owned(),
      })
  }

  /// Get the queries directory for a given language.
  pub fn get_queries_dir(lang_config: &LanguageConfig, lang: impl AsRef<str>) -> Option<PathBuf> {
    match lang_config.queries.source {
      Some(Source::Local { ref path }) => Some(path.clone()),

      Some(Source::Git { ref pin, .. }) => {
        let lang = lang.as_ref();
        dirs::data_dir().map(|dir| dir.join(format!("kak-tree-sitter/queries/{lang}/{pin}")))
      }

      None => {
        // FIXME: not sure this is wanted?
        None
      }
    }
  }

  /// Get an iterator on languages and associated configuration.
  pub fn iter(&self) -> impl Iterator<Item = (&str, &LanguageConfig)> {
    self
      .language
      .iter()
      .map(|(name, config)| (name.as_str(), config))
  }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RemoveDefaultHighlighter(pub bool);

impl Default for RemoveDefaultHighlighter {
  fn default() -> Self {
    Self(true)
  }
}

impl From<bool> for RemoveDefaultHighlighter {
  fn from(value: bool) -> Self {
    Self(value)
  }
}

impl From<RemoveDefaultHighlighter> for bool {
  fn from(RemoveDefaultHighlighter(value): RemoveDefaultHighlighter) -> Self {
    value
  }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct FileTypeHook(pub bool);

impl Default for FileTypeHook {
  fn default() -> Self {
    Self(true)
  }
}

impl From<bool> for FileTypeHook {
  fn from(value: bool) -> Self {
    Self(value)
  }
}

impl From<FileTypeHook> for bool {
  fn from(FileTypeHook(value): FileTypeHook) -> Self {
    value
  }
}

/// Specific language configuration.
///
/// It is possible to configure the grammar and queries part of a language, as well as some specific Kakoune options.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LanguageConfig {
  // optional grammar name; will use the name of the language by default
  pub grammar: Option<String>,
  pub queries: QueriesConfig,

  #[serde(default)]
  pub remove_default_highlighter: RemoveDefaultHighlighter,

  #[serde(default)]
  pub filetype_hook: FileTypeHook,

  // language aliases that use the same language config
  #[serde(default)]
  pub aliases: HashSet<String>,
}

impl LanguageConfig {
  fn merge_user_config(&mut self, user_config: UserLanguageConfig) -> Result<(), ConfigError> {
    self.grammar = user_config.grammar.or_else(|| self.grammar.clone());

    if let Some(user_queries) = user_config.queries {
      self.queries.merge_user_config(user_queries)?;
    }

    self.remove_default_highlighter = user_config
      .remove_default_highlighter
      .unwrap_or(self.remove_default_highlighter.0)
      .into();
    self.filetype_hook = user_config
      .filetype_hook
      .unwrap_or(self.filetype_hook.0)
      .into();

    self.aliases = user_config.aliases.unwrap_or_default();

    Ok(())
  }
}

impl TryFrom<UserLanguageConfig> for LanguageConfig {
  type Error = ConfigError;

  fn try_from(user_config: UserLanguageConfig) -> Result<Self, Self::Error> {
    let Some(queries) = user_config.queries else {
      return Err(ConfigError::missing_opt("queries"));
    };

    Ok(Self {
      grammar: user_config.grammar,
      queries: QueriesConfig::try_from(queries)?,
      remove_default_highlighter: user_config
        .remove_default_highlighter
        .unwrap_or(true)
        .into(),
      filetype_hook: user_config.filetype_hook.unwrap_or(true).into(),
      aliases: user_config.aliases.unwrap_or_default(),
    })
  }
}

/// Grammar configuration.
///
/// Most of the options are used by the controller only.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GrammarConfig {
  /// Source from where to get the grammar.
  pub source: Source,

  /// Path to find the grammar source inside the downloaded content.
  #[serde(default = "GrammarConfig::default_path")]
  pub path: PathBuf,

  /// Compile command to run to compile the grammar.
  ///
  /// Should always be `cc` but who knows.
  #[serde(default = "GrammarConfig::default_compile")]
  pub compile: String,

  /// Compiler arguments.
  #[serde(default = "GrammarConfig::default_compile_args")]
  pub compile_args: Vec<String>,

  /// Compiler extra arguments.
  ///
  /// Should be used to pass optimization and debug flags, mainly.
  #[serde(default = "GrammarConfig::default_compile_flags")]
  pub compile_flags: Vec<String>,

  /// Link command to run to link the grammar.
  ///
  /// Should always be `cc`, but, still, who knows.
  #[serde(default = "GrammarConfig::default_compile")]
  pub link: String,

  /// Linker arguments.
  #[serde(default = "GrammarConfig::default_link_args")]
  pub link_args: Vec<String>,

  /// Linker extra arguments.
  ///
  /// Should be used to pass optimization and debug flags, mainly.
  #[serde(default = "GrammarConfig::default_compile_flags")]
  pub link_flags: Vec<String>,
}

impl GrammarConfig {
  fn default_path() -> PathBuf {
    "src".into()
  }

  fn default_compile() -> String {
    "cc".to_owned()
  }

  fn default_compile_args() -> Vec<String> {
    ["-c", "-fpic", "../parser.c", "-I", ".."]
      .into_iter()
      .map(String::from)
      .collect()
  }

  fn default_compile_flags() -> Vec<String> {
    ["-O3"].into_iter().map(String::from).collect()
  }

  fn default_link_args() -> Vec<String> {
    ["-shared", "-fpic", "parser.o", "-o", "{lang}.so"]
      .into_iter()
      .map(String::from)
      .collect()
  }

  fn merge_user_config(&mut self, user_config: UserGrammarConfig) -> Result<(), ConfigError> {
    if let Some(source) = user_config.source {
      self.source.merge_user_config(source);
    }

    if let Some(path) = user_config.path {
      self.path = path;
    }

    if let Some(compile) = user_config.compile {
      self.compile = compile;
    }

    if let Some(compile_args) = user_config.compile_args {
      self.compile_args = compile_args;
    }

    if let Some(compile_flags) = user_config.compile_flags {
      self.compile_flags = compile_flags;
    }

    if let Some(link) = user_config.link {
      self.link = link;
    }

    if let Some(link_args) = user_config.link_args {
      self.link_args = link_args;
    }

    if let Some(link_flags) = user_config.link_flags {
      self.link_flags = link_flags;
    }

    Ok(())
  }
}

impl TryFrom<UserGrammarConfig> for GrammarConfig {
  type Error = ConfigError;

  fn try_from(user_config: UserGrammarConfig) -> Result<Self, Self::Error> {
    let Some(source) = user_config.source else {
      return Err(ConfigError::missing_opt("source"));
    };

    Ok(Self {
      source: Source::try_from(source)?,
      path: user_config.path.unwrap_or_else(GrammarConfig::default_path),
      compile: user_config
        .compile
        .unwrap_or_else(GrammarConfig::default_compile),
      compile_args: user_config
        .compile_args
        .unwrap_or_else(GrammarConfig::default_compile_args),
      compile_flags: user_config
        .compile_flags
        .unwrap_or_else(GrammarConfig::default_compile_flags),
      link: user_config
        .link
        .unwrap_or_else(GrammarConfig::default_compile),
      link_args: user_config
        .link_args
        .unwrap_or_else(GrammarConfig::default_link_args),
      link_flags: user_config
        .link_flags
        .unwrap_or_else(GrammarConfig::default_compile_flags),
    })
  }
}

/// Queries configuration.
///
/// Most of the options are used by the controller only.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QueriesConfig {
  /// Source from where to get the queries.
  pub source: Option<Source>,

  /// Path to go to where to find the queries directory.
  #[serde(default = "QueriesConfig::default_path")]
  pub path: PathBuf,
}

impl QueriesConfig {
  fn default_path() -> PathBuf {
    "runtime/queries/{lang}".into()
  }

  pub fn normalized_path(&self, lang: &str) -> PathBuf {
    self
      .path
      .display()
      .to_string()
      .replace("{lang}", lang)
      .into()
  }

  fn merge_user_config(
    &mut self,
    user_config: UserLanguageQueriesConfig,
  ) -> Result<(), ConfigError> {
    match (&mut self.source, user_config.source) {
      (_, None) => (),

      (None, Some(source)) => self.source = Some(Source::try_from(source)?),

      (Some(source), Some(user_source)) => {
        source.merge_user_config(user_source);
      }
    }

    if let Some(path) = user_config.path {
      self.path = path;
    }

    Ok(())
  }
}

impl TryFrom<UserLanguageQueriesConfig> for QueriesConfig {
  type Error = ConfigError;

  fn try_from(user_config: UserLanguageQueriesConfig) -> Result<Self, Self::Error> {
    let source = if let Some(source) = user_config.source {
      Some(Source::try_from(source)?)
    } else {
      None
    };

    Ok(Self {
      source,
      path: user_config.path.unwrap_or_else(QueriesConfig::default_path),
    })
  }
}

/// User version of configuration.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserConfig {
  pub features: Option<UserFeaturesConfig>,
  pub highlight: Option<UserHighlightConfig>,
  pub grammar: Option<HashMap<String, UserGrammarConfig>>,
  pub language: Option<HashMap<String, UserLanguageConfig>>,
}

impl UserConfig {
  /// Load the config from the default user location (XDG).
  pub fn load_from_xdg() -> Result<Self, ConfigError> {
    log::debug!("loading user configuration");

    let dir = dirs::config_dir().ok_or(ConfigError::NoConfigDir)?;
    let path = dir.join("kak-tree-sitter/config.toml");
    Self::load(path)
  }

  /// Load the configuration from a given path.
  fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
    let path = path.as_ref();

    if !matches!(path.try_exists(), Ok(true)) {
      log::debug!("no config file at {path}", path = path.display());
      return Err(ConfigError::NoUserConfig);
    }

    log::debug!("loading configuration from {path}", path = path.display());

    let content = fs::read_to_string(path).map_err(|err| ConfigError::CannotReadConfig {
      path: path.to_owned(),
      err,
    })?;

    toml::from_str(&content).map_err(|err| ConfigError::CannotParseConfig {
      err: err.to_string(),
    })
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserFeaturesConfig {
  pub highlighting: Option<bool>,
  pub text_objects: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserHighlightConfig {
  pub groups: HashSet<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLanguageConfig {
  pub grammar: Option<String>,
  pub queries: Option<UserLanguageQueriesConfig>,
  pub remove_default_highlighter: Option<bool>,
  pub filetype_hook: Option<bool>,
  pub aliases: Option<HashSet<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserGrammarConfig {
  pub source: Option<UserSource>,
  pub path: Option<PathBuf>,
  pub compile: Option<String>,
  pub compile_args: Option<Vec<String>>,
  pub compile_flags: Option<Vec<String>>,
  pub link: Option<String>,
  pub link_args: Option<Vec<String>>,
  pub link_flags: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLanguageQueriesConfig {
  pub source: Option<UserSource>,
  pub path: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
  use std::{collections::HashSet, path::PathBuf};

  use crate::{
    Config, ConfigError, FeaturesConfig, GrammarConfig, GrammarsConfig, HighlightConfig,
    LanguageConfig, LanguagesConfig, QueriesConfig, UserConfig, UserGrammarConfig,
    source::{Source, UserSource},
  };

  #[test]
  fn user_merge() {
    // we have a config and see that we can alter it by merging with a user config
    let main_config = Config {
      features: FeaturesConfig {
        highlighting: true,
        text_objects: true,
      },
      highlight: HighlightConfig {
        groups: ["foo".to_owned(), "bar".to_owned(), "zoo".to_owned()]
          .into_iter()
          .collect(),
      },
      grammars: GrammarsConfig {
        grammar: [(
          "rust".to_owned(),
          GrammarConfig {
            source: Source::local("file://hello"),
            path: PathBuf::from("src"),
            compile: "".to_owned(),
            compile_args: Vec::new(),
            compile_flags: Vec::new(),
            link: "".to_owned(),
            link_args: Vec::new(),
            link_flags: Vec::new(),
          },
        )]
        .into(),
      },
      languages: LanguagesConfig {
        language: [(
          "rust".to_owned(),
          LanguageConfig {
            grammar: None,
            queries: QueriesConfig {
              source: None,
              path: PathBuf::from("runtime/queries/rust"),
            },
            remove_default_highlighter: true.into(),
            filetype_hook: true.into(),
            aliases: HashSet::new(),
          },
        )]
        .into(),
      },
    };

    // merging a default user config to a config shouldn’t change anything
    {
      let mut config = main_config.clone();
      let user_config = UserConfig::default();
      assert!(config.merge_user_config(user_config).is_ok());
      assert_eq!(main_config, config);
    }

    // deeply changing some config for Rust
    {
      let mut config = main_config.clone();
      let user_config = UserConfig {
        grammar: Some(
          [(
            "rust".to_owned(),
            UserGrammarConfig {
              source: Some(UserSource::git("git_source".to_owned(), "pin".to_owned())),
              link_args: Some(vec!["link".to_owned(), "args".to_owned()]),
              ..Default::default()
            },
          )]
          .into(),
        ),
        ..Default::default()
      };

      assert!(config.merge_user_config(user_config).is_ok());

      let prev_rust_config = main_config.languages.get_lang_config("rust").unwrap();
      let new_rust_config = config.languages.get_lang_config("rust").unwrap();

      assert_eq!(prev_rust_config.queries, new_rust_config.queries);

      let new_rust_grammar_config = config.grammars.get_grammar_config("rust").unwrap();

      assert_eq!(
        new_rust_grammar_config.source,
        Source::git("git_source", "pin".to_owned())
      );
      assert_eq!(
        new_rust_grammar_config.link_args,
        vec!["link".to_owned(), "args".to_owned()]
      );
    }
  }

  #[test]
  fn user_config() -> Result<(), ConfigError> {
    let toml = r#"[grammar.rust.source.git]
      pin = "foo""#;
    let config = toml::from_str::<UserConfig>(toml).unwrap();
    let source = config
      .grammar
      .as_ref()
      .unwrap()
      .get("rust")
      .unwrap()
      .source
      .as_ref()
      .unwrap();

    assert!(matches!(source, UserSource::Git { pin, .. } if pin.as_deref() == Some("foo")));

    Ok(())
  }
}
