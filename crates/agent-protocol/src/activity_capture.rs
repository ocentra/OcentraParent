use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityObservationMode {
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "active-window")]
    ActiveWindow,
    #[serde(rename = "network-snapshot")]
    NetworkSnapshot,
}

impl ActivityObservationMode {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Snapshot => constants::activity_capture::OBSERVATION_MODE_SNAPSHOT,
            Self::ActiveWindow => constants::activity_capture::OBSERVATION_MODE_ACTIVE_WINDOW,
            Self::NetworkSnapshot => constants::activity_capture::OBSERVATION_MODE_NETWORK_SNAPSHOT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityCaptureCapabilityStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "access-denied")]
    AccessDenied,
    #[serde(rename = "no-active-window")]
    NoActiveWindow,
    #[serde(rename = "no-network-observations")]
    NoNetworkObservations,
    #[serde(rename = "adapter-error")]
    AdapterError,
}

impl ActivityCaptureCapabilityStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Available => constants::activity_capture::CAPABILITY_STATUS_AVAILABLE,
            Self::Unavailable => constants::activity_capture::CAPABILITY_STATUS_UNAVAILABLE,
            Self::AccessDenied => constants::activity_capture::CAPABILITY_STATUS_ACCESS_DENIED,
            Self::NoActiveWindow => constants::activity_capture::CAPABILITY_STATUS_NO_ACTIVE_WINDOW,
            Self::NoNetworkObservations => {
                constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS
            }
            Self::AdapterError => constants::activity_capture::CAPABILITY_STATUS_ADAPTER_ERROR,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityNetworkProtocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

impl ActivityNetworkProtocol {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Tcp => constants::activity_capture::NETWORK_PROTOCOL_TCP,
            Self::Udp => constants::activity_capture::NETWORK_PROTOCOL_UDP,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityNetworkTcpState {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "listen")]
    Listen,
    #[serde(rename = "syn-sent")]
    SynSent,
    #[serde(rename = "syn-received")]
    SynReceived,
    #[serde(rename = "established")]
    Established,
    #[serde(rename = "fin-wait-1")]
    FinWait1,
    #[serde(rename = "fin-wait-2")]
    FinWait2,
    #[serde(rename = "close-wait")]
    CloseWait,
    #[serde(rename = "closing")]
    Closing,
    #[serde(rename = "last-ack")]
    LastAck,
    #[serde(rename = "time-wait")]
    TimeWait,
    #[serde(rename = "delete-tcb")]
    DeleteTcb,
    #[serde(rename = "unknown")]
    Unknown,
}

impl ActivityNetworkTcpState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Closed => constants::activity_capture::TCP_STATE_CLOSED,
            Self::Listen => constants::activity_capture::TCP_STATE_LISTEN,
            Self::SynSent => constants::activity_capture::TCP_STATE_SYN_SENT,
            Self::SynReceived => constants::activity_capture::TCP_STATE_SYN_RECEIVED,
            Self::Established => constants::activity_capture::TCP_STATE_ESTABLISHED,
            Self::FinWait1 => constants::activity_capture::TCP_STATE_FIN_WAIT_1,
            Self::FinWait2 => constants::activity_capture::TCP_STATE_FIN_WAIT_2,
            Self::CloseWait => constants::activity_capture::TCP_STATE_CLOSE_WAIT,
            Self::Closing => constants::activity_capture::TCP_STATE_CLOSING,
            Self::LastAck => constants::activity_capture::TCP_STATE_LAST_ACK,
            Self::TimeWait => constants::activity_capture::TCP_STATE_TIME_WAIT,
            Self::DeleteTcb => constants::activity_capture::TCP_STATE_DELETE_TCB,
            Self::Unknown => constants::activity_capture::TCP_STATE_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityDomainAttributionStatus {
    #[serde(rename = "domain-observed")]
    DomainObserved,
    #[serde(rename = "ip-only")]
    IpOnly,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl ActivityDomainAttributionStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::DomainObserved => {
                constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED
            }
            Self::IpOnly => constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_IP_ONLY,
            Self::Unavailable => constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityProcessAttributionStatus {
    #[serde(rename = "process-attributed")]
    ProcessAttributed,
    #[serde(rename = "process-unknown")]
    ProcessUnknown,
}

impl ActivityProcessAttributionStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ProcessAttributed => {
                constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED
            }
            Self::ProcessUnknown => constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_UNKNOWN,
        }
    }
}
