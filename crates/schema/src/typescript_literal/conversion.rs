const JSON_KEY_SEPARATOR: &str = "\": ";

pub(super) fn convert_json_line(line: &str) -> Vec<String> {
    let converted = convert_json_key(line);
    let converted = convert_json_strings(&converted);
    let converted = select_double_quotes_for_apostrophes(line, converted);
    wrap_converted_line(line, converted)
}

fn convert_json_key(line: &str) -> String {
    let trimmed = line.trim_start();
    let Some(key_and_rest) = trimmed.strip_prefix('"') else {
        return line.to_owned();
    };
    let Some((key, rest)) = key_and_rest.split_once(JSON_KEY_SEPARATOR) else {
        return line.to_owned();
    };
    let indent = &line[..line.len() - trimmed.len()];
    format!("{indent}{key}: {rest}")
}

fn convert_json_strings(line: &str) -> String {
    let mut output = String::with_capacity(line.len());
    let mut in_string = false;
    let mut escaped = false;
    for character in line.chars() {
        push_converted_character(&mut output, &mut in_string, &mut escaped, character);
    }
    if escaped {
        output.push('\\');
    }
    output
}

fn push_converted_character(
    output: &mut String,
    in_string: &mut bool,
    escaped: &mut bool,
    character: char,
) {
    if *escaped {
        if character != '"' {
            output.push('\\');
        }
        output.push(character);
        *escaped = false;
    } else if *in_string && character == '\\' {
        *escaped = true;
    } else if character == '"' {
        output.push('\'');
        *in_string = !*in_string;
    } else if *in_string && character == '\'' {
        output.push_str("\\'");
    } else {
        output.push(character);
    }
}

fn select_double_quotes_for_apostrophes(source: &str, mut output: String) -> String {
    if !source.contains('\'') {
        return output;
    }
    output = output.replace("\\'", "'");
    replace_string_value_quotes(&mut output);
    output
}

fn replace_string_value_quotes(output: &mut String) {
    let Some(separator_index) = output.find(": '") else {
        return;
    };
    let opening_index = separator_index + 2;
    let tail = &output[opening_index + 1..];
    let closing_index = tail
        .strip_suffix("',")
        .map(str::len)
        .or_else(|| tail.strip_suffix('\'').map(str::len));
    let Some(relative_closing_index) = closing_index else {
        return;
    };
    let closing_index = opening_index + 1 + relative_closing_index;
    output.replace_range(opening_index..=opening_index, "\"");
    output.replace_range(closing_index..=closing_index, "\"");
}

fn wrap_converted_line(source: &str, output: String) -> Vec<String> {
    if output.len() <= super::PRINT_WIDTH {
        return vec![output];
    }
    let Some(separator_index) = output.find(": '") else {
        return vec![output];
    };
    let value = &output[separator_index + 2..];
    if !is_complete_string_value(value) {
        return vec![output];
    }
    let prefix = &output[..separator_index];
    let indent = " ".repeat(source.len() - source.trim_start().len() + 2);
    vec![format!("{prefix}:"), format!("{indent}{value}")]
}

fn is_complete_string_value(value: &str) -> bool {
    value.starts_with('\'') && (value.ends_with("',") || value.ends_with('\''))
}
