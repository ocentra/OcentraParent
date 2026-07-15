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

#[path = "../../lan_ai_job_submit_transition.rs"]
mod job_transition;

use crate::{
    event_builder::build_event,
    lan_pairing::{
        authority::validate_authorized_lan_ai_job, extend_log_fields,
        runtime_validation::validate_command_target, LanPairingRuntime,
    },
    lan_pairing_audit::{controller_lease_audit_fields, rejected_control_audit_fields},
    lan_pairing_payload::parse_intent,
};

use super::fields::{
    lan_ai_job_fields, lan_ai_provider_fields, lan_ai_provider_fields_for_rejection,
    payload_string, LanAiJobField,
};

pub(crate) fn lan_ai_job_submit(
    runtime: &LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let origin = LanPairingOptionalText(origin.0);
    let observed_origin = origin.0.as_deref();
    match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(runtime, &command, &intent)
            .and_then(|()| validate_authorized_lan_ai_job(runtime, observed_origin, &intent))
        {
            Ok(()) => lan_ai_job_routed_event(runtime, command, &intent, &origin),
            Err(reason) => lan_ai_rejection_event(
                runtime,
                command,
                &reason,
                Some(&intent),
                &origin,
                LanPairingAuditEventType::LanAiJobRejected,
            ),
        },
        Err(reason) => lan_ai_rejection_event(
            runtime,
            command,
            &reason,
            None,
            &origin,
            LanPairingAuditEventType::LanAiJobRejected,
        ),
    }
}

pub(crate) fn lan_ai_rejection_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    reason: &LanPairingRejectionReason,
    intent: Option<&LanParentIntentEnvelope>,
    origin: &LanPairingOptionalText,
    audit_event_type: LanPairingAuditEventType,
) -> AgentEventEnvelope {
    let mut payload = match intent {
        Some(intent) => {
            controller_lease_audit_fields(&command, intent, origin, audit_event_type, Some(reason))
        }
        None => rejected_control_audit_fields(&command, reason, None, origin),
    };
    extend_log_fields(
        &mut payload,
        lan_ai_provider_fields_for_rejection(runtime, reason),
    );
    extend_log_fields(&mut payload, lan_ai_job_rejected_fields(&command, intent));
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

fn lan_ai_job_routed_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
) -> AgentEventEnvelope {
    if !runtime.lan_ai_provider_available() {
        return lan_ai_job_degraded_event(runtime, command, intent, origin);
    }

    let requested_capability = payload_string(&command.payload, &LanAiJobField::CapabilityFlags)
        .unwrap_or_else(|| {
            constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION
                .to_string()
                .into()
        });
    if !runtime.lan_ai_provider_supports_capability(&requested_capability) {
        return lan_ai_unsupported_capability_event(
            runtime,
            command,
            intent,
            origin,
            &requested_capability,
        );
    }

    job_transition::lan_ai_job_transition_event(
        runtime,
        command,
        intent,
        origin,
        &requested_capability,
    )
}

fn lan_ai_job_degraded_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
) -> AgentEventEnvelope {
    let mut payload = controller_lease_audit_fields(
        &command,
        intent,
        origin,
        LanPairingAuditEventType::LanAiJobDegraded,
        None,
    );
    extend_log_fields(&mut payload, lan_ai_provider_fields(runtime));
    extend_log_fields(
        &mut payload,
        lan_ai_job_fields(
            &command,
            intent,
            LanPairingText(constants::value::LAN_AI_JOB_STATE_ACCEPTED.to_string()),
            LanPairingText(constants::value::LAN_AI_JOB_STATE_DEGRADED.to_string()),
            LanPairingText(constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE.to_string()),
            None,
        ),
    );
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

fn lan_ai_unsupported_capability_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
    requested_capability: &LanPairingText,
) -> AgentEventEnvelope {
    let mut event = lan_ai_rejection_event(
        runtime,
        command,
        &LanPairingRejectionReason::LanAiJobUnauthorized,
        Some(intent),
        origin,
        LanPairingAuditEventType::LanAiJobRejected,
    );
    event.payload.insert(
        constants::field::LAN_AI_PROVIDER_ROUTING_STATE.to_string(),
        LogFieldValue::String(
            constants::value::LAN_AI_PROVIDER_ROUTING_UNSUPPORTED_CAPABILITY.to_string(),
        ),
    );
    event.payload.insert(
        constants::field::LOCAL_AI_CAPABILITY_FLAGS.to_string(),
        LogFieldValue::String(requested_capability.0.as_str().to_owned()),
    );
    event
}

fn lan_ai_job_rejected_fields(
    command: &AgentCommandEnvelope,
    intent: Option<&LanParentIntentEnvelope>,
) -> LogFields {
    let mut fields = LogFields::new();
    if let Some(intent) = intent {
        extend_log_fields(
            &mut fields,
            lan_ai_job_fields(
                command,
                intent,
                LanPairingText(constants::value::LAN_AI_JOB_STATE_REJECTED.to_string()),
                LanPairingText(constants::value::LAN_AI_JOB_STATE_REJECTED.to_string()),
                LanPairingText(
                    constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE.to_string(),
                ),
                None,
            ),
        );
    }
    fields
}
