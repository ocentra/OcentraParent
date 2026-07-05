use super::app_risk_detection::{
    sample_app_risk_detection_matrix, APP_RISK_DETECTION_SCHEMA_VERSION,
};

const APP_RISK_DETECTION_MATRIX_JSON_EXPECT_MESSAGE: &str = "app risk detection matrix json";
const APP_RISK_DETECTION_INDENT_UNIT: &str = " ";
const APP_RISK_DETECTION_JSON_KEY_VALUE_SEPARATOR: &str = "\": ";
const APP_RISK_DETECTION_TYPESCRIPT_LINE_BREAK: &str = "\n";
const APP_RISK_DETECTION_JSON_DOUBLE_QUOTE: char = '"';
const APP_RISK_DETECTION_TYPESCRIPT_SINGLE_QUOTE: char = '\'';
const APP_RISK_DETECTION_SCHEMA_VERSION_TOKEN: &str = "__APP_RISK_DETECTION_SCHEMA_VERSION__";
const APP_RISK_DETECTION_MATRIX_JSON_TOKEN: &str = "__APP_RISK_DETECTION_MATRIX_JSON__";
const APP_RISK_DETECTION_CONTRACTS_TEMPLATE: &str =
    include_str!("app_risk_detection_contracts.template.txt");
const APP_RISK_DETECTION_RULES_TEMPLATE: &str =
    include_str!("app_risk_detection_rules.template.txt");

pub fn app_risk_detection_contracts_typescript() -> String {
    let matrix_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_app_risk_detection_matrix()),
        APP_RISK_DETECTION_MATRIX_JSON_EXPECT_MESSAGE,
    );
    let matrix_typescript = json_object_to_typescript_literal(&matrix_json);

    APP_RISK_DETECTION_CONTRACTS_TEMPLATE
        .replace(
            APP_RISK_DETECTION_SCHEMA_VERSION_TOKEN,
            APP_RISK_DETECTION_SCHEMA_VERSION,
        )
        .replace(APP_RISK_DETECTION_MATRIX_JSON_TOKEN, &matrix_typescript)
        .replace("{{", "{")
        .replace("}}", "}")
}

pub fn app_risk_detection_contract_rules_typescript() -> String {
    APP_RISK_DETECTION_RULES_TEMPLATE.to_owned()
}

fn json_object_to_typescript_literal(json: &str) -> String {
    json.lines()
        .map(|line| {
            let trimmed = line.trim_start();
            if let Some(key_and_rest) = trimmed.strip_prefix(APP_RISK_DETECTION_JSON_DOUBLE_QUOTE) {
                if let Some((key, rest)) =
                    key_and_rest.split_once(APP_RISK_DETECTION_JSON_KEY_VALUE_SEPARATOR)
                {
                    let indent = APP_RISK_DETECTION_INDENT_UNIT.repeat(line.len() - trimmed.len());
                    return format!("{indent}{key}: {rest}");
                }
            }
            line.to_owned()
        })
        .collect::<Vec<_>>()
        .join(APP_RISK_DETECTION_TYPESCRIPT_LINE_BREAK)
        .chars()
        .map(|ch| {
            if ch == APP_RISK_DETECTION_JSON_DOUBLE_QUOTE {
                APP_RISK_DETECTION_TYPESCRIPT_SINGLE_QUOTE
            } else {
                ch
            }
        })
        .collect()
}
