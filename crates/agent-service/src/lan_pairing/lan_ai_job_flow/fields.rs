use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::fields::fields_from_pairs;
use crate::lan_pairing::LanPairingRuntime;

pub(crate) enum LanAiJobField {
    CapabilityFlags,
    JobId,
}

pub(crate) fn lan_ai_provider_fields(runtime: &LanPairingRuntime) -> LogFields {
    let provider_status = runtime.lan_ai_provider_status_value();
    let capability_flags = runtime.lan_ai_provider_capability_flags();
    fields_from_pairs(vec![
        (
            constants::field::LAN_AI_PROVIDER_STATUS,
            LogFieldValue::String(provider_status.0.clone()),
        ),
        (
            constants::field::LAN_AI_PROVIDER_ROUTING_STATE,
            LogFieldValue::String(runtime.lan_ai_provider_routing_state().0.clone()),
        ),
        (
            constants::field::LAN_AI_PROVIDER_CUSTODY_LABEL,
            LogFieldValue::String(
                constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER.to_string(),
            ),
        ),
        (
            constants::field::LOCAL_AI_PROVIDER_ID,
            LogFieldValue::String(provider_id_for_status(&provider_status).0),
        ),
        (
            constants::field::LOCAL_AI_EXECUTION_STATE,
            LogFieldValue::String(execution_state_for_status(&provider_status).0),
        ),
        (
            constants::field::LOCAL_AI_PROVIDER_SOURCE,
            LogFieldValue::String(provider_source_for_status(&provider_status).0),
        ),
        (
            constants::field::LOCAL_AI_ADAPTER_READINESS_STATE,
            LogFieldValue::String(readiness_for_status(&provider_status).0),
        ),
        (
            constants::field::LOCAL_AI_CAPABILITY_FLAGS,
            LogFieldValue::String(capability_flags.to_string()),
        ),
        (
            constants::field::LOCAL_AI_UNAVAILABLE_REASON,
            LogFieldValue::String(unavailable_reason_for_status(&provider_status).0),
        ),
    ])
}

pub(crate) fn lan_ai_provider_fields_for_rejection(
    runtime: &LanPairingRuntime,
    reason: &ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason,
) -> LogFields {
    let mut fields = lan_ai_provider_fields(runtime);
    if matches!(
        reason,
        ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::Anonymous
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::WrongOrigin
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::WrongDevice
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::Expired
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::Replayed
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::Malformed
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::Stale
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::Offline
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::Revoked
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::LocalNetworkDisabled
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::UnsupportedRoute
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::UnselectedDevice
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::ControllerLeaseMissing
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::ControllerLeaseExpired
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::WrongController
            | ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::TakeoverDenied
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

pub(crate) fn lan_ai_job_fields(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    job_status: LanPairingText,
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
            LogFieldValue::String(job_status.0),
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

pub(crate) fn lan_ai_job_rejected_fields(
    command: &AgentCommandEnvelope,
    intent: Option<&LanParentIntentEnvelope>,
) -> LogFields {
    let mut fields = LogFields::new();
    if let Some(intent) = intent {
        crate::lan_pairing::extend_log_fields(
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

pub(crate) fn lan_ai_job_id(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
) -> LanPairingText {
    payload_string(&command.payload, LanAiJobField::JobId)
        .unwrap_or_else(|| intent.intent_id.clone().into())
}

pub(crate) fn local_ai_result_id(intent: &LanParentIntentEnvelope) -> LanPairingText {
    let mut result_id = String::from(constants::local_ai_runtime::RESULT_ID_PREFIX);
    result_id.push_str(&intent.intent_id);
    result_id.into()
}

pub(crate) fn payload_string(
    fields: &LogFields,
    field_name: LanAiJobField,
) -> Option<LanPairingText> {
    let field_name = match field_name {
        LanAiJobField::CapabilityFlags => constants::field::LOCAL_AI_CAPABILITY_FLAGS,
        LanAiJobField::JobId => constants::field::LAN_AI_JOB_ID,
    };
    fields.get(field_name).and_then(|value| match value {
        LogFieldValue::String(value) if !value.is_empty() => Some(LanPairingText(value.clone())),
        _ => None,
    })
}

pub(crate) fn provider_id_for_status(provider_status: &LanPairingText) -> LanPairingText {
    if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE
        || provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_BUSY
        || provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
    {
        constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI
            .to_string()
            .into()
    } else {
        constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED
            .to_string()
            .into()
    }
}

pub(crate) fn execution_state_for_status(provider_status: &LanPairingText) -> LanPairingText {
    if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE {
        constants::local_ai_runtime::EXECUTION_STATE_DRY_RUN_READY
            .to_string()
            .into()
    } else {
        constants::local_ai_runtime::EXECUTION_STATE_DISABLED
            .to_string()
            .into()
    }
}

pub(crate) fn provider_source_for_status(provider_status: &LanPairingText) -> LanPairingText {
    if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE
        || provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_BUSY
        || provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
    {
        constants::local_ai_runtime::PROVIDER_SOURCE_LOCAL_CONFIG
            .to_string()
            .into()
    } else {
        constants::local_ai_runtime::PROVIDER_SOURCE_UNAVAILABLE
            .to_string()
            .into()
    }
}

pub(crate) fn readiness_for_status(provider_status: &LanPairingText) -> LanPairingText {
    if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE {
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_READY
            .to_string()
            .into()
    } else {
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_NOT_READY
            .to_string()
            .into()
    }
}

pub(crate) fn unavailable_reason_for_status(provider_status: &LanPairingText) -> LanPairingText {
    if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE {
        constants::value::EMPTY.to_string().into()
    } else if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_BUSY {
        constants::local_ai_runtime::DEGRADED_OVERLOADED
            .to_string()
            .into()
    } else if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED {
        constants::local_ai_runtime::DEGRADED_PROVIDER_UNAVAILABLE
            .to_string()
            .into()
    } else {
        constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED
            .to_string()
            .into()
    }
}
