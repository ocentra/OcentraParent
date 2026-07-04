use super::app_game_preview_source_freshness::APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID;

const APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID_TOKEN: &str =
    "__APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID__";

const VALUES_TEMPLATE: &str = include_str!("app_game_preview_source_freshness_values.template.txt");

const RULES_TEMPLATE: &str = include_str!("app_game_preview_source_freshness_rules.template.txt");

const DATA_TEMPLATE: &str = include_str!("app_game_preview_source_freshness_data.template.txt");

pub fn app_game_preview_source_freshness_values_typescript() -> String {
    VALUES_TEMPLATE.replace(
        APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID_TOKEN,
        APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID,
    )
}

pub fn app_game_preview_source_freshness_rules_typescript() -> String {
    RULES_TEMPLATE.to_string()
}

pub fn app_game_preview_source_freshness_data_typescript() -> String {
    DATA_TEMPLATE.to_string()
}
