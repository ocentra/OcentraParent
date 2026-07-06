fn browser_generated_values_template(path: &str) -> &'static str {
    match path {
        "browser_ai_analysis_values.template.txt" => include_str!("browser_ai_analysis_values.template.txt"),
        "browser_control_identifiers.template.txt" => include_str!("browser_control_identifiers.template.txt"),
        "browser_control_manifest.template.txt" => include_str!("browser_control_manifest.template.txt"),
        "browser_schemas.template.txt" => include_str!("browser_schemas.template.txt"),
        "browser_target_schemas.template.txt" => include_str!("browser_target_schemas.template.txt"),
        "browser_unmanaged_process_schemas.template.txt" => include_str!("browser_unmanaged_process_schemas.template.txt"),
        _ => unreachable!("unknown browser generated values template"),
    }
}

pub fn browser_ai_analysis_values_typescript() -> String {
    browser_generated_values_template("browser_ai_analysis_values.template.txt").to_string()
}

pub fn browser_control_identifiers_typescript() -> String {
    browser_generated_values_template("browser_control_identifiers.template.txt").to_string()
}

pub fn browser_control_manifest_typescript() -> String {
    browser_generated_values_template("browser_control_manifest.template.txt").to_string()
}

pub fn browser_schemas_typescript() -> String {
    browser_generated_values_template("browser_schemas.template.txt").to_string()
}

pub fn browser_target_schemas_typescript() -> String {
    browser_generated_values_template("browser_target_schemas.template.txt").to_string()
}

pub fn browser_unmanaged_process_schemas_typescript() -> String {
    browser_generated_values_template("browser_unmanaged_process_schemas.template.txt").to_string()
}
