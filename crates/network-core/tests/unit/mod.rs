mod network_control_catalog;
mod network_flow;
mod runtime_flow;

#[test]
fn declares_network_core_boundary() {
    assert_eq!(
        ocentra_network_core::network_runtime::CRATE_NAME,
        "ocentra-network-core"
    );
}
