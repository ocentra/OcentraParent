#[test]
fn crate_name_identifies_agent_core_boundary() {
    assert_eq!(
        ocentra_parent_agent_core::crate_name(),
        env!("CARGO_PKG_NAME")
    );
}
