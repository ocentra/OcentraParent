use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::Value;

use ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeReport;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkRuntimeEventPayload;

use crate::network_runtime_stream_event_payloads;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NetworkRuntimeServiceStreamEntry {
    pub(crate) stream_type: String,
    pub(crate) event_ref: String,
    pub(crate) payload: Value,
}

impl Serialize for NetworkRuntimeServiceStreamEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entry =
            serializer.serialize_struct(constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM, 3)?;
        entry.serialize_field(constants::field::EVENT_TYPE, &self.stream_type)?;
        entry.serialize_field(constants::field::EVENT_REF, &self.event_ref)?;
        entry.serialize_field(constants::field::PAYLOAD, &self.payload)?;
        entry.end()
    }
}

pub(crate) fn stream_entries_from_report(
    report: &NetworkRuntimeReport,
) -> Vec<NetworkRuntimeServiceStreamEntry> {
    report
        .stored_events
        .iter()
        .filter_map(|event| {
            let decoded = event.decode::<NetworkRuntimeEventPayload>().ok()?;
            let stream_type = event.contract.event_type.as_str().to_string();
            let event_ref = event_ref(
                event.correlation_id.as_str(),
                event.contract.event_type.as_str(),
            );
            protocol_payload(&stream_type, &event_ref, &decoded.payload).map(|payload| {
                NetworkRuntimeServiceStreamEntry {
                    stream_type,
                    event_ref,
                    payload,
                }
            })
        })
        .collect()
}

fn protocol_payload(
    event_name: &str,
    event_ref: &str,
    payload: &NetworkRuntimeEventPayload,
) -> Option<Value> {
    match event_name {
        constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED => {
            Some(network_runtime_stream_event_payloads::network_flow_observed(event_ref, payload))
        }
        constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED => {
            Some(network_runtime_stream_event_payloads::network_domain_observed(event_ref, payload))
        }
        constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED => Some(
            network_runtime_stream_event_payloads::network_activity_classified(event_ref, payload),
        ),
        constants::network_flow::EVENT_AI_ANALYSIS_REQUESTED => Some(
            network_runtime_stream_event_payloads::network_ai_analysis_requested(
                event_ref, payload,
            ),
        ),
        constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED => Some(
            network_runtime_stream_event_payloads::network_ai_analysis_completed(
                event_ref, payload,
            ),
        ),
        constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED => Some(
            network_runtime_stream_event_payloads::network_policy_evaluation_requested(
                event_ref, payload,
            ),
        ),
        constants::network_flow::EVENT_POLICY_DECISION_COMPLETED => Some(
            network_runtime_stream_event_payloads::network_policy_decision_completed(
                event_ref, payload,
            ),
        ),
        constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED => Some(
            network_runtime_stream_event_payloads::network_enforcement_command_issued(
                event_ref, payload,
            ),
        ),
        constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED => Some(
            network_runtime_stream_event_payloads::network_enforcement_result_observed(
                event_ref, payload,
            ),
        ),
        constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED => Some(
            network_runtime_stream_event_payloads::network_audit_entry_committed(
                event_ref, payload,
            ),
        ),
        constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED => Some(
            network_runtime_stream_event_payloads::network_portal_read_model_updated(
                event_ref, payload,
            ),
        ),
        _ => None,
    }
}

fn event_ref(correlation_id: &str, event_name: &str) -> String {
    let mut value = String::from(correlation_id);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(event_name);
    value
}
