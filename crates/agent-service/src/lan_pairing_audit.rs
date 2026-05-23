use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, LanPairingRejectionReason, LanParentIntentEnvelope,
    LogFieldValue, LogFields,
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
    let route_id = intent
        .map(|intent| intent.route_id.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::LAN_ROUTE_ID));
    let intent_id = intent
        .map(|intent| intent.intent_id.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::LAN_INTENT_ID));
    let pairing_id = intent
        .map(|intent| intent.pairing_id.as_str())
        .or_else(|| payload_string(&command.payload, constants::field::LAN_PAIRING_ID));
    let observed_origin =
        origin.or_else(|| payload_string(&command.payload, constants::field::ORIGIN));

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
    ];

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

    fields_from_pairs(pairs)
}

fn payload_string<'a>(fields: &'a LogFields, key: &str) -> Option<&'a str> {
    fields.get(key).and_then(|value| match value {
        LogFieldValue::String(value) if !value.is_empty() => Some(value.as_str()),
        _ => None,
    })
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
        LanPairingRejectionReason::Revoked => constants::value::LAN_REASON_REVOKED,
        LanPairingRejectionReason::LocalNetworkDisabled => {
            constants::value::LAN_REASON_UNSUPPORTED_ROUTE
        }
        LanPairingRejectionReason::UnsupportedRoute => {
            constants::value::LAN_REASON_UNSUPPORTED_ROUTE
        }
    }
}
