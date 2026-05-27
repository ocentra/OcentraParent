use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, LanPairingProof, LanPairingRejectionReason,
    LanParentIntentEnvelope, LogFieldValue, LogFields, ParentEvidenceReference,
};

use self::values::{
    authentication_state_value, intent_kind_value, parent_authority_value, reason_value,
};
use crate::fields::fields_from_pairs;
use crate::lan_pairing::LanPairingChallengeState;

mod values;

pub(crate) fn accepted_control_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: Option<&str>,
) -> LogFields {
    control_audit_fields(
        command,
        constants::value::LAN_CONTROL_ACCEPTED,
        constants::value::LAN_AUDIT_CONTROL_ACCEPTED,
        None,
        Some(intent),
        origin,
    )
}

pub(crate) fn accepted_pairing_audit_fields(
    command: &AgentCommandEnvelope,
    proof: &LanPairingProof,
) -> LogFields {
    pairing_audit_fields(
        command,
        constants::value::LAN_CONTROL_ACCEPTED,
        constants::value::LAN_AUDIT_PAIRING_PROOF_ACCEPTED,
        None,
        Some(proof),
    )
}

pub(crate) fn rejected_pairing_audit_fields(
    command: &AgentCommandEnvelope,
    reason: &LanPairingRejectionReason,
) -> LogFields {
    pairing_audit_fields(
        command,
        constants::value::LAN_CONTROL_REJECTED,
        constants::value::LAN_AUDIT_PAIRING_PROOF_REJECTED,
        Some(reason),
        None,
    )
}

pub(crate) fn selected_route_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: Option<&str>,
) -> LogFields {
    control_audit_fields(
        command,
        constants::value::LAN_CONTROL_ACCEPTED,
        constants::value::LAN_AUDIT_ROUTE_SELECTED,
        None,
        Some(intent),
        origin,
    )
}

pub(crate) fn revoked_route_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: Option<&str>,
) -> LogFields {
    control_audit_fields(
        command,
        constants::value::LAN_CONTROL_ACCEPTED,
        constants::value::LAN_AUDIT_PAIRING_REVOKED,
        None,
        Some(intent),
        origin,
    )
}

pub(crate) fn controller_lease_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: Option<&str>,
    audit_event_type: &'static str,
    reason: Option<&LanPairingRejectionReason>,
) -> LogFields {
    control_audit_fields(
        command,
        if reason.is_some() {
            constants::value::LAN_CONTROL_REJECTED
        } else {
            constants::value::LAN_CONTROL_ACCEPTED
        },
        audit_event_type,
        reason,
        Some(intent),
        origin,
    )
}

pub(crate) fn rejected_control_audit_fields(
    command: &AgentCommandEnvelope,
    reason: &LanPairingRejectionReason,
    intent: Option<&LanParentIntentEnvelope>,
    origin: Option<&str>,
) -> LogFields {
    control_audit_fields(
        command,
        constants::value::LAN_CONTROL_REJECTED,
        constants::value::LAN_AUDIT_CONTROL_REJECTED,
        Some(reason),
        intent,
        origin,
    )
}

pub(crate) fn challenge_issued_audit_fields(challenge: &LanPairingChallengeState) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_AUDIT_EVENT_ID,
            LogFieldValue::String(challenge.challenge_id.clone()),
        ),
        (
            constants::field::LAN_AUDIT_EVENT_TYPE,
            LogFieldValue::String(constants::value::LAN_AUDIT_PAIRING_CHALLENGE_ISSUED.to_string()),
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

fn control_audit_fields(
    command: &AgentCommandEnvelope,
    state: &str,
    audit_event_type: &str,
    reason: Option<&LanPairingRejectionReason>,
    intent: Option<&LanParentIntentEnvelope>,
    origin: Option<&str>,
) -> LogFields {
    let audit_event_id = intent
        .map(|intent| intent.intent_id.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::LAN_INTENT_ID))
        .unwrap_or(command.message_id.as_str());
    let mut pairs = vec![
        (
            constants::field::LAN_CONTROL_STATE,
            LogFieldValue::String(state.to_string()),
        ),
        (
            constants::field::LAN_AUDIT_EVENT_ID,
            LogFieldValue::String(audit_event_id.to_string()),
        ),
        (
            constants::field::LAN_AUDIT_EVENT_TYPE,
            LogFieldValue::String(audit_event_type.to_string()),
        ),
        (
            constants::field::LAN_CHILD_DEVICE_ID,
            LogFieldValue::String(command.target.device_id.clone()),
        ),
        (
            constants::field::LAN_AUTHENTICATION_STATE,
            LogFieldValue::String(authentication_state_value(reason).to_string()),
        ),
    ];

    pairs.extend(optional_control_audit_pairs(
        command, reason, intent, origin,
    ));
    fields_from_pairs(pairs)
}

