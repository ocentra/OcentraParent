use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingAuditEventType, LanPairingProof, LanPairingRejectionReason, LanSignedChildAgentClaim,
    LanSignedChildAgentMessageKind,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::fields::fields_from_pairs;
use crate::lan_pairing::{extend_log_fields, LanPairingChallengeState};
use crate::lan_pairing_audit::audit_payload::fallback_evidence_reference_fields;
use crate::lan_pairing_audit::values::{authentication_state_value, reason_value};

pub(super) fn accepted_pairing_audit_fields(
    command: &AgentCommandEnvelope,
    proof: &LanPairingProof,
) -> LogFields {
    pairing_audit_fields(
        command,
        LogFieldValue::String(constants::value::LAN_CONTROL_ACCEPTED.to_string()),
        LanPairingAuditEventType::PairingProofAccepted,
        None,
        Some(proof),
    )
}

pub(super) fn rejected_pairing_audit_fields(
    command: &AgentCommandEnvelope,
    reason: &LanPairingRejectionReason,
) -> LogFields {
    pairing_audit_fields(
        command,
        LogFieldValue::String(constants::value::LAN_CONTROL_REJECTED.to_string()),
        LanPairingAuditEventType::PairingProofRejected,
        Some(reason),
        None,
    )
}

pub(super) fn challenge_issued_audit_fields(challenge: &LanPairingChallengeState) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_AUDIT_EVENT_ID,
            LogFieldValue::String(challenge.challenge_id.clone()),
        ),
        (
            constants::field::LAN_AUDIT_EVENT_TYPE,
            audit_event_type_value(LanPairingAuditEventType::PairingChallengeIssued),
        ),
        (
            constants::field::LAN_CHALLENGE_ID,
            LogFieldValue::String(challenge.challenge_id.clone()),
        ),
        (
            constants::field::LAN_CHILD_DEVICE_ID,
            LogFieldValue::String(challenge.child_device_id.clone()),
        ),
        (
            constants::field::LAN_PARENT_DEVICE_ID,
            LogFieldValue::String(challenge.parent_device_id.clone()),
        ),
        (
            constants::field::LAN_ROUTE_ID,
            LogFieldValue::String(challenge.route_id.clone()),
        ),
        (
            constants::field::ORIGIN,
            LogFieldValue::String(challenge.origin.clone()),
        ),
        (
            constants::field::LAN_PROOF_DIGEST,
            LogFieldValue::String(challenge.proof_digest.clone()),
        ),
        (
            constants::field::STARTED_AT,
            LogFieldValue::String(challenge.issued_at.clone()),
        ),
        (
            constants::field::STALE_AT,
            LogFieldValue::String(challenge.expires_at.clone()),
        ),
    ])
}

pub(super) fn signed_child_agent_audit_fields(claim: &LanSignedChildAgentClaim) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_SIGNED_CHILD_AGENT_VERIFICATION,
            LogFieldValue::String(
                constants::value::LAN_SIGNED_CHILD_AGENT_VERIFICATION_ACCEPTED.to_string(),
            ),
        ),
        (
            constants::field::LAN_SIGNED_CHILD_AGENT_STATUS,
            LogFieldValue::String(
                constants::lan_pairing::PRODUCTION_PROOF_STATE_MANUAL_REQUIRED.to_string(),
            ),
        ),
        (
            constants::field::LAN_SIGNED_CHILD_AGENT_MESSAGE_KIND,
            signed_child_agent_message_kind_value(&claim.message_kind),
        ),
        (
            constants::field::LAN_CHILD_DEVICE_ID,
            LogFieldValue::String(claim.child_device_id.clone()),
        ),
        (
            constants::field::LAN_PARENT_DEVICE_ID,
            LogFieldValue::String(claim.parent_device_id.clone()),
        ),
        (
            constants::field::LAN_ROUTE_ID,
            LogFieldValue::String(claim.route_id.clone()),
        ),
    ])
}

fn pairing_audit_fields(
    command: &AgentCommandEnvelope,
    state: LogFieldValue,
    audit_event_type: LanPairingAuditEventType,
    reason: Option<&LanPairingRejectionReason>,
    proof: Option<&LanPairingProof>,
) -> LogFields {
    let pairs = vec![
        (constants::field::LAN_CONTROL_STATE, state),
        (
            constants::field::LAN_AUDIT_EVENT_ID,
            LogFieldValue::String(command.message_id.clone()),
        ),
        (
            constants::field::LAN_AUDIT_EVENT_TYPE,
            audit_event_type_value(audit_event_type),
        ),
        (
            constants::field::LAN_CHILD_DEVICE_ID,
            LogFieldValue::String(
                proof
                    .map(|proof| proof.child_device_id.clone())
                    .unwrap_or_else(|| command.target.device_id.clone()),
            ),
        ),
    ];
    let mut fields = fields_from_pairs(pairs);
    extend_log_fields(
        &mut fields,
        optional_pairing_audit_fields(command, reason, proof),
    );
    fields
}

fn optional_pairing_audit_fields(
    command: &AgentCommandEnvelope,
    reason: Option<&LanPairingRejectionReason>,
    proof: Option<&LanPairingProof>,
) -> LogFields {
    let payload_string = |field_name: &'static str| -> Option<String> {
        command
            .payload
            .get(field_name)
            .and_then(|value| match value {
                LogFieldValue::String(value) if !value.is_empty() => Some(value.to_string()),
                _ => None,
            })
    };
    let mut pairs = Vec::new();
    let mut push_optional_pair = |field: &'static str, value: Option<LogFieldValue>| {
        if let Some(value) = value {
            pairs.push((field, value));
        }
    };
    for (field, value) in [
        (
            constants::field::LAN_ROUTE_ID,
            proof.map(|proof| proof.route_id.clone()),
        ),
        (
            constants::field::LAN_PAIRING_ID,
            proof.map(|proof| proof.pairing_id.clone()),
        ),
        (
            constants::field::LAN_PARENT_DEVICE_ID,
            proof.map(|proof| proof.parent_device_id.clone()),
        ),
        (
            constants::field::ORIGIN,
            proof.map(|proof| proof.origin.clone()),
        ),
    ] {
        push_optional_pair(
            field,
            value
                .or_else(|| payload_string(field))
                .map(LogFieldValue::String),
        );
    }
    push_optional_pair(
        constants::field::LAN_AUTHENTICATION_STATE,
        reason.map(|reason| authentication_state_value(Some(reason))),
    );
    push_optional_pair(
        constants::field::LAN_REJECTION_REASON,
        reason.map(reason_value),
    );

    let mut fields = fields_from_pairs(pairs);
    extend_log_fields(
        &mut fields,
        fallback_evidence_reference_fields(&command.payload),
    );
    fields
}

fn signed_child_agent_message_kind_value(
    message_kind: &LanSignedChildAgentMessageKind,
) -> LogFieldValue {
    let value = serde_json::to_value(message_kind)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_default();
    LogFieldValue::String(value)
}

fn audit_event_type_value(audit_event_type: LanPairingAuditEventType) -> LogFieldValue {
    let value = serde_json::to_value(audit_event_type)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_default();
    LogFieldValue::String(value)
}
