//! Supported languages.
//!
//! Languages have different objects (grammars, queries, etc.) living at runtime and must be loaded beforehand.

use std::{
  cell::{LazyCell, RefCell},
  collections::HashMap,
  ops::Deref,
  path::PathBuf,
  rc::Rc,
};

use kak_tree_sitter_config::Config;
use tree_house::{highlighter::Highlight, text_object::TextObjectQuery};
use tree_house_bindings::Query;

use crate::{error::OhNo, kakoune::face::Face, tree_sitter::queries::Queries};

use super::discovery::DiscoveredLang;

pub struct Language {
  pub name: String,
  pub language: tree_house::Language,
  pub textobject_query: Option<TextObjectQuery>,

  lang_config: tree_house::LanguageConfig,
}

impl Language {
  pub fn lang_name(&self) -> &str {
    &self.name
  }

  pub fn language(&self) -> tree_house::Language {
    self.language
  }
}

/// A cached language, or a blocklisted one that previously failed to load.
pub enum CachedLanguage {
  Loaded(Box<Language>),
  LoadFailed,
}

impl From<Language> for CachedLanguage {
  fn from(lang: Language) -> Self {
    Self::Loaded(Box::new(lang))
  }
}

/// All loaded languages that can be used to parse buffers.
pub struct Languages {
  langs: HashMap<String, LazyLang>,
  lang_ids: Vec<String>,
  faces: Rc<Vec<Face>>,
}

type LazyLang = LazyCell<CachedLanguage, Box<dyn FnOnce() -> CachedLanguage + 'static>>;

/// Cache of already-loaded grammars keyed by their .so path.
///
/// Multiple languages can share a grammar file (e.g. `json` and `jsonc`), so
/// we deduplicate by path rather than by language name.
pub type Grammar2Cache = Rc<RefCell<HashMap<PathBuf, tree_house_bindings::Grammar>>>;

impl Languages {
  pub fn new(config: &Config, discovered: HashMap<String, DiscoveredLang>) -> Self {
    let mut hl_names: Vec<_> = config.highlight.groups.iter().cloned().collect();

    // Sort descending so longer (more specific) capture group names match first.
    hl_names.sort_by(|a, b| b.cmp(a));

    let faces = Rc::new(hl_names.iter().map(Face::from_capture_group).collect());
    let hl_names = Rc::new(hl_names);
    let grammars2: Grammar2Cache = Rc::new(RefCell::new(HashMap::new()));

    // Sort language names for a stable idx → name mapping required by tree-house.
    let mut lang_names: Vec<String> = discovered.keys().cloned().collect();
    lang_names.sort();

    let mut discovered = discovered;

    let lang_list: Vec<_> = lang_names
      .into_iter()
      .zip(0u32..)
      .map(|(lang_name, idx)| {
        // Remove from the map so the closure captures owned data.
        let disc = discovered.remove(&lang_name).expect("lang in sorted list must be in map");

        let hl_names = hl_names.clone();
        let grammars2 = grammars2.clone();
        let lang_name2 = lang_name.clone();

        let lazy = LazyLang::new(Box::new(move || {
          match Self::load_lang(
            &hl_names,
            &grammars2,
            &lang_name2,
            &disc,
            tree_house::Language(idx),
          ) {
            Ok(lang) => CachedLanguage::Loaded(Box::new(lang)),
            Err(err) => {
              log::error!("cannot lazy load language '{lang_name2}'; will not try again: {err}");
              CachedLanguage::LoadFailed
            }
          }
        }));

        (lang_name, lazy)
      })
      .collect();

    let lang_ids = lang_list.iter().map(|(name, _)| name.clone()).collect();
    let langs = lang_list.into_iter().collect();

    Self {
      langs,
      lang_ids,
      faces,
    }
  }

