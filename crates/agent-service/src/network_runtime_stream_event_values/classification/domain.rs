use ocentra_parent_agent_protocol::activity_capture::ActivityDomainAttributionStatus;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkDomainAttributionKind, NetworkRuntimeEventPayload,
};

pub(crate) fn domain_attribution(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkDomainAttributionKind {
    match payload.domain_attribution_status {
        ActivityDomainAttributionStatus::DomainObserved => NetworkDomainAttributionKind::DnsAnswer,
        ActivityDomainAttributionStatus::IpOnly => NetworkDomainAttributionKind::IpOnly,
        ActivityDomainAttributionStatus::Unavailable => NetworkDomainAttributionKind::Unavailable,
    }
}
