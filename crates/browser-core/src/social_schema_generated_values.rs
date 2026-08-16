#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeneratedTypescript(&'static str);

impl GeneratedTypescript {
    pub(crate) const fn new(value: &'static str) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for GeneratedTypescript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

pub fn social_alert_report_intent_values_typescript() -> GeneratedTypescript {
    GeneratedTypescript::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/schema-domain/src/generated-social-alert-report-intent-values.ts"
    )))
}

pub fn social_audit_explanation_read_model_values_typescript() -> GeneratedTypescript {
    GeneratedTypescript::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/schema-domain/src/generated-social-audit-explanation-read-model-values.ts"
    )))
}

pub fn social_dashboard_ux_values_typescript() -> GeneratedTypescript {
    GeneratedTypescript::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/schema-domain/src/generated-social-dashboard-ux-values.ts"
    )))
}

pub fn social_policy_compiler_values_typescript() -> GeneratedTypescript {
    GeneratedTypescript::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/schema-domain/src/generated-social-policy-compiler-values.ts"
    )))
}
