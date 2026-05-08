//! Auto-discovery of grammars and query files from search paths.

use std::{
  collections::{HashMap, HashSet},
  env,
  path::{Path, PathBuf},
};

use kak_tree_sitter_config::Config;

/// A fully-resolved language, ready for loading.
#[derive(Debug)]
pub struct DiscoveredLang {
  pub grammar_path: PathBuf,
  pub highlights: PathBuf,
  pub injections: Option<PathBuf>,
  pub locals: Option<PathBuf>,
  pub textobjects: Option<PathBuf>,
}

/// Expand `~` and `${VAR}` / `$VAR` in a path pattern.
fn expand_path(pattern: &str) -> String {
  // expand leading ~
  let s = if let Some(rest) = pattern.strip_prefix("~/") {
    let home = dirs::home_dir()
      .unwrap_or_default()
      .display()
      .to_string();
    format!("{home}/{rest}")
  } else if pattern == "~" {
    dirs::home_dir()
      .unwrap_or_default()
      .display()
      .to_string()
  } else {
    pattern.to_owned()
  };

  // expand ${VAR} and $VAR
  let mut result = String::with_capacity(s.len());
  let mut chars = s.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch != '$' {
      result.push(ch);
      continue;
    }
    if chars.peek() == Some(&'{') {
      chars.next(); // consume '{'
      let var: String = chars.by_ref().take_while(|&c| c != '}').collect();
      result.push_str(&env::var(&var).unwrap_or_default());
    } else {
      let mut var = String::new();
      while let Some(&c) = chars.peek() {
        if c.is_ascii_alphanumeric() || c == '_' {
          var.push(c);
          chars.next();
        } else {
          break;
        }
      }
      result.push_str(&env::var(&var).unwrap_or_default());
    }
  }

  result
}

/// Given a pattern containing `{lang}` and a candidate name, extract the
/// language name if the candidate matches.
///
/// Example: `extract_lang("libtree-sitter-{lang}.so", "libtree-sitter-rust.so")` → `Some("rust")`
fn extract_lang(pattern: &str, candidate: &str) -> Option<String> {
  let (prefix, suffix) = pattern.split_once("{lang}")?;
  let without_prefix = candidate.strip_prefix(prefix)?;
  let lang = without_prefix.strip_suffix(suffix)?;
  if lang.is_empty() {
    return None;
  }
  Some(lang.to_owned())
}

/// Scan a grammar search pattern and return `(lang_name, path)` pairs.
///
/// `{lang}` must appear in the final path component (the filename).
fn scan_grammar_pattern(pattern: &str) -> Vec<(String, PathBuf)> {
  let expanded = expand_path(pattern);
  let path = Path::new(&expanded);

  let Some(parent) = path.parent() else {
    return vec![];
  };
  let Some(file_pat) = path.file_name().and_then(|f| f.to_str()) else {
    return vec![];
  };

  if !file_pat.contains("{lang}") {
    log::warn!(
      "grammar pattern {pattern:?}: {{lang}} must appear in the filename component; skipping"
    );
    return vec![];
  }

  let Ok(entries) = std::fs::read_dir(parent) else {
    return vec![];
  };

  entries
    .flatten()
    .filter(|e| e.path().is_file())
    .filter_map(|e| {
      let fname = e.file_name();
      let fname = fname.to_str()?;
      let lang = extract_lang(file_pat, fname)?;
      Some((lang, e.path()))
    })
    .collect()
}

/// Scan a query search pattern and return `(lang_name, query_dir)` pairs.
///
/// `{lang}` must appear in the final path component (the directory name).
fn scan_query_pattern(pattern: &str) -> Vec<(String, PathBuf)> {
  let expanded = expand_path(pattern);
  let path = Path::new(&expanded);

  let Some(parent) = path.parent() else {
    return vec![];
  };
  let Some(dir_pat) = path.file_name().and_then(|f| f.to_str()) else {
    return vec![];
  };

  if !dir_pat.contains("{lang}") {
    log::warn!(
      "query pattern {pattern:?}: {{lang}} must appear in the last path component; skipping"
    );
    return vec![];
  }

  let Ok(entries) = std::fs::read_dir(parent) else {
    return vec![];
  };

  entries
    .flatten()
    .filter(|e| e.path().is_dir())
    .filter_map(|e| {
      let dname = e.file_name();
      let dname = dname.to_str()?;
      let lang = extract_lang(dir_pat, dname)?;
      Some((lang, e.path()))
    })
    .collect()
}

/// Scan all grammar search paths and return a map of `lang → grammar .so path`.
///
/// Later entries in `config.search_paths.grammars` override earlier ones.
fn discover_grammars(config: &Config) -> HashMap<String, PathBuf> {
  let mut grammars: HashMap<String, PathBuf> = HashMap::new();

  for pattern in &config.search_paths.grammars {
    for (lang, path) in scan_grammar_pattern(pattern) {
      if let Some(prev) = grammars.insert(lang.clone(), path.clone()) {
        log::debug!(
          "grammar for '{lang}': {} overridden by {}",
          prev.display(),
          path.display()
        );
      }
    }
  }

  grammars
}

/// Query file types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum QueryFile {
  Highlights,
  Injections,
  Locals,
  TextObjects,
}

