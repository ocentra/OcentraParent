use ocentra_parent_agent_protocol::{constants, AgentEventEnvelope, LogFieldValue, LogFields};

use crate::{
    lan_pairing_test_support::{
        assert_rejection, health_command, intent_payload, paired_runtime, serialize_command,
    },
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn lan_pairing_requires_single_active_controller_lease_before_control() {
    let runtime = paired_runtime().await;
    let missing_lease =
        rejected_controller_lease_control(runtime.clone(), missing_controller_lease_payload())
            .await;
    let expired_lease =
        rejected_controller_lease_control(runtime.clone(), expired_controller_lease_payload())
            .await;
    let wrong_controller =
        rejected_controller_lease_control(runtime, second_controller_payload()).await;

    assert_rejection(
        &missing_lease,
        constants::value::LAN_REASON_CONTROLLER_LEASE_MISSING,
    );
    assert_rejection(
        &expired_lease,
        constants::value::LAN_REASON_CONTROLLER_LEASE_EXPIRED,
    );
    assert_rejection(
        &wrong_controller,
        constants::value::LAN_REASON_WRONG_CONTROLLER,
    );
    assert_eq!(
        wrong_controller
            .payload
            .get(constants::field::LAN_CONTROLLER_LEASE_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SECOND_CONTROLLER_LEASE_ID.to_string()
        ))
    );
}

async fn rejected_controller_lease_control(
    runtime: crate::lan_pairing::LanPairingRuntime,
    payload: LogFields,
) -> AgentEventEnvelope {
    handle_command_text_for_test(
        &serialize_command(health_command(payload)),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await
}

fn missing_controller_lease_payload() -> LogFields {
    let mut payload = controller_lease_payload(constants::lan_pairing::RULE_QUERY_INTENT_ID);
    for key in [
        constants::field::LAN_CONTROLLER_LEASE_ID,
        constants::field::LAN_CONTROLLER_DEVICE_ID,
        constants::field::LAN_PARENT_ACTOR_ID,
        constants::field::LAN_CONTROLLER_LEASE_ISSUED_AT,
        constants::field::LAN_CONTROLLER_LEASE_EXPIRES_AT,
    ] {
        payload.remove(key);
    }
    payload
}

fn expired_controller_lease_payload() -> LogFields {
    let mut payload = controller_lease_payload(constants::lan_pairing::RULE_UPDATE_INTENT_ID);
    payload.insert(
        constants::field::LAN_CONTROLLER_LEASE_EXPIRES_AT.to_string(),
        LogFieldValue::String(constants::lan_pairing::CONTROLLER_LEASE_EXPIRED_AT.to_string()),
    );
    payload
}

fn second_controller_payload() -> LogFields {
    let mut payload = controller_lease_payload(constants::lan_pairing::APPROVAL_DECISION_INTENT_ID);
    payload.insert(
        constants::field::LAN_CONTROLLER_LEASE_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::SECOND_CONTROLLER_LEASE_ID.to_string()),
    );
    payload.insert(
        constants::field::LAN_CONTROLLER_DEVICE_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::SECOND_PARENT_DEVICE_ID.to_string()),
    );
    payload.insert(
        constants::field::LAN_PARENT_ACTOR_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::SECOND_PARENT_ACTOR_ID.to_string()),
    );
    payload
}

fn controller_lease_payload(intent_id: &str) -> LogFields {
    intent_payload(
        intent_id,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::PROOF_DIGEST,
        constants::lan_pairing::EXPIRES_AT,
    )
}
