mod read_model;

use self::read_model::network_flow_read_model_from_payload_impl;
use super::*;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowDigest,
    ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel,
    NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
    NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID,
    NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT,
    NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS,
};
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::ACTIVITY_QUERY_SCHEMA_VERSION;
use ocentra_schema::parent_ui_bridge::{
    ParentContractReferenceId, ParentNetworkRuntimeEventChainStreamSnapshot,
    ParentNetworkRuntimeEventResultSnapshot, ParentNetworkRuntimeEventValueSnapshot,
    ParentRuntimeEventType,
};

pub(super) fn app_game_read_model_from_response<T>(
    response_event: &AgentEventEnvelope,
    field_name: &str,
    label: &str,
) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let value = response_json_payload_field(response_event, field_name)?;
    serde_json::from_value(value)
        .map_err(|error| format!("agent-service app/game {label} payload parse failed: {error}"))
}

pub(super) fn activity_surface_read_model_from_response<T>(
    response_event: &AgentEventEnvelope,
    expected_kind: &str,
    label: &str,
) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let actual_kind = response_event
        .payload
        .get(constants::field::ACTIVITY_READ_MODEL_KIND)
        .and_then(log_field_string)
        .ok_or_else(|| {
            format!(
                "agent-service {label} read model payload missing {}",
                constants::field::ACTIVITY_READ_MODEL_KIND
            )
        })?;
    if actual_kind != expected_kind {
        return Err(format!(
            "agent-service {label} read model payload expected kind {expected_kind}, received {actual_kind}"
        ));
    }
    let value = response_json_payload_field(response_event, constants::field::ACTIVITY_READ_MODEL)?;
    serde_json::from_value(value)
        .map_err(|error| format!("agent-service {label} read model payload parse failed: {error}"))
}

pub(crate) fn response_json_payload_field(
    response_event: &AgentEventEnvelope,
    field_name: &str,
) -> Result<Value, String> {
    let value = response_event
        .payload
        .get(field_name)
        .and_then(log_field_string)
        .ok_or_else(|| format!("agent-service response payload missing {field_name}"))?;
    serde_json::from_str::<Value>(value).map_err(|error| {
        format!("agent-service response payload field {field_name} contained invalid JSON: {error}")
    })
}

pub(super) fn network_flow_read_model_from_payload(
    payload: &LogFields,
) -> Result<ActivityNetworkFlowReadModel, String> {
    network_flow_read_model_from_payload_impl(payload)
}

fn network_flow_digest(payload: &LogFields) -> Option<ActivityNetworkFlowDigest> {
    let raw = payload
        .get(constants::field::ACTIVITY_DIGEST)
        .and_then(log_field_string)?;
    let mut digest = serde_json::from_str::<ActivityNetworkFlowDigest>(raw).ok()?;
    digest
        .evidence
        .retain(|evidence| !evidence.evidence_id.trim().is_empty());
    Some(digest)
}

pub(super) fn network_runtime_event_chain_stream_from_payload(
    payload: &LogFields,
) -> ParentNetworkRuntimeEventChainStreamSnapshot {
    let events = payload
        .get(constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM)
        .and_then(log_field_string)
        .and_then(parse_network_runtime_entries_json)
        .unwrap_or_default();
    let invalid_event_count = events.iter().filter(|entry| !entry.ok).count() as u64;

    ParentNetworkRuntimeEventChainStreamSnapshot {
        streamed_event_count: optional_u64_field(
            payload,
            constants::field::NETWORK_RUNTIME_STREAMED_EVENTS,
        ),
        events,
        invalid_event_count,
    }
}

fn parse_network_runtime_entries_json(
    raw: &str,
) -> Option<Vec<ParentNetworkRuntimeEventResultSnapshot>> {
    let parsed = serde_json::from_str::<Vec<Value>>(raw).ok()?;
    Some(
        parsed
            .into_iter()
            .map(|entry| parse_network_runtime_event_entry(&entry))
            .collect(),
    )
}

fn parse_network_runtime_event_entry(entry: &Value) -> ParentNetworkRuntimeEventResultSnapshot {
    let event_type = entry
        .get(constants::field::EVENT_TYPE)
        .and_then(Value::as_str)
        .and_then(|value| ParentRuntimeEventType::parse(value.to_owned()));
    let Some(event_type_value) = event_type.as_ref() else {
        return invalid_network_runtime_event("invalid-event-type", None);
    };
    if !is_supported_network_runtime_event_type(event_type_value.as_str()) {
        return invalid_network_runtime_event("invalid-event-type", event_type);
    }

    let Some(payload_value) = entry.get(constants::field::PAYLOAD) else {
        return invalid_network_runtime_event("invalid-payload", Some(event_type_value.clone()));
    };
    let Some(payload_object) = payload_value.as_object() else {
        return invalid_network_runtime_event("invalid-payload", Some(event_type_value.clone()));
    };

    ParentNetworkRuntimeEventResultSnapshot {
        ok: true,
        reason: None,
        event_type: Some(event_type_value.clone()),
        value: Some(ParentNetworkRuntimeEventValueSnapshot {
            ai_analysis_ref: payload_object
                .get("aiAnalysisRef")
                .and_then(Value::as_str)
                .and_then(|value| ParentContractReferenceId::parse(value.to_owned())),
            policy_decision_ref: payload_object
                .get("policyDecisionRef")
                .and_then(Value::as_str)
                .and_then(|value| ParentContractReferenceId::parse(value.to_owned())),
            enforcement_result_ref: payload_object
                .get("enforcementResultRef")
                .and_then(Value::as_str)
                .and_then(|value| ParentContractReferenceId::parse(value.to_owned())),
        }),
    }
}

fn invalid_network_runtime_event(
    reason: &str,
    event_type: Option<ParentRuntimeEventType>,
) -> ParentNetworkRuntimeEventResultSnapshot {
    ParentNetworkRuntimeEventResultSnapshot {
        ok: false,
        reason: Some(reason.to_string()),
        event_type,
        value: None,
    }
}

fn is_supported_network_runtime_event_type(event_type: &str) -> bool {
    matches!(
        event_type,
        constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED
            | constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED
            | constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED
            | constants::network_flow::EVENT_AI_ANALYSIS_REQUESTED
            | constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED
            | constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED
            | constants::network_flow::EVENT_POLICY_DECISION_COMPLETED
            | constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
            | constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED
            | constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED
            | constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED
    )
}
