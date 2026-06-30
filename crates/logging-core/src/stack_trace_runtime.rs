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
    if let Some(rest) = value.strip_prefix("file://") {
        return normalize_path(&trim_windows_file_url_prefix(&percent_decode(rest)));
    }
    normalize_path(value)
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
    let mut segments = Vec::new();
    let mut current = String::new();
    for character in stem.chars() {
        if character.is_ascii_alphanumeric() {
            current.push(character);
            continue;
        }
        if !current.is_empty() {
            segments.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        segments.push(current);
    }

    segments
        .into_iter()
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => {
                    let mut title = String::new();
                    title.push(first.to_ascii_uppercase());
                    title.push_str(chars.as_str());
                    title
                }
                None => String::new(),
            }
        })
        .collect()
}

pub fn resolve_logger_context(
    module_name: &str,
    frame: Option<&StackFrame>,
    module_context_suffix: &str,
) -> String {
    if let Some(function_name) = frame
        .and_then(|frame| frame.function_name.as_deref())
        .map(str::trim)
        .filter(|function_name| !function_name.is_empty())
    {
        if function_name.contains('.') {
            return function_name.to_owned();
        }
        return format!("{module_name}.{function_name}");
    }

    format!("{module_name}.{module_context_suffix}")
}

pub fn resolve_logger_source(module_name: &str, frame: Option<&StackFrame>) -> String {
    if let Some(function_name) = frame
        .and_then(|frame| frame.function_name.as_deref())
        .filter(|function_name| function_name.contains('.'))
    {
        return function_name
            .split('.')
            .next()
            .unwrap_or(module_name)
            .to_owned();
    }

    module_name.to_owned()
}

pub fn parse_stack_trace(stack_trace: &str) -> Vec<StackFrame> {
    stack_trace
        .lines()
        .filter_map(|line| parse_frame_line(line.trim_end_matches('\r')))
        .collect()
}

pub fn stack_trace_runtime_typescript() -> &'static str {
    STACK_TRACE_RUNTIME_TYPESCRIPT
}

fn parse_frame_line(line: &str) -> Option<StackFrame> {
    let trimmed = line.trim();
    if !trimmed.starts_with("at ") {
        return None;
    }

    let body = &trimmed[3..];
    if let Some(location_open) = body.find(" (") {
        if body.ends_with(')') {
            let function_name = body[..location_open].trim();
            let location = &body[(location_open + 2)..(body.len() - 1)];
            let (file, file_path, line, column) = parse_location(location);
            return Some(StackFrame {
                function_name: (!function_name.is_empty()).then(|| function_name.to_owned()),
                file,
                file_path,
                line,
                column,
            });
        }
    }

    let (file, file_path, line, column) = parse_location(body);
    Some(StackFrame {
        function_name: None,
        file,
        file_path,
        line,
        column,
    })
}

fn parse_location(location: &str) -> (Option<String>, Option<String>, Option<u32>, Option<u32>) {
    let trimmed = location.trim();
    let Some(column_separator) = trimmed.rfind(':') else {
        return parse_location_without_line_info(trimmed);
    };
    let Some(line_separator) = trimmed[..column_separator].rfind(':') else {
        return parse_location_without_line_info(trimmed);
    };

    let Some(line) = parse_integer_segment(trimmed, line_separator + 1, column_separator) else {
        return parse_location_without_line_info(trimmed);
    };
    let Some(column) = parse_integer_segment(trimmed, column_separator + 1, trimmed.len()) else {
        return parse_location_without_line_info(trimmed);
    };

    let file_path = decode_file_path(&trimmed[..line_separator]);
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

fn parse_integer_segment(value: &str, start: usize, end: usize) -> Option<u32> {
    if start >= end {
        return None;
    }

    let mut parsed = 0u32;
    for byte in value.as_bytes().iter().skip(start).take(end - start) {
        if !byte.is_ascii_digit() {
            return None;
        }
        parsed = parsed.checked_mul(10)?.checked_add((byte - b'0') as u32)?;
    }
    Some(parsed)
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let (Some(high), Some(low)) =
                (hex_value(bytes[index + 1]), hex_value(bytes[index + 2]))
            {
                decoded.push(high * 16 + low);
                index += 3;
                continue;
            }
        }

        decoded.push(bytes[index]);
        index += 1;
    }

    String::from_utf8_lossy(&decoded).into_owned()
}

fn trim_windows_file_url_prefix(pathname: &str) -> String {
    let bytes = pathname.as_bytes();
    if bytes.len() >= 3 && bytes[0] == b'/' && bytes[1].is_ascii_alphabetic() && bytes[2] == b':' {
        pathname[1..].to_owned()
    } else {
        pathname.to_owned()
    }
}

fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

const STACK_TRACE_RUNTIME_TYPESCRIPT: &str = r#"/* generated from crates/logging-core/src/stack_trace_runtime.rs */

export interface GeneratedStackFrame {
  readonly functionName: string | null;
  readonly file: string | null;
  readonly filePath: string | null;
  readonly line: number | null;
  readonly column: number | null;
}

