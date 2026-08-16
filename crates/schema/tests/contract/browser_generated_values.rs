use ocentra_schema::browser_generated_values_ts::{
    browser_ai_analysis_values_typescript, browser_control_identifiers_typescript,
    browser_control_manifest_typescript, browser_schemas_typescript,
    browser_target_schemas_typescript, browser_unmanaged_process_schemas_typescript,
};

#[test]
fn browser_generated_values_typescript_stays_checked_in() {
    assert_eq!(
        include_str!(
            "../../../../packages/schema-domain/src/generated-browser-ai-analysis-values.ts"
        ),
        browser_ai_analysis_values_typescript()
    );
    assert_eq!(
        include_str!(
            "../../../../packages/schema-domain/src/generated-browser-control-identifiers.ts"
        ),
        browser_control_identifiers_typescript()
    );
    assert_eq!(
        include_str!(
            "../../../../packages/schema-domain/src/generated-browser-control-manifest.ts"
        ),
        browser_control_manifest_typescript()
    );
    assert_eq!(
        include_str!("../../../../packages/schema-domain/src/generated-browser-schemas.ts"),
        browser_schemas_typescript()
    );
    assert_eq!(
        include_str!("../../../../packages/schema-domain/src/generated-browser-target-schemas.ts"),
        browser_target_schemas_typescript()
    );
    assert_eq!(
        include_str!(
            "../../../../packages/schema-domain/src/generated-browser-unmanaged-process-schemas.ts"
        ),
        browser_unmanaged_process_schemas_typescript()
    );
}
