//! Configuration for both the daemon and client.

pub mod error;

use std::{
  collections::{HashMap, HashSet},
  fs,
  path::{Path, PathBuf},
};

use error::ConfigError;
use serde::{Deserialize, Serialize};

/// Configuration object used in the server.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
  pub features: FeaturesConfig,
  pub highlight: HighlightConfig,
  pub search_paths: SearchPathsConfig,
  #[serde(flatten)]
  pub languages: LanguagesConfig,
}

impl Config {
  pub const DEFAULT_CONFIG_CONTENT: &'static str = include_str!("../default-config.toml");

  pub fn default_config() -> Result<Self, ConfigError> {
    log::debug!("loading default configuration");
    toml::from_str(Self::DEFAULT_CONFIG_CONTENT).map_err(|err| ConfigError::CannotParseConfig {
      err: err.to_string(),
    })
  }

  pub fn load_user(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
    let mut config = Self::default_config()?;
    match UserConfig::load(path) {
      Ok(user_config) => {
        config.merge_user_config(user_config)?;
      }

      Err(ConfigError::NoUserConfig) => return Ok(config),

      Err(err) => {
        log::warn!("cannot load user config: {err}");
      }
    }
    Ok(config)
  }

  pub fn load_from_xdg() -> Result<Self, ConfigError> {
    let dir = dirs::config_dir().ok_or(ConfigError::NoConfigDir)?;
    let path = dir.join("kak-tree-sitter/config.toml");
    Self::load_user(path)
  }

  pub fn merge_user_config(&mut self, user_config: UserConfig) -> Result<(), ConfigError> {
    if let Some(features) = user_config.features {
      self.features.merge_user_config(features);
    }
    if let Some(user_highlight) = user_config.highlight {
      self.highlight.merge_user_config(user_highlight);
    }
    if let Some(search_paths) = user_config.search_paths {
      self.search_paths.merge_user_config(search_paths);
    }
    if let Some(language) = user_config.language {
      self.languages.merge_user_config(language);
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

/// Highlight capture group configuration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HighlightConfig {
  pub groups: HashSet<String>,
}

impl HighlightConfig {
  fn merge_user_config(&mut self, user_config: UserHighlightConfig) {
    self.groups.extend(user_config.groups);
  }
}

/// Filesystem search paths for grammar and query auto-discovery.
///
/// Patterns use `{lang}` as a placeholder for the language name.
/// Later entries take precedence over earlier ones.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SearchPathsConfig {
  pub grammars: Vec<String>,
  pub queries: Vec<String>,
}

impl SearchPathsConfig {
  fn merge_user_config(&mut self, user: UserSearchPathsConfig) {
    // user paths appended so they override defaults
    self.grammars.extend(user.grammars);
    self.queries.extend(user.queries);
  }
}

/// Per-language configuration overrides.
///
/// Most languages need no entry here; the auto-discovery system handles
/// them automatically. Entries are only needed for aliases, grammar/query
/// name redirects, or explicit file path overrides.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LanguagesConfig {
  #[serde(default)]
  pub language: HashMap<String, LanguageConfig>,
}

impl LanguagesConfig {
  fn merge_user_config(&mut self, user_config: HashMap<String, UserLanguageConfig>) {
    for (lang, user_lang) in user_config {
      if let Some(config) = self.language.get_mut(&lang) {
        config.merge_user_config(user_lang);
      } else {
        self.language.insert(lang, LanguageConfig::from(user_lang));
      }
    }
  }

  pub fn get(&self, name: &str) -> Option<&LanguageConfig> {
    self.language.get(name)
  }
}

/// Optional overrides for a single language.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LanguageConfig {
  /// Redirect to a different grammar file name.
  ///
  /// For example, `jsonc` sets `grammar = "json"` because there is no
  /// separate `libtree-sitter-jsonc.so`; it reuses the JSON grammar.
  pub grammar: Option<String>,

  /// Use a different name when scanning query search paths.
  ///
  /// For example, `csharp` sets `query_lang = "c-sharp"` when Helix stores
  /// its queries under `runtime/queries/c-sharp/`.
  pub query_lang: Option<String>,

  #[serde(default)]
  pub remove_default_highlighter: RemoveDefaultHighlighter,

  #[serde(default)]
  pub filetype_hook: FileTypeHook,

  #[serde(default)]
  pub aliases: HashSet<String>,

  /// Explicit per-file query path overrides.
  #[serde(default)]
  pub queries: LanguageQueryOverrides,
}

impl LanguageConfig {
  /// Effective grammar name used for grammar file discovery.
  pub fn grammar_name<'a>(&'a self, lang: &'a str) -> &'a str {
    self.grammar.as_deref().unwrap_or(lang)
  }

  /// Effective name used for query directory discovery.
  ///
  /// Falls back to the grammar name if no explicit `query_lang` is set,
  /// then to the language name itself.
  pub fn query_name<'a>(&'a self, lang: &'a str) -> &'a str {
    self
      .query_lang
      .as_deref()
      .or(self.grammar.as_deref())
      .unwrap_or(lang)
  }