export function normalizeGeneratedStackPath(value: string): string {
  return value.replaceAll('\\', '/');
}

function generatedPercentDecode(value: string): string {
  try {
    return decodeURIComponent(value);
  } catch {
    return value;
  }
}

function generatedTrimWindowsFileUrlPrefix(pathname: string): string {
  if (pathname.length >= 3 && pathname[0] === '/' && /[A-Za-z]/.test(pathname[1] ?? '') && pathname[2] === ':') {
    return pathname.slice(1);
  }
  return pathname;
}

export function decodeGeneratedStackFilePath(value: string): string {
  if (value.startsWith('file://')) {
    return normalizeGeneratedStackPath(generatedTrimWindowsFileUrlPrefix(generatedPercentDecode(value.slice('file://'.length))));
  }
  return normalizeGeneratedStackPath(value);
}

export function fileNameFromGeneratedPath(filePath: string | null): string | null {
  if (filePath == null) {
    return null;
  }
  const normalized = normalizeGeneratedStackPath(filePath);
  const lastSlash = normalized.lastIndexOf('/');
  return lastSlash >= 0 ? normalized.slice(lastSlash + 1) : normalized;
}

export function moduleNameFromGeneratedPath(filePath: string): string {
  const fileName = (fileNameFromGeneratedPath(filePath) ?? '').replace(/\.[^.]+$/, '');
  return fileName
    .split(/[^a-zA-Z0-9]+/)
    .filter((segment) => segment.length > 0)
    .map((segment) => segment.slice(0, 1).toUpperCase() + segment.slice(1))
    .join('');
}

function parseGeneratedIntegerSegment(value: string, start: number, end: number): number | null {
  if (start >= end) {
    return null;
  }

  let parsed = 0;
  for (let index = start; index < end; index += 1) {
    const digit = value.charCodeAt(index) - 48;
    if (digit < 0 || digit > 9) {
      return null;
    }
    parsed = parsed * 10 + digit;
  }
  return parsed;
}

function parseGeneratedLocationWithoutLineInfo(location: string): Omit<GeneratedStackFrame, 'functionName'> {
  const filePath = decodeGeneratedStackFilePath(location);
  return {
    file: fileNameFromGeneratedPath(filePath),
    filePath,
    line: null,
    column: null,
  };
}

function parseGeneratedLocation(location: string): Omit<GeneratedStackFrame, 'functionName'> {
  const trimmed = location.trim();
  const columnSeparator = trimmed.lastIndexOf(':');
  if (columnSeparator < 0) {
    return parseGeneratedLocationWithoutLineInfo(trimmed);
  }

  const lineSeparator = trimmed.lastIndexOf(':', columnSeparator - 1);
  if (lineSeparator < 0) {
    return parseGeneratedLocationWithoutLineInfo(trimmed);
  }

  const line = parseGeneratedIntegerSegment(trimmed, lineSeparator + 1, columnSeparator);
  const column = parseGeneratedIntegerSegment(trimmed, columnSeparator + 1, trimmed.length);
  if (line == null || column == null) {
    return parseGeneratedLocationWithoutLineInfo(trimmed);
  }

  const filePath = decodeGeneratedStackFilePath(trimmed.slice(0, lineSeparator));
  return {
    file: fileNameFromGeneratedPath(filePath),
    filePath,
    line,
    column,
  };
}

function parseGeneratedFrameLine(line: string): GeneratedStackFrame | null {
  const trimmed = line.trim();
  if (!trimmed.startsWith('at ')) {
    return null;
  }

  const body = trimmed.slice(3);
  const locationOpen = body.indexOf(' (');
  if (locationOpen >= 0 && body.endsWith(')')) {
    const functionName = body.slice(0, locationOpen).trim();
    const location = body.slice(locationOpen + 2, body.length - 1);
    return {
      functionName: functionName.length > 0 ? functionName : null,
      ...parseGeneratedLocation(location),
    };
  }

  return {
    functionName: null,
    ...parseGeneratedLocation(body),
  };
}

export function parseGeneratedStackTrace(stackTrace: string): GeneratedStackFrame[] {
  return String(stackTrace)
    .split('\n')
    .map((line) => parseGeneratedFrameLine(line.endsWith('\r') ? line.slice(0, -1) : line))
    .filter((frame): frame is GeneratedStackFrame => frame != null);
}

export function resolveGeneratedLoggerContext(
  moduleName: string,
  frame: GeneratedStackFrame | null,
  moduleContextSuffix: string
): string {
  if (frame?.functionName != null && frame.functionName.trim().length > 0) {
    return frame.functionName.includes('.') ? frame.functionName : `${moduleName}.${frame.functionName}`;
  }
  return `${moduleName}.${moduleContextSuffix}`;
}

export function resolveGeneratedLoggerSource(moduleName: string, frame: GeneratedStackFrame | null): string {
  if (frame?.functionName != null && frame.functionName.includes('.')) {
    return frame.functionName.split('.')[0] ?? moduleName;
  }
  return moduleName;
}
"#;
