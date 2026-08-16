use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::fields::fields_from_pairs;
use crate::lan_pairing::LanPairingRuntime;

#[path = "fields/provider_status.rs"]
mod provider_status;

use self::provider_status::{
    execution_state_for_status, provider_id_for_status, provider_source_for_status,
    readiness_for_status, unavailable_reason_for_status,
};

pub(crate) enum LanAiJobField {
    CapabilityFlags,
    JobId,
}

pub(crate) fn lan_ai_provider_fields(runtime: &LanPairingRuntime) -> LogFields {
    let provider_status = runtime.lan_ai_provider_status_value();
    let routing_state = runtime.lan_ai_provider_routing_state().0;
    let capability_flags = runtime.lan_ai_provider_capability_flags();
    fields_from_pairs(vec![
        (
            constants::field::LAN_AI_PROVIDER_STATUS,
            LogFieldValue::String(provider_status.0.clone()),
        ),
        (
            constants::field::LAN_AI_PROVIDER_ROUTING_STATE,
            LogFieldValue::String(routing_state),
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

pub(crate) fn lan_ai_job_id(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
) -> LanPairingText {
    payload_string(&command.payload, &LanAiJobField::JobId)
        .unwrap_or_else(|| intent.intent_id.as_str().into())
}

pub(crate) fn local_ai_result_id(intent: &LanParentIntentEnvelope) -> LanPairingText {
    let mut result_id = String::from(constants::local_ai_runtime::RESULT_ID_PREFIX);
    result_id.push_str(&intent.intent_id);
    result_id.into()
}

pub(crate) fn payload_string(
    fields: &LogFields,
    field_name: &LanAiJobField,
) -> Option<LanPairingText> {
    let field_name = match field_name {
        LanAiJobField::CapabilityFlags => constants::field::LOCAL_AI_CAPABILITY_FLAGS,
        LanAiJobField::JobId => constants::field::LAN_AI_JOB_ID,
    };
    fields.get(field_name).and_then(|value| match value {
        LogFieldValue::String(value) if !value.is_empty() => {
            Some(LanPairingText(value.as_str().to_owned()))
        }
        _ => None,
    })
}