  fn merge_user_config(&mut self, user: UserLanguageConfig) {
    if let Some(grammar) = user.grammar {
      self.grammar = Some(grammar);
    }
    if let Some(query_lang) = user.query_lang {
      self.query_lang = Some(query_lang);
    }
    if let Some(v) = user.remove_default_highlighter {
      self.remove_default_highlighter = v.into();
    }
    if let Some(v) = user.filetype_hook {
      self.filetype_hook = v.into();
    }
    if let Some(aliases) = user.aliases {
      self.aliases = aliases;
    }
    if let Some(queries) = user.queries {
      self.queries.merge_user_config(queries);
    }
  }
}

impl From<UserLanguageConfig> for LanguageConfig {
  fn from(user: UserLanguageConfig) -> Self {
    Self {
      grammar: user.grammar,
      query_lang: user.query_lang,
      remove_default_highlighter: user.remove_default_highlighter.unwrap_or(true).into(),
      filetype_hook: user.filetype_hook.unwrap_or(true).into(),
      aliases: user.aliases.unwrap_or_default(),
      queries: user
        .queries
        .map(LanguageQueryOverrides::from)
        .unwrap_or_default(),
    }
  }
}

/// Explicit per-file query path overrides for a language.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LanguageQueryOverrides {
  pub highlights: Option<PathBuf>,
  pub injections: Option<PathBuf>,
  pub locals: Option<PathBuf>,
  pub textobjects: Option<PathBuf>,
}

impl LanguageQueryOverrides {
  fn merge_user_config(&mut self, user: UserLanguageQueryOverrides) {
    if let Some(p) = user.highlights {
      self.highlights = Some(p);
    }
    if let Some(p) = user.injections {
      self.injections = Some(p);
    }
    if let Some(p) = user.locals {
      self.locals = Some(p);
    }
    if let Some(p) = user.textobjects {
      self.textobjects = Some(p);
    }
  }
}

impl From<UserLanguageQueryOverrides> for LanguageQueryOverrides {
  fn from(user: UserLanguageQueryOverrides) -> Self {
    Self {
      highlights: user.highlights,
      injections: user.injections,
      locals: user.locals,
      textobjects: user.textobjects,
    }
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

/// User version of the configuration (all fields optional).
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserConfig {
  pub features: Option<UserFeaturesConfig>,
  pub highlight: Option<UserHighlightConfig>,
  pub search_paths: Option<UserSearchPathsConfig>,
  pub language: Option<HashMap<String, UserLanguageConfig>>,
}

impl UserConfig {
  pub fn load_from_xdg() -> Result<Self, ConfigError> {
    log::debug!("loading user configuration");
    let dir = dirs::config_dir().ok_or(ConfigError::NoConfigDir)?;
    let path = dir.join("kak-tree-sitter/config.toml");
    Self::load(path)
  }

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
pub struct UserSearchPathsConfig {
  pub grammars: Vec<String>,
  pub queries: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLanguageConfig {
  pub grammar: Option<String>,
  pub query_lang: Option<String>,
  pub remove_default_highlighter: Option<bool>,
  pub filetype_hook: Option<bool>,
  pub aliases: Option<HashSet<String>>,
  pub queries: Option<UserLanguageQueryOverrides>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLanguageQueryOverrides {
  pub highlights: Option<PathBuf>,
  pub injections: Option<PathBuf>,
  pub locals: Option<PathBuf>,
  pub textobjects: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
  use crate::{Config, ConfigError, UserConfig, UserLanguageConfig};

  #[test]
  fn user_config() -> Result<(), ConfigError> {
    let toml = r#"[language.odin]
      aliases = ["odin-lang"]"#;
    let config = toml::from_str::<UserConfig>(toml).unwrap();
    let lang = config
      .language
      .as_ref()
      .unwrap()
      .get("odin")
      .unwrap();

    assert!(lang.aliases.as_ref().unwrap().contains("odin-lang"));
    Ok(())
  }

  #[test]
  fn user_merge() {
    let mut config = Config::default_config().unwrap();
    let original_grammar_count = config.search_paths.grammars.len();

    let user_config = UserConfig {
      search_paths: Some(crate::UserSearchPathsConfig {
        grammars: vec!["/custom/path/{lang}.so".to_owned()],
        queries: vec![],
      }),
      language: Some(
        [(
          "odin".to_owned(),
          UserLanguageConfig {
            aliases: Some(["odin-lang".to_owned()].into()),
            ..Default::default()
          },
        )]
        .into(),
      ),
      ..Default::default()
    };

    assert!(config.merge_user_config(user_config).is_ok());

    // user grammar path appended
    assert_eq!(config.search_paths.grammars.len(), original_grammar_count + 1);
    assert_eq!(
      config.search_paths.grammars.last().unwrap(),
      "/custom/path/{lang}.so"
    );

    // language override applied
    let odin = config.languages.get("odin").unwrap();
    assert!(odin.aliases.contains("odin-lang"));
  }
}
