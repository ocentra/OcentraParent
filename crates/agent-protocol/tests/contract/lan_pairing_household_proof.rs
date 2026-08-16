use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdProofReadModel;

use super::lan_pairing::lan_pairing_helpers::{
    json_surface_contains_marker, read_model, JsonSurfaceMarker,
};

#[test]
fn v09_production_discovery_household_proof_read_model_serializes_honest_route_states(
) -> serde_json::Result<()> {
    let serialized = serde_json::to_value(read_model())?;
    let reparsed = serde_json::from_value::<V09ProductionDiscoveryHouseholdProofReadModel>(
        serialized.clone(),
    )?;
    assert_honest_route_states(&serialized, &reparsed);
    assert_no_raw_markers(&serialized);

    Ok(())
}

fn assert_honest_route_states(
    serialized: &serde_json::Value,
    reparsed: &V09ProductionDiscoveryHouseholdProofReadModel,
) {
    assert_eq!(
        serialized["proofBoundary"],
        "local-real-service-not-physical-household-lan"
    );
    assert_eq!(
        serialized["productReadinessDecision"],
        "not-ready-for-product-ready-household-lan-claim"
    );
    assert_eq!(
        serialized["routeChecks"][1]["sourceState"],
        "failed-unpaired"
    );
    assert_eq!(
        serialized["routeChecks"][2]["rejectionReason"],
        constants::value::LAN_REASON_WRONG_ORIGIN
    );
    assert_eq!(
        serialized["routeChecks"][3]["rejectionReason"],
        constants::value::LAN_REASON_WRONG_DEVICE
    );
    assert_eq!(
        serialized["restartRecovery"][0]["routeRecoveryState"],
        "registry-restored-after-restart"
    );
    assert_eq!(
        serialized["sourceDeviceStates"][3]["proofState"],
        "manual-required"
    );
    assert_eq!(reparsed.manual_household_proof_checklist.len(), 11);
}

fn assert_no_raw_markers(serialized: &serde_json::Value) {
    assert!(!json_surface_contains_marker(
        serialized,
        JsonSurfaceMarker::RawEvidence,
    ));
    assert!(!json_surface_contains_marker(
        serialized,
        JsonSurfaceMarker::RawToken,
    ));
    assert!(!json_surface_contains_marker(
        serialized,
        JsonSurfaceMarker::ActivitySqlite,
    ));
}

#[test]
fn v09_production_discovery_household_proof_read_model_rejects_missing_required_fields() {
    let result = serde_json::from_value::<V09ProductionDiscoveryHouseholdProofReadModel>(
        serde_json::json!({
            "schemaVersion": constants::lan_pairing::SCHEMA_VERSION_TEXT
        }),
    );

    assert_eq!(
        result.err().as_ref().map(serde_json::Error::classify),
        Some(serde_json::error::Category::Data)
    );
}