  /// Load a specific language from a [`DiscoveredLang`].
  fn load_lang(
    hl_names: &[String],
    grammars2: &Grammar2Cache,
    lang_name: &str,
    disc: &DiscoveredLang,
    language: tree_house::Language,
  ) -> Result<Language, OhNo> {
    log::info!("loading language '{lang_name}'");

    // Derive the language part of the symbol name from the grammar filename.
    // tree-house_bindings::Grammar::new(name, path) looks up `tree_sitter_{name}`, so we must
    // pass only the language name portion (e.g. "odin"), not the full "tree_sitter_odin".
    //
    // Strip well-known prefixes in order of specificity:
    //   libtree-sitter-rust.so → "rust"
    //   tree-sitter-rust.so    → "rust"
    //   libodin.so             → "odin"
    let grammar_symbol = disc
      .grammar_path
      .file_stem()
      .and_then(|s| s.to_str())
      .map(|stem| {
        let lang_part = stem
          .strip_prefix("libtree-sitter-")
          .or_else(|| stem.strip_prefix("tree-sitter-"))
          .or_else(|| stem.strip_prefix("lib"))
          .unwrap_or(stem);
        lang_part.replace(['.', '-'], "_")
      })
      .unwrap_or_else(|| lang_name.replace(['.', '-'], "_"));

    // Load or reuse a cached grammar for this .so file.
    let grammar = {
      let cached = grammars2.borrow().get(&disc.grammar_path).copied();
      if let Some(g) = cached {
        log::debug!("  grammar {} already loaded; reusing", disc.grammar_path.display());
        g
      } else {
        log::debug!("  grammar path: {}", disc.grammar_path.display());
        let g = unsafe {
          tree_house_bindings::Grammar::new(&grammar_symbol, &disc.grammar_path).map_err(
            |err| OhNo::CannotLoadGrammar2 {
              lang: lang_name.to_owned(),
              err: format!("{err:?}"),
            },
          )?
        };
        grammars2.borrow_mut().insert(disc.grammar_path.clone(), g);
        g
      }
    };

    log::debug!("  highlights: {}", disc.highlights.display());
    let queries = Queries::load_from_discovered(disc);

    let textobject_query = queries
      .text_objects
      .as_deref()
      .map(|q| {
        Query::new(grammar, q, |_pat, _pred| Ok(())).map(|query| Some(TextObjectQuery { query }))
      })
      .unwrap_or_else(|| Ok(None))?;

    let lang_config = tree_house::LanguageConfig::new(
      grammar,
      queries.highlights.as_deref().unwrap_or(""),
      queries.injections.as_deref().unwrap_or(""),
      queries.locals.as_deref().unwrap_or(""),
    )?;

    lang_config.configure(|name| {
      hl_names
        .iter()
        .position(|hl_name| name.starts_with(hl_name))
        .map(|idx| Highlight::new(idx as _))
    });

    Ok(Language {
      name: lang_name.to_owned(),
      language,
      textobject_query,
      lang_config,
    })
  }

  pub fn get(&self, lang: impl AsRef<str>) -> Result<&Language, OhNo> {
    let lang_name = lang.as_ref();
    self
      .langs
      .get(lang_name)
      .ok_or_else(|| OhNo::UnknownLang {
        lang: lang_name.to_owned(),
      })
      .and_then(|lang| match lang.deref() {
        CachedLanguage::Loaded(language) => Ok(language.as_ref()),
        CachedLanguage::LoadFailed => Err(OhNo::TriedLoadingOnceLang {
          lang: lang_name.to_owned(),
        }),
      })
  }

  pub fn faces(&self) -> &Rc<Vec<Face>> {
    &self.faces
  }
}

impl tree_house::LanguageLoader for Languages {
  fn language_for_marker(
    &self,
    marker: tree_house::InjectionLanguageMarker,
  ) -> Option<tree_house::Language> {
    match marker {
      tree_house::InjectionLanguageMarker::Name(name) => {
        self.get(name).ok().map(|lang| lang.language)
      }

      tree_house::InjectionLanguageMarker::Match(name)
      | tree_house::InjectionLanguageMarker::Filename(name)
      | tree_house::InjectionLanguageMarker::Shebang(name) => {
        self.get(name.as_str()?).ok().map(|lang| lang.language)
      }
    }
  }

  fn get_config(&self, lang: tree_house::Language) -> Option<&tree_house::LanguageConfig> {
    let lang_name = self.lang_ids.get(lang.0 as usize)?.as_str();
    self.get(lang_name).ok().map(|lang| &lang.lang_config)
  }
}
