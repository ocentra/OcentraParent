use ocentra_lan_core::lan_mdns_advertiser::current_platform_support;
use ocentra_lan_core::lan_pairing::{
    evaluate_lan_mdns_advertisement_lifecycle, LanMdnsAdvertisementLifecycleInput,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use std::fmt::Display;
use std::primitive::str as TestStr;
use std::string::String as TestString;

type TestText = TestString;

pub(crate) fn assert_status_support_surface(event: &AgentEventEnvelope) {
    assert_status_support_surface_with_persistence(
        event,
        constants::value::LAN_PERSISTENCE_IN_MEMORY_FAIL_CLOSED,
        constants::value::LAN_RESTART_FAIL_CLOSED_UNPAIRED,
    );
}

pub(crate) fn assert_persistent_status_support_surface(event: &AgentEventEnvelope) {
    assert_status_support_surface_with_persistence(
        event,
        constants::value::LAN_PERSISTENCE_LOCAL_JSON_REGISTRY,
        constants::value::LAN_RESTART_RESTORE_TRUSTED_REGISTRY_UNSELECTED,
    );
}

fn assert_status_support_surface_with_persistence(
    event: &AgentEventEnvelope,
    persistence_mode: impl Display,
    restart_behavior: impl Display,
) {
    let persistence_mode = persistence_mode.to_string();
    let restart_behavior = restart_behavior.to_string();
    assert_transport_support_surface(event);
    assert_runtime_support_surface(event, persistence_mode, restart_behavior);
}

fn assert_transport_support_surface(event: &AgentEventEnvelope) {
    assert_eq!(
        event.payload.get(constants::field::TRANSPORT),
        Some(&LogFieldValue::String(
            constants::value::TRANSPORT_WEBSOCKET.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SUPPORTED_WEBSOCKET_COMMANDS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SUPPORTED_WEBSOCKET_COMMANDS
                .join(&constants::delimiter::LIST.to_string())
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_UNSUPPORTED_HTTP_ENDPOINTS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_PATHS
                .join(&constants::delimiter::LIST.to_string())
        ))
    );
}

fn assert_runtime_support_surface(
    event: &AgentEventEnvelope,
    persistence_mode: impl Display,
    restart_behavior: impl Display,
) {
    let persistence_mode = persistence_mode.to_string();
    let restart_behavior = restart_behavior.to_string();
    let expected_mdns =
        evaluate_lan_mdns_advertisement_lifecycle(LanMdnsAdvertisementLifecycleInput {
            desired_present: false,
            running: false,
            platform_support: current_platform_support(),
        });
    assert_eq!(
        event.payload.get(constants::field::LAN_DISCOVERY_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()
        ))
    );
    assert_explicit_discovery_state(event);
    assert_eq!(
        event.payload.get(constants::field::LAN_CHALLENGE_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_PROOF_PREVIEW_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()
        ))
    );
    assert_lan_ai_provider_support_surface(event);
    assert_eq!(
        event.payload.get(constants::field::LAN_PERSISTENCE_MODE),
        Some(&LogFieldValue::String(persistence_mode))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_RESTART_BEHAVIOR),
        Some(&LogFieldValue::String(restart_behavior))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_PROOF_MODE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PROOF_DIRECT_PROOF_SUBMIT.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_ROUTE_REQUIREMENTS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::ROUTE_REQUIREMENTS
                .join(&constants::delimiter::LIST.to_string())
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_MANUAL_PROOF_GAPS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::MANUAL_PROOF_GAPS.join(&constants::delimiter::LIST.to_string())
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_MDNS_ADVERTISEMENT_LIFECYCLE),
        Some(&LogFieldValue::String(
            expected_mdns.lifecycle_action.as_str().to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_MDNS_ADVERTISEMENT_SUPPORT),
        Some(&LogFieldValue::String(
            expected_mdns.platform_support.as_str().to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_MDNS_ADVERTISEMENT_CONFIRMATION),
        Some(&LogFieldValue::String(
            constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY.into()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::PRODUCTION_PROOF_STATE_MANUAL_REQUIRED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_REPLAY_OBSERVED_COUNT),
        Some(&LogFieldValue::Number(0.0))
    );
}

fn assert_explicit_discovery_state(event: &AgentEventEnvelope) {
    let discovery_state = match event.payload.get(constants::field::LAN_DISCOVERY_STATE) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => constants::error::UNEXPECTED_LAN_DISCOVERY_STATE,
    };

    assert!(matches!(
        discovery_state,
        constants::value::LAN_DISCOVERY_STATE_DISCOVERED
            | constants::value::LAN_DISCOVERY_STATE_PENDING
            | constants::value::LAN_DISCOVERY_STATE_PAIRED
            | constants::value::LAN_DISCOVERY_STATE_REVOKED
            | constants::value::LAN_DISCOVERY_STATE_STALE
            | constants::value::LAN_DISCOVERY_STATE_OFFLINE
            | constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE
    ));
}

