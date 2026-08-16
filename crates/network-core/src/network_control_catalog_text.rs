use super::tokens::title_from_token;

pub(crate) fn searchable_text(parts: &[&str]) -> String {
    parts.join(" ").to_lowercase()
}

pub(crate) fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles
        .iter()
        .any(|needle| contains_pattern(haystack, needle))
}

fn contains_pattern(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    if !needle.chars().all(|ch| ch.is_ascii_alphanumeric()) || needle.len() > 4 {
        return haystack.contains(needle);
    }

    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    let mut index = 0;

    while index + needle_bytes.len() <= haystack_bytes.len() {
        if &haystack_bytes[index..index + needle_bytes.len()] == needle_bytes {
            let before_is_word = index > 0 && haystack_bytes[index - 1].is_ascii_alphanumeric();
            let after_index = index + needle_bytes.len();
            let after_is_word = after_index < haystack_bytes.len()
                && haystack_bytes[after_index].is_ascii_alphanumeric();
            if !before_is_word && !after_is_word {
                return true;
            }
        }
        index += 1;
    }

    false
}

pub(crate) fn matrix_option_labels(source_text: &str) -> Vec<String> {
    if !source_text.starts_with("Capability matrix row |") {
        return Vec::new();
    }
    source_text
        .split(" | ")
        .skip(1)
        .map(|part| {
            if let Some(separator_index) = part.find('=') {
                let heading = &part[..separator_index];
                let value = &part[separator_index + 1..];
                format!("{heading}: {value}")
            } else {
                format!("Cell: {part}")
            }
        })
        .collect()
}

pub(crate) fn split_explicit_options(source_text: &str) -> Vec<String> {
    let source = source_text.trim_end_matches('.');
    let chars: Vec<char> = source.chars().collect();
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut index = 0;

    while index < chars.len() {
        let tail = chars[index..].iter().collect::<String>().to_lowercase();
        if chars[index] == ',' || chars[index] == ';' {
            push_option_part(&mut parts, &mut current);
            index += 1;
            continue;
        }
        if tail.starts_with(" or ") {
            push_option_part(&mut parts, &mut current);
            index += 4;
            continue;
        }
        current.push(chars[index]);
        index += 1;
    }

    push_option_part(&mut parts, &mut current);
    parts
}

fn push_option_part(parts: &mut Vec<String>, current: &mut String) {
    let trimmed = current.trim();
    if !trimmed.is_empty() {
        parts.push(trimmed.to_owned());
    }
    current.clear();
}

pub(crate) fn clean_option_label(value: &str) -> String {
    let normalized = value
        .trim()
        .trim_end_matches('.')
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-");
    title_from_token(&normalized)
}

pub(crate) fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let mut result = first.to_uppercase().collect::<String>();
    result.push_str(chars.as_str());
    result
}

pub(crate) fn lower_first(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let mut result = first.to_lowercase().collect::<String>();
    result.push_str(chars.as_str());
    result
}
