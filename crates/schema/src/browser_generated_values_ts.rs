#[derive(Clone, Copy)]
enum BrowserGeneratedValuesTemplate {
    AiAnalysisValues,
    ControlIdentifiers,
    ControlManifest,
    TargetSchemas,
    UnmanagedProcessSchemas,
}

fn browser_generated_values_template(template: BrowserGeneratedValuesTemplate) -> &'static str {
    match template {
        BrowserGeneratedValuesTemplate::AiAnalysisValues => {
            include_str!("browser_ai_analysis_values.template.txt")
        }
        BrowserGeneratedValuesTemplate::ControlIdentifiers => {
            include_str!("browser_control_identifiers.template.txt")
        }
        BrowserGeneratedValuesTemplate::ControlManifest => {
            include_str!("browser_control_manifest.template.txt")
        }
        BrowserGeneratedValuesTemplate::TargetSchemas => {
            include_str!("browser_target_schemas.template.txt")
        }
        BrowserGeneratedValuesTemplate::UnmanagedProcessSchemas => {
            include_str!("browser_unmanaged_process_schemas.template.txt")
        }
    }
}

pub fn browser_ai_analysis_values_typescript() -> String {
    browser_generated_values_template(BrowserGeneratedValuesTemplate::AiAnalysisValues).to_string()
}

pub fn browser_control_identifiers_typescript() -> String {
    browser_generated_values_template(BrowserGeneratedValuesTemplate::ControlIdentifiers)
        .to_string()
}

pub fn browser_control_manifest_typescript() -> String {
    browser_generated_values_template(BrowserGeneratedValuesTemplate::ControlManifest).to_string()
}

pub fn browser_schemas_typescript() -> String {
    [
        include_str!("browser_schemas.template.txt"),
        include_str!("browser_schemas.consistency.template.txt"),
        include_str!("browser_schemas.url-parsing.template.txt"),
    ]
    .concat()
}

pub fn browser_target_schemas_typescript() -> String {
    browser_generated_values_template(BrowserGeneratedValuesTemplate::TargetSchemas).to_string()
}

pub fn browser_unmanaged_process_schemas_typescript() -> String {
    browser_generated_values_template(BrowserGeneratedValuesTemplate::UnmanagedProcessSchemas)
        .to_string()
}
