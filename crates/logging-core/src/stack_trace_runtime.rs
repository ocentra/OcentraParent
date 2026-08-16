#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StackFrame {
    pub function_name: Option<String>,
    pub file: Option<String>,
    pub file_path: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

pub fn normalize_path(value: &str) -> String {
    value.replace('\\', "/")
}

pub fn decode_file_path(value: &str) -> String {
    value
        .strip_prefix("file://")
        .map(|rest| normalize_path(&trim_windows_file_url_prefix(&percent_decode(rest))))
        .unwrap_or_else(|| normalize_path(value))
}

pub fn file_name_from_path(file_path: Option<&str>) -> Option<String> {
    file_path.map(|file_path| {
        let normalized = normalize_path(file_path);
        normalized
            .rsplit_once('/')
            .map(|(_, file_name)| file_name.to_owned())
            .unwrap_or(normalized)
    })
}

pub fn module_name_from_path(file_path: &str) -> String {
    let file_name = file_name_from_path(Some(file_path)).unwrap_or_default();
    let stem = file_name
        .rsplit_once('.')
        .map(|(stem, _)| stem)
        .unwrap_or(file_name.as_str());
    stem.split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            chars.next().map_or(String::new(), |first| {
                let mut title = String::new();
                title.push(first.to_ascii_uppercase());
                title.push_str(chars.as_str());
                title
            })
        })
        .collect()
}

pub fn resolve_logger_context(
    module_name: &str,
    frame: Option<&StackFrame>,
    module_context_suffix: &str,
) -> String {
    frame
        .and_then(|frame| frame.function_name.as_deref())
        .map(str::trim)
        .filter(|function_name| !function_name.is_empty())
        .map(|function_name| {
            if function_name.contains('.') {
                function_name.to_owned()
            } else {
                format!("{module_name}.{function_name}")
            }
        })
        .unwrap_or_else(|| format!("{module_name}.{module_context_suffix}"))
}

pub fn resolve_logger_source(module_name: &str, frame: Option<&StackFrame>) -> String {
    frame
        .and_then(|frame| frame.function_name.as_deref())
        .filter(|function_name| function_name.contains('.'))
        .and_then(|function_name| function_name.split('.').next())
        .unwrap_or(module_name)
        .to_owned()
}

pub fn parse_stack_trace(stack_trace: &str) -> Vec<StackFrame> {
    stack_trace
        .lines()
        .filter_map(|line| parse_frame_line(line.trim_end_matches('\r')))
        .collect()
}

pub fn stack_trace_runtime_typescript() -> &'static str {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../logging-core-generated/stack_trace_runtime.ts"
    ))
}

fn parse_frame_line(line: &str) -> Option<StackFrame> {
    let trimmed = line.trim();
    let body = trimmed.strip_prefix("at ")?;
    let parsed = body
        .split_once(" (")
        .and_then(|(function_name, location)| {
            location.strip_suffix(')').map(|location| {
                (
                    (!function_name.trim().is_empty()).then(|| function_name.trim().to_owned()),
                    location,
                )
            })
        })
        .map(|(function_name, location)| {
            let (file, file_path, line, column) = parse_location(location);
            StackFrame {
                function_name,
                file,
                file_path,
                line,
                column,
            }
        });

    Some(parsed.unwrap_or_else(|| {
        let (file, file_path, line, column) = parse_location(body);
        StackFrame {
            function_name: None,
            file,
            file_path,
            line,
            column,
        }
    }))
}

fn parse_location(location: &str) -> (Option<String>, Option<String>, Option<u32>, Option<u32>) {
    let trimmed = location.trim();
    let Some((file_and_line, column_text)) = trimmed.rsplit_once(':') else {
        return parse_location_without_line_info(trimmed);
    };
    let Some((file_text, line_text)) = file_and_line.rsplit_once(':') else {
        return parse_location_without_line_info(trimmed);
    };
    let Some(line) = parse_integer_segment(line_text) else {
        return parse_location_without_line_info(trimmed);
    };
    let Some(column) = parse_integer_segment(column_text) else {
        return parse_location_without_line_info(trimmed);
    };

    let file_path = decode_file_path(file_text);
    let file = file_name_from_path(Some(&file_path));
    (file, Some(file_path), Some(line), Some(column))
}

fn parse_location_without_line_info(
    location: &str,
) -> (Option<String>, Option<String>, Option<u32>, Option<u32>) {
    let file_path = decode_file_path(location);
    let file = file_name_from_path(Some(&file_path));
    (file, Some(file_path), None, None)
}

fn parse_integer_segment(value: &str) -> Option<u32> {
    value.parse().ok()
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while let Some(&byte) = bytes.get(index) {
        if byte == b'%' && index + 2 < bytes.len() {
            if let Some(decoded_byte) = bytes
                .get(index + 1..index + 3)
                .and_then(|slice| std::str::from_utf8(slice).ok())
                .and_then(|slice| u8::from_str_radix(slice, 16).ok())
            {
                decoded.push(decoded_byte);
                index += 3;
                continue;
            }
        }

        decoded.push(byte);
        index += 1;
    }

    String::from_utf8_lossy(&decoded).into_owned()
}

fn trim_windows_file_url_prefix(pathname: &str) -> String {
    pathname
        .strip_prefix('/')
        .filter(|rest| {
            rest.len() >= 2
                && rest.as_bytes()[0].is_ascii_alphabetic()
                && rest.as_bytes()[1] == b':'
        })
        .map_or_else(|| pathname.to_owned(), |rest| rest.to_owned())
}
