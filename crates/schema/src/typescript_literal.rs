mod conversion;
mod formatting;

const LINE_BREAK: &str = "\n";
const PRINT_WIDTH: usize = 120;

pub fn json_object_to_typescript_literal(json: &str) -> String {
    formatting::wrap_long_typescript_lines(&formatting::json_to_typescript_literal(json, true))
}

pub(crate) fn json_array_to_typescript_literal(json: &str) -> String {
    let compact = formatting::json_to_typescript_literal(json, true);
    let output = if json.trim_start().starts_with('[') && compact.lines().count() == 1 {
        formatting::json_to_typescript_literal(json, false)
    } else {
        compact
    };
    formatting::wrap_long_typescript_lines(&output)
}
