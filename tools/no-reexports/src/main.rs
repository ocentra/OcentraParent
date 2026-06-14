use anyhow::{bail, Context, Result};
use quote::ToTokens;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
use syn::{Item, Visibility};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
enum Scope {
    All,
    Diff { base: String, head: String },
    Paths(Vec<PathBuf>),
}

fn main() -> Result<()> {
    let scope = parse_args(env::args_os().skip(1).map(PathBuf::from).collect())?;
    let rust_files = match scope {
        Scope::All => collect_all_files()?,
        Scope::Diff { base, head } => collect_diff_files(&base, &head)?,
        Scope::Paths(paths) => collect_explicit_files(&paths)?,
    };

    if rust_files.is_empty() {
        println!("Architecture lint skipped: no Rust source files matched the requested scope.");
        return Ok(());
    }

    let mut failures = Vec::new();

    for file in rust_files {
        scan_file(&file, &mut failures)?;
    }

    if failures.is_empty() {
        println!("Rust architecture lint passed.");
        return Ok(());
    }

    eprintln!("BARREL/REEXPORT BAN FAILED");
    eprintln!("Public Rust re-exports are forbidden in this repo.");
    eprintln!();

    for failure in failures {
        eprintln!("  - {failure}");
    }

    eprintln!();
    eprintln!("Fix: replace `pub use ...` with direct module imports and explicit module paths.");
    std::process::exit(1);
}

fn parse_args(args: Vec<PathBuf>) -> Result<Scope> {
    if args.len() == 1 && args[0] == PathBuf::from("--all") {
        return Ok(Scope::All);
    }

    if args.len() == 4 && args[0] == PathBuf::from("--base") && args[2] == PathBuf::from("--head") {
        return Ok(Scope::Diff {
            base: args[1].to_string_lossy().into_owned(),
            head: args[3].to_string_lossy().into_owned(),
        });
    }

    if !args.is_empty() {
        return Ok(Scope::Paths(args));
    }

    bail!(
        "usage:\n  cargo lint-architecture --all\n  cargo lint-architecture --base <sha> --head <sha>\n  cargo lint-architecture <path> [more paths]"
    );
}

fn collect_all_files() -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for root in ["crates", "tools"] {
        let root_path = Path::new(root);
        if root_path.exists() {
            files.extend(walk_rust_files(root_path)?);
        }
    }

    Ok(files)
}

fn collect_diff_files(base: &str, head: &str) -> Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .args(["diff", "--name-only", "--diff-filter=ACMR", base, head, "--", "crates", "tools"])
        .output()
        .context("failed to run git diff for Rust architecture scope")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git diff failed for Rust architecture scope: {}", stderr.trim());
    }

    Ok(unique_paths(
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|line| is_rust_file(Path::new(line)))
            .map(PathBuf::from)
            .collect(),
    ))
}

fn collect_explicit_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for path in paths {
        if path.is_dir() {
            files.extend(walk_rust_files(path)?);
            continue;
        }

        if is_rust_file(path) {
            files.push(path.clone());
        }
    }

    Ok(unique_paths(files))
}

fn walk_rust_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !is_skipped_dir(entry))
    {
        let entry = entry?;
        if entry.file_type().is_file() && is_rust_file(entry.path()) {
            files.push(entry.path().to_path_buf());
        }
    }

    Ok(files)
}

fn scan_file(path: &Path, failures: &mut Vec<String>) -> Result<()> {
    let source = fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let file = syn::parse_file(&source).with_context(|| format!("failed to parse {}", path.display()))?;
    check_items(path, &file.items, failures);
    Ok(())
}

fn check_items(path: &Path, items: &[Item], failures: &mut Vec<String>) {
    for item in items {
        match item {
            Item::Use(item_use) => {
                if !matches!(item_use.vis, Visibility::Inherited) {
                    failures.push(format!(
                        "{}: forbidden public re-export `{}`",
                        path.display(),
                        item_use.to_token_stream()
                    ));
                }
            }
            Item::Mod(item_mod) => {
                if let Some((_, nested_items)) = &item_mod.content {
                    check_items(path, nested_items, failures);
                }
            }
            _ => {}
        }
    }
}

fn is_rust_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("rs"))
}

fn is_skipped_dir(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    entry.file_type().is_dir()
        && matches!(name.as_ref(), ".git" | "target" | "node_modules" | "dist" | "build" | "coverage")
}

fn unique_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut unique = Vec::new();

    for path in paths {
        if !unique.iter().any(|existing| existing == &path) {
            unique.push(path);
        }
    }

    unique
}
