use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
    AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue,
    LogFields, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    fields::fields_from_pairs, lan_pairing::LanPairingRuntime,
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn lan_pairing_accepts_pairing_proof_then_allows_paired_health_route() {
    let runtime = LanPairingRuntime::empty();
    let proof_event = handle_command_text_for_test(
        &serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let health_event = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        proof_event.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_eq!(
        proof_event.payload.get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PAIRING_PAIRED.to_string()
        ))
    );
    assert_status_support_surface(&proof_event);
    assert_eq!(runtime.trusted_device_count(), 1);
    assert_eq!(health_event.event, AgentEventName::AgentHealthReported);
    assert_accepted_control(&health_event);
}

#[tokio::test]
async fn lan_pairing_rejects_anonymous_wrong_origin_wrong_device_and_revoked_routes() {
    let runtime = paired_runtime().await;

    let anonymous = handle_command_text_for_test(
        &serialize_command(health_command(LogFields::new())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let wrong_origin = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::WRONG_ORIGIN.to_string()),
    )
    .await;
    let wrong_device = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentHealthCheck,
            AgentMessageTarget {
                device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
                route: AgentRoute::LocalNetwork,
            },
            intent_payload(
                constants::lan_pairing::INTENT_ID,
                constants::peer::LOCAL_DEV_AGENT,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
        )),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let unsupported_route = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload_for_route(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::ROUTE_ID_UNSUPPORTED,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_rejection(&anonymous, constants::value::LAN_REASON_ANONYMOUS);
    assert_rejection(&wrong_origin, constants::value::LAN_REASON_WRONG_ORIGIN);
    assert_eq!(
        wrong_origin.payload.get(constants::field::ORIGIN),
        Some(&LogFieldValue::String(
            constants::lan_pairing::WRONG_ORIGIN.to_string()
        ))
    );
    assert_rejection(&wrong_device, constants::value::LAN_REASON_WRONG_DEVICE);
    assert_rejection(
        &unsupported_route,
        constants::value::LAN_REASON_UNSUPPORTED_ROUTE,
    );
}

#[tokio::test]
async fn lan_pairing_rejects_revoked_route_before_new_control_intent() {
    let runtime = paired_runtime().await;
    assert!(runtime.revoke_pairing_for_test(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::OBSERVED_AT
    ));
    let revoked = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_rejection(&revoked, constants::value::LAN_REASON_REVOKED);
}

#[tokio::test]
async fn lan_pairing_rejects_stale_and_replayed_routes() {
    let runtime = paired_runtime().await;
    let stale = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRED_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let first = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::REPLAYED_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let replayed = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::REPLAYED_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_rejection(&stale, constants::value::LAN_REASON_STALE);
    assert_eq!(first.event, AgentEventName::AgentHealthReported);
    assert_accepted_control(&first);
    assert_rejection(&replayed, constants::value::LAN_REASON_REPLAYED);
}

#[tokio::test]
async fn lan_pairing_status_get_is_explicit_for_loopback_and_signed_lan_routes() {
    let loopback = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingStatusGet,
            AgentMessageTarget {
                device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
                platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
                route: AgentRoute::Localhost,
            },
            LogFields::new(),
        )),
        LanPairingRuntime::empty(),
        None,
    )
    .await;
    let runtime = paired_runtime().await;
    let anonymous_lan = handle_command_text_for_test(
        &serialize_command(status_command(LogFields::new())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let audited_lan = handle_command_text_for_test(
        &serialize_command(status_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        loopback.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_eq!(
        loopback.payload.get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PAIRING_UNPAIRED.to_string()
        ))
    );
    assert_status_support_surface(&loopback);
    assert_eq!(
        loopback.payload.get(constants::field::LAN_CONTROL_STATE),
        None
    );
    assert_rejection(&anonymous_lan, constants::value::LAN_REASON_ANONYMOUS);
    assert_eq!(
        audited_lan.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_eq!(
        audited_lan.payload.get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PAIRING_PAIRED.to_string()
        ))
    );
    assert_status_support_surface(&audited_lan);
    assert_accepted_control(&audited_lan);
}

