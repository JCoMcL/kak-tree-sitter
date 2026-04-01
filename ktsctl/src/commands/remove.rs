//! Module to remove resources.

use std::fs;

use colored::Colorize as _;
use kak_tree_sitter_config::{Config, GrammarConfig, LanguageConfig, source::Source};

use crate::{error::HellNo, resources::Resources, ui::report::Report};

/// Delete resources associated with a given language.
pub fn remove<'lang>(
  report: Report,
  config: &Config,
  resources: &Resources,
  grammar: bool,
  queries: bool,
  prune: bool,
  langs: impl Iterator<Item = &'lang str>,
) {
  for lang in langs {
    report!(report, "working {lang}");
    let report = report.incr();

    if let Err(err) = remove_lang(report, config, resources, grammar, queries, prune, lang) {
      report_error!(report, "{err}", err = err.to_string().red());
    }
  }
}

/// Delete resources associated with a given language.
pub fn remove_lang(
  report: Report,
  config: &Config,
  resources: &Resources,
  grammar: bool,
  queries: bool,
  prune: bool,
  lang: &str,
) -> Result<(), HellNo> {
  report!(report, "removing resources for {lang}");
  let report = report.incr();

  let lang_config = config.languages.get_lang_config(lang)?;
  let grammar_config = config
    .grammars
    .get_grammar_config(lang_config.grammar.as_deref().unwrap_or(lang))?;
  let mut errors = Vec::new();

  if grammar {
    remove_grammar(report, resources, lang, grammar_config, prune, &mut errors);
  }

  if queries {
    remove_queries(report, resources, lang, lang_config, prune, &mut errors);
  }

  if errors.is_empty() {
    report_success!(report, "{lang} removed");
  } else {
    report_error!(report, "cannot remove {lang}");
    let report = report.incr();

    for err in errors {
      report_error!(report, "{}", err.red());
    }
  }

  Ok(())
}

fn remove_grammar(
  report: Report,
  resources: &Resources,
  lang: &str,
  grammar_config: &GrammarConfig,
  prune: bool,
  errors: &mut Vec<String>,
) {
  if prune {
    let dir = resources.grammars_dir(lang);
    if let Ok(true) = dir.try_exists() {
      report!(report, "removing {lang} grammar", lang = lang.blue());
      let report = report.incr();

      if let Err(err) = fs::remove_dir_all(dir) {
        errors.push(format!(
          "cannot remove {lang} grammar: {err}",
          lang = lang.blue(),
          err = err.to_string().red()
        ));
      } else {
        report_success!(report, "removed {lang} grammar", lang = lang.blue());
      }
    } else {
      report_info!(report, "{lang} grammar already removed", lang = lang.blue());
    }
  } else {
    let grammar_path = resources.grammar_path_from_config(lang, grammar_config);

    if let Ok(true) = grammar_path.try_exists() {
      report!(report, "removing {lang} grammar", lang = lang.blue());
      let report = report.incr();

      if let Err(err) = fs::remove_file(grammar_path) {
        errors.push(format!(
          "cannot remove {lang} grammar: {err}",
          lang = lang.blue(),
          err = err.to_string().red()
        ));
      } else {
        report_success!(report, "removed {lang} grammar", lang = lang.blue());
      }
    } else {
      report_info!(report, "{lang} grammar already removed", lang = lang.blue());
    }
  }
}

fn remove_queries(
  report: Report,
  resources: &Resources,
  lang: &str,
  lang_config: &LanguageConfig,
  prune: bool,
  errors: &mut Vec<String>,
) {
  let dir = if prune {
    Some(resources.queries_dir(lang))
  } else {
    resources.queries_dir_from_config(lang, lang_config)
  };

  if let Some(dir) = dir {
    if let Ok(true) = dir.try_exists() {
      report!(report, "removing {lang} queries", lang = lang.blue());
      let report = report.incr();

      if let Err(err) = fs::remove_dir_all(dir) {
        errors.push(format!(
          "cannot remove {lang} queries: {err}",
          lang = lang.blue(),
          err = err.to_string().red()
        ));
      } else {
        report_success!(report, "removed {lang} queries", lang = lang.blue());
      }
    } else {
      report_info!(report, "{lang} queries already removed", lang = lang.blue());
    }
  }
}

