use std::{env, fmt::Write as FmtWrite, fs, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut version = env!("CARGO_PKG_VERSION").to_owned();
  if let Some(sha1) = git_sha1() {
    write!(&mut version, "-{sha1}")?;
  }

  println!("cargo:rustc-env=VERSION={version}");

  generate_faces_kak()?;

  Ok(())
}

fn generate_faces_kak() -> Result<(), Box<dyn std::error::Error>> {
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
  let config_path = manifest_dir
    .parent()
    .expect("crate has no parent directory")
    .join("kak-tree-sitter-config/default-config.toml");

  println!("cargo:rerun-if-changed={}", config_path.display());

  let config_content = fs::read_to_string(&config_path)?;
  let groups = parse_highlight_groups(&config_content);
  let faces_content = build_faces_kak(&groups);

  let output_path = manifest_dir.join("rc/faces.kak");

  // Only write when content differs to avoid spurious rebuilds of rc.rs.
  let existing = fs::read_to_string(&output_path).unwrap_or_default();
  if existing != faces_content {
    fs::write(&output_path, &faces_content)?;
  }

  Ok(())
}

/// Extract the list of capture-group names from the `[highlight] groups` array
/// in the given TOML source.  Uses line-by-line scanning rather than a full
/// TOML parser to avoid adding a build-time dependency.
fn parse_highlight_groups(content: &str) -> Vec<String> {
  let mut in_highlight = false;
  let mut in_groups = false;
  let mut groups = Vec::new();

  for line in content.lines() {
    let trimmed = line.trim();

    if trimmed == "[highlight]" {
      in_highlight = true;
      continue;
    }

    if in_highlight && trimmed.starts_with('[') {
      break; // entered the next TOML section
    }

    if in_highlight && trimmed.starts_with("groups") {
      in_groups = true;
    }

    if in_groups {
      // Each array element sits on its own line as a quoted string.
      if let Some(after_open) = trimmed.strip_prefix('"') {
        if let Some(name) = after_open.split('"').next() {
          groups.push(name.to_owned());
        }
      }

      if trimmed == "]" {
        break;
      }
    }
  }

  groups
}

fn build_faces_kak(groups: &[String]) -> String {
  let mut faces: Vec<(String, String)> = groups
    .iter()
    .map(|group| {
      let name = group.replace('.', "_");
      let parent = name
        .rfind('_')
        .map(|i| name[..i].to_owned())
        .unwrap_or_else(|| "default".to_owned());
      (name, parent)
    })
    .collect();

  faces.sort_by(|a, b| a.0.cmp(&b.0));

  let mut out = String::new();
  out.push_str(
    "# Generated from kak-tree-sitter-config/default-config.toml — do not edit directly.\n",
  );
  out.push_str("# Regenerate by running: cargo build\n");
  out.push_str("#\n");
  out.push_str(
    "# Top-level faces fall back to \"default\"; load your colorscheme AFTER this file\n",
  );
  out.push_str("# so that the colorscheme's definitions take precedence.\n");
  out.push('\n');

  for (name, parent) in &faces {
    writeln!(out, "set-face global {name} {parent}").unwrap();
  }

  out
}

fn git_sha1() -> Option<String> {
  Command::new("git")
    .args(["rev-parse", "--short", "HEAD"])
    .output()
    .ok()
    .filter(|stdout| stdout.status.success())
    .and_then(|stdout| String::from_utf8(stdout.stdout).ok())
    .map(|hash| hash.trim().to_owned())
}
