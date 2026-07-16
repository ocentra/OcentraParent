use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use std::fmt::Display;

use crate::{
    app::{
        fields::fields_from_pairs, lan_pairing::LanPairingRuntime,
        websocket::handle_command_text_for_test,
    },
    lan_pairing_test_assertions::{
        assert_accepted_control, assert_rejection, assert_status_selection,
        assert_status_support_surface,
    },
    lan_pairing_test_commands::{
        health_command, health_command_for_target, intent_payload, intent_payload_for_pairing,
        paired_runtime, pairing_command, pairing_command_for_target, proof_payload,
        proof_payload_for_pairing, route_select_command, route_select_command_for_target,
        serialize_command, status_command,
    },
    lan_pairing_test_multidevice_commands::second_proof_payload,
    test_text::TestText,
};

#[tokio::test]
async fn lan_pairing_proof_reports_trusted_but_unselected_state() {
    let runtime = LanPairingRuntime::empty();
    let proof_event = handle_command_text_for_test(
        serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let health_before_selection = handle_command_text_for_test(
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
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
    payload = without_fields(
        payload,
        [TestText::from_display(constants::field::LAN_PROOF_DIGEST)],
    );
    let proof_event = handle_command_text_for_test(
        serialize_command(pairing_command(payload)),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
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
        serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let select_event =
        select_first_child(runtime.clone(), constants::lan_pairing::SELECT_INTENT_ID).await;
    let health_event = handle_command_text_for_test(
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
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
    let runtime = paired_runtime().await;
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
    let device_id_delimiter = constants::delimiter::LIST.to_string();
    assert_route_selected(
        &select_first,
        constants::lan_pairing::CHILD_DEVICE_ID,
        [
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
        ]
        .join(device_id_delimiter.as_str()),
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

#[tokio::test]
async fn lan_pairing_local_child_identity_rejects_wrong_agent_port_before_execution() {
    let runtime = LanPairingRuntime::empty_with_local_child_device_id(Some(
        TestText::from_display(constants::lan_pairing::CHILD_DEVICE_ID),
    ));
    let wrong_port_proof = handle_command_text_for_test(
        serialize_command(pairing_command_for_target(
            constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
            second_proof_payload(),
        )),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let wrong_port_control = handle_command_text_for_test(
        serialize_command(health_command_for_target(
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
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(wrong_port_proof.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        wrong_port_proof
            .payload
            .get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_PAIRING_PROOF_REJECTED.to_string()
        ))
    );
    assert_eq!(
        wrong_port_proof
            .payload
            .get(constants::field::LAN_REJECTION_REASON),
        Some(&LogFieldValue::String(
            constants::value::LAN_REASON_WRONG_DEVICE.to_string()
        ))
    );
    assert_rejection(
        &wrong_port_control,
        constants::value::LAN_REASON_WRONG_DEVICE,
    );
    assert_eq!(runtime.trusted_device_count(), 0);
}

#[tokio::test]
async fn lan_pairing_status_issues_challenge_preview_before_pairing_proof() {
    let runtime = LanPairingRuntime::empty();
    let challenge = issued_challenge(runtime.clone()).await;
    let proof_payload =
        proof_payload_from_challenge(&challenge, constants::lan_pairing::EXPIRES_AT);
    let proof_event = handle_command_text_for_test(
        serialize_command(pairing_command(proof_payload)),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        challenge.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_status_support_surface(&challenge);
    assert_eq!(
        challenge.payload.get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PAIRING_PAIRING.to_string()
        ))
    );
    assert_eq!(
        challenge
            .payload
            .get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_PAIRING_CHALLENGE_ISSUED.to_string()
        ))
    );
    assert_eq!(
        proof_event.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_eq!(runtime.trusted_device_count(), 1);
}

#[tokio::test]
async fn lan_pairing_challenge_preview_rejects_wrong_origin_stale_and_malformed_proofs() {
    macro_rules! assert_pairing_rejection {
        ($event:expr, $reason:expr) => {{
            assert_eq!($event.event, AgentEventName::AgentCommandRejected);
            assert_eq!(
                $event.payload.get(constants::field::LAN_AUDIT_EVENT_TYPE),
                Some(&LogFieldValue::String(
                    constants::value::LAN_AUDIT_PAIRING_PROOF_REJECTED.to_string()
                ))
            );
            assert_eq!(
                $event.payload.get(constants::field::LAN_REJECTION_REASON),
                Some(&LogFieldValue::String($reason.to_string()))
            );
        }};
    }

    let runtime = LanPairingRuntime::empty();
    let challenge = issued_challenge(runtime.clone()).await;
    let mut wrong_origin =
        proof_payload_from_challenge(&challenge, constants::lan_pairing::EXPIRES_AT);
    wrong_origin.insert(
        constants::field::ORIGIN.to_string(),
        LogFieldValue::String(constants::lan_pairing::WRONG_ORIGIN.to_string()),
    );
    let mut malformed =
        proof_payload_from_challenge(&challenge, constants::lan_pairing::EXPIRES_AT);
    malformed = without_fields(
        malformed,
        [TestText::from_display(constants::field::LAN_PROOF_DIGEST)],
    );
    let stale = proof_payload_from_challenge(&challenge, constants::lan_pairing::EXPIRED_AT);
    let accepted = proof_payload_from_challenge(&challenge, constants::lan_pairing::EXPIRES_AT);
    let wrong_origin_event = submit_proof(runtime.clone(), wrong_origin).await;
    let malformed_event = submit_proof(runtime.clone(), malformed).await;
    let stale_event = submit_proof(runtime.clone(), stale).await;
    let accepted_event = submit_proof(runtime.clone(), accepted.clone()).await;
    let replayed_event = submit_proof(runtime, accepted).await;

    assert_pairing_rejection!(
        wrong_origin_event,
        constants::value::LAN_REASON_WRONG_ORIGIN
    );
    assert_pairing_rejection!(malformed_event, constants::value::LAN_REASON_MALFORMED);
    assert_pairing_rejection!(stale_event, constants::value::LAN_REASON_STALE);
    assert_eq!(
        accepted_event.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_pairing_rejection!(replayed_event, constants::value::LAN_REASON_REPLAYED);
}

async fn pair_second_child_and_select_it(runtime: LanPairingRuntime) {
    let _ = handle_command_text_for_test(
        serialize_command(pairing_command_for_target(
            constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
            second_proof_payload(),
        )),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let _ = handle_command_text_for_test(
        serialize_command(route_select_command_for_target(
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
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
}

async fn issued_challenge(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(
        serialize_command(status_command(challenge_request_payload(
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

async fn submit_proof(runtime: LanPairingRuntime, payload: LogFields) -> AgentEventEnvelope {
    handle_command_text_for_test(
        serialize_command(pairing_command(payload)),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

fn challenge_request_payload(expires_at: impl Display) -> LogFields {
    let expires_at = TestText::from_display(expires_at);
    fields_from_pairs(vec![
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
            constants::field::STARTED_AT,
            LogFieldValue::String(constants::lan_pairing::ISSUED_AT.to_string()),
        ),
        (
            constants::field::STALE_AT,
            LogFieldValue::String(expires_at.to_string()),
        ),
    ])
}

fn proof_payload_from_challenge(event: &AgentEventEnvelope, expires_at: impl Display) -> LogFields {
    let expires_at = TestText::from_display(expires_at);
    let challenge_id = payload_string(event, constants::field::LAN_CHALLENGE_ID);
    let proof_digest = payload_string(event, constants::field::LAN_PROOF_DIGEST);
    let mut payload = proof_payload_for_pairing(
        constants::lan_pairing::PAIRING_ID,
        challenge_id.0.as_str(),
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        proof_digest.0.as_str(),
    );
    payload.insert(
        constants::field::STALE_AT.to_string(),
        LogFieldValue::String(expires_at.to_string()),
    );
    payload
}

fn payload_string(event: &AgentEventEnvelope, field: impl Display) -> TestText {
    let field = TestText::from_display(field);
    match event.payload.get(field.0.as_str()) {
        Some(LogFieldValue::String(value)) => TestText::from_display(value),
        _ => TestText::from_display(format!("{field} missing string payload")),
    }
}

async fn select_first_child(
    runtime: LanPairingRuntime,
    intent_id: impl Display,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    let intent_id = TestText::from_display(intent_id);
    handle_command_text_for_test(
        serialize_command(route_select_command(intent_payload(
            intent_id.to_string(),
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

async fn health_for_first_child(
    runtime: LanPairingRuntime,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    handle_command_text_for_test(
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

async fn health_for_second_child(
    runtime: LanPairingRuntime,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    handle_command_text_for_test(
        serialize_command(health_command_for_target(
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
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

fn assert_route_selected(
    event: &AgentEventEnvelope,
    child_device_id: impl Display,
    trusted_device_ids: impl Display,
) {
    let child_device_id = TestText::from_display(child_device_id);
    let trusted_device_ids = TestText::from_display(trusted_device_ids);
    assert_eq!(event.event, AgentEventName::AgentLanPairingStatusReported);
    assert_status_selection(
        event,
        constants::value::LAN_AUTH_PAIRED,
        child_device_id.0.as_str(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        trusted_device_ids.0.as_str(),
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

fn without_fields<I, T>(payload: LogFields, keys: I) -> LogFields
where
    I: IntoIterator<Item = T>,
    T: Display,
{
    let mut inner = payload.into_inner();
    for key in keys {
        let key = TestText::from_display(key);
        inner.remove(key.0.as_str());
    }
    LogFields::from(inner)
}
