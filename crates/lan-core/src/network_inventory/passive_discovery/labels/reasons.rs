use super::super::LanPassiveDiscoveryTriggerReason;

pub(super) fn trigger_reason_label(
    trigger_reason: &LanPassiveDiscoveryTriggerReason,
) -> &'static str {
    match trigger_reason {
        LanPassiveDiscoveryTriggerReason::WifiSsidChanged => "wifi-ssid-changed",
        LanPassiveDiscoveryTriggerReason::DefaultGatewayChanged => "default-gateway-changed",
        LanPassiveDiscoveryTriggerReason::IpAddressChanged => "ip-address-changed",
        LanPassiveDiscoveryTriggerReason::InterfaceUp => "interface-up",
        LanPassiveDiscoveryTriggerReason::InterfaceDown => "interface-down",
        LanPassiveDiscoveryTriggerReason::AppResumed => "app-resumed",
        LanPassiveDiscoveryTriggerReason::HeartbeatLost => "heartbeat-lost",
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved => "passive-packet-observed",
    }
}
