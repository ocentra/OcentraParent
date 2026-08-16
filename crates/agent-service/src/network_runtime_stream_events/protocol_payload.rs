use serde_json::Value;

use ocentra_parent_agent_protocol::network_flow::NetworkRuntimeEventPayload;

use crate::network_runtime_stream_event_payloads;
use crate::network_runtime_stream_event_values as values;

use super::event_kind::NetworkRuntimeStreamEventKind;

pub(super) fn protocol_payload(
    event_kind: NetworkRuntimeStreamEventKind,
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    match event_kind {
        NetworkRuntimeStreamEventKind::FlowObserved => {
            network_runtime_stream_event_payloads::network_flow_observed(event_ref, payload)
        }
        NetworkRuntimeStreamEventKind::DomainObserved => {
            network_runtime_stream_event_payloads::network_domain_observed(event_ref, payload)
        }
        NetworkRuntimeStreamEventKind::ActivityClassified => {
            network_runtime_stream_event_payloads::network_activity_classified(event_ref, payload)
        }
        NetworkRuntimeStreamEventKind::AiAnalysisRequested => {
            network_runtime_stream_event_payloads::network_ai_analysis_requested(event_ref, payload)
        }
        NetworkRuntimeStreamEventKind::AiAnalysisCompleted => {
            network_runtime_stream_event_payloads::network_ai_analysis_completed(event_ref, payload)
        }
        NetworkRuntimeStreamEventKind::PolicyEvaluationRequested => {
            network_runtime_stream_event_payloads::network_policy_evaluation_requested(
                event_ref, payload,
            )
        }
        NetworkRuntimeStreamEventKind::PolicyDecisionCompleted => {
            network_runtime_stream_event_payloads::network_policy_decision_completed(
                event_ref, payload,
            )
        }
        NetworkRuntimeStreamEventKind::EnforcementCommandIssued => {
            network_runtime_stream_event_payloads::network_enforcement_command_issued(
                event_ref, payload,
            )
        }
        NetworkRuntimeStreamEventKind::EnforcementResultObserved => {
            network_runtime_stream_event_payloads::network_enforcement_result_observed(
                event_ref, payload,
            )
        }
        NetworkRuntimeStreamEventKind::AuditEntryCommitted => {
            network_runtime_stream_event_payloads::network_audit_entry_committed(event_ref, payload)
        }
        NetworkRuntimeStreamEventKind::PortalReadModelUpdated => {
            network_runtime_stream_event_payloads::network_portal_read_model_updated(
                event_ref, payload,
            )
        }
    }
}
