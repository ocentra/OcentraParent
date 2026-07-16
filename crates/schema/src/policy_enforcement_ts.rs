const GENERATED_POLICY_TEMPLATE: &str = include_str!("generated-policy.template.txt");

fn generated_enforcement_template() -> String {
    [
        include_str!("generated-enforcement.imports.template.txt"),
        include_str!("generated-enforcement.contracts.template.txt"),
        include_str!("generated-enforcement.boundaries.template.txt"),
        include_str!("generated-enforcement.constants.template.txt"),
    ]
    .concat()
}

pub fn generated_policy_typescript() -> String {
    GENERATED_POLICY_TEMPLATE.to_string()
}

pub fn generated_enforcement_typescript() -> String {
    generated_enforcement_template()
}
