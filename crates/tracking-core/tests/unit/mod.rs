#[path = "alerting.rs"]
mod alerting;
#[path = "expected_place.rs"]
mod expected_place;
#[path = "generated_bridge_contracts.rs"]
mod generated_bridge_contracts;
#[path = "geofence.rs"]
mod geofence;
#[path = "local_place_store.rs"]
mod local_place_store;
#[path = "missing_device.rs"]
mod missing_device;
#[path = "nearby_place.rs"]
mod nearby_place;
#[path = "read_model.rs"]
mod read_model;
#[path = "retention_runtime.rs"]
mod retention_runtime;
#[path = "retention_settings.rs"]
mod retention_settings;
#[path = "runtime_side_branch.rs"]
mod runtime_side_branch;
#[path = "status.rs"]
mod status;
#[path = "tracking_runtime_contract.rs"]
mod tracking_runtime_contract;

#[test]
fn declares_tracking_core_boundary() {
    assert_eq!(ocentra_tracking_core::CRATE_NAME, "ocentra-tracking-core");
    assert_eq!(
        ocentra_tracking_core::evidence_crate_name(),
        "ocentra-evidence"
    );
}
