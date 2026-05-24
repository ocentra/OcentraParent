use ocentra_parent_agent_protocol::{constants, AgentEventEnvelope, AgentEventName, LogFieldValue};

use crate::{
    lan_pairing::LanPairingRuntime,
    lan_pairing_test_support::{
        assert_accepted_control, assert_rejection, assert_status_selection,
        assert_status_support_surface, health_command, health_command_for_target, intent_payload,
        intent_payload_for_pairing, pairing_command, pairing_command_for_target, proof_payload,
        route_select_command, route_select_command_for_target, second_proof_payload,
        serialize_command,
    },
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn lan_pairing_proof_reports_trusted_but_unselected_state() {
    let runtime = LanPairingRuntime::empty();
    let proof_event = handle_command_text_for_test(
        &serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let health_before_selection = handle_command_text_for_test(
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
        proof_event
            .payload
            .get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_PAIRING_PROOF_ACCEPTED.to_string()
        ))
    );
    assert_eq!(
        proof_event
            .payload
            .get(constants::field::LAN_EVIDENCE_REFERENCE_IDS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::EVIDENCE_REFERENCE_ID.to_string()
        ))
    );
    assert_status_support_surface(&proof_event);
    assert_status_selection(
        &proof_event,
        constants::value::LAN_AUTH_UNPAIRED,
        constants::value::EMPTY,
        constants::value::EMPTY,
        constants::lan_pairing::CHILD_DEVICE_ID,
    );
    assert_eq!(runtime.trusted_device_count(), 1);
    assert_rejection(
        &health_before_selection,
        constants::value::LAN_REASON_UNSELECTED_DEVICE,
    );
}

#[tokio::test]
async fn lan_pairing_rejected_proof_keeps_audit_evidence_reference() {
    let runtime = LanPairingRuntime::empty();
    let mut payload = proof_payload();
    payload.remove(constants::field::LAN_PROOF_DIGEST);
    let proof_event = handle_command_text_for_test(
        &serialize_command(pairing_command(payload)),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(proof_event.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        proof_event
            .payload
            .get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_PAIRING_PROOF_REJECTED.to_string()
        ))
    );
    assert_eq!(
        proof_event
            .payload
            .get(constants::field::LAN_EVIDENCE_REFERENCE_IDS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::EVIDENCE_REFERENCE_ID.to_string()
        ))
    );
    assert_eq!(
        proof_event.payload.get(constants::field::LAN_PROOF_DIGEST),
        None
    );
}

#[tokio::test]
async fn lan_pairing_route_select_allows_selected_child_control() {
    let runtime = LanPairingRuntime::empty();
    let _ = handle_command_text_for_test(
        &serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let select_event =
        select_first_child(runtime.clone(), constants::lan_pairing::SELECT_INTENT_ID).await;
    let health_event = handle_command_text_for_test(
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

    assert_route_selected(
        &select_event,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
    );
    assert_eq!(health_event.event, AgentEventName::AgentHealthReported);
    assert_accepted_control(&health_event);
}

#[tokio::test]
async fn lan_pairing_route_select_makes_multi_device_control_explicit() {
    let runtime = crate::lan_pairing_test_support::paired_runtime().await;
    pair_second_child_and_select_it(runtime.clone()).await;
    let first_before_selection = health_for_first_child(runtime.clone()).await;
    let select_first = select_first_child(
        runtime.clone(),
        constants::lan_pairing::SELECT_BACK_INTENT_ID,
    )
    .await;
    let first_after_selection = health_for_first_child(runtime.clone()).await;
    let second_after_selection = health_for_second_child(runtime).await;

    assert_rejection(
        &first_before_selection,
        constants::value::LAN_REASON_UNSELECTED_DEVICE,
    );
    assert_route_selected(
        &select_first,
        constants::lan_pairing::CHILD_DEVICE_ID,
        &[
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
        ]
        .join(&constants::delimiter::LIST.to_string()),
    );
    assert_eq!(
        first_after_selection.event,
        AgentEventName::AgentHealthReported
    );
    assert_accepted_control(&first_after_selection);
    assert_rejection(
        &second_after_selection,
        constants::value::LAN_REASON_UNSELECTED_DEVICE,
    );
    assert_second_child_unselected_rejection(&second_after_selection);
}

async fn pair_second_child_and_select_it(runtime: LanPairingRuntime) {
    let _ = handle_command_text_for_test(
        &serialize_command(pairing_command_for_target(
            constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
            second_proof_payload(),
        )),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let _ = handle_command_text_for_test(
        &serialize_command(route_select_command_for_target(
            constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
            intent_payload_for_pairing(
                constants::lan_pairing::SECOND_SELECT_INTENT_ID,
                constants::lan_pairing::SECOND_PAIRING_ID,
                constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
                constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK,
                constants::lan_pairing::SECOND_PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
                constants::value::LAN_INTENT_CONFIGURATION_UPDATE,
            ),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
}

async fn select_first_child(
    runtime: LanPairingRuntime,
    intent_id: &str,
) -> ocentra_parent_agent_protocol::AgentEventEnvelope {
    handle_command_text_for_test(
        &serialize_command(route_select_command(intent_payload(
            intent_id,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await
}

async fn health_for_first_child(
    runtime: LanPairingRuntime,
) -> ocentra_parent_agent_protocol::AgentEventEnvelope {
    handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await
}

async fn health_for_second_child(
    runtime: LanPairingRuntime,
) -> ocentra_parent_agent_protocol::AgentEventEnvelope {
    handle_command_text_for_test(
        &serialize_command(health_command_for_target(
            constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
            intent_payload_for_pairing(
                constants::lan_pairing::SECOND_INTENT_ID,
                constants::lan_pairing::SECOND_PAIRING_ID,
                constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
                constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK,
                constants::lan_pairing::SECOND_PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
                constants::value::LAN_INTENT_HEALTH_QUERY,
            ),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await
}

fn assert_route_selected(
    event: &AgentEventEnvelope,
    child_device_id: &str,
    trusted_device_ids: &str,
) {
    assert_eq!(event.event, AgentEventName::AgentLanPairingStatusReported);
    assert_status_selection(
        event,
        constants::value::LAN_AUTH_PAIRED,
        child_device_id,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        trusted_device_ids,
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_ROUTE_SELECTED.to_string()
        ))
    );
}

fn assert_second_child_unselected_rejection(event: &AgentEventEnvelope) {
    assert_eq!(
        event.payload.get(constants::field::LAN_INTENT_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SECOND_INTENT_ID.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_PAIRING_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SECOND_PAIRING_ID.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_ROUTE_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK.to_string()
        ))
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
