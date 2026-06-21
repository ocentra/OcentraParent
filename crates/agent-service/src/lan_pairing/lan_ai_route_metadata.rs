use ocentra_parent_agent_core::{
    household_ai_provider_route::{
        select_household_ai_provider_route, HouseholdAiProviderCandidate, HouseholdAiRouteRequest,
    },
    household_ai_provider_route_state::{HouseholdAiProviderResourceState, HouseholdAiWorkClass},
};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, LanParentIntentEnvelope, LogFieldValue, LogFields,
};

use crate::{fields::fields_from_pairs, lan_pairing::LanPairingRuntime};

pub(crate) fn lan_ai_household_route_fields(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    requested_capability: &str,
) -> LogFields {
    let request = household_route_request(command, intent, requested_capability);
    let route =
        select_household_ai_provider_route(&request, &[household_provider_candidate(runtime)]);
    fields_from_pairs(vec![
        selected_provider_peer_field(route.selected_provider_peer_id),
        (
            constants::field::LAN_AI_SELECTED_ROUTE_REASON,
            LogFieldValue::String(route.selected_reason_label),
        ),
        (
            constants::field::LAN_AI_PROVIDER_POLICY_AUTHORITY,
            LogFieldValue::String(
                constants::household_mesh::POLICY_AUTHORITY_CHILD_AGENT_ONLY.to_string(),
            ),
        ),
        (
            constants::field::LAN_AI_PROVIDER_CAN_PUBLISH_POLICY,
            LogFieldValue::Boolean(false),
        ),
        (
            constants::field::LAN_AI_RAW_SCREEN_TRANSFERRED,
            LogFieldValue::Boolean(false),
        ),
        (
            constants::field::LAN_AI_CHILD_VALIDATES_PROVIDER_RESULT,
            LogFieldValue::Boolean(true),
        ),
        (
            constants::field::LAN_AI_CLAIM_ID,
            LogFieldValue::String(lan_ai_claim_id(&request.job_id)),
        ),
        (
            constants::field::LAN_AI_LEASE_ID,
            LogFieldValue::String(lan_ai_lease_id(&request.job_id)),
        ),
    ])
}

fn household_provider_candidate(runtime: &LanPairingRuntime) -> HouseholdAiProviderCandidate {
    let mut candidate = HouseholdAiProviderCandidate::parent_desktop();
    candidate.provider_peer_id = runtime.device_role_read_model().physical_device_id;
    candidate.resource_state = if runtime.lan_ai_provider_available() {
        HouseholdAiProviderResourceState::Ready
    } else {
        HouseholdAiProviderResourceState::Degraded
    };
    candidate
}

fn household_route_request(
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    requested_capability: &str,
) -> HouseholdAiRouteRequest {
    HouseholdAiRouteRequest {
        job_id: lan_ai_job_id(command, intent),
        work_class: household_work_class_for_capability(requested_capability),
        allow_mobile_fallback: false,
        required_custody_label: constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER
            .to_string(),
    }
}

fn selected_provider_peer_field(
    selected_provider_peer_id: Option<String>,
) -> (&'static str, LogFieldValue) {
    (
        constants::field::LAN_AI_SELECTED_PROVIDER_PEER_ID,
        LogFieldValue::String(
            selected_provider_peer_id.unwrap_or_else(|| constants::value::UNKNOWN_HOST.to_string()),
        ),
    )
}

fn household_work_class_for_capability(capability: &str) -> HouseholdAiWorkClass {
    if capability == constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION
        || capability == constants::local_ai_runtime::CAPABILITY_SUMMARIZATION
    {
        HouseholdAiWorkClass::LightText
    } else {
        HouseholdAiWorkClass::HeavyScreenVision
    }
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

fn lan_ai_claim_id(job_id: &str) -> String {
    let mut claim_id = String::from(constants::lan_pairing::LAN_AI_CLAIM_ID_PREFIX);
    claim_id.push_str(job_id);
    claim_id
}

fn lan_ai_lease_id(job_id: &str) -> String {
    let mut lease_id = String::from(constants::lan_pairing::LAN_AI_LEASE_ID_PREFIX);
    lease_id.push_str(job_id);
    lease_id
}
