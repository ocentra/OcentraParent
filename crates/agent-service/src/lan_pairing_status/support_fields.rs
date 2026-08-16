use ocentra_lan_core::lan_mdns_advertiser::current_platform_support;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementConfirmationState;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::{
    fields::fields_from_pairs,
    lan_pairing::{extend_log_fields, LanPairingRuntime},
};

use super::{pairing_status, state_projection};

pub(super) fn support_surface_fields(runtime: &LanPairingRuntime) -> LogFields {
    let status = pairing_status(runtime);
    let mut fields = fields_from_pairs(vec![
        (
            constants::field::TRANSPORT,
            LogFieldValue::String(constants::value::TRANSPORT_WEBSOCKET.to_string()),
        ),
        (
            constants::field::LAN_SUPPORTED_WEBSOCKET_COMMANDS,
            LogFieldValue::String(
                constants::lan_pairing::SUPPORTED_WEBSOCKET_COMMANDS
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_UNSUPPORTED_HTTP_ENDPOINTS,
            LogFieldValue::String(
                constants::lan_pairing::PLANNED_HTTP_ENDPOINT_PATHS
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_DISCOVERY_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()),
        ),
        (
            constants::field::LAN_DISCOVERY_STATE,
            LogFieldValue::String(state_projection::discovery_state(&status).to_string()),
        ),
        (
            constants::field::LAN_CHALLENGE_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()),
        ),
        (
            constants::field::LAN_PROOF_PREVIEW_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()),
        ),
        (
            constants::field::LAN_AI_JOB_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()),
        ),
        (
            constants::field::LAN_PERSISTENCE_MODE,
            LogFieldValue::String(runtime.persistence_mode().to_string()),
        ),
        (
            constants::field::LAN_RESTART_BEHAVIOR,
            LogFieldValue::String(runtime.restart_behavior().to_string()),
        ),
        (
            constants::field::LAN_PROOF_MODE,
            LogFieldValue::String(constants::value::LAN_PROOF_DIRECT_PROOF_SUBMIT.to_string()),
        ),
        (
            constants::field::LAN_ROUTE_REQUIREMENTS,
            LogFieldValue::String(
                constants::lan_pairing::ROUTE_REQUIREMENTS
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_MANUAL_PROOF_GAPS,
            LogFieldValue::String(
                constants::lan_pairing::MANUAL_PROOF_GAPS
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
    ]);
    extend_log_fields(&mut fields, lan_ai_provider_support_fields());
    extend_log_fields(&mut fields, mdns_advertisement_support_fields(runtime));
    extend_log_fields(&mut fields, signed_child_agent_support_fields(runtime));
    fields
}

fn mdns_advertisement_support_fields(runtime: &LanPairingRuntime) -> LogFields {
    let lifecycle = LanPairingRuntime::mdns_advertisement_lifecycle(
        runtime.signed_child_agent_family_hash.is_some(),
        false,
        current_platform_support(),
    );
    fields_from_pairs(vec![
        (
            constants::field::LAN_MDNS_ADVERTISEMENT_LIFECYCLE,
            LogFieldValue::String(lifecycle.lifecycle_action.as_str().to_string()),
        ),
        (
            constants::field::LAN_MDNS_ADVERTISEMENT_SUPPORT,
            LogFieldValue::String(lifecycle.platform_support.as_str().to_string()),
        ),
        (
            constants::field::LAN_MDNS_ADVERTISEMENT_CONFIRMATION,
            LogFieldValue::String(
                LanMdnsAdvertisementConfirmationState::HintOnly
                    .as_str()
                    .to_string(),
            ),
        ),
    ])
}

fn signed_child_agent_support_fields(runtime: &LanPairingRuntime) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_SIGNED_CHILD_AGENT_STATUS,
            LogFieldValue::String(
                constants::lan_pairing::PRODUCTION_PROOF_STATE_MANUAL_REQUIRED.to_string(),
            ),
        ),
        (
            constants::field::LAN_SIGNED_CHILD_AGENT_REPLAY_OBSERVED_COUNT,
            LogFieldValue::Number(runtime.signed_child_agent_replay_observation_count() as f64),
        ),
    ])
}

fn lan_ai_provider_support_fields() -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_AI_PROVIDER_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()),
        ),
        (
            constants::field::LAN_AI_PROVIDER_ROUTING_STATE,
            LogFieldValue::String(
                constants::value::LAN_AI_PROVIDER_ROUTING_UNAVAILABLE.to_string(),
            ),
        ),
        (
            constants::field::LAN_AI_PROVIDER_CUSTODY_LABEL,
            LogFieldValue::String(
                constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER.to_string(),
            ),
        ),
    ])
}
