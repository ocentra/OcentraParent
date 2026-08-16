use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus, ActivityNetworkProtocol,
    ActivityNetworkTcpState, ActivityProcessAttributionStatus,
};

use crate::network_capture_adapter::platform_network_snapshot;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkObservation {
    pub status: ActivityCaptureCapabilityStatus,
    pub protocol: Option<ActivityNetworkProtocol>,
    pub local_ip: Option<String>,
    pub local_port: Option<u16>,
    pub destination_ip: Option<String>,
    pub destination_port: Option<u16>,
    pub destination_domain: Option<String>,
    pub tcp_state: Option<ActivityNetworkTcpState>,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub associated_pid_count: usize,
}

impl NetworkObservation {
    pub fn degraded(status: ActivityCaptureCapabilityStatus) -> Self {
        Self {
            status,
            protocol: None,
            local_ip: None,
            local_port: None,
            destination_ip: None,
            destination_port: None,
            destination_domain: None,
            tcp_state: None,
            pid: None,
            process_name: None,
            associated_pid_count: 0,
        }
    }

    pub fn domain_attribution_status(&self) -> ActivityDomainAttributionStatus {
        if self.destination_domain.is_some() {
            return ActivityDomainAttributionStatus::DomainObserved;
        }
        if self.destination_ip.is_some() {
            return ActivityDomainAttributionStatus::IpOnly;
        }
        ActivityDomainAttributionStatus::Unavailable
    }

    pub fn process_attribution_status(&self) -> ActivityProcessAttributionStatus {
        if self.pid.is_some() {
            return ActivityProcessAttributionStatus::ProcessAttributed;
        }
        ActivityProcessAttributionStatus::ProcessUnknown
    }
}

pub fn collect_network_snapshot(limit: usize) -> Vec<NetworkObservation> {
    if limit == 0 {
        return Vec::new();
    }
    match platform_network_snapshot(limit) {
        Ok(mut observations) => {
            if observations.is_empty() {
                observations.push(NetworkObservation::degraded(
                    ActivityCaptureCapabilityStatus::NoNetworkObservations,
                ));
            }
            observations
        }
        Err(()) => vec![NetworkObservation::degraded(
            ActivityCaptureCapabilityStatus::AdapterError,
        )],
    }
}
