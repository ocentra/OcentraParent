const VALUES_TEMPLATE: &str = include_str!("app_game_timer_service_readiness_values.template.txt");

const RULES_TEMPLATES: [&str; 29] = [
    include_str!("app_game_timer_service_readiness_rules.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-01.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-02.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-03.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-04.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-05.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-06.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-07.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-08.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-09.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-10.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-11.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-12.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-13.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-14.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-15.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-16.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-17.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-18.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-19.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-20.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-21.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-22.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-23.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-24.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-25.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-26.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-27.template.txt"),
    include_str!("app_game_timer_service_readiness_rules.part-28.template.txt"),
];

pub fn app_game_timer_service_readiness_values_typescript() -> String {
    VALUES_TEMPLATE.to_string()
}

pub fn app_game_timer_service_readiness_rules_typescript() -> String {
    RULES_TEMPLATES.concat()
}
