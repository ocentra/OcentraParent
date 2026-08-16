use serde::{Deserialize, Serialize};

use crate::constants;

macro_rules! protocol_str_lookup {
    ($self:expr, [$($value:expr),+ $(,)?]) => {{
        const VALUES: &[&str] = &[$($value),+];
        VALUES[*$self as usize]
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                constants::activity_capture::OBSERVATION_MODE_SNAPSHOT,
                constants::activity_capture::OBSERVATION_MODE_ACTIVE_WINDOW,
                constants::activity_capture::OBSERVATION_MODE_NETWORK_SNAPSHOT,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                constants::activity_capture::CAPABILITY_STATUS_AVAILABLE,
                constants::activity_capture::CAPABILITY_STATUS_UNAVAILABLE,
                constants::activity_capture::CAPABILITY_STATUS_ACCESS_DENIED,
                constants::activity_capture::CAPABILITY_STATUS_NO_ACTIVE_WINDOW,
                constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS,
                constants::activity_capture::CAPABILITY_STATUS_ADAPTER_ERROR,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ActivityNetworkProtocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

impl ActivityNetworkProtocol {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                constants::activity_capture::NETWORK_PROTOCOL_TCP,
                constants::activity_capture::NETWORK_PROTOCOL_UDP,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                constants::activity_capture::TCP_STATE_CLOSED,
                constants::activity_capture::TCP_STATE_LISTEN,
                constants::activity_capture::TCP_STATE_SYN_SENT,
                constants::activity_capture::TCP_STATE_SYN_RECEIVED,
                constants::activity_capture::TCP_STATE_ESTABLISHED,
                constants::activity_capture::TCP_STATE_FIN_WAIT_1,
                constants::activity_capture::TCP_STATE_FIN_WAIT_2,
                constants::activity_capture::TCP_STATE_CLOSE_WAIT,
                constants::activity_capture::TCP_STATE_CLOSING,
                constants::activity_capture::TCP_STATE_LAST_ACK,
                constants::activity_capture::TCP_STATE_TIME_WAIT,
                constants::activity_capture::TCP_STATE_DELETE_TCB,
                constants::activity_capture::TCP_STATE_UNKNOWN,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED,
                constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_IP_ONLY,
                constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_UNAVAILABLE,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ActivityProcessAttributionStatus {
    #[serde(rename = "process-attributed")]
    ProcessAttributed,
    #[serde(rename = "process-unknown")]
    ProcessUnknown,
}

impl ActivityProcessAttributionStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED,
                constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_UNKNOWN,
            ]
        )
    }
}
