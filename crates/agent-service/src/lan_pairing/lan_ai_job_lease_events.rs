use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LanPairingRejectionReason,
    LanParentIntentEnvelope, LogFieldValue, LogFields, LogLevel,
};

use crate::{
    event_builder::build_event,
    fields::fields_from_pairs,
    lan_pairing::{
        lan_ai_job::lan_ai_provider_fields, lan_ai_route_metadata::lan_ai_household_route_fields,
        LanPairingRuntime,
    },
    lan_pairing_audit::controller_lease_audit_fields,
    lan_pairing_runtime_state::LanAiJobLeaseState,
};

pub(crate) fn lan_ai_job_completed_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: Option<&str>,
    requested_capability: &str,
    lease: &LanAiJobLeaseState,
) -> AgentEventEnvelope {
    let mut payload = controller_lease_audit_fields(
        &command,
        intent,
        origin,
        constants::value::LAN_AUDIT_LAN_AI_JOB_COMPLETED,
        None,
    );
    payload.extend(lan_ai_provider_fields(runtime));
    payload.insert(
        constants::field::LOCAL_AI_CAPABILITY_FLAGS.to_string(),
        LogFieldValue::String(requested_capability.to_string()),
    );
    payload.extend(lan_ai_household_route_fields(
        runtime,
        &command,
        intent,
        requested_capability,
    ));
    payload.extend(lan_ai_job_lease_fields(lease));
    payload.extend(lan_ai_job_result_fields(
        &command,
        intent,
        constants::value::LAN_AI_JOB_STATE_COMPLETED,
        constants::local_ai_runtime::GENERATION_STATE_COMPLETE,
        Some(constants::value::LAN_AI_PROVIDER_RESULT_REDACTED),
    ));
    build_lan_ai_job_event(command, LogLevel::Info, payload)
}

pub(crate) fn lan_ai_job_duplicate_rejected_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: Option<&str>,
    lease: &LanAiJobLeaseState,
) -> AgentEventEnvelope {
    let mut payload = controller_lease_audit_fields(
        &command,
        intent,
        origin,
        constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
        Some(&LanPairingRejectionReason::LanAiJobUnauthorized),
    );
    payload.extend(lan_ai_provider_fields(runtime));
    payload.extend(lan_ai_job_lease_fields(lease));
    payload.extend(lan_ai_job_result_fields(
        &command,
        intent,
        constants::value::LAN_AI_JOB_STATE_REJECTED,
        constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE,
        None,
    ));
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

pub(crate) fn lan_ai_job_lease_state_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: Option<&str>,
    lease: &LanAiJobLeaseState,
) -> AgentEventEnvelope {
    let mut payload = controller_lease_audit_fields(
        &command,
        intent,
        origin,
        constants::value::LAN_AUDIT_LAN_AI_JOB_DEGRADED,
        None,
    );
    payload.extend(lan_ai_provider_fields(runtime));
    payload.extend(lan_ai_job_lease_fields(lease));
    payload.extend(lan_ai_job_result_fields(
        &command,
        intent,
        constants::value::LAN_AI_JOB_STATE_DEGRADED,
        constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE,
        None,
    ));
    build_lan_ai_job_event(command, LogLevel::Warn, payload)
}

fn lan_ai_job_result_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    job_state: &'static str,
    generation_state: &'static str,
    output_text: Option<&'static str>,
) -> LogFields {
    let mut fields = fields_from_pairs(vec![
        (
            constants::field::LAN_AI_JOB_ID,
            LogFieldValue::String(lan_ai_job_id(command, intent)),
        ),
        (
            constants::field::LAN_AI_JOB_STATUS,
            LogFieldValue::String(constants::value::LAN_AI_JOB_STATE_ACCEPTED.to_string()),
        ),
        (
            constants::field::LAN_AI_JOB_STATE,
            LogFieldValue::String(job_state.to_string()),
        ),
        (
            constants::field::LOCAL_AI_RESULT_ID,
            LogFieldValue::String(local_ai_result_id(intent)),
        ),
        (
            constants::field::LOCAL_AI_GENERATION_STATE,
            LogFieldValue::String(generation_state.to_string()),
        ),
    ]);
    if let Some(output_text) = output_text {
        fields.insert(
            constants::field::LOCAL_AI_OUTPUT_TEXT.to_string(),
            LogFieldValue::String(output_text.to_string()),
        );
    }
    fields
}

fn lan_ai_job_lease_fields(lease: &LanAiJobLeaseState) -> LogFields {
    let mut fields = fields_from_pairs(vec![
        (
            constants::field::LAN_AI_CLAIM_ID,
            LogFieldValue::String(lease.claim_id.clone()),
        ),
        (
            constants::field::LAN_AI_LEASE_ID,
            LogFieldValue::String(lease.lease_id.clone()),
        ),
        (
            constants::field::LAN_AI_LEASE_STATE,
            LogFieldValue::String(lease.lease_state.to_string()),
        ),
        (
            constants::field::LAN_AI_LEASE_ATTEMPT_COUNT,
            LogFieldValue::Number(lease.attempt_count as f64),
        ),
    ]);
    if let Some(reason) = lease.dead_letter_reason {
        fields.insert(
            constants::field::LAN_AI_DEAD_LETTER_REASON.to_string(),
            LogFieldValue::String(reason.to_string()),
        );
    }
    fields
}

fn build_lan_ai_job_event(
    command: AgentCommandEnvelope,
    level: LogLevel,
    payload: LogFields,
) -> AgentEventEnvelope {
    build_event(
        constants::lan_pairing::EVENT_LAN_AI_JOB_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLanAiJobReported,
        level,
        payload,
        None,
    )
}

fn lan_ai_job_id(command: &AgentCommandEnvelope, intent: &LanParentIntentEnvelope) -> String {
    command
        .payload
        .get(constants::field::LAN_AI_JOB_ID)
        .and_then(|value| match value {
            LogFieldValue::String(value) if !value.is_empty() => Some(value.as_str()),
            _ => None,
        })
        .unwrap_or(intent.intent_id.as_str())
        .to_string()
}

fn local_ai_result_id(intent: &LanParentIntentEnvelope) -> String {
    let mut result_id = String::from(constants::local_ai_runtime::RESULT_ID_PREFIX);
    result_id.push_str(&intent.intent_id);
    result_id
}