async fn paired_runtime() -> LanPairingRuntime {
    let runtime = LanPairingRuntime::empty();
    let _ = handle_command_text_for_test(
        &serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    runtime
}

fn assert_accepted_control(event: &AgentEventEnvelope) {
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
}

fn assert_status_support_surface(event: &AgentEventEnvelope) {
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

fn assert_rejection(event: &AgentEventEnvelope, reason: &str) {
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
}

fn pairing_command(payload: LogFields) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentLanPairingProofSubmit,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        payload,
    )
}

fn health_command(payload: LogFields) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentHealthCheck,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        payload,
    )
}

fn status_command(payload: LogFields) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentLanPairingStatusGet,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        payload,
    )
}

fn command_for_target(
    command: AgentCommandName,
    target: AgentMessageTarget,
    payload: LogFields,
) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::lan_pairing::INTENT_ID.to_string(),
        sent_at: constants::lan_pairing::ISSUED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target,
        command,
        payload,
    }
}

fn local_network_target(device_id: &str) -> AgentMessageTarget {
    AgentMessageTarget {
        device_id: device_id.to_string(),
        platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
        route: AgentRoute::LocalNetwork,
    }
}

fn proof_payload() -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_PAIRING_ID,
            LogFieldValue::String(constants::lan_pairing::PAIRING_ID.to_string()),
        ),
        (
            constants::field::LAN_CHALLENGE_ID,
            LogFieldValue::String(constants::lan_pairing::CHALLENGE_ID.to_string()),
        ),
        (
            constants::field::LAN_CHILD_DEVICE_ID,
            LogFieldValue::String(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
        ),
        (
            constants::field::LAN_PARENT_DEVICE_ID,
            LogFieldValue::String(constants::lan_pairing::PARENT_DEVICE_ID.to_string()),
        ),
        (
            constants::field::LAN_ROUTE_ID,
            LogFieldValue::String(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        ),
        (
            constants::field::ORIGIN,
            LogFieldValue::String(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
        ),
        (
            constants::field::LAN_PROOF_DIGEST,
            LogFieldValue::String(constants::lan_pairing::PROOF_DIGEST.to_string()),
        ),
        (
            constants::field::STARTED_AT,
            LogFieldValue::String(constants::lan_pairing::ISSUED_AT.to_string()),
        ),
        (
            constants::field::STALE_AT,
            LogFieldValue::String(constants::lan_pairing::EXPIRES_AT.to_string()),
        ),
    ])
}

fn intent_payload(
    intent_id: &str,
    target_device_id: &str,
    proof_digest: &str,
    expires_at: &str,
) -> LogFields {
    intent_payload_for_route(
        intent_id,
        target_device_id,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        proof_digest,
        expires_at,
    )
}

fn intent_payload_for_route(
    intent_id: &str,
    target_device_id: &str,
    route_id: &str,
    proof_digest: &str,
    expires_at: &str,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_INTENT_ID,
            LogFieldValue::String(intent_id.to_string()),
        ),
        (
            constants::field::LAN_PAIRING_ID,
            LogFieldValue::String(constants::lan_pairing::PAIRING_ID.to_string()),
        ),
        (
            constants::field::LAN_CHILD_DEVICE_ID,
            LogFieldValue::String(target_device_id.to_string()),
        ),
        (
            constants::field::LAN_ROUTE_ID,
            LogFieldValue::String(route_id.to_string()),
        ),
        (
            constants::field::ORIGIN,
            LogFieldValue::String(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
        ),
        (
            constants::field::LAN_PROOF_DIGEST,
            LogFieldValue::String(proof_digest.to_string()),
        ),
        (
            constants::field::STARTED_AT,
            LogFieldValue::String(constants::lan_pairing::ISSUED_AT.to_string()),
        ),
        (
            constants::field::STALE_AT,
            LogFieldValue::String(expires_at.to_string()),
        ),
    ])
}

fn serialize_command(command: AgentCommandEnvelope) -> String {
    serde_json::to_string(&command).expect(constants::error::AGENT_EVENT_SERIALIZES)
}
