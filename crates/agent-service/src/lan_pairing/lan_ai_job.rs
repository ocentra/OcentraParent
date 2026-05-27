use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LanPairingRejectionReason,
    LanParentIntentEnvelope, LogFieldValue, LogFields, LogLevel,
};

use crate::{
    event_builder::build_event,
    fields::fields_from_pairs,
    lan_pairing::{
        authority::{validate_authorized_lan_ai_job, validate_observer_read_intent},
        validate_command_target, LanPairingRuntime,
    },
    lan_pairing_audit::{controller_lease_audit_fields, rejected_control_audit_fields},
    lan_pairing_payload::parse_intent,
};

pub(crate) fn lan_ai_provider_status_get(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let observed_origin = origin.as_deref();
    match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_observer_read_intent(&runtime, observed_origin, &intent))
        {
            Ok(()) => {
                let audit_fields = controller_lease_audit_fields(
                    &command,
                    &intent,
                    observed_origin,
                    constants::value::LAN_AUDIT_LAN_AI_PROVIDER_ADVERTISED,
                    None,
                );
                let mut event = crate::lan_pairing_status::pairing_status_event(&runtime, command);
                event.payload.extend(audit_fields);
                event.payload.extend(lan_ai_provider_fields());
                event
            }
            Err(reason) => lan_ai_rejection_event(
                command,
                reason,
                Some(&intent),
                observed_origin,
                constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
            ),
        },
        Err(reason) => lan_ai_rejection_event(
            command,
            reason,
            None,
            observed_origin,
            constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
        ),
    }
}

pub(crate) fn lan_ai_job_submit(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let observed_origin = origin.as_deref();
    match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_authorized_lan_ai_job(&runtime, observed_origin, &intent))
        {
            Ok(()) => lan_ai_job_degraded_event(command, &intent, observed_origin),
            Err(reason) => lan_ai_rejection_event(
                command,
                reason,
                Some(&intent),
                observed_origin,
                constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
            ),
        },
        Err(reason) => lan_ai_rejection_event(
            command,
            reason,
            None,
            observed_origin,
            constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
        ),
    }
}

fn lan_ai_job_degraded_event(
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: Option<&str>,
) -> AgentEventEnvelope {
    let mut payload = controller_lease_audit_fields(
        &command,
        intent,
        origin,
        constants::value::LAN_AUDIT_LAN_AI_JOB_DEGRADED,
        None,
    );
    payload.extend(lan_ai_provider_fields());
    payload.extend(lan_ai_job_fields(&command, intent));
    build_event(
        constants::lan_pairing::EVENT_LAN_AI_JOB_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLanAiJobReported,
        LogLevel::Warn,
        payload,
        None,
    )
}

fn lan_ai_rejection_event(
    command: AgentCommandEnvelope,
    reason: LanPairingRejectionReason,
    intent: Option<&LanParentIntentEnvelope>,
    origin: Option<&str>,
    audit_event_type: &'static str,
) -> AgentEventEnvelope {
    let mut payload = match intent {
        Some(intent) => {
            controller_lease_audit_fields(&command, intent, origin, audit_event_type, Some(&reason))
        }
        None => rejected_control_audit_fields(&command, &reason, None, origin),
    };
    payload.extend(lan_ai_provider_fields());
    build_event(
        constants::event_id::COMMAND_REJECTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        payload,
        None,
    )
}

fn lan_ai_provider_fields() -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_AI_PROVIDER_STATUS,
            LogFieldValue::String(constants::value::LAN_AI_PROVIDER_STATUS_UNAVAILABLE.to_string()),
        ),
        (
            constants::field::LOCAL_AI_PROVIDER_ID,
            LogFieldValue::String(
                constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED.to_string(),
            ),
        ),
        (
            constants::field::LOCAL_AI_EXECUTION_STATE,
            LogFieldValue::String(
                constants::local_ai_runtime::EXECUTION_STATE_DISABLED.to_string(),
            ),
        ),
        (
            constants::field::LOCAL_AI_PROVIDER_SOURCE,
            LogFieldValue::String(
                constants::local_ai_runtime::PROVIDER_SOURCE_UNAVAILABLE.to_string(),
            ),
        ),
        (
            constants::field::LOCAL_AI_ADAPTER_READINESS_STATE,
            LogFieldValue::String(
                constants::local_ai_runtime::ADAPTER_READINESS_STATE_NOT_READY.to_string(),
            ),
        ),
        (
            constants::field::LOCAL_AI_CAPABILITY_FLAGS,
            LogFieldValue::String(constants::local_ai_runtime::CAPABILITY_FLAGS_NONE.to_string()),
        ),
        (
            constants::field::LOCAL_AI_UNAVAILABLE_REASON,
            LogFieldValue::String(
                constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string(),
            ),
        ),
    ])
}

fn lan_ai_job_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_AI_JOB_ID,
            LogFieldValue::String(lan_ai_job_id(command, intent)),
        ),
        (
            constants::field::LAN_AI_JOB_STATE,
            LogFieldValue::String(constants::value::LAN_AI_JOB_STATE_DEGRADED.to_string()),
        ),
        (
            constants::field::LOCAL_AI_RESULT_ID,
            LogFieldValue::String(local_ai_result_id(intent)),
        ),
        (
            constants::field::LOCAL_AI_GENERATION_STATE,
            LogFieldValue::String(
                constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE.to_string(),
            ),
        ),
    ])
}

fn lan_ai_job_id(command: &AgentCommandEnvelope, intent: &LanParentIntentEnvelope) -> String {
    payload_string(&command.payload, constants::field::LAN_AI_JOB_ID)
        .unwrap_or(intent.intent_id.as_str())
        .to_string()
}

fn local_ai_result_id(intent: &LanParentIntentEnvelope) -> String {
    let mut result_id = String::from(constants::local_ai_runtime::RESULT_ID_PREFIX);
    result_id.push_str(&intent.intent_id);
    result_id
}

fn payload_string<'a>(fields: &'a LogFields, key: &str) -> Option<&'a str> {
    fields.get(key).and_then(|value| match value {
        LogFieldValue::String(value) if !value.is_empty() => Some(value.as_str()),
        _ => None,
    })
}
