use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::network_flow::{
    self as flow, NetworkEvidenceGrade, NetworkRuntimeEventPayload,
    NetworkRuntimeEvidenceGrade as CoreNetworkEvidenceGrade,
};

use super::NetworkRuntimeStreamText;

pub(crate) fn custody(payload: &NetworkRuntimeEventPayload) -> NetworkRuntimeStreamText {
    if payload.capability_status == ActivityCaptureCapabilityStatus::Available {
        return NetworkRuntimeStreamText(
            flow::NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        );
    }
    NetworkRuntimeStreamText(flow::NETWORK_FLOW_CUSTODY_UNAVAILABLE.to_string())
}

pub(crate) fn evidence_grade(payload: &NetworkRuntimeEventPayload) -> NetworkEvidenceGrade {
    match payload.evidence_grade {
        CoreNetworkEvidenceGrade::DomainAndProcessMetadata => NetworkEvidenceGrade::A,
        CoreNetworkEvidenceGrade::IpOrProcessPartialMetadata => NetworkEvidenceGrade::C,
        CoreNetworkEvidenceGrade::AdapterUnavailable => NetworkEvidenceGrade::D,
    }
}

pub(crate) fn confidence(payload: &NetworkRuntimeEventPayload) -> f32 {
    match payload.evidence_grade {
        CoreNetworkEvidenceGrade::DomainAndProcessMetadata => 1.0,
        CoreNetworkEvidenceGrade::IpOrProcessPartialMetadata => 0.5,
        CoreNetworkEvidenceGrade::AdapterUnavailable => 0.0,
    }
}
