use serde::Serialize;
use serde_json::Value;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkClaimBoundary, NetworkRuntimeEventPayload,
};

use super::{NetworkRuntimeStreamRef, NetworkRuntimeStreamText};

pub(crate) fn no_claim_boundary() -> NetworkClaimBoundary {
    NetworkClaimBoundary {
        exact_url_available: false,
        decrypted_https_payload_available: false,
        message_content_available: false,
        search_query_available: false,
        adapter_action_executed: false,
    }
}

pub(crate) fn previous_event_ref(payload: &NetworkRuntimeEventPayload) -> NetworkRuntimeStreamRef {
    NetworkRuntimeStreamRef(
        payload
            .previous_phase_ref
            .clone()
            .unwrap_or_else(|| payload.evidence_ref.clone()),
    )
}

pub(crate) fn ref_or_current(
    value: &Option<NetworkRuntimeStreamRef>,
    event_ref: &NetworkRuntimeStreamRef,
) -> NetworkRuntimeStreamRef {
    value.clone().unwrap_or_else(|| event_ref.clone())
}

pub(crate) fn evidence_refs(payload: &NetworkRuntimeEventPayload) -> Vec<NetworkRuntimeStreamRef> {
    vec![NetworkRuntimeStreamRef(payload.evidence_ref.clone())]
}

pub(crate) fn parent_rule_refs() -> Vec<NetworkRuntimeStreamRef> {
    vec![NetworkRuntimeStreamRef(
        constants::network_flow::TEST_PARENT_RULE_REF.to_string(),
    )]
}

pub(crate) fn uncertainty_codes() -> Vec<NetworkRuntimeStreamText> {
    vec![NetworkRuntimeStreamText(
        constants::network_flow::UNCERTAINTY_NETWORK_ONLY_NO_EXACT_URL.to_string(),
    )]
}

pub(crate) fn json_value<T>(value: T) -> Value
where
    T: Serialize,
{
    match serde_json::to_value(value) {
        Ok(json) => json,
        Err(_error) => Value::Null,
    }
}
