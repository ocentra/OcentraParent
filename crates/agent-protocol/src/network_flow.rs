use serde::{Deserialize, Serialize};

use crate::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityEvidenceRef, ActivityNetworkProtocol, ActivityNetworkTcpState, ActivityObserver,
    ActivityProcessAttributionStatus,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityNetworkCustodyState {
    #[serde(rename = "live-local-child-agent")]
    LiveLocalChildAgent,
    #[serde(rename = "live-lan-child-agent")]
    LiveLanChildAgent,
    #[serde(rename = "child-device-journal")]
    ChildDeviceJournal,
    #[serde(rename = "child-device-query-store")]
    ChildDeviceQueryStore,
    #[serde(rename = "parent-device-cache")]
    ParentDeviceCache,
    #[serde(rename = "parent-owned-export")]
    ParentOwnedExport,
    #[serde(rename = "ocentra-hosted-non-activity")]
    OcentraHostedNonActivity,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl ActivityNetworkCustodyState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::LiveLocalChildAgent => constants::network_flow::CUSTODY_LIVE_LOCAL_CHILD_AGENT,
            Self::LiveLanChildAgent => constants::network_flow::CUSTODY_LIVE_LAN_CHILD_AGENT,
            Self::ChildDeviceJournal => constants::network_flow::CUSTODY_CHILD_DEVICE_JOURNAL,
            Self::ChildDeviceQueryStore => {
                constants::network_flow::CUSTODY_CHILD_DEVICE_QUERY_STORE
            }
            Self::ParentDeviceCache => constants::network_flow::CUSTODY_PARENT_DEVICE_CACHE,
            Self::ParentOwnedExport => constants::network_flow::CUSTODY_PARENT_OWNED_EXPORT,
            Self::OcentraHostedNonActivity => {
                constants::network_flow::CUSTODY_OCENTRA_HOSTED_NON_ACTIVITY
            }
            Self::Unavailable => constants::network_flow::CUSTODY_UNAVAILABLE,
        }
    }

    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::network_flow::CUSTODY_LIVE_LOCAL_CHILD_AGENT => {
                Some(Self::LiveLocalChildAgent)
            }
            constants::network_flow::CUSTODY_LIVE_LAN_CHILD_AGENT => Some(Self::LiveLanChildAgent),
            constants::network_flow::CUSTODY_CHILD_DEVICE_JOURNAL => Some(Self::ChildDeviceJournal),
            constants::network_flow::CUSTODY_CHILD_DEVICE_QUERY_STORE => {
                Some(Self::ChildDeviceQueryStore)
            }
            constants::network_flow::CUSTODY_PARENT_DEVICE_CACHE => Some(Self::ParentDeviceCache),
            constants::network_flow::CUSTODY_PARENT_OWNED_EXPORT => Some(Self::ParentOwnedExport),
            constants::network_flow::CUSTODY_OCENTRA_HOSTED_NON_ACTIVITY => {
                Some(Self::OcentraHostedNonActivity)
            }
            constants::network_flow::CUSTODY_UNAVAILABLE => Some(Self::Unavailable),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityNetworkFlowIndicatorKind {
    #[serde(rename = "new-destination")]
    NewDestination,
    #[serde(rename = "high-volume")]
    HighVolume,
    #[serde(rename = "vpn-proxy-tunnel")]
    VpnProxyTunnel,
    #[serde(rename = "repeated-failure")]
    RepeatedFailure,
    #[serde(rename = "unusual-unknown-process")]
    UnusualUnknownProcess,
    #[serde(rename = "adapter-unavailable")]
    AdapterUnavailable,
    #[serde(rename = "encrypted-content-unavailable")]
    EncryptedContentUnavailable,
}

impl ActivityNetworkFlowIndicatorKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::NewDestination => constants::network_flow::INDICATOR_NEW_DESTINATION,
            Self::HighVolume => constants::network_flow::INDICATOR_HIGH_VOLUME,
            Self::VpnProxyTunnel => constants::network_flow::INDICATOR_VPN_PROXY_TUNNEL,
            Self::RepeatedFailure => constants::network_flow::INDICATOR_REPEATED_FAILURE,
            Self::UnusualUnknownProcess => {
                constants::network_flow::INDICATOR_UNUSUAL_UNKNOWN_PROCESS
            }
            Self::AdapterUnavailable => constants::network_flow::INDICATOR_ADAPTER_UNAVAILABLE,
            Self::EncryptedContentUnavailable => {
                constants::network_flow::INDICATOR_ENCRYPTED_CONTENT_UNAVAILABLE
            }
        }
    }

    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::network_flow::INDICATOR_NEW_DESTINATION => Some(Self::NewDestination),
            constants::network_flow::INDICATOR_HIGH_VOLUME => Some(Self::HighVolume),
            constants::network_flow::INDICATOR_VPN_PROXY_TUNNEL => Some(Self::VpnProxyTunnel),
            constants::network_flow::INDICATOR_REPEATED_FAILURE => Some(Self::RepeatedFailure),
            constants::network_flow::INDICATOR_UNUSUAL_UNKNOWN_PROCESS => {
                Some(Self::UnusualUnknownProcess)
            }
            constants::network_flow::INDICATOR_ADAPTER_UNAVAILABLE => {
                Some(Self::AdapterUnavailable)
            }
            constants::network_flow::INDICATOR_ENCRYPTED_CONTENT_UNAVAILABLE => {
                Some(Self::EncryptedContentUnavailable)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkEndpoint {
    pub ip: Option<String>,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowCounters {
    pub connection_count: f64,
    pub bytes_sent: Option<f64>,
    pub bytes_received: Option<f64>,
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowObservation {
    pub schema_version: u16,
    pub event_id: String,
    pub observed_at: String,
    pub observer: ActivityObserver,
    pub capability_status: ActivityCaptureCapabilityStatus,
    pub adapter_id: String,
    pub protocol: Option<ActivityNetworkProtocol>,
    pub tcp_state: Option<ActivityNetworkTcpState>,
    pub local_endpoint: ActivityNetworkEndpoint,
    pub destination_endpoint: ActivityNetworkEndpoint,
    pub destination_domain: Option<String>,
    pub domain_attribution_status: ActivityDomainAttributionStatus,
    pub process_attribution_status: ActivityProcessAttributionStatus,
    pub process_id: Option<u64>,
    pub process_name: Option<String>,
    pub counters: ActivityNetworkFlowCounters,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody: ActivityNetworkCustodyState,
    pub limit: u64,
    pub returned: u64,
    pub capability_status: ActivityCaptureCapabilityStatus,
    pub rows: Vec<ActivityNetworkFlowObservation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowRollup {
    pub key: String,
    pub label: String,
    pub connection_count: f64,
    pub bytes_sent: Option<f64>,
    pub bytes_received: Option<f64>,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowIndicator {
    pub kind: ActivityNetworkFlowIndicatorKind,
    pub label: String,
    pub observed_at: String,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowDigest {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody: ActivityNetworkCustodyState,
    pub evidence: Vec<ActivityEvidenceRef>,
    pub top_processes: Vec<ActivityNetworkFlowRollup>,
    pub top_destinations: Vec<ActivityNetworkFlowRollup>,
    pub unusual_indicators: Vec<ActivityNetworkFlowIndicator>,
}
