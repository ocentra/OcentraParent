const GENERATED_POLICY_TEMPLATE: &str = include_str!("generated-policy.template.txt");
const GENERATED_ENFORCEMENT_TEMPLATE: &str = include_str!("generated-enforcement.template.txt");

pub fn generated_policy_typescript() -> String {
    GENERATED_POLICY_TEMPLATE.to_string()
}

pub fn generated_enforcement_typescript() -> String {
    GENERATED_ENFORCEMENT_TEMPLATE.to_string()
}
