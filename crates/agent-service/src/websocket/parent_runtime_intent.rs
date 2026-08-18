use ocentra_parent_agent_protocol::{
    constants,
    logging::{LogFieldValue, LogLevel},
    transport::{
        AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, ParentRuntimeIntentIngressKind,
        ParentRuntimeIntentIngressResult,
    },
};
use serde::Deserialize;

use crate::{
    event_builder::build_event, fields::fields_from_pairs, json_contract::serialize_json_string,
};

#[path = "parent_runtime_intent/result.rs"]
mod result;

// The ingress accepts only a kind marker. Authority-bearing delivery,
// evaluation, transition, acknowledgement, origin, and child-state payloads
// must come from an owning parent runtime once an authenticated handoff exists.
#[derive(Debug, Deserialize)]
#[serde(tag = "intentKind", deny_unknown_fields)]
pub(super) enum ParentRuntimeIntentRequest {
    #[serde(rename = "tracking-child-check-in-request")]
    TrackingChildCheckInRequest,
    #[serde(rename = "policy-control-delivery")]
    PolicyControlDelivery,
}

impl ParentRuntimeIntentRequest {
    pub(super) fn kind(&self) -> ParentRuntimeIntentIngressKind {
        match self {
            Self::TrackingChildCheckInRequest => {
                ParentRuntimeIntentIngressKind::TrackingChildCheckInRequest
            }
            Self::PolicyControlDelivery => ParentRuntimeIntentIngressKind::PolicyControlDelivery,
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct IngressReason(pub(super) String);

pub(crate) async fn build_parent_runtime_intent_ingress_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let result = execute_parent_runtime_intent_ingress(&command).await;
    build_event(
        constants::event_id::PARENT_RUNTIME_INTENT_INGRESS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentRuntimeIntentIngressReported,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::PARENT_RUNTIME_INTENT_INGRESS_RESULT,
            LogFieldValue::String(serialize_json_string(&result).0),
        )]),
        None,
    )
}

async fn execute_parent_runtime_intent_ingress(
    command: &AgentCommandEnvelope,
) -> ParentRuntimeIntentIngressResult {
    let Some(LogFieldValue::String(request_text)) = command
        .payload
        .get(constants::field::PARENT_RUNTIME_INTENT_INGRESS_REQUEST)
    else {
        return result::rejected(
            command,
            ParentRuntimeIntentIngressKind::Unknown,
            IngressReason(
                constants::parent_controller::INGRESS_REJECTION_INVALID_REQUEST.to_string(),
            ),
        );
    };
    let request = match serde_json::from_str::<ParentRuntimeIntentRequest>(request_text) {
        Ok(request) => request,
        Err(_) => {
            return result::rejected(
                command,
                ParentRuntimeIntentIngressKind::Unknown,
                IngressReason(
                    constants::parent_controller::INGRESS_REJECTION_INVALID_REQUEST.to_string(),
                ),
            );
        }
    };
    // The WebSocket path currently has no authenticated session authority
    // producer. Keep the ingress fail-closed until that owner can bind a
    // session-issued authority to this envelope; never trust source/route or
    // caller-provided policy/tracking state as a substitute.
    result::manual_required(
        command,
        request.kind(),
        IngressReason(
            constants::parent_controller::INGRESS_NO_CLAIM_AUTHENTICATED_SESSION_UNAVAILABLE
                .to_string(),
        ),
    )
}
