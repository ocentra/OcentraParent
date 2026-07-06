const POLICY_CONSUMPTION_TEMPLATE: &str =
    include_str!("app_game_source_freshness_policy_consumption.template.txt");

const POLICY_CONSUMPTION_DATA_TEMPLATE: &str =
    include_str!("app_game_source_freshness_policy_consumption_data.template.txt");

pub fn app_game_source_freshness_policy_consumption_typescript() -> String {
    POLICY_CONSUMPTION_TEMPLATE.to_string()
}

pub fn app_game_source_freshness_policy_consumption_data_typescript() -> String {
    POLICY_CONSUMPTION_DATA_TEMPLATE.to_string()
}
