use ocentra_parent_agent_protocol::{
    constants,
    tracking::runtime_event::TrackingChildCheckInRequestedEvent,
    transport::{
        AgentCommandEnvelope, ParentRuntimeIntentIngressClaimState, ParentRuntimeIntentIngressKind,
        ParentRuntimeIntentIngressResult,
    },
};
use ocentra_parent_runtime_core::tracking_dispatch::{
    ChildRuntimePublishState, ParentRuntimeChangeRequest,
};

use super::{journal, result, IngressEventId, IngressReason};

pub(super) async fn execute(
    command: &AgentCommandEnvelope,
    dispatch_request: ParentRuntimeChangeRequest,
    event: TrackingChildCheckInRequestedEvent,
    kind: ParentRuntimeIntentIngressKind,
) -> ParentRuntimeIntentIngressResult {
    let event_id = IngressEventId(command.message_id.clone());
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
    if let Err(error) = journal::persist_before_dispatch(&event, metadata).await {
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

    match ocentra_parent_runtime_core::tracking_child_check_in_request_flow::
        publish_parent_tracking_child_check_in_request_event_flow(dispatch_request, &event)
        .await
    {
        Ok(report)
            if report.dispatch_decision.child_runtime_publish_state
                == ChildRuntimePublishState::Publish =>
        {
            result::published(command, kind, event_id)
        }
        Ok(_) => result::manual_required(
            command,
            kind,
            IngressReason(
                constants::parent_controller::INGRESS_NO_CLAIM_DISPATCH_BLOCKED.to_string(),
            ),
            Some(event_id),
            ParentRuntimeIntentIngressClaimState::Claimed,
        ),
        Err(error) => result::unavailable(
            command,
            kind,
            IngressReason(
                constants::parent_controller::INGRESS_NO_CLAIM_EVENTING_UNAVAILABLE.to_string(),
            ),
            Some(IngressReason(error.to_string())),
            ParentRuntimeIntentIngressClaimState::Claimed,
        ),
    }
}
