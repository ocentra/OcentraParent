use super::app_game_preview_source_freshness::APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID;

const APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID_TOKEN: &str =
    "__APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID__";

const VALUES_TEMPLATE: &str = include_str!("app_game_preview_source_freshness_values.template.txt");

const RULES_TEMPLATES: [&str; 24] = [
    include_str!("app_game_preview_source_freshness_rules.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-01.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-02.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-03.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-04.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-05.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-06.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-07.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-08.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-09.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-10.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-11.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-12.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-13.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-14.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-15.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-16.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-17.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-18.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-19.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-20.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-21.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-22.template.txt"),
    include_str!("app_game_preview_source_freshness_rules.part-23.template.txt"),
];

const DATA_TEMPLATE: &str = include_str!("app_game_preview_source_freshness_data.template.txt");

pub fn app_game_preview_source_freshness_values_typescript() -> String {
    VALUES_TEMPLATE.replace(
        APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID_TOKEN,
        APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID,
    )
}

pub fn app_game_preview_source_freshness_rules_typescript() -> String {
    RULES_TEMPLATES.concat()
}

pub fn app_game_preview_source_freshness_data_typescript() -> String {
    DATA_TEMPLATE.to_string()
}
