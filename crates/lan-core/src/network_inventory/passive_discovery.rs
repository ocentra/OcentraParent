use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

pub mod collection;
pub mod dhcp;
pub mod dns_like;
pub mod labels;
pub mod packet;
pub mod raw_socket;
pub mod snmp;
pub mod ssdp;
mod state;
pub mod summaries;
pub mod text;
pub mod udp_multicast;
pub mod ws_discovery;
pub mod xml;

pub const LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES: usize = 2048;
pub const LAN_PASSIVE_DISCOVERY_MAX_SUMMARY_BYTES: usize = 512;

macro_rules! passive_discovery_text_id {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

passive_discovery_text_id!(LanPassiveDiscoveryDeviceId);
passive_discovery_text_id!(LanPassiveDiscoveryEventId);
passive_discovery_text_id!(LanPassiveDiscoveryScanSessionId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPassiveDiscoverySource {
    Arp,
    Dhcp,
    Mdns,
    Ssdp,
    WsDiscovery,
    Llmnr,
    Netbios,
    OcentraBeacon,
    AllowedSnmpResponse,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPassiveDiscoveryTriggerReason {
    WifiSsidChanged,
    DefaultGatewayChanged,
    IpAddressChanged,
    InterfaceUp,
    InterfaceDown,
    AppResumed,
    HeartbeatLost,
    PassivePacketObserved,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPassiveDiscoveryListenerLifecycleState {
    Running,
    Stopped,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPassiveDiscoveryEventKind {
    PassiveUpdate,
    RescanTrigger,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPassiveDiscoveryRecordOutcome {
    Recorded,
    Deduplicated,
    Stopped,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPassiveDiscoveryPacket {
    pub schema_version: u16,
    pub source: LanPassiveDiscoverySource,
    pub trigger_reason: LanPassiveDiscoveryTriggerReason,
    pub observed_at: String,
    pub device_id: Option<LanPassiveDiscoveryDeviceId>,
    pub scan_session_id: Option<LanPassiveDiscoveryScanSessionId>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LanPassiveDiscoveryPacketParseError {
    EmptyPayload,
    OversizedPayload {
        payload_len: usize,
        max_payload_len: usize,
    },
    MalformedPayload,
    UnsupportedSchemaVersion {
        schema_version: u16,
        expected_schema_version: u16,
    },
    EmptyObservedAt,
    EmptySummary,
    OversizedSummary {
        summary_len: usize,
        max_summary_len: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LanPassiveDiscoveryPacketIngestOutcome {
    Recorded,
    Deduplicated,
    Rejected(LanPassiveDiscoveryPacketParseError),
    Stopped,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPassiveDiscoveryRawSocketProtocol {
    Arp,
    Dhcp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LanPassiveDiscoveryRawSocketSupport {
    AvailableCollector {
        protocol: LanPassiveDiscoveryRawSocketProtocol,
        platform: String,
        collector_labels: Vec<String>,
        reason: String,
    },
    UnsupportedPlatform {
        protocol: LanPassiveDiscoveryRawSocketProtocol,
        platform: String,
        reason: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LanPassiveDiscoveryRawSocketCaptureOutcome {
    Captured {
        protocol: LanPassiveDiscoveryRawSocketProtocol,
        collector_labels: Vec<String>,
        observed_count: usize,
        recorded_count: usize,
    },
    Unsupported(LanPassiveDiscoveryRawSocketSupport),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LanPassiveDiscoveryUdpMulticastSupport {
    Available {
        source: LanPassiveDiscoverySource,
        multicast_group: String,
        port: u16,
    },
    AvailableBroadcast {
        source: LanPassiveDiscoverySource,
        port: u16,
    },
    Unsupported {
        source: LanPassiveDiscoverySource,
        reason: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LanPassiveDiscoveryUdpMulticastCaptureOutcome {
    Captured {
        source: LanPassiveDiscoverySource,
        received_datagram_count: usize,
    },
    Failed {
        source: LanPassiveDiscoverySource,
        received_datagram_count: usize,
        issue: LanPassiveDiscoveryUdpListenerIssue,
    },
    Unsupported(LanPassiveDiscoveryUdpMulticastSupport),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPassiveDiscoveryUdpListenerIssueKind {
    UnsupportedSource,
    InvalidMulticastGroup,
    SocketConfigurationFailed,
    BindFailed,
    AddressInUse,
    PermissionDenied,
    NoLocalIpv4Interface,
    AppleLocalNetworkPermissionRequired,
    MulticastJoinFailed,
    ReceiveFailed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPassiveDiscoveryUdpListenerIssue {
    pub source: LanPassiveDiscoverySource,
    pub kind: LanPassiveDiscoveryUdpListenerIssueKind,
    pub os_error_code: Option<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanPassiveDiscoveryLocalNeighborSource {
    WindowsNeighborTable,
    LinuxProcNetArp,
    LinuxIpNeigh,
    MacosArp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LanPassiveDiscoveryLocalNeighborCollectionOutcome {
    Captured {
        source: LanPassiveDiscoveryLocalNeighborSource,
        source_label: &'static str,
        observed_count: usize,
        recorded_count: usize,
    },
    Unsupported {
        source: LanPassiveDiscoveryLocalNeighborSource,
        source_label: &'static str,
        reason: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPassiveDiscoveryEventRow {
    pub schema_version: u16,
    pub event_id: LanPassiveDiscoveryEventId,
    pub event_kind: LanPassiveDiscoveryEventKind,
    pub observed_at: String,
    pub previous_event_id: Option<LanPassiveDiscoveryEventId>,
    pub source: Option<LanPassiveDiscoverySource>,
    pub trigger_reason: LanPassiveDiscoveryTriggerReason,
    pub device_id: Option<LanPassiveDiscoveryDeviceId>,
    pub scan_session_id: Option<LanPassiveDiscoveryScanSessionId>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPassiveDiscoveryEventHistory {
    pub schema_version: u16,
    pub generated_at: String,
    pub lifecycle_state: LanPassiveDiscoveryListenerLifecycleState,
    pub max_rows: usize,
    pub dropped_row_count: u64,
    pub latest_event_id: Option<LanPassiveDiscoveryEventId>,
    pub latest_observed_at: Option<String>,
    pub rows: Vec<LanPassiveDiscoveryEventRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanPassiveDiscoveryListenerState {
    generated_at: String,
    lifecycle_state: LanPassiveDiscoveryListenerLifecycleState,
    max_rows: usize,
    dropped_row_count: u64,
    rows: VecDeque<LanPassiveDiscoveryEventRow>,
}

pub fn collect_raw_socket_protocol_passive_updates(
    state: &mut LanPassiveDiscoveryListenerState,
    protocol: LanPassiveDiscoveryRawSocketProtocol,
    read_timeout: std::time::Duration,
) -> LanPassiveDiscoveryRawSocketCaptureOutcome {
    raw_socket::collect_raw_socket_protocol_passive_updates(state, protocol, read_timeout)
}

pub fn collect_udp_multicast_passive_packets(
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
    read_timeout: std::time::Duration,
) -> LanPassiveDiscoveryUdpMulticastCaptureOutcome {
    udp_multicast::collect_udp_multicast_passive_packets(
        state,
        source,
        max_datagram_count,
        read_timeout,
    )
}

pub fn ingest_allowed_snmp_response_packet(
    state: &mut LanPassiveDiscoveryListenerState,
    payload: &[u8],
) -> LanPassiveDiscoveryPacketIngestOutcome {
    udp_multicast::ingest_allowed_snmp_response_packet(state, payload)
}

pub fn collect_allowed_snmp_response_packets(
    socket: &std::net::UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    max_datagram_count: usize,
) -> usize {
    udp_multicast::collect_allowed_snmp_response_packets(socket, state, max_datagram_count)
}
