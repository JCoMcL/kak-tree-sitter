//! rc file used by Kakoune to inject kak-tree-sitter commands.

use crate::cli::Cli;

/// Main RC file.
pub fn static_kak() -> &'static str {
  include_str!("../../rc/static.kak")
}

/// Text-objects related file.
pub fn text_objects_kak() -> &'static str {
  include_str!("../../rc/text-objects.kak")
}

/// Face hierarchy declarations, generated at build time from default-config.toml.
pub fn faces_kak() -> &'static str {
  include_str!("../../rc/faces.kak")
}

/// kak-tree-sitter CLI arguments.
pub fn cli_args_opt_kak(cli: &Cli) -> String {
  let mut opt = "set-option global tree_sitter_cli_args".to_owned();

  // verbosity
  if cli.verbose > 0 {
    opt.push_str(&format!(" -{}", "v".repeat(cli.verbose as _)));
  }

  // config
  if let Some(path) = &cli.config {
    opt.push_str(&format!(" --config={}", path.display()));
  }

  opt
}
