use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LanPairingRejectionReason,
    LanParentIntentEnvelope, LogFieldValue, LogFields, LogLevel,
};

use crate::{
    event_builder::build_event,
    fields::fields_from_pairs,
    lan_pairing::{
        authority::{validate_authorized_lan_ai_job, validate_observer_read_intent},
        lan_ai_job_lease_events::{
            lan_ai_job_completed_event, lan_ai_job_duplicate_rejected_event,
            lan_ai_job_lease_state_event,
        },
        validate_command_target, LanPairingRuntime,
    },
    lan_pairing_audit::{controller_lease_audit_fields, rejected_control_audit_fields},
    lan_pairing_payload::parse_intent,
    lan_pairing_runtime_state::LanAiJobLeaseTransition,
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
                event.payload.extend(lan_ai_provider_fields(&runtime));
                event
            }
            Err(reason) => lan_ai_rejection_event(
                &runtime,
                command,
                reason,
                Some(&intent),
                observed_origin,
                constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
            ),
        },
        Err(reason) => lan_ai_rejection_event(
            &runtime,
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
            Ok(()) => lan_ai_job_routed_event(runtime, command, &intent, observed_origin),
            Err(reason) => lan_ai_rejection_event(
                &runtime,
                command,
                reason,
                Some(&intent),
                observed_origin,
                constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
            ),
        },
        Err(reason) => lan_ai_rejection_event(
            &runtime,
            command,
            reason,
            None,
            observed_origin,
            constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
        ),
    }
}

fn lan_ai_job_routed_event(
    runtime: LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: Option<&str>,
) -> AgentEventEnvelope {
    if !runtime.lan_ai_provider_available() {
        return lan_ai_job_degraded_event(&runtime, command, intent, origin);
    }

    let requested_capability = payload_string(
        &command.payload,
        constants::field::LOCAL_AI_CAPABILITY_FLAGS,
    )
    .unwrap_or(constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION)
    .to_string();
    if !runtime.lan_ai_provider_supports_capability(&requested_capability) {
        let mut event = lan_ai_rejection_event(
            &runtime,
            command,
            LanPairingRejectionReason::LanAiJobUnauthorized,
            Some(intent),
            origin,
            constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
        );
        event.payload.insert(
            constants::field::LAN_AI_PROVIDER_ROUTING_STATE.to_string(),
            LogFieldValue::String(
                constants::value::LAN_AI_PROVIDER_ROUTING_UNSUPPORTED_CAPABILITY.to_string(),
            ),
        );
        return event;
    }

    let job_id = lan_ai_job_id(&command, intent);
    match runtime.claim_lan_ai_job_lease(&job_id) {
        Ok(LanAiJobLeaseTransition::Claimed(lease)) => {
            let completed_lease = runtime.complete_lan_ai_job_lease(&job_id).unwrap_or(lease);
            lan_ai_job_completed_event(
                &runtime,
                command,
                intent,
                origin,
                &requested_capability,
                &completed_lease,
            )
        }
        Ok(LanAiJobLeaseTransition::DuplicateCompleted(lease)) => lan_ai_job_completed_event(
            &runtime,
            command,
            intent,
            origin,
            &requested_capability,
            &lease,
        ),
        Ok(LanAiJobLeaseTransition::DuplicateActiveRejected(lease)) => {
            lan_ai_job_duplicate_rejected_event(&runtime, command, intent, origin, &lease)
        }
        Ok(LanAiJobLeaseTransition::ExpiredRequeued(lease)) => {
            lan_ai_job_lease_state_event(&runtime, command, intent, origin, &lease)
        }
        Ok(LanAiJobLeaseTransition::DeadLettered(lease)) => {
            lan_ai_job_lease_state_event(&runtime, command, intent, origin, &lease)
        }
        Err(reason) => lan_ai_rejection_event(
            &runtime,
            command,
            reason,
            Some(intent),
            origin,
            constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
        ),
    }
}

