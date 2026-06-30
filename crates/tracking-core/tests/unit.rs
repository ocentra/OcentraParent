#[path = "unit/alerting.rs"]
mod alerting;
#[path = "unit/expected_place.rs"]
mod expected_place;
#[path = "unit/generated_bridge_contracts.rs"]
mod generated_bridge_contracts;
#[path = "unit/geofence.rs"]
mod geofence;
#[path = "unit/local_place_store.rs"]
mod local_place_store;
#[path = "unit/missing_device.rs"]
mod missing_device;
#[path = "unit/nearby_place.rs"]
mod nearby_place;
#[path = "unit/read_model.rs"]
mod read_model;
#[path = "unit/retention_runtime.rs"]
mod retention_runtime;
#[path = "unit/retention_settings.rs"]
mod retention_settings;
#[path = "unit/runtime_side_branch.rs"]
mod runtime_side_branch;
#[path = "unit/status.rs"]
mod status;

#[test]
fn declares_tracking_core_boundary() {
    assert_eq!(ocentra_tracking_core::CRATE_NAME, "ocentra-tracking-core");
    assert_eq!(
        ocentra_tracking_core::evidence_crate_name(),
        "ocentra-evidence"
    );
}
