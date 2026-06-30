use super::{
    LanPassiveDiscoveryEventKind, LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};

pub fn passive_event_id(
    event_kind: &LanPassiveDiscoveryEventKind,
    source: Option<&LanPassiveDiscoverySource>,
    trigger_reason: &LanPassiveDiscoveryTriggerReason,
    observed_at: &str,
    device_id: Option<&str>,
    scan_session_id: Option<&str>,
) -> String {
    let mut parts = vec![
        String::from("lan-passive"),
        compact_identifier(event_kind_label(event_kind)),
        compact_identifier(trigger_reason_label(trigger_reason)),
        compact_identifier(observed_at),
    ];
    if let Some(source) = source {
        parts.push(compact_identifier(passive_source_label(source)));
    }
    if let Some(device_id) = device_id {
        parts.push(compact_identifier(device_id));
    }
    if let Some(scan_session_id) = scan_session_id {
        parts.push(compact_identifier(scan_session_id));
    }
    parts.join("-")
}

pub fn event_kind_label(event_kind: &LanPassiveDiscoveryEventKind) -> &'static str {
    match event_kind {
        LanPassiveDiscoveryEventKind::PassiveUpdate => "update",
        LanPassiveDiscoveryEventKind::RescanTrigger => "trigger",
    }
}

pub fn trigger_reason_label(trigger_reason: &LanPassiveDiscoveryTriggerReason) -> &'static str {
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

pub fn passive_source_label(source: &LanPassiveDiscoverySource) -> &'static str {
    match source {
        LanPassiveDiscoverySource::Arp => "arp",
        LanPassiveDiscoverySource::Dhcp => "dhcp",
        LanPassiveDiscoverySource::Mdns => "mdns",
        LanPassiveDiscoverySource::Ssdp => "ssdp",
        LanPassiveDiscoverySource::WsDiscovery => "ws-discovery",
        LanPassiveDiscoverySource::Llmnr => "llmnr",
        LanPassiveDiscoverySource::Netbios => "netbios",
        LanPassiveDiscoverySource::OcentraBeacon => "ocentra-beacon",
        LanPassiveDiscoverySource::AllowedSnmpResponse => "allowed-snmp-response",
    }
}

pub fn compact_identifier(value: &str) -> String {
    let compacted = value
        .chars()
        .map(|character| match character {
            'a'..='z' | '0'..='9' | '-' | '_' => character,
            'A'..='Z' => character.to_ascii_lowercase(),
            _ => '-',
        })
        .collect::<String>();
    compacted.trim_matches('-').to_string()
}