fn optional_control_audit_pairs(
    command: &AgentCommandEnvelope,
    reason: Option<&LanPairingRejectionReason>,
    intent: Option<&LanParentIntentEnvelope>,
    origin: Option<&str>,
) -> Vec<(&'static str, LogFieldValue)> {
    let mut pairs = optional_control_identity_pairs(command, intent, origin);
    if let Some(reason) = reason {
        pairs.push((
            constants::field::LAN_REJECTION_REASON,
            LogFieldValue::String(reason_value(reason).to_string()),
        ));
    }
    match intent {
        Some(intent) => pairs.extend(evidence_reference_pairs(Some(
            intent.evidence_references.as_slice(),
        ))),
        None => pairs.extend(fallback_evidence_reference_pairs(&command.payload)),
    }

    pairs
}

fn optional_control_identity_pairs(
    command: &AgentCommandEnvelope,
    intent: Option<&LanParentIntentEnvelope>,
    origin: Option<&str>,
) -> Vec<(&'static str, LogFieldValue)> {
    let mut pairs = Vec::new();
    push_optional_control_pair(
        &mut pairs,
        constants::field::LAN_ROUTE_ID,
        intent
            .map(|intent| intent.route_id.as_str())
            .or_else(|| payload_string(&command.payload, constants::field::LAN_ROUTE_ID)),
    );
    push_optional_control_pair(
        &mut pairs,
        constants::field::LAN_INTENT_ID,
        intent
            .map(|intent| intent.intent_id.as_str())
            .or_else(|| payload_string(&command.payload, constants::field::LAN_INTENT_ID)),
    );
    push_optional_control_pair(
        &mut pairs,
        constants::field::LAN_INTENT_KIND,
        intent
            .map(|intent| intent_kind_value(&intent.intent_kind))
            .or_else(|| payload_string(&command.payload, constants::field::LAN_INTENT_KIND)),
    );
    push_optional_control_pair(
        &mut pairs,
        constants::field::LAN_PAIRING_ID,
        intent
            .map(|intent| intent.pairing_id.as_str())
            .or_else(|| payload_string(&command.payload, constants::field::LAN_PAIRING_ID)),
    );
    push_optional_control_pair(
        &mut pairs,
        constants::field::LAN_CONTROLLER_LEASE_ID,
        intent
            .map(|intent| intent.controller_lease_id.as_str())
            .or_else(|| {
                payload_string(&command.payload, constants::field::LAN_CONTROLLER_LEASE_ID)
            }),
    );
    push_optional_control_pair(
        &mut pairs,
        constants::field::LAN_CONTROLLER_DEVICE_ID,
        intent
            .map(|intent| intent.controller_device_id.as_str())
            .or_else(|| {
                payload_string(&command.payload, constants::field::LAN_CONTROLLER_DEVICE_ID)
            }),
    );
    push_optional_control_pair(
        &mut pairs,
        constants::field::LAN_PARENT_ACTOR_ID,
        intent
            .map(|intent| intent.parent_actor_id.as_str())
            .or_else(|| payload_string(&command.payload, constants::field::LAN_PARENT_ACTOR_ID)),
    );
    push_optional_control_pair(
        &mut pairs,
        constants::field::LAN_PARENT_AUTHORITY,
        intent
            .map(|intent| parent_authority_value(&intent.parent_authority))
            .or_else(|| payload_string(&command.payload, constants::field::LAN_PARENT_AUTHORITY)),
    );
    push_optional_control_pair(
        &mut pairs,
        constants::field::ORIGIN,
        origin.or_else(|| payload_string(&command.payload, constants::field::ORIGIN)),
    );

    pairs
}

