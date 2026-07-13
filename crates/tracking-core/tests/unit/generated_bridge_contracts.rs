use ocentra_tracking_core::generated_bridge::{
    tracking_control_catalog_build_generated_typescript,
    tracking_control_catalog_data_generated_typescript,
    tracking_control_catalog_generated_typescript,
    tracking_control_catalog_metadata_classifiers_generated_typescript,
    tracking_control_catalog_metadata_generated_typescript,
    tracking_control_catalog_metadata_layout_generated_typescript,
    tracking_control_catalog_options_generated_typescript,
    tracking_control_catalog_schema_generated_typescript, tracking_runtime_generated_typescript,
};

#[test]
fn tracking_generated_helpers_remain_rust_owned_and_marked() {
    let source = tracking_runtime_generated_typescript();
    assert!(source.starts_with(
        "import { ActivityEvidenceKind } from '@ocentra-parent/schema-domain/evidence-kinds';"
    ));
    assert!(source.contains(
        "export function evaluateTrackingGeofenceTransitionGenerated(input: TrackingGeofenceEvaluationInput) {"
    ));
    assert!(source.contains(
        "export function evaluateTrackingExpectedPlaceDecisionGenerated(input: TrackingExpectedPlaceEvaluationInput) {"
    ));
}

#[test]
fn tracking_control_generated_catalog_surfaces_stay_checked_in() {
    let checked_in = [
        (
            include_str!(
                "../../../../packages/schema-domain/src/tracking-control-catalog-build.ts"
            ),
            tracking_control_catalog_build_generated_typescript(),
        ),
        (
            include_str!("../../../../packages/schema-domain/src/tracking-control-catalog-data.ts"),
            tracking_control_catalog_data_generated_typescript(),
        ),
        (
            include_str!(
                "../../../../packages/schema-domain/src/tracking-control-catalog-schema.ts"
            ),
            tracking_control_catalog_schema_generated_typescript(),
        ),
        (
            include_str!(
                "../../../../packages/schema-domain/src/tracking-control-catalog-metadata.ts"
            ),
            tracking_control_catalog_metadata_generated_typescript(),
        ),
        (
            include_str!(
                "../../../../packages/schema-domain/src/tracking-control-catalog-metadata-classifiers.ts"
            ),
            tracking_control_catalog_metadata_classifiers_generated_typescript(),
        ),
        (
            include_str!(
                "../../../../packages/schema-domain/src/tracking-control-catalog-metadata-layout.ts"
            ),
            tracking_control_catalog_metadata_layout_generated_typescript(),
        ),
        (
            include_str!(
                "../../../../packages/schema-domain/src/tracking-control-catalog-options.ts"
            ),
            tracking_control_catalog_options_generated_typescript(),
        ),
        (
            include_str!("../../../../packages/schema-domain/src/tracking-control-catalog.ts"),
            tracking_control_catalog_generated_typescript(),
        ),
    ];

    for (checked_in_source, generated_source) in checked_in {
        assert_eq!(checked_in_source, generated_source);
    }
}
