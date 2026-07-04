const VALUES_TEMPLATE: &str = include_str!("app_game_timer_service_readiness_values.template.txt");

const RULES_TEMPLATE: &str = include_str!("app_game_timer_service_readiness_rules.template.txt");

pub fn app_game_timer_service_readiness_values_typescript() -> String {
    VALUES_TEMPLATE.to_string()
}

pub fn app_game_timer_service_readiness_rules_typescript() -> String {
    RULES_TEMPLATE.to_string()
}