fn push_optional_control_pair(
    pairs: &mut Vec<(&'static str, LogFieldValue)>,
    field: &'static str,
    value: Option<&str>,
) {
    if let Some(value) = value {
        pairs.push((field, LogFieldValue::String(value.to_string())));
    }
}

fn pairing_audit_fields(
    command: &AgentCommandEnvelope,
    state: &str,
    audit_event_type: &str,
    reason: Option<&LanPairingRejectionReason>,
    proof: Option<&LanPairingProof>,
) -> LogFields {
    let mut pairs = vec![
        (
            constants::field::LAN_CONTROL_STATE,
            LogFieldValue::String(state.to_string()),
        ),
        (
            constants::field::LAN_AUDIT_EVENT_ID,
            LogFieldValue::String(command.message_id.clone()),
        ),
        (
            constants::field::LAN_AUDIT_EVENT_TYPE,
            LogFieldValue::String(audit_event_type.to_string()),
        ),
        (
            constants::field::LAN_CHILD_DEVICE_ID,
            LogFieldValue::String(pairing_child_device_id(command, proof)),
        ),
    ];
    pairs.extend(optional_pairing_audit_pairs(command, reason, proof));
    fields_from_pairs(pairs)
}

fn optional_pairing_audit_pairs(
    command: &AgentCommandEnvelope,
    reason: Option<&LanPairingRejectionReason>,
    proof: Option<&LanPairingProof>,
) -> Vec<(&'static str, LogFieldValue)> {
    let route_id = proof
        .map(|proof| proof.route_id.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::LAN_ROUTE_ID));
    let pairing_id = proof
        .map(|proof| proof.pairing_id.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::LAN_PAIRING_ID));
    let parent_device_id = proof
        .map(|proof| proof.parent_device_id.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::LAN_PARENT_DEVICE_ID));
    let observed_origin = proof
        .map(|proof| proof.origin.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::ORIGIN));
    let mut pairs = Vec::new();

    if let Some(value) = route_id {
        pairs.push((
            constants::field::LAN_ROUTE_ID,
            LogFieldValue::String(value.to_string()),
        ));
    }
    if let Some(value) = pairing_id {
        pairs.push((
            constants::field::LAN_PAIRING_ID,
            LogFieldValue::String(value.to_string()),
        ));
    }
    if let Some(value) = parent_device_id {
        pairs.push((
            constants::field::LAN_PARENT_DEVICE_ID,
            LogFieldValue::String(value.to_string()),
        ));
    }
    if let Some(value) = observed_origin {
        pairs.push((
            constants::field::ORIGIN,
            LogFieldValue::String(value.to_string()),
        ));
    }
    if let Some(reason) = reason {
        pairs.push((
            constants::field::LAN_AUTHENTICATION_STATE,
            LogFieldValue::String(authentication_state_value(Some(reason)).to_string()),
        ));
        pairs.push((
            constants::field::LAN_REJECTION_REASON,
            LogFieldValue::String(reason_value(reason).to_string()),
        ));
    }
    pairs.extend(fallback_evidence_reference_pairs(&command.payload));

    pairs
}

fn evidence_reference_pairs(
    evidence_references: Option<&[ParentEvidenceReference]>,
) -> Vec<(&'static str, LogFieldValue)> {
    let Some(evidence_references) = evidence_references else {
        return Vec::new();
    };
    let evidence_reference_ids = evidence_references
        .iter()
        .map(|reference| reference.evidence_reference_id.as_str())
        .collect::<Vec<_>>()
        .join(&constants::delimiter::LIST.to_string());
    vec![
        (
            constants::field::LAN_EVIDENCE_REFERENCE_COUNT,
            LogFieldValue::Number(evidence_references.len() as f64),
        ),
        (
            constants::field::LAN_EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(evidence_reference_ids),
        ),
    ]
}

fn fallback_evidence_reference_pairs(fields: &LogFields) -> Vec<(&'static str, LogFieldValue)> {
    let Some(evidence_reference_ids) =
        payload_string(fields, constants::field::LAN_EVIDENCE_REFERENCE_IDS)
    else {
        return Vec::new();
    };
    let evidence_reference_count = evidence_reference_ids
        .split(constants::delimiter::LIST)
        .filter(|evidence_id| !evidence_id.is_empty())
        .count();
    vec![
        (
            constants::field::LAN_EVIDENCE_REFERENCE_COUNT,
            LogFieldValue::Number(evidence_reference_count as f64),
        ),
        (
            constants::field::LAN_EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(evidence_reference_ids.to_string()),
        ),
    ]
}

fn pairing_child_device_id(
    command: &AgentCommandEnvelope,
    proof: Option<&LanPairingProof>,
) -> String {
    proof
        .map(|proof| proof.child_device_id.clone())
        .unwrap_or_else(|| command.target.device_id.clone())
}

fn payload_string<'a>(fields: &'a LogFields, key: &str) -> Option<&'a str> {
    fields.get(key).and_then(|value| match value {
        LogFieldValue::String(value) if !value.is_empty() => Some(value.as_str()),
        _ => None,
    })
}