impl QueryFile {
  fn filename(self) -> &'static str {
    match self {
      Self::Highlights => "highlights.scm",
      Self::Injections => "injections.scm",
      Self::Locals => "locals.scm",
      Self::TextObjects => "textobjects.scm",
    }
  }

  fn all() -> impl Iterator<Item = Self> {
    [
      Self::Highlights,
      Self::Injections,
      Self::Locals,
      Self::TextObjects,
    ]
    .into_iter()
  }
}

type QueryMap = HashMap<String, HashMap<QueryFile, PathBuf>>;

/// Scan all query search paths and return a map of `lang → {file_type → path}`.
///
/// Later entries in `config.search_paths.queries` override earlier ones.
fn discover_queries(config: &Config) -> QueryMap {
  let mut queries: QueryMap = HashMap::new();

  for pattern in &config.search_paths.queries {
    for (lang, dir) in scan_query_pattern(pattern) {
      let entry = queries.entry(lang.clone()).or_default();
      for file_type in QueryFile::all() {
        let path = dir.join(file_type.filename());
        if path.exists() {
          if let Some(prev) = entry.insert(file_type, path.clone()) {
            log::warn!(
              "{} for '{lang}': {} overridden by {}",
              file_type.filename(),
              prev.display(),
              path.display()
            );
          }
        }
      }
    }
  }

  queries
}

/// Discover all available languages by combining grammar and query search results.
pub fn discover(config: &Config) -> HashMap<String, DiscoveredLang> {
  let grammars = discover_grammars(config);
  let queries = discover_queries(config);

  // All candidate language names: from grammar filenames, query dirs, and explicit config entries.
  let candidates: HashSet<String> = grammars
    .keys()
    .chain(queries.keys())
    .chain(config.languages.language.keys())
    .cloned()
    .collect();

  let mut discovered: HashMap<String, DiscoveredLang> = HashMap::new();
  let mut with_queries: HashSet<String> = queries.keys().cloned().collect();

  for lang in candidates {
    let lang_cfg = config.languages.get(&lang);

    // Grammar lookup: use the redirected name if configured.
    let grammar_name = lang_cfg.map(|lc| lc.grammar_name(&lang)).unwrap_or(&lang);
    let Some(grammar_path) = grammars.get(grammar_name).cloned() else {
      if queries.contains_key(&lang) {
        log::warn!(
          "queries found for '{lang}' (grammar name: '{grammar_name}') but no grammar .so was discovered; language will not be enabled"
        );
      }
      continue;
    };

    // Query lookup: use the redirected name if configured.
    let query_name = lang_cfg.map(|lc| lc.query_name(&lang)).unwrap_or(&lang);
    let discovered_files = queries.get(query_name);
    with_queries.remove(query_name);

    // Highlights are mandatory.
    let highlights = lang_cfg
      .and_then(|lc| lc.queries.highlights.clone())
      .or_else(|| {
        discovered_files
          .and_then(|f| f.get(&QueryFile::Highlights))
          .cloned()
      });

    let Some(highlights) = highlights else {
      log::debug!(
        "grammar found for '{lang}' at {} but no highlights.scm; language will not be enabled",
        grammar_path.display()
      );
      continue;
    };

    let get_query = |file_type: QueryFile, override_path: &Option<PathBuf>| -> Option<PathBuf> {
      override_path
        .clone()
        .or_else(|| discovered_files.and_then(|f| f.get(&file_type)).cloned())
    };

    let overrides = lang_cfg.map(|lc| &lc.queries);
    let injections = get_query(QueryFile::Injections, &overrides.and_then(|o| o.injections.clone()));
    let locals = get_query(QueryFile::Locals, &overrides.and_then(|o| o.locals.clone()));
    let textobjects = get_query(QueryFile::TextObjects, &overrides.and_then(|o| o.textobjects.clone()));

    log::info!(
      "discovered language '{lang}': grammar={}, highlights={}",
      grammar_path.display(),
      highlights.display()
    );

    discovered.insert(
      lang,
      DiscoveredLang {
        grammar_path,
        highlights,
        injections,
        locals,
        textobjects,
      },
    );
  }

  // Warn about query dirs that had no corresponding grammar at all.
  for lang in with_queries {
    if !discovered.contains_key(&lang) {
      log::debug!(
        "query files found for '{lang}' but no grammar was discovered; skipping"
      );
    }
  }

  discovered
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_expand_path_home() {
    let home = dirs::home_dir().unwrap_or_default().display().to_string();
    assert_eq!(expand_path("~/foo/bar"), format!("{home}/foo/bar"));
  }

  #[test]
  fn test_expand_path_env() {
    unsafe { std::env::set_var("KTS_TEST_VAR", "hello") };
    assert_eq!(expand_path("${KTS_TEST_VAR}/world"), "hello/world");
    assert_eq!(expand_path("$KTS_TEST_VAR/world"), "hello/world");
  }

  #[test]
  fn test_extract_lang() {
    assert_eq!(
      extract_lang("libtree-sitter-{lang}.so", "libtree-sitter-rust.so"),
      Some("rust".to_owned())
    );
    assert_eq!(
      extract_lang("{lang}", "python"),
      Some("python".to_owned())
    );
    assert_eq!(
      extract_lang("libtree-sitter-{lang}.so", "libtree-sitter-.so"),
      None
    );
    assert_eq!(
      extract_lang("libtree-sitter-{lang}.so", "libsomething-rust.so"),
      None
    );
  }
}