fn lan_ai_job_degraded_event(
    runtime: &LanPairingRuntime,
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
    payload.extend(lan_ai_provider_fields(runtime));
    payload.extend(lan_ai_job_fields(
        &command,
        intent,
        constants::value::LAN_AI_JOB_STATE_ACCEPTED,
        constants::value::LAN_AI_JOB_STATE_DEGRADED,
        constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE,
        None,
    ));
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
    runtime: &LanPairingRuntime,
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
    payload.extend(lan_ai_provider_fields_for_rejection(runtime, &reason));
    payload.extend(lan_ai_job_rejected_fields(&command, intent));
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

pub(crate) fn lan_ai_provider_fields(runtime: &LanPairingRuntime) -> LogFields {
    let provider_status = runtime.lan_ai_provider_status_value();
    let capability_flags = runtime.lan_ai_provider_capability_flags();
    fields_from_pairs(vec![
        (
            constants::field::LAN_AI_PROVIDER_STATUS,
            LogFieldValue::String(provider_status.to_string()),
        ),
        (
            constants::field::LAN_AI_PROVIDER_ROUTING_STATE,
            LogFieldValue::String(runtime.lan_ai_provider_routing_state().to_string()),
        ),
        (
            constants::field::LAN_AI_PROVIDER_CUSTODY_LABEL,
            LogFieldValue::String(
                constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER.to_string(),
            ),
        ),
        (
            constants::field::LOCAL_AI_PROVIDER_ID,
            LogFieldValue::String(provider_id_for_status(provider_status).to_string()),
        ),
        (
            constants::field::LOCAL_AI_EXECUTION_STATE,
            LogFieldValue::String(execution_state_for_status(provider_status).to_string()),
        ),
        (
            constants::field::LOCAL_AI_PROVIDER_SOURCE,
            LogFieldValue::String(provider_source_for_status(provider_status).to_string()),
        ),
        (
            constants::field::LOCAL_AI_ADAPTER_READINESS_STATE,
            LogFieldValue::String(readiness_for_status(provider_status).to_string()),
        ),
        (
            constants::field::LOCAL_AI_CAPABILITY_FLAGS,
            LogFieldValue::String(capability_flags),
        ),
        (
            constants::field::LOCAL_AI_UNAVAILABLE_REASON,
            LogFieldValue::String(unavailable_reason_for_status(provider_status).to_string()),
        ),
    ])
}

fn lan_ai_provider_fields_for_rejection(
    runtime: &LanPairingRuntime,
    reason: &LanPairingRejectionReason,
) -> LogFields {
    let mut fields = lan_ai_provider_fields(runtime);
    if matches!(
        reason,
        LanPairingRejectionReason::Anonymous
            | LanPairingRejectionReason::WrongOrigin
            | LanPairingRejectionReason::WrongDevice
            | LanPairingRejectionReason::Expired
            | LanPairingRejectionReason::Replayed
            | LanPairingRejectionReason::Malformed
            | LanPairingRejectionReason::Stale
            | LanPairingRejectionReason::Offline
            | LanPairingRejectionReason::Revoked
            | LanPairingRejectionReason::LocalNetworkDisabled
            | LanPairingRejectionReason::UnsupportedRoute
            | LanPairingRejectionReason::UnselectedDevice
            | LanPairingRejectionReason::ControllerLeaseMissing
            | LanPairingRejectionReason::ControllerLeaseExpired
            | LanPairingRejectionReason::WrongController
            | LanPairingRejectionReason::TakeoverDenied
    ) {
        fields.insert(
            constants::field::LAN_AI_PROVIDER_ROUTING_STATE.to_string(),
            LogFieldValue::String(
                constants::value::LAN_AI_PROVIDER_ROUTING_UNAVAILABLE.to_string(),
            ),
        );
    }
    fields
}

fn lan_ai_job_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    job_status: &'static str,
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
            LogFieldValue::String(job_status.to_string()),
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

fn lan_ai_job_rejected_fields(
    command: &AgentCommandEnvelope,
    intent: Option<&LanParentIntentEnvelope>,
) -> LogFields {
    let mut fields = LogFields::new();
    if let Some(intent) = intent {
        fields.extend(lan_ai_job_fields(
            command,
            intent,
            constants::value::LAN_AI_JOB_STATE_REJECTED,
            constants::value::LAN_AI_JOB_STATE_REJECTED,
            constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE,
            None,
        ));
    }
    fields
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

fn provider_id_for_status(provider_status: &str) -> &'static str {
    if provider_status == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE
        || provider_status == constants::value::LAN_AI_PROVIDER_STATUS_BUSY
        || provider_status == constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
    {
        constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI
    } else {
        constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED
    }
}

fn execution_state_for_status(provider_status: &str) -> &'static str {
    if provider_status == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE {
        constants::local_ai_runtime::EXECUTION_STATE_DRY_RUN_READY
    } else {
        constants::local_ai_runtime::EXECUTION_STATE_DISABLED
    }
}

fn provider_source_for_status(provider_status: &str) -> &'static str {
    if provider_status == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE
        || provider_status == constants::value::LAN_AI_PROVIDER_STATUS_BUSY
        || provider_status == constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
    {
        constants::local_ai_runtime::PROVIDER_SOURCE_LOCAL_CONFIG
    } else {
        constants::local_ai_runtime::PROVIDER_SOURCE_UNAVAILABLE
    }
}

fn readiness_for_status(provider_status: &str) -> &'static str {
    if provider_status == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE {
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_READY
    } else {
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_NOT_READY
    }
}

fn unavailable_reason_for_status(provider_status: &str) -> &'static str {
    if provider_status == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE {
        constants::value::EMPTY
    } else if provider_status == constants::value::LAN_AI_PROVIDER_STATUS_BUSY {
        constants::local_ai_runtime::DEGRADED_OVERLOADED
    } else if provider_status == constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED {
        constants::local_ai_runtime::DEGRADED_PROVIDER_UNAVAILABLE
    } else {
        constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED
    }
}
