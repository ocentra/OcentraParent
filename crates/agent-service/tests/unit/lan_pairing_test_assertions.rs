use ocentra_lan_core::lan_mdns_advertiser::current_platform_support;
use ocentra_lan_core::lan_pairing::{
    evaluate_lan_mdns_advertisement_lifecycle, LanMdnsAdvertisementLifecycleInput,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

type TestText = String;

pub(crate) fn assert_accepted_control(event: &AgentEventEnvelope) {
    assert_accepted_control_for_intent(event, constants::lan_pairing::INTENT_ID);
}

pub(crate) fn assert_accepted_control_for_intent(event: &AgentEventEnvelope, intent_id: TestText) {
    let intent_id: TestText = intent_id.into();
    assert_eq!(
        event.payload.get(constants::field::LAN_CONTROL_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_CONTROL_ACCEPTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AUDIT_EVENT_ID),
        Some(&LogFieldValue::String(intent_id))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_CONTROL_ACCEPTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_ROUTE_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::ORIGIN),
        Some(&LogFieldValue::String(
            constants::lan_pairing::ALLOWED_ORIGIN.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_CONTROLLER_LEASE_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::CONTROLLER_LEASE_ID.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_CONTROLLER_DEVICE_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::PARENT_DEVICE_ID.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_PARENT_ACTOR_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::PARENT_ACTOR_ID.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AUTHENTICATION_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUTH_PAIRED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_EVIDENCE_REFERENCE_COUNT),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_EVIDENCE_REFERENCE_IDS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::EVIDENCE_REFERENCE_ID.to_string()
        ))
    );
}

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

pub(crate) fn assert_persistent_selected_route_support_surface(event: &AgentEventEnvelope) {
    assert_status_support_surface_with_persistence(
        event,
        constants::value::LAN_PERSISTENCE_LOCAL_JSON_REGISTRY,
        constants::value::LAN_RESTART_RESTORE_TRUSTED_REGISTRY_SELECTED_ROUTE,
    );
}

fn assert_status_support_surface_with_persistence(
    event: &AgentEventEnvelope,
    persistence_mode: TestText,
    restart_behavior: TestText,
) {
    let persistence_mode: TestText = persistence_mode.into();
    let restart_behavior: TestText = restart_behavior.into();
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
    persistence_mode: TestText,
    restart_behavior: TestText,
) {
    let persistence_mode: TestText = persistence_mode.into();
    let restart_behavior: TestText = restart_behavior.into();
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
    match event.payload.get(constants::field::LAN_DISCOVERY_STATE) {
        Some(LogFieldValue::String(value))
            if [
                constants::value::LAN_DISCOVERY_STATE_DISCOVERED,
                constants::value::LAN_DISCOVERY_STATE_PENDING,
                constants::value::LAN_DISCOVERY_STATE_PAIRED,
                constants::value::LAN_DISCOVERY_STATE_REVOKED,
                constants::value::LAN_DISCOVERY_STATE_STALE,
                constants::value::LAN_DISCOVERY_STATE_OFFLINE,
                constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE,
            ]
            .contains(&value.as_str()) => {}
        _ => unreachable!("{}", constants::error::UNEXPECTED_LAN_DISCOVERY_STATE),
    }
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

pub(crate) fn assert_selected_device_reachability(
    event: &AgentEventEnvelope,
    reachability: TestText,
) {
    let reachability: TestText = reachability.into();
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SELECTED_DEVICE_REACHABILITY),
        Some(&LogFieldValue::String(reachability))
    );
}

pub(crate) fn assert_status_selection(
    event: &AgentEventEnvelope,
    authentication_state: TestText,
    selected_child_device_id: TestText,
    selected_route_id: TestText,
    trusted_device_ids: TestText,
) {
    let authentication_state: TestText = authentication_state.into();
    let selected_child_device_id: TestText = selected_child_device_id.into();
    let selected_route_id: TestText = selected_route_id.into();
    let trusted_device_ids: TestText = trusted_device_ids.into();
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
    pairing_id: TestText,
    trust_state: TestText,
    stale_at: TestText,
    offline_at: TestText,
) {
    let pairing_id: TestText = pairing_id.into();
    let trust_state: TestText = trust_state.into();
    let stale_at: TestText = stale_at.into();
    let offline_at: TestText = offline_at.into();
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
    pub(crate) authentication_state: &'a str,
    pub(crate) selected_child_device_id: &'a str,
    pub(crate) selected_route_id: &'a str,
    pub(crate) trusted_device_ids: &'a str,
    pub(crate) pairing_id: &'a str,
    pub(crate) trust_state: &'a str,
    pub(crate) stale_at: &'a str,
    pub(crate) offline_at: &'a str,
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

pub(crate) fn assert_rejection(event: &AgentEventEnvelope, reason: TestText) {
    assert_rejection_with_audit(event, reason, constants::value::LAN_AUDIT_CONTROL_REJECTED);
}

pub(crate) fn assert_rejection_with_audit(
    event: &AgentEventEnvelope,
    reason: TestText,
    audit_type: TestText,
) {
    let reason: TestText = reason.into();
    let audit_type: TestText = audit_type.into();
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
            expected_authentication_state(reason).to_string()
        ))
    );
}

fn expected_authentication_state(reason: TestText) -> &'static str {
    let reason: TestText = reason.into();
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
