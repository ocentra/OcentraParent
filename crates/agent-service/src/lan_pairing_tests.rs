use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandName, AgentEventName, AgentMessageTarget, AgentRoute,
    LogFieldValue, LogFields,
};

use crate::{
    lan_pairing::LanPairingRuntime,
    lan_pairing_test_support::{
        assert_accepted_control, assert_rejection, assert_status_selection,
        assert_status_support_surface, command_for_target, health_command, intent_payload,
        intent_payload_for_pairing, local_network_target, paired_runtime, serialize_command,
        status_command,
    },
    websocket::handle_command_text_for_test,
};

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
            local_network_target(constants::peer::LOCAL_DEV_AGENT),
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
        &serialize_command(health_command(intent_payload_for_pairing(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::PAIRING_ID,
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
    assert_status_selection(
        &loopback,
        constants::value::LAN_AUTH_UNPAIRED,
        constants::value::EMPTY,
        constants::value::EMPTY,
        constants::value::EMPTY,
    );
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
