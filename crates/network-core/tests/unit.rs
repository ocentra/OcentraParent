#[path = "unit/generated_bridge_contracts.rs"]
mod generated_bridge_contracts;
#[path = "unit/network_control_catalog.rs"]
mod network_control_catalog;
#[path = "unit/network_flow.rs"]
mod network_flow;
#[path = "unit/runtime_flow.rs"]
mod runtime_flow;

#[test]
fn declares_network_core_boundary() {
    assert_eq!(
        ocentra_network_core::network_runtime::CRATE_NAME,
        "ocentra-network-core"
    );
}
