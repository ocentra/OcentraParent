pub fn browser_social_feed_route_classification_typescript() -> String {
    include_str!("browser_social_feed_route_classification.template.txt").to_string()
}

pub fn browser_social_feed_video_route_gate_typescript() -> String {
    [
        include_str!("browser_social_feed_video_route_gate.template.txt"),
        "\n",
        include_str!("browser_social_feed_video_route_gate/guards.template.txt"),
    ]
    .concat()
}

pub fn browser_social_feed_video_route_gate_guards_typescript() -> String {
    include_str!("browser_social_feed_video_route_gate_guards.template.txt").to_string()
}

pub fn browser_social_feed_video_route_gate_values_typescript() -> String {
    include_str!("browser_social_feed_video_route_gate_values.template.txt").to_string()
}

pub fn browser_social_platform_route_schemas_typescript() -> String {
    include_str!("browser_social_platform_route_schemas.template.txt").to_string()
}

pub fn browser_social_video_metadata_typescript() -> String {
    include_str!("browser_social_video_metadata.template.txt").to_string()
}

pub fn browser_url_intelligence_rules_typescript() -> String {
    include_str!("browser_url_intelligence_rules.template.txt").to_string()
}

pub fn browser_url_intelligence_schemas_typescript() -> String {
    include_str!("browser_url_intelligence_schemas.template.txt").to_string()
}