/// Prune everything, removing unpinned data.
pub fn prune_unpinned(
  report: Report,
  config: &Config,
  resources: &Resources,
) -> Result<(), HellNo> {
  for (lang, lang_config) in config.languages.iter() {
    let grammar_config = config
      .grammars
      .get_grammar_config(lang_config.grammar.as_deref().unwrap_or(lang))?;
    let mut errors = Vec::new();

    let report = report.incr();
    report!(report, "working {}", lang.blue());

    if let Err(err) = prune_unpinned_lang(resources, lang, lang_config, grammar_config, &mut errors)
    {
      errors.push(format!("{err}"));
    }

    if errors.is_empty() {
      report_success!(report, "pruned {}", lang.blue());
    } else {
      report_error!(
        report,
        "cannot prune {lang}:\n  {err}",
        lang = lang.blue(),
        err = errors.join("\n  ").red()
      );
    }
  }

  Ok(())
}

fn prune_unpinned_lang(
  resources: &Resources,
  lang: &str,
  lang_config: &LanguageConfig,
  grammar_config: &GrammarConfig,
  errors: &mut Vec<String>,
) -> Result<(), HellNo> {
  prune_unpinned_grammar(resources, lang, grammar_config, errors)?;
  prune_unpinned_queries(resources, lang, lang_config, errors)?;
  Ok(())
}

fn prune_unpinned_grammar(
  resources: &Resources,
  lang: &str,
  grammar_config: &GrammarConfig,
  errors: &mut Vec<String>,
) -> Result<(), HellNo> {
  let grammar_dir = resources.grammars_dir(lang);
  let grammar_dir = grammar_dir
    .read_dir()
    .map_err(|_| HellNo::NoGrammarDirForLang {
      lang: lang.to_owned(),
    })?;

  for entry in grammar_dir.flatten() {
    match &grammar_config.source {
      Source::Local { path } => {
        if entry.path() != *path
          && let Err(err) = fs::remove_file(entry.path())
        {
          errors.push(format!(
            "cannot prune grammar for {lang} at {path}: {err}",
            lang = lang.blue(),
            path = entry.path().display(),
            err = err.to_string().red(),
          ));
        }
      }

      Source::Git { pin, .. } => {
        if !entry
          .path()
          .file_name()
          .unwrap()
          .to_str()
          .unwrap()
          .starts_with(pin)
          && let Err(err) = fs::remove_file(entry.path())
        {
          errors.push(format!(
            "cannot prune grammar for {lang} at {path}: {err}",
            lang = lang.blue(),
            path = entry.path().display(),
            err = err.to_string().red(),
          ));
        }
      }
    }
  }

  Ok(())
}

fn prune_unpinned_queries(
  resources: &Resources,
  lang: &str,
  lang_config: &LanguageConfig,
  errors: &mut Vec<String>,
) -> Result<(), HellNo> {
  let queries_dir = resources.queries_dir(lang);
  let queries_dir = queries_dir
    .read_dir()
    .map_err(|_| HellNo::NoQueriesDirForLang {
      lang: lang.to_owned(),
    })?;

  for entry in queries_dir.flatten() {
    match &lang_config.queries.source {
      Some(Source::Local { path }) => {
        if entry.path() != *path
          && let Err(err) = fs::remove_file(entry.path())
        {
          errors.push(format!(
            "cannot prune queries for {lang} at {path}: {err}",
            lang = lang.blue(),
            path = entry.path().display(),
            err = err.to_string().red(),
          ));
        }
      }

      Some(Source::Git { pin, .. }) => {
        if entry.path().file_name().unwrap().to_str() != Some(pin)
          && let Err(err) = fs::remove_dir_all(entry.path())
        {
          errors.push(format!(
            "cannot prune queries for {lang} at {path}: {err}",
            lang = lang.blue(),
            path = entry.path().display(),
            err = err.to_string().red(),
          ));
        }
      }

      // no source means that we use the same as the grammar
      None => todo!(),
    }
  }

  Ok(())
}
