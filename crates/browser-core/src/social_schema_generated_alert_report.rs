use crate::social_schema_generated_values::GeneratedTypescript;

pub fn social_alert_report_intent_typescript() -> GeneratedTypescript {
    GeneratedTypescript::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/schema-domain/src/generated-social-alert-report-intent.ts"
    )))
}

pub fn social_alert_report_provider_preflight_proof_typescript() -> GeneratedTypescript {
    GeneratedTypescript::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/schema-domain/src/generated-social-alert-report-provider-preflight-proof.ts"
    )))
}

pub fn social_alert_report_provider_receipt_boundary_proof_typescript() -> GeneratedTypescript {
    GeneratedTypescript::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/schema-domain/src/generated-social-alert-report-provider-receipt-boundary-proof.ts"
    )))
}

pub fn social_alert_report_provider_status_handoff_proof_typescript() -> GeneratedTypescript {
    GeneratedTypescript::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/schema-domain/src/generated-social-alert-report-provider-status-handoff-proof.ts"
    )))
}
