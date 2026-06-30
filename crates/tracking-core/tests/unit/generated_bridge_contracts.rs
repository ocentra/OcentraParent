use ocentra_tracking_core::generated_bridge::{
    tracking_control_catalog_generated_typescript,
    tracking_control_catalog_metadata_generated_typescript,
    tracking_control_catalog_schema_generated_typescript,
    tracking_local_place_store_generated_typescript,
    tracking_poi_provider_adapter_generated_typescript,
    tracking_policy_compiler_runtime_proof_generated_typescript,
    tracking_retention_runtime_generated_typescript, tracking_runtime_generated_typescript,
};

#[test]
fn tracking_generated_helpers_remain_rust_owned_and_marked() {
    let generated = [
        (
            tracking_runtime_generated_typescript(),
            "import { ActivityEvidenceKind } from '@ocentra-parent/schema-domain/evidence-kinds';",
            "export function evaluateTrackingGeofenceTransitionGenerated(input: TrackingGeofenceEvaluationInput) {",
        ),
        (
            tracking_retention_runtime_generated_typescript(),
            "export function applyTrackingRetentionDeleteGenerated(",
            "export function applyTrackingRetentionDeleteGenerated(input: {",
        ),
        (
            tracking_local_place_store_generated_typescript(),
            "import { ActivityEvidenceRefSchema } from '@ocentra-parent/schema-domain/evidence-contracts';",
            "export function createTrackingLocalParentDefinedPlaceStoreGenerated(",
        ),
        (
            tracking_poi_provider_adapter_generated_typescript(),
            "import {\n  GooglePlacesNearbySearchResponseSchema,",
            "export function buildGooglePlacesNearbySearchRequestGenerated(input: TrackingGooglePlacesNearbySearchInput) {",
        ),
        (
            tracking_policy_compiler_runtime_proof_generated_typescript(),
            "import {\n  TrackingAlertIntentSchema,",
            "export function compileTrackingPolicyRuntimeProofDecisionGenerated(",
        ),
    ];

    for (source, prefix, signature) in generated {
        assert!(source.starts_with(prefix));
        assert_eq!(
            source
                .lines()
                .find(|line| line.starts_with("export function ")),
            Some(signature)
        );
    }
}

#[test]
fn tracking_control_generated_catalog_surfaces_stay_checked_in() {
    let checked_in = [
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
            include_str!("../../../../packages/schema-domain/src/tracking-control-catalog.ts"),
            tracking_control_catalog_generated_typescript(),
        ),
    ];

    for (checked_in_source, generated_source) in checked_in {
        assert_eq!(checked_in_source, generated_source);
    }
}
