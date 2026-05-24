use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, LanPairingIntentKind, LanPairingProof,
    LanPairingRejectionReason, LanParentIntentEnvelope, LogFieldValue, LogFields,
    ParentEvidenceReference,
};

use crate::fields::fields_from_pairs;

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

fn control_audit_fields(
    command: &AgentCommandEnvelope,
    state: &str,
    audit_event_type: &str,
    reason: Option<&LanPairingRejectionReason>,
    intent: Option<&LanParentIntentEnvelope>,
    origin: Option<&str>,
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
    let route_id = intent
        .map(|intent| intent.route_id.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::LAN_ROUTE_ID));
    let intent_id = intent
        .map(|intent| intent.intent_id.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::LAN_INTENT_ID));
    let intent_kind = intent
        .map(|intent| intent_kind_value(&intent.intent_kind))
        .or_else(|| payload_string(&command.payload, constants::field::LAN_INTENT_KIND));
    let pairing_id = intent
        .map(|intent| intent.pairing_id.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::LAN_PAIRING_ID));
    let observed_origin =
        origin.or_else(|| payload_string(&command.payload, constants::field::ORIGIN));
    let mut pairs = Vec::new();

    if let Some(value) = route_id {
        pairs.push((
            constants::field::LAN_ROUTE_ID,
            LogFieldValue::String(value.to_string()),
        ));
    }
    if let Some(value) = intent_id {
        pairs.push((
            constants::field::LAN_INTENT_ID,
            LogFieldValue::String(value.to_string()),
        ));
    }
    if let Some(value) = intent_kind {
        pairs.push((
            constants::field::LAN_INTENT_KIND,
            LogFieldValue::String(value.to_string()),
        ));
    }
    if let Some(value) = pairing_id {
        pairs.push((
            constants::field::LAN_PAIRING_ID,
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

fn intent_kind_value(intent_kind: &LanPairingIntentKind) -> &'static str {
    match intent_kind {
        LanPairingIntentKind::HealthQuery => constants::value::LAN_INTENT_HEALTH_QUERY,
        LanPairingIntentKind::RuleQuery => constants::value::LAN_INTENT_RULE_QUERY,
        LanPairingIntentKind::RuleUpdate => constants::value::LAN_INTENT_RULE_UPDATE,
        LanPairingIntentKind::ApprovalDecision => constants::value::LAN_INTENT_APPROVAL_DECISION,
        LanPairingIntentKind::ConfigurationUpdate => {
            constants::value::LAN_INTENT_CONFIGURATION_UPDATE
        }
    }
}

fn reason_value(reason: &LanPairingRejectionReason) -> &'static str {
    match reason {
        LanPairingRejectionReason::Anonymous => constants::value::LAN_REASON_ANONYMOUS,
        LanPairingRejectionReason::WrongOrigin => constants::value::LAN_REASON_WRONG_ORIGIN,
        LanPairingRejectionReason::WrongDevice => constants::value::LAN_REASON_WRONG_DEVICE,
        LanPairingRejectionReason::Expired => constants::value::LAN_REASON_EXPIRED,
        LanPairingRejectionReason::Replayed => constants::value::LAN_REASON_REPLAYED,
        LanPairingRejectionReason::Malformed => constants::value::LAN_REASON_MALFORMED,
        LanPairingRejectionReason::Stale => constants::value::LAN_REASON_STALE,
        LanPairingRejectionReason::Offline => constants::value::LAN_REASON_OFFLINE,
        LanPairingRejectionReason::Revoked => constants::value::LAN_REASON_REVOKED,
        LanPairingRejectionReason::LocalNetworkDisabled => {
            constants::value::LAN_REASON_UNSUPPORTED_ROUTE
        }
        LanPairingRejectionReason::UnsupportedRoute => {
            constants::value::LAN_REASON_UNSUPPORTED_ROUTE
        }
        LanPairingRejectionReason::UnselectedDevice => {
            constants::value::LAN_REASON_UNSELECTED_DEVICE
        }
    }
}

fn authentication_state_value(reason: Option<&LanPairingRejectionReason>) -> &'static str {
    match reason {
        None => constants::value::LAN_AUTH_PAIRED,
        Some(LanPairingRejectionReason::Anonymous)
        | Some(LanPairingRejectionReason::Malformed)
        | Some(LanPairingRejectionReason::WrongOrigin) => {
            constants::value::LAN_AUTH_UNAUTHENTICATED
        }
        Some(_) => constants::value::LAN_AUTH_PAIRED,
    }
}
