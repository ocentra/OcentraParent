mod unit {
    mod retention_settings;
}

#[test]
fn declares_tracking_core_boundary() {
    assert_eq!(ocentra_tracking_core::CRATE_NAME, "ocentra-tracking-core");
    assert_eq!(
        ocentra_tracking_core::evidence_crate_name(),
        "ocentra-evidence"
    );
}
