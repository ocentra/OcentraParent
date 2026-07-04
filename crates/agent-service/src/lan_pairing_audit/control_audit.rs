use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingAuditEventType, LanPairingOptionalText, LanPairingRejectionReason,
    LanParentIntentEnvelope,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::fields::fields_from_pairs;
use crate::lan_pairing::extend_log_fields;
use crate::lan_pairing_audit::audit_payload::{
    evidence_reference_fields, fallback_evidence_reference_fields,
};
use crate::lan_pairing_audit::values::{
    authentication_state_value, intent_kind_value, parent_authority_value, reason_value,
};

pub(super) fn accepted_control_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
) -> LogFields {
    control_audit_fields(
        command,
        LogFieldValue::String(constants::value::LAN_CONTROL_ACCEPTED.to_string()),
        LanPairingAuditEventType::ControlAccepted,
        None,
        Some(intent),
        origin,
    )
}

pub(super) fn selected_route_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
) -> LogFields {
    control_audit_fields(
        command,
        LogFieldValue::String(constants::value::LAN_CONTROL_ACCEPTED.to_string()),
        LanPairingAuditEventType::RouteSelected,
        None,
        Some(intent),
        origin,
    )
}

pub(super) fn revoked_route_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
) -> LogFields {
    control_audit_fields(
        command,
        LogFieldValue::String(constants::value::LAN_CONTROL_ACCEPTED.to_string()),
        LanPairingAuditEventType::PairingRevoked,
        None,
        Some(intent),
        origin,
    )
}

pub(super) fn controller_lease_audit_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
    audit_event_type: LanPairingAuditEventType,
    reason: Option<&LanPairingRejectionReason>,
) -> LogFields {
    control_audit_fields(
        command,
        if reason.is_some() {
            LogFieldValue::String(constants::value::LAN_CONTROL_REJECTED.to_string())
        } else {
            LogFieldValue::String(constants::value::LAN_CONTROL_ACCEPTED.to_string())
        },
        audit_event_type,
        reason,
        Some(intent),
        origin,
    )
}

pub(super) fn rejected_control_audit_fields(
    command: &AgentCommandEnvelope,
    reason: &LanPairingRejectionReason,
    intent: Option<&LanParentIntentEnvelope>,
    origin: &LanPairingOptionalText,
) -> LogFields {
    control_audit_fields(
        command,
        LogFieldValue::String(constants::value::LAN_CONTROL_REJECTED.to_string()),
        LanPairingAuditEventType::ControlRejected,
        Some(reason),
        intent,
        origin,
    )
}

fn control_audit_fields(
    command: &AgentCommandEnvelope,
    state: LogFieldValue,
    audit_event_type: LanPairingAuditEventType,
    reason: Option<&LanPairingRejectionReason>,
    intent: Option<&LanParentIntentEnvelope>,
    origin: &LanPairingOptionalText,
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
    let audit_event_id = intent
        .map(|intent| intent.intent_id.clone())
        .or_else(|| payload_string(constants::field::LAN_INTENT_ID))
        .unwrap_or_else(|| command.message_id.clone());
    let mut pairs = vec![
        (constants::field::LAN_CONTROL_STATE, state),
        (
            constants::field::LAN_AUDIT_EVENT_ID,
            LogFieldValue::String(audit_event_id),
        ),
        (
            constants::field::LAN_AUDIT_EVENT_TYPE,
            audit_event_type_value(audit_event_type),
        ),
        (
            constants::field::LAN_CHILD_DEVICE_ID,
            LogFieldValue::String(command.target.device_id.clone()),
        ),
        (
            constants::field::LAN_AUTHENTICATION_STATE,
            authentication_state_value(reason),
        ),
    ];

    let mut fields = fields_from_pairs(pairs);
    extend_log_fields(
        &mut fields,
        optional_control_audit_fields(command, reason, intent, origin),
    );
    fields
}

