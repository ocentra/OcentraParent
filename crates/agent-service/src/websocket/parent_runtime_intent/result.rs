use ocentra_parent_agent_protocol::{
    transport::{
        AgentCommandEnvelope, ParentRuntimeIntentIngressClaimState, ParentRuntimeIntentIngressKind,
        ParentRuntimeIntentIngressResult, ParentRuntimeIntentIngressState,
    },
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use super::{IngressEventId, IngressReason};

pub(super) fn published(
    command: &AgentCommandEnvelope,
    kind: ParentRuntimeIntentIngressKind,
    event_id: IngressEventId,
) -> ParentRuntimeIntentIngressResult {
    ParentRuntimeIntentIngressResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: command.message_id.clone(),
        intent_kind: kind,
        state: ParentRuntimeIntentIngressState::Published,
        journal_state: ParentRuntimeIntentIngressClaimState::Claimed,
        eventing_publish_state: ParentRuntimeIntentIngressClaimState::Claimed,
        event_id: Some(event_id.0),
        rejection_reason: None,
        no_claim_reason: None,
        child_transport_claimed: false,
    }
}

pub(super) fn rejected(
    command: &AgentCommandEnvelope,
    kind: ParentRuntimeIntentIngressKind,
    reason: IngressReason,
) -> ParentRuntimeIntentIngressResult {
    ParentRuntimeIntentIngressResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: command.message_id.clone(),
        intent_kind: kind,
        state: ParentRuntimeIntentIngressState::Rejected,
        journal_state: ParentRuntimeIntentIngressClaimState::Unclaimed,
        eventing_publish_state: ParentRuntimeIntentIngressClaimState::Unclaimed,
        event_id: None,
        rejection_reason: Some(reason.0),
        no_claim_reason: Some(
            ocentra_parent_agent_protocol::constants::parent_controller::
                INGRESS_REJECTION_INVALID_REQUEST
                .to_string(),
        ),
        child_transport_claimed: false,
    }
}

pub(super) fn manual_required(
    command: &AgentCommandEnvelope,
    kind: ParentRuntimeIntentIngressKind,
    reason: IngressReason,
    event_id: Option<IngressEventId>,
    journal_state: ParentRuntimeIntentIngressClaimState,
) -> ParentRuntimeIntentIngressResult {
    ParentRuntimeIntentIngressResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: command.message_id.clone(),
        intent_kind: kind,
        state: ParentRuntimeIntentIngressState::ManualRequired,
        journal_state,
        eventing_publish_state: ParentRuntimeIntentIngressClaimState::Unclaimed,
        event_id: event_id.map(|value| value.0),
        rejection_reason: None,
        no_claim_reason: Some(reason.0),
        child_transport_claimed: false,
    }
}

pub(super) fn unavailable(
    command: &AgentCommandEnvelope,
    kind: ParentRuntimeIntentIngressKind,
    reason: IngressReason,
    detail: Option<IngressReason>,
    journal_state: ParentRuntimeIntentIngressClaimState,
) -> ParentRuntimeIntentIngressResult {
    ParentRuntimeIntentIngressResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: command.message_id.clone(),
        intent_kind: kind,
        state: ParentRuntimeIntentIngressState::Unavailable,
        journal_state,
        eventing_publish_state: ParentRuntimeIntentIngressClaimState::Unclaimed,
        event_id: None,
        rejection_reason: detail.map(|value| value.0),
        no_claim_reason: Some(reason.0),
        child_transport_claimed: false,
    }
}
