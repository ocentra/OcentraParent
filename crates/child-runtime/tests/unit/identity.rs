#[test]
fn child_runtime_declares_tracking_core_dependency() {
    assert_eq!(ocentra_child_runtime::CRATE_NAME, "ocentra-child-runtime");
}