fn optional_control_audit_fields(
    command: &AgentCommandEnvelope,
    reason: Option<&LanPairingRejectionReason>,
    intent: Option<&LanParentIntentEnvelope>,
    origin: &LanPairingOptionalText,
) -> LogFields {
    let mut fields = optional_control_identity_fields(command, intent, origin);
    if let Some(reason) = reason {
        fields.insert(
            constants::field::LAN_REJECTION_REASON.to_string(),
            reason_value(reason),
        );
    }
    match intent {
        Some(intent) => extend_log_fields(
            &mut fields,
            evidence_reference_fields(Some(intent.evidence_references.as_slice())),
        ),
        None => extend_log_fields(
            &mut fields,
            fallback_evidence_reference_fields(&command.payload),
        ),
    }
    fields
}

fn optional_control_identity_fields(
    command: &AgentCommandEnvelope,
    intent: Option<&LanParentIntentEnvelope>,
    origin: &LanPairingOptionalText,
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
    push_optional_pair(
        constants::field::LAN_ROUTE_ID,
        intent
            .map(|intent| LogFieldValue::String(intent.route_id.clone()))
            .or_else(|| payload_string(constants::field::LAN_ROUTE_ID).map(LogFieldValue::String)),
    );
    push_optional_pair(
        constants::field::LAN_INTENT_ID,
        intent
            .map(|intent| LogFieldValue::String(intent.intent_id.clone()))
            .or_else(|| payload_string(constants::field::LAN_INTENT_ID).map(LogFieldValue::String)),
    );
    push_optional_pair(
        constants::field::LAN_INTENT_KIND,
        intent
            .map(|intent| intent_kind_value(&intent.intent_kind))
            .or_else(|| {
                payload_string(constants::field::LAN_INTENT_KIND).map(LogFieldValue::String)
            }),
    );
    push_optional_pair(
        constants::field::LAN_PAIRING_ID,
        intent
            .map(|intent| LogFieldValue::String(intent.pairing_id.clone()))
            .or_else(|| {
                payload_string(constants::field::LAN_PAIRING_ID).map(LogFieldValue::String)
            }),
    );
    push_optional_pair(
        constants::field::LAN_CONTROLLER_LEASE_ID,
        intent
            .map(|intent| LogFieldValue::String(intent.controller_lease_id.clone()))
            .or_else(|| {
                payload_string(constants::field::LAN_CONTROLLER_LEASE_ID).map(LogFieldValue::String)
            }),
    );
    push_optional_pair(
        constants::field::LAN_CONTROLLER_DEVICE_ID,
        intent
            .map(|intent| LogFieldValue::String(intent.controller_device_id.clone()))
            .or_else(|| {
                payload_string(constants::field::LAN_CONTROLLER_DEVICE_ID)
                    .map(LogFieldValue::String)
            }),
    );
    push_optional_pair(
        constants::field::LAN_PARENT_ACTOR_ID,
        intent
            .map(|intent| LogFieldValue::String(intent.parent_actor_id.clone()))
            .or_else(|| {
                payload_string(constants::field::LAN_PARENT_ACTOR_ID).map(LogFieldValue::String)
            }),
    );
    push_optional_pair(
        constants::field::LAN_PARENT_AUTHORITY,
        intent
            .map(|intent| parent_authority_value(&intent.parent_authority))
            .or_else(|| {
                payload_string(constants::field::LAN_PARENT_AUTHORITY).map(LogFieldValue::String)
            }),
    );
    push_optional_pair(
        constants::field::ORIGIN,
        origin
            .0
            .as_ref()
            .map(|value| LogFieldValue::String(value.clone()))
            .or_else(|| payload_string(constants::field::ORIGIN).map(LogFieldValue::String)),
    );

    fields_from_pairs(pairs)
}

fn audit_event_type_value(audit_event_type: LanPairingAuditEventType) -> LogFieldValue {
    let value = serde_json::to_value(audit_event_type)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_default();
    LogFieldValue::String(value)
}
