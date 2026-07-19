const AGENT_PROTOCOL_SOURCE: &str =
    include_str!("../../../agent-protocol/src/lan_pairing_browser_add_device_state.rs");
const PARENT_BRIDGE: &str = include_str!("../../../../apps/portal/generated/parent-ui-bridge.ts");
const PORTAL_CONTRACTS: &str =
    include_str!("../../../../packages/portal-domain/src/generated-portal-contracts.ts");

#[test]
fn generated_lan_discovery_event_kinds_cover_the_authoritative_rust_enum() {
    for (variant, encoded) in lan_discovery_event_kinds() {
        assert!(
            AGENT_PROTOCOL_SOURCE.contains(&format!("    {variant},")),
            "LanDiscoveryEventKind omits {variant}"
        );
        assert!(
            PARENT_BRIDGE.contains(&format!("{variant}: \"{encoded}\"")),
            "ParentAgentLanDiscoveryEventKind omits {variant}"
        );
        assert!(
            PORTAL_CONTRACTS.contains(&format!("{variant}: \"{encoded}\"")),
            "GeneratedPortalAgentLanDiscoveryEventKind omits {variant}"
        );
    }
}

fn lan_discovery_event_kinds() -> [(&'static str, &'static str); 11] {
    [
        ("InterfaceChanged", "interface-changed"),
        ("ScanStarted", "scan-started"),
        ("ScanFinished", "scan-finished"),
        ("EvidenceFound", "evidence-found"),
        ("DeviceFound", "device-found"),
        ("DeviceUpdated", "device-updated"),
        ("DeviceOnline", "device-online"),
        ("DeviceOffline", "device-offline"),
        ("AgentDiscovered", "agent-discovered"),
        ("AgentConfirmed", "agent-confirmed"),
        ("UnknownDetected", "unknown-detected"),
    ]
}
