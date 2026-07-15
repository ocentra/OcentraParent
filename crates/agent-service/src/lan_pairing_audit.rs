use ocentra_parent_agent_protocol::lan_pairing::LanPairingAuditEventType;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProof;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentClaim;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::lan_pairing::LanPairingChallengeState;

#[path = "lan_pairing_audit/audit_payload.rs"]
mod audit_payload;
#[path = "lan_pairing_audit/control_audit.rs"]
mod control_audit;
#[path = "lan_pairing_audit/pairing_audit.rs"]
mod pairing_audit;
#[path = "lan_pairing_audit/values.rs"]
mod values;

pub(crate) fn accepted_control_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
) -> LogFields {
    control_audit::accepted_control_audit_fields(command, intent, origin)
}

pub(crate) fn accepted_pairing_audit_fields(
    command: &AgentCommandEnvelope,
    proof: &LanPairingProof,
) -> LogFields {
    pairing_audit::accepted_pairing_audit_fields(command, proof)
}

pub(crate) fn rejected_pairing_audit_fields(
    command: &AgentCommandEnvelope,
    reason: &LanPairingRejectionReason,
) -> LogFields {
    pairing_audit::rejected_pairing_audit_fields(command, reason)
}

pub(crate) fn selected_route_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
) -> LogFields {
    control_audit::selected_route_audit_fields(command, intent, origin)
}

pub(crate) fn revoked_route_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
) -> LogFields {
    control_audit::revoked_route_audit_fields(command, intent, origin)
}

pub(crate) fn controller_lease_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
    audit_event_type: LanPairingAuditEventType,
    reason: Option<&LanPairingRejectionReason>,
) -> LogFields {
    control_audit::controller_lease_audit_fields(command, intent, origin, audit_event_type, reason)
}

pub(crate) fn rejected_control_audit_fields(
    command: &AgentCommandEnvelope,
    reason: &LanPairingRejectionReason,
    intent: Option<&LanParentIntentEnvelope>,
    origin: &LanPairingOptionalText,
) -> LogFields {
    control_audit::rejected_control_audit_fields(command, reason, intent, origin)
}

pub(crate) fn challenge_issued_audit_fields(challenge: &LanPairingChallengeState) -> LogFields {
    pairing_audit::challenge_issued_audit_fields(challenge)
}

pub(crate) fn signed_child_agent_audit_fields(claim: &LanSignedChildAgentClaim) -> LogFields {
    pairing_audit::signed_child_agent_audit_fields(claim)
}