fn assert_lan_ai_provider_support_surface(event: &AgentEventEnvelope) {
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_PROVIDER_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AI_PROVIDER_ROUTING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_PROVIDER_ROUTING_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AI_PROVIDER_CUSTODY_LABEL),
        Some(&LogFieldValue::String(
            constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_JOB_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()
        ))
    );
}

pub(crate) fn assert_status_selection(
    event: &AgentEventEnvelope,
    authentication_state: impl Display,
    selected_child_device_id: impl Display,
    selected_route_id: impl Display,
    trusted_device_ids: impl Display,
) {
    let authentication_state = authentication_state.to_string();
    let selected_child_device_id = selected_child_device_id.to_string();
    let selected_route_id = selected_route_id.to_string();
    let trusted_device_ids = trusted_device_ids.to_string();
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AUTHENTICATION_STATE),
        Some(&LogFieldValue::String(authentication_state))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SELECTED_CHILD_DEVICE_ID),
        Some(&LogFieldValue::String(selected_child_device_id))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_SELECTED_ROUTE_ID),
        Some(&LogFieldValue::String(selected_route_id))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_TRUSTED_DEVICE_IDS),
        Some(&LogFieldValue::String(trusted_device_ids))
    );
}

pub(crate) fn assert_status_selected_route_trust(
    event: &AgentEventEnvelope,
    pairing_id: impl Display,
    trust_state: impl Display,
    stale_at: impl Display,
    offline_at: impl Display,
) {
    let pairing_id = pairing_id.to_string();
    let trust_state = trust_state.to_string();
    let stale_at = stale_at.to_string();
    let offline_at = offline_at.to_string();
    assert_eq!(
        event.payload.get(constants::field::LAN_SELECTED_PAIRING_ID),
        Some(&LogFieldValue::String(pairing_id))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SELECTED_ROUTE_TRUST_STATE),
        Some(&LogFieldValue::String(trust_state))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SELECTED_ROUTE_STALE_AT),
        Some(&LogFieldValue::String(stale_at))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SELECTED_ROUTE_OFFLINE_AT),
        Some(&LogFieldValue::String(offline_at))
    );
}

#[derive(Clone, Copy)]
pub(crate) struct SelectedRouteCustodyExpectation<'a> {
    pub(crate) authentication_state: &'a TestStr,
    pub(crate) selected_child_device_id: &'a TestStr,
    pub(crate) selected_route_id: &'a TestStr,
    pub(crate) trusted_device_ids: &'a TestStr,
    pub(crate) pairing_id: &'a TestStr,
    pub(crate) trust_state: &'a TestStr,
    pub(crate) stale_at: &'a TestStr,
    pub(crate) offline_at: &'a TestStr,
}

pub(crate) fn assert_status_selected_route_custody(
    event: &AgentEventEnvelope,
    expectation: SelectedRouteCustodyExpectation<'_>,
) {
    assert_status_selection(
        event,
        expectation.authentication_state,
        expectation.selected_child_device_id,
        expectation.selected_route_id,
        expectation.trusted_device_ids,
    );
    assert_status_selected_route_trust(
        event,
        expectation.pairing_id,
        expectation.trust_state,
        expectation.stale_at,
        expectation.offline_at,
    );
}

pub(crate) fn assert_rejection(event: &AgentEventEnvelope, reason: impl Display) {
    assert_rejection_with_audit(event, reason, constants::value::LAN_AUDIT_CONTROL_REJECTED);
}

pub(crate) fn assert_rejection_with_audit(
    event: &AgentEventEnvelope,
    reason: impl Display,
    audit_type: impl Display,
) {
    let reason = reason.to_string();
    let audit_type = audit_type.to_string();
    assert_eq!(event.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        event.payload.get(constants::field::LAN_CONTROL_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_CONTROL_REJECTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(audit_type))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_REJECTION_REASON),
        Some(&LogFieldValue::String(reason.clone()))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AUTHENTICATION_STATE),
        Some(&LogFieldValue::String(
            expected_authentication_state(&reason).to_string()
        ))
    );
}

fn expected_authentication_state(reason: &TestText) -> &'static TestStr {
    let reason = reason.as_str();
    if reason == constants::value::LAN_REASON_ANONYMOUS
        || reason == constants::value::LAN_REASON_CONTROLLER_LEASE_MISSING
        || reason == constants::value::LAN_REASON_CONTROLLER_LEASE_EXPIRED
        || reason == constants::value::LAN_REASON_OBSERVER_READ_ONLY
        || reason == constants::value::LAN_REASON_TAKEOVER_DENIED
        || reason == constants::value::LAN_REASON_LAN_AI_PROVIDER_UNAVAILABLE
        || reason == constants::value::LAN_REASON_LAN_AI_JOB_UNAUTHORIZED
        || reason == constants::value::LAN_REASON_WRONG_ORIGIN
        || reason == constants::value::LAN_REASON_WRONG_CONTROLLER
        || reason == constants::value::LAN_REASON_MALFORMED
        || reason == constants::value::LAN_REASON_SIGNED_CHILD_AGENT_CONTEXT_UNAVAILABLE
    {
        constants::value::LAN_AUTH_UNAUTHENTICATED
    } else {
        constants::value::LAN_AUTH_PAIRED
    }
}
