use std::collections::VecDeque;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

pub mod collection;
pub mod dhcp;
pub mod dns_like;
pub mod labels;
pub mod packet;
pub mod raw_socket;
pub mod snmp;
pub mod ssdp;
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

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
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
    Unsupported(LanPassiveDiscoveryUdpMulticastSupport),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanPassiveDiscoveryLocalNeighborSource {
    WindowsNeighborTable,
    LinuxProcNetArp,
    LinuxIpNeigh,
    MacosArp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LanPassiveDiscoveryLocalNeighborCollectionOutcome {
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

struct PassiveDiscoveryEventInput<'a> {
    event_kind: LanPassiveDiscoveryEventKind,
    source: Option<LanPassiveDiscoverySource>,
    trigger_reason: LanPassiveDiscoveryTriggerReason,
    observed_at: &'a str,
    device_id: Option<&'a str>,
    scan_session_id: Option<&'a str>,
    summary: String,
}

impl LanPassiveDiscoveryListenerState {
    pub const SCHEMA_VERSION: u16 = 1;
    pub const DEFAULT_MAX_ROWS: usize = 128;

    pub fn running(generated_at: String) -> Self {
        Self::with_capacity(generated_at, Self::DEFAULT_MAX_ROWS)
    }

    pub fn with_capacity(generated_at: String, max_rows: usize) -> Self {
        Self {
            generated_at,
            lifecycle_state: LanPassiveDiscoveryListenerLifecycleState::Running,
            max_rows: max_rows.max(1),
            dropped_row_count: 0,
            rows: VecDeque::new(),
        }
    }

    pub fn lifecycle_state(&self) -> LanPassiveDiscoveryListenerLifecycleState {
        self.lifecycle_state.clone()
    }

    pub fn is_running(&self) -> bool {
        self.lifecycle_state == LanPassiveDiscoveryListenerLifecycleState::Running
    }

    pub fn stop(&mut self) {
        self.lifecycle_state = LanPassiveDiscoveryListenerLifecycleState::Stopped;
    }

    pub fn ingest_udp_packet(&mut self, payload: &[u8]) -> LanPassiveDiscoveryPacketIngestOutcome {
        if !self.is_running() {
            return LanPassiveDiscoveryPacketIngestOutcome::Stopped;
        }

        match packet::parse_passive_discovery_packet(payload) {
            Ok(packet) => match self.record_passive_packet(packet) {
                LanPassiveDiscoveryRecordOutcome::Recorded => {
                    LanPassiveDiscoveryPacketIngestOutcome::Recorded
                }
                LanPassiveDiscoveryRecordOutcome::Deduplicated => {
                    LanPassiveDiscoveryPacketIngestOutcome::Deduplicated
                }
                LanPassiveDiscoveryRecordOutcome::Stopped => {
                    LanPassiveDiscoveryPacketIngestOutcome::Stopped
                }
            },
            Err(error) => LanPassiveDiscoveryPacketIngestOutcome::Rejected(error),
        }
    }

    pub fn record_passive_packet(
        &mut self,
        packet: LanPassiveDiscoveryPacket,
    ) -> LanPassiveDiscoveryRecordOutcome {
        self.record_passive_update(
            packet.source,
            packet.trigger_reason,
            &packet.observed_at,
            packet.device_id.as_deref(),
            packet.scan_session_id.as_deref(),
            packet.summary,
        )
    }

    pub fn record_passive_update(
        &mut self,
        source: LanPassiveDiscoverySource,
        trigger_reason: LanPassiveDiscoveryTriggerReason,
        observed_at: &str,
        device_id: Option<&str>,
        scan_session_id: Option<&str>,
        summary: impl Into<String>,
    ) -> LanPassiveDiscoveryRecordOutcome {
        self.record_event(PassiveDiscoveryEventInput {
            event_kind: LanPassiveDiscoveryEventKind::PassiveUpdate,
            source: Some(source),
            trigger_reason,
            observed_at,
            device_id,
            scan_session_id,
            summary: summary.into(),
        })
    }

    pub fn record_rescan_trigger(
        &mut self,
        trigger_reason: LanPassiveDiscoveryTriggerReason,
        observed_at: &str,
        summary: impl Into<String>,
    ) -> LanPassiveDiscoveryRecordOutcome {
        self.record_event(PassiveDiscoveryEventInput {
            event_kind: LanPassiveDiscoveryEventKind::RescanTrigger,
            source: None,
            trigger_reason,
            observed_at,
            device_id: None,
            scan_session_id: None,
            summary: summary.into(),
        })
    }

    pub fn snapshot(&self) -> LanPassiveDiscoveryEventHistory {
        let latest = self.rows.back();
        LanPassiveDiscoveryEventHistory {
            schema_version: Self::SCHEMA_VERSION,
            generated_at: self.generated_at.clone(),
            lifecycle_state: self.lifecycle_state.clone(),
            max_rows: self.max_rows,
            dropped_row_count: self.dropped_row_count,
            latest_event_id: latest.map(|row| row.event_id.clone()),
            latest_observed_at: latest.map(|row| row.observed_at.clone()),
            rows: self.rows.iter().cloned().collect(),
        }
    }

    pub fn rows(&self) -> Vec<LanPassiveDiscoveryEventRow> {
        self.rows.iter().cloned().collect()
    }

    fn record_event(
        &mut self,
        input: PassiveDiscoveryEventInput<'_>,
    ) -> LanPassiveDiscoveryRecordOutcome {
        let PassiveDiscoveryEventInput {
            event_kind,
            source,
            trigger_reason,
            observed_at,
            device_id,
            scan_session_id,
            summary,
        } = input;
        if !self.is_running() {
            return LanPassiveDiscoveryRecordOutcome::Stopped;
        }

        let event_id = labels::passive_event_id(
            &event_kind,
            source.as_ref(),
            &trigger_reason,
            observed_at,
            device_id,
            scan_session_id,
        );

        if self
            .rows
            .iter()
            .any(|row| row.event_id.eq_ignore_ascii_case(&event_id))
        {
            return LanPassiveDiscoveryRecordOutcome::Deduplicated;
        }

        if self.rows.len() >= self.max_rows {
            let _ = self.rows.pop_front();
            self.dropped_row_count = self.dropped_row_count.saturating_add(1);
        }

        let previous_event_id = self.rows.back().map(|row| row.event_id.clone());
        self.rows.push_back(LanPassiveDiscoveryEventRow {
            schema_version: Self::SCHEMA_VERSION,
            event_id: event_id.into(),
            event_kind,
            observed_at: observed_at.to_string(),
            previous_event_id,
            source,
            trigger_reason,
            device_id: device_id.map(LanPassiveDiscoveryDeviceId::from),
            scan_session_id: scan_session_id.map(LanPassiveDiscoveryScanSessionId::from),
            summary,
        });
        LanPassiveDiscoveryRecordOutcome::Recorded
    }
}
