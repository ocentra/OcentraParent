use serde_json::Value;
use std::collections::BTreeSet;

pub trait ValueOrUnreachable<T> {
    fn value_or_unreachable(self, context: &str) -> T;
}

impl<T> ValueOrUnreachable<T> for Option<T> {
    fn value_or_unreachable(self, context: &str) -> T {
        match self {
            Some(value) => value,
            None => unreachable!("{context}"),
        }
    }
}

impl<T, E> ValueOrUnreachable<T> for Result<T, E> {
    fn value_or_unreachable(self, context: &str) -> T {
        match self {
            Ok(value) => value,
            Err(_) => unreachable!("{context}"),
        }
    }
}

pub trait ErrorOrUnreachable<E> {
    fn error_or_unreachable(self, context: &str) -> E;
}

impl<T, E> ErrorOrUnreachable<E> for Result<T, E> {
    fn error_or_unreachable(self, context: &str) -> E {
        match self {
            Ok(_) => unreachable!("{context}"),
            Err(error) => error,
        }
    }
}

pub fn exported_names(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            line.strip_prefix("export ")
                .or_else(|| line.strip_prefix("export type "))
                .and_then(|line| line.split_once([' ', ':', '(']).map(|(name, _)| name))
        })
        .filter(|name| !name.is_empty())
        .map(|name| name.trim_matches(|ch: char| !ch.is_alphanumeric() && ch != '_'))
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect()
}

pub fn imported_names(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            line.strip_prefix("import { ")
                .and_then(|line| line.split_once(" } from ").map(|(names, _)| names))
        })
        .flat_map(|names| names.split(','))
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect()
}

pub fn import_paths(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            line.split_once(" from '")
                .and_then(|(_, rest)| rest.split_once('\''))
                .map(|(path, _)| path)
        })
        .map(str::to_owned)
        .collect()
}

pub fn assert_exports_include(source: &str, expected: &[&str]) {
    let names = exported_names(source);
    for expected_name in expected {
        assert!(names.contains(*expected_name), "missing export {expected_name}");
    }
}

pub fn assert_import_names_include(source: &str, expected: &[&str]) {
    let names = imported_names(source);
    for expected_name in expected {
        assert!(names.contains(*expected_name), "missing import {expected_name}");
    }
}

pub fn assert_import_paths_include(source: &str, expected: &[&str]) {
    let paths = import_paths(source);
    for expected_path in expected {
        assert!(paths.contains(*expected_path), "missing import path {expected_path}");
    }
}

pub fn property_string_value(source: &str, property: &str) -> Option<String> {
    source.lines().find_map(|line| {
        let line = line.trim();
        let prefix = format!("{property}: '");
        line.strip_prefix(&prefix)
            .and_then(|line| line.strip_suffix("',"))
            .map(str::to_owned)
    })
}

pub fn module_specifiers(source: &str) -> Vec<&str> {
    let mut specifiers = Vec::new();
    let mut rest = source;

    while let Some((_, after_from)) = rest.split_once(" from '") {
        let Some((specifier, after_specifier)) = after_from.split_once('\'') else {
            break;
        };
        specifiers.push(specifier);
        rest = after_specifier;
    }

    specifiers
}

pub fn extract_json_block(source: &str, prefix: &str, suffix: &str) -> Value {
    let block = extract_typescript_block(source, prefix, suffix);
    let block = block.replace(",\n]", "\n]").replace(",\n}", "\n}");
    serde_json::from_str(&block).expect("generated json block parses")
}

pub fn extract_typescript_block(source: &str, prefix: &str, suffix: &str) -> String {
    let start = source
        .find(prefix)
        .expect("typescript block prefix to exist");
    let remainder = &source[start + prefix.len()..];
    let end = remainder.find(suffix).expect("typescript block suffix to exist");
    remainder[..end].trim().replace("\r\n", "\n")
}
