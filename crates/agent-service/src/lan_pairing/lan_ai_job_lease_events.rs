use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingAuditEventType;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    event_builder::build_event,
    fields::fields_from_pairs,
    lan_pairing::{
        extend_log_fields, lan_ai_job_flow::fields::lan_ai_provider_fields,
        lan_ai_route_metadata::lan_ai_household_route_fields, LanPairingRuntime,
    },
    lan_pairing_audit::controller_lease_audit_fields,
    lan_pairing_runtime_state::job_leases::LanAiJobLeaseState,
};

pub(crate) fn lan_ai_job_completed_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
    requested_capability: &LanPairingText,
    lease: &LanAiJobLeaseState,
) -> AgentEventEnvelope {
    let mut payload = controller_lease_audit_fields(
        &command,
        intent,
        origin,
        LanPairingAuditEventType::LanAiJobCompleted,
        None,
    );
    extend_log_fields(&mut payload, lan_ai_provider_fields(runtime));
    payload.insert(
        constants::field::LOCAL_AI_CAPABILITY_FLAGS.to_string(),
        LogFieldValue::String(requested_capability.0.clone()),
    );
    extend_log_fields(
        &mut payload,
        lan_ai_household_route_fields(runtime, &command, intent, requested_capability),
    );
    extend_log_fields(&mut payload, lan_ai_job_lease_fields(lease));
    extend_log_fields(
        &mut payload,
        lan_ai_job_result_fields(
            &command,
            intent,
            LanPairingText(constants::value::LAN_AI_JOB_STATE_COMPLETED.to_string()),
            LanPairingText(constants::local_ai_runtime::GENERATION_STATE_COMPLETE.to_string()),
            Some(LanPairingText(
                constants::value::LAN_AI_PROVIDER_RESULT_REDACTED.to_string(),
            )),
        ),
    );
    build_lan_ai_job_event(command, LogLevel::Info, payload)
}

pub(crate) fn lan_ai_job_duplicate_rejected_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
    lease: &LanAiJobLeaseState,
) -> AgentEventEnvelope {
    let mut payload = controller_lease_audit_fields(
        &command,
        intent,
        origin,
        LanPairingAuditEventType::LanAiJobRejected,
        Some(&LanPairingRejectionReason::LanAiJobUnauthorized),
    );
    extend_log_fields(&mut payload, lan_ai_provider_fields(runtime));
    extend_log_fields(&mut payload, lan_ai_job_lease_fields(lease));
    extend_log_fields(
        &mut payload,
        lan_ai_job_result_fields(
            &command,
            intent,
            LanPairingText(constants::value::LAN_AI_JOB_STATE_REJECTED.to_string()),
            LanPairingText(constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE.to_string()),
            None,
        ),
    );
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
    origin: &LanPairingOptionalText,
    lease: &LanAiJobLeaseState,
) -> AgentEventEnvelope {
    let mut payload = controller_lease_audit_fields(
        &command,
        intent,
        origin,
        LanPairingAuditEventType::LanAiJobDegraded,
        None,
    );
    extend_log_fields(&mut payload, lan_ai_provider_fields(runtime));
    extend_log_fields(&mut payload, lan_ai_job_lease_fields(lease));
    extend_log_fields(
        &mut payload,
        lan_ai_job_result_fields(
            &command,
            intent,
            LanPairingText(constants::value::LAN_AI_JOB_STATE_DEGRADED.to_string()),
            LanPairingText(constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE.to_string()),
            None,
        ),
    );
    build_lan_ai_job_event(command, LogLevel::Warn, payload)
}

fn lan_ai_job_result_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    job_state: LanPairingText,
    generation_state: LanPairingText,
    output_text: Option<LanPairingText>,
) -> LogFields {
    let mut fields = fields_from_pairs(vec![
        (
            constants::field::LAN_AI_JOB_ID,
            LogFieldValue::String(lan_ai_job_id(command, intent).0),
        ),
        (
            constants::field::LAN_AI_JOB_STATUS,
            LogFieldValue::String(constants::value::LAN_AI_JOB_STATE_ACCEPTED.to_string()),
        ),
        (
            constants::field::LAN_AI_JOB_STATE,
            LogFieldValue::String(job_state.0),
        ),
        (
            constants::field::LOCAL_AI_RESULT_ID,
            LogFieldValue::String(local_ai_result_id(intent).0),
        ),
        (
            constants::field::LOCAL_AI_GENERATION_STATE,
            LogFieldValue::String(generation_state.0),
        ),
    ]);
    if let Some(output_text) = output_text {
        fields.insert(
            constants::field::LOCAL_AI_OUTPUT_TEXT.to_string(),
            LogFieldValue::String(output_text.0),
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

enum LanAiJobField {
    JobId,
}

fn lan_ai_job_id(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
) -> LanPairingText {
    payload_string(&command.payload, &LanAiJobField::JobId)
        .unwrap_or_else(|| intent.intent_id.as_str().into())
}

fn local_ai_result_id(intent: &LanParentIntentEnvelope) -> LanPairingText {
    let mut result_id = String::from(constants::local_ai_runtime::RESULT_ID_PREFIX);
    result_id.push_str(&intent.intent_id);
    result_id.into()
}

fn payload_string(fields: &LogFields, field_name: &LanAiJobField) -> Option<LanPairingText> {
    let field_name = match field_name {
        LanAiJobField::JobId => constants::field::LAN_AI_JOB_ID,
    };
    fields.get(field_name).and_then(|value| match value {
        LogFieldValue::String(value) if !value.is_empty() => {
            Some(LanPairingText(value.as_str().to_owned()))
        }
        _ => None,
    })
}
