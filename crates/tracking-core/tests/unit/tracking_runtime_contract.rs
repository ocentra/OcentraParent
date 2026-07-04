use ocentra_tracking_core::generated_bridge::tracking_runtime_generated_typescript;

#[test]
fn tracking_runtime_contract_stays_rust_owned_and_replaces_schema_domain_owner() {
    let source = tracking_runtime_generated_typescript();

    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/tracking-runtime';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/tracking-evidence';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/tracking-geofence';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/tracking-primitives';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("export interface TrackingGeofenceEvaluationInput {")
            .count(),
        1
    );
    assert_eq!(
        source
            .matches("export interface TrackingExpectedPlaceEvaluationInput {")
            .count(),
        1
    );
    assert_eq!(source.matches("type TrackingReasonCode =").count(), 1);
}
