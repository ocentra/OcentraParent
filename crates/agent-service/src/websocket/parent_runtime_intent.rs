use ocentra_parent_agent_protocol::{
    constants,
    logging::{LogFieldValue, LogLevel},
    tracking::runtime_event::TrackingChildCheckInRequestedEvent,
    transport::{
        AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
        ParentRuntimeIntentIngressClaimState, ParentRuntimeIntentIngressKind,
        ParentRuntimeIntentIngressResult,
    },
};
use ocentra_parent_runtime_core::{
    policy_control_dispatch::{
        ParentPolicyControlAcknowledgementState, ParentRuntimePolicyControlOriginState,
    },
    tracking_dispatch::ParentRuntimeChangeRequest,
};
use ocentra_policy_control_core::{
    policy_authority::PolicyEvaluationRequestedEvent,
    policy_delivery::{PolicyDeliveryRecord, PolicyDeliveryTransition},
};
use serde::Deserialize;

use crate::{
    event_builder::build_event, fields::fields_from_pairs, json_contract::serialize_json_string,
};

#[path = "parent_runtime_intent/journal.rs"]
mod journal;
#[path = "parent_runtime_intent/policy.rs"]
mod policy;
#[path = "parent_runtime_intent/result.rs"]
mod result;
#[path = "parent_runtime_intent/tracking.rs"]
mod tracking;

#[derive(Debug, Deserialize)]
#[serde(tag = "intentKind", deny_unknown_fields)]
pub(super) enum ParentRuntimeIntentRequest {
    #[serde(rename = "tracking-child-check-in-request")]
    TrackingChildCheckInRequest {
        dispatch_request: ParentRuntimeChangeRequest,
        event: TrackingChildCheckInRequestedEvent,
    },
    #[serde(rename = "policy-control-delivery")]
    PolicyControlDelivery {
        queued_delivery: PolicyDeliveryRecord,
        policy_evaluation_event: PolicyEvaluationRequestedEvent,
        child_runtime_transitions: Vec<PolicyDeliveryTransition>,
        child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
        origin_state: ParentRuntimePolicyControlOriginState,
    },
}

impl ParentRuntimeIntentRequest {
    pub(super) fn kind(&self) -> ParentRuntimeIntentIngressKind {
        match self {
            Self::TrackingChildCheckInRequest { .. } => {
                ParentRuntimeIntentIngressKind::TrackingChildCheckInRequest
            }
            Self::PolicyControlDelivery { .. } => {
                ParentRuntimeIntentIngressKind::PolicyControlDelivery
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct IngressEventId(pub(super) String);

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
    let kind = request.kind();
    if !trusted_local_portal(command) {
        return result::manual_required(
            command,
            kind,
            IngressReason(
                constants::parent_controller::INGRESS_NO_CLAIM_INVALID_SOURCE.to_string(),
            ),
            None,
            ParentRuntimeIntentIngressClaimState::Unclaimed,
        );
    }

    match request {
        ParentRuntimeIntentRequest::TrackingChildCheckInRequest {
            dispatch_request,
            event,
        } => tracking::execute(command, dispatch_request, event, kind).await,
        ParentRuntimeIntentRequest::PolicyControlDelivery {
            queued_delivery,
            policy_evaluation_event,
            child_runtime_transitions,
            child_acknowledgement_state,
            origin_state,
        } => {
            policy::execute(
                command,
                queued_delivery,
                policy_evaluation_event,
                child_runtime_transitions,
                child_acknowledgement_state,
                origin_state,
                kind,
            )
            .await
        }
    }
}

fn trusted_local_portal(command: &AgentCommandEnvelope) -> bool {
    command.source.role == ocentra_parent_agent_protocol::transport::AgentPeerRole::Portal
        && command.target.route == ocentra_parent_agent_protocol::transport::AgentRoute::Localhost
}
