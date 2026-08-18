use ocentra_eventing::bus::EventBus;
use ocentra_parent_agent_protocol::{
    constants,
    transport::{
        AgentCommandEnvelope, ParentRuntimeIntentIngressClaimState, ParentRuntimeIntentIngressKind,
        ParentRuntimeIntentIngressResult,
    },
};
use ocentra_parent_runtime_core::{
    policy_control_dispatch::{
        ParentPolicyControlAcknowledgementState, ParentRuntimePolicyControlOriginState,
        ParentRuntimePolicyControlPublishState,
    },
    policy_control_update_flow::publish_parent_policy_control_delivery_event_flow,
};
use ocentra_policy_control_core::{
    policy_authority::PolicyEvaluationRequestedEvent,
    policy_delivery::{PolicyDeliveryRecord, PolicyDeliveryTransition},
};

use super::{journal, result, IngressEventId, IngressReason};

pub(super) async fn execute(
    command: &AgentCommandEnvelope,
    queued_delivery: PolicyDeliveryRecord,
    policy_evaluation_event: PolicyEvaluationRequestedEvent,
    child_runtime_transitions: Vec<PolicyDeliveryTransition>,
    child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
    origin_state: ParentRuntimePolicyControlOriginState,
    kind: ParentRuntimeIntentIngressKind,
) -> ParentRuntimeIntentIngressResult {
    let report = match publish_parent_policy_control_delivery_event_flow(
        &queued_delivery,
        &policy_evaluation_event,
        &child_runtime_transitions,
        child_acknowledgement_state,
        origin_state,
    ) {
        Ok(report) => report,
        Err(error) => {
            return result::rejected(command, kind, IngressReason(error.to_string()));
        }
    };
    let event_id = IngressEventId(report.dispatch_event.dispatch_id.as_str().to_string());
    let metadata = match journal::event_metadata(command, &event_id) {
        Ok(metadata) => metadata,
        Err(error) => {
            return result::unavailable(
                command,
                kind,
                IngressReason(
                    constants::parent_controller::INGRESS_NO_CLAIM_JOURNAL_UNAVAILABLE.to_string(),
                ),
                Some(IngressReason(error.to_string())),
                ParentRuntimeIntentIngressClaimState::Unclaimed,
            );
        }
    };
    if let Err(error) =
        journal::persist_before_dispatch(&report.dispatch_event, metadata.clone()).await
    {
        return result::unavailable(
            command,
            kind,
            IngressReason(
                constants::parent_controller::INGRESS_NO_CLAIM_JOURNAL_UNAVAILABLE.to_string(),
            ),
            Some(IngressReason(error.to_string())),
            ParentRuntimeIntentIngressClaimState::Unclaimed,
        );
    }
    let bus = EventBus::new();
    if let Err(error) = bus.publish(report.dispatch_event.clone(), metadata).await {
        return result::unavailable(
            command,
            kind,
            IngressReason(
                constants::parent_controller::INGRESS_NO_CLAIM_EVENTING_UNAVAILABLE.to_string(),
            ),
            Some(IngressReason(error.to_string())),
            ParentRuntimeIntentIngressClaimState::Claimed,
        );
    }

    if report.dispatch_event.decision.child_runtime_publish_state
        == ParentRuntimePolicyControlPublishState::Publish
    {
        result::published(command, kind, event_id)
    } else {
        result::manual_required(
            command,
            kind,
            IngressReason(
                constants::parent_controller::INGRESS_NO_CLAIM_DISPATCH_BLOCKED.to_string(),
            ),
            Some(event_id),
            ParentRuntimeIntentIngressClaimState::Claimed,
        )
    }
}
