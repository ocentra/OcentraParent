use super::{conversion, LINE_BREAK, PRINT_WIDTH};

pub(super) fn json_to_typescript_literal(json: &str, compact_scalar_arrays: bool) -> String {
    let converted_lines = json
        .lines()
        .flat_map(conversion::convert_json_line)
        .collect::<Vec<_>>();
    let formatted_lines = compact_arrays(converted_lines, compact_scalar_arrays);
    add_trailing_commas(&formatted_lines).join(LINE_BREAK)
}

fn compact_arrays(lines: Vec<String>, enabled: bool) -> Vec<String> {
    if !enabled {
        return lines;
    }
    let mut formatted = Vec::with_capacity(lines.len());
    let mut index = 0;
    while index < lines.len() {
        if let Some((line, next_index)) = compact_array_at(&lines, index) {
            formatted.push(line);
            index = next_index;
        } else {
            formatted.push(lines[index].clone());
            index += 1;
        }
    }
    formatted
}

fn compact_array_at(lines: &[String], index: usize) -> Option<(String, usize)> {
    let line = lines.get(index)?;
    if !line.trim().ends_with('[') {
        return None;
    }
    let closing_index = find_closing_array(lines, index + 1)?;
    let items = &lines[index + 1..closing_index];
    if !items.iter().all(|item| is_scalar_array_item(item)) {
        return None;
    }
    let item_text = items
        .iter()
        .map(|item| item.trim().trim_end_matches(','))
        .collect::<Vec<_>>()
        .join(", ");
    let prefix = &line[..line.len() - 1];
    let closing = lines[closing_index].trim();
    let suffix = if closing.ends_with(',') { "," } else { "" };
    let candidate = format!("{prefix}[{item_text}]{suffix}");
    (candidate.len() <= PRINT_WIDTH).then_some((candidate, closing_index + 1))
}

fn find_closing_array(lines: &[String], mut index: usize) -> Option<usize> {
    while index < lines.len() {
        if matches!(lines[index].trim(), "]" | "],") {
            return Some(index);
        }
        index += 1;
    }
    None
}

fn is_scalar_array_item(item: &str) -> bool {
    let item = item.trim_end_matches(',');
    !item.is_empty()
        && !item.contains('{')
        && !item.contains('}')
        && !item.contains('[')
        && !item.contains(']')
}

fn add_trailing_commas(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .enumerate()
        .map(|(index, line)| add_trailing_comma(line, lines.get(index + 1)))
        .collect()
}

fn add_trailing_comma(line: &str, next: Option<&String>) -> String {
    let trimmed = line.trim_end();
    if needs_trailing_comma(trimmed, next) {
        format!("{trimmed},")
    } else {
        line.to_owned()
    }
}

fn needs_trailing_comma(line: &str, next: Option<&String>) -> bool {
    matches!(
        next.map(|line| line.trim().trim_end_matches(',')),
        Some("}" | "]")
    ) && !line.is_empty()
        && !line.ends_with(',')
        && !line.ends_with('{')
        && !line.ends_with('[')
}

pub(super) fn wrap_long_typescript_lines(source: &str) -> String {
    source
        .lines()
        .flat_map(wrap_long_line)
        .collect::<Vec<_>>()
        .join(LINE_BREAK)
}

fn wrap_long_line(line: &str) -> Vec<String> {
    if line.len() <= PRINT_WIDTH {
        return vec![line.to_owned()];
    }
    split_long_string_property(line).unwrap_or_else(|| vec![line.to_owned()])
}

fn split_long_string_property(line: &str) -> Option<Vec<String>> {
    let separator_index = line.find(": '")?;
    let value = &line[separator_index + 2..];
    if !is_complete_string_value(value) {
        return None;
    }
    let prefix = &line[..separator_index];
    let indent = " ".repeat(line.len() - line.trim_start().len() + 2);
    Some(vec![format!("{prefix}:"), format!("{indent}{value}")])
}

fn is_complete_string_value(value: &str) -> bool {
    value.starts_with('\'') && (value.ends_with("',") || value.ends_with('\''))
}
