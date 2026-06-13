#[path = "unit/expected_place.rs"]
mod expected_place;
#[path = "unit/missing_device.rs"]
mod missing_device;
#[path = "unit/nearby_place.rs"]
mod nearby_place;
#[path = "unit/retention_settings.rs"]
mod retention_settings;

#[test]
fn declares_tracking_core_boundary() {
    assert_eq!(ocentra_tracking_core::CRATE_NAME, "ocentra-tracking-core");
    assert_eq!(
        ocentra_tracking_core::evidence_crate_name(),
        "ocentra-evidence"
    );
}
