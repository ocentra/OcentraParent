use ocentra_parent_agent_protocol::{constants, AgentEventEnvelope, AgentEventName, LogFieldValue};

pub(crate) fn assert_accepted_control(event: &AgentEventEnvelope) {
    assert_eq!(
        event.payload.get(constants::field::LAN_CONTROL_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_CONTROL_ACCEPTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AUDIT_EVENT_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::INTENT_ID.to_string()
        ))
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
    assert_eq!(
        event.payload.get(constants::field::LAN_DISCOVERY_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_CHALLENGE_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_PROOF_PREVIEW_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_PERSISTENCE_MODE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PERSISTENCE_IN_MEMORY_FAIL_CLOSED.to_string()
        ))
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
}

pub(crate) fn assert_selected_device_reachability(event: &AgentEventEnvelope, reachability: &str) {
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SELECTED_DEVICE_REACHABILITY),
        Some(&LogFieldValue::String(reachability.to_string()))
    );
}

pub(crate) fn assert_status_selection(
    event: &AgentEventEnvelope,
    authentication_state: &str,
    selected_child_device_id: &str,
    selected_route_id: &str,
    trusted_device_ids: &str,
) {
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AUTHENTICATION_STATE),
        Some(&LogFieldValue::String(authentication_state.to_string()))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SELECTED_CHILD_DEVICE_ID),
        Some(&LogFieldValue::String(selected_child_device_id.to_string()))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_SELECTED_ROUTE_ID),
        Some(&LogFieldValue::String(selected_route_id.to_string()))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_TRUSTED_DEVICE_IDS),
        Some(&LogFieldValue::String(trusted_device_ids.to_string()))
    );
}

pub(crate) fn assert_rejection(event: &AgentEventEnvelope, reason: &str) {
    assert_eq!(event.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        event.payload.get(constants::field::LAN_CONTROL_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_CONTROL_REJECTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_CONTROL_REJECTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_REJECTION_REASON),
        Some(&LogFieldValue::String(reason.to_string()))
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

fn expected_authentication_state(reason: &str) -> &'static str {
    if reason == constants::value::LAN_REASON_ANONYMOUS
        || reason == constants::value::LAN_REASON_WRONG_ORIGIN
        || reason == constants::value::LAN_REASON_MALFORMED
    {
        constants::value::LAN_AUTH_UNAUTHENTICATED
    } else {
        constants::value::LAN_AUTH_PAIRED
    }
}
