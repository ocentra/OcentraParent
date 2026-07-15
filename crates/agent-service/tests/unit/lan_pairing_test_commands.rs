use std::fmt::Display;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentPeer;
use ocentra_parent_agent_protocol::transport::AgentPeerRole;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::app::{
    fields::fields_from_pairs, lan_pairing::LanPairingRuntime,
    websocket::handle_command_text_for_test,
};
use crate::test_text::TestText;

const PAIRED_RUNTIME_LEASE_INTENT_ID: &str = "intent-paired-runtime-lease";

pub(crate) async fn paired_runtime() -> LanPairingRuntime {
    let runtime = LanPairingRuntime::empty();
    let pairing_event = handle_command_text_for_test(
        serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    assert_eq!(
        pairing_event.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentLanPairingStatusReported,
        "pairing ceremony failed: {:?}",
        pairing_event.payload
    );
    assert_eq!(
        runtime.trusted_device_count(),
        1,
        "pairing ceremony did not establish trusted registry: {:?}",
        pairing_event.payload
    );
    let mut lease_payload = intent_payload_for_kind(
        PAIRED_RUNTIME_LEASE_INTENT_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::PROOF_DIGEST,
        constants::lan_pairing::EXPIRES_AT,
        constants::value::LAN_INTENT_CONTROLLER_LEASE_RENEW,
    );
    lease_payload.insert(
        constants::field::LAN_INTENT_KIND.to_string(),
        LogFieldValue::String(constants::value::LAN_INTENT_CONTROLLER_LEASE_RENEW.to_string()),
    );
    let mut lease_command = command_for_target(
        AgentCommandName::AgentLanPairingControllerLeaseRenew,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        lease_payload,
    );
    lease_command.message_id = PAIRED_RUNTIME_LEASE_INTENT_ID.to_string();
    let lease_event = handle_command_text_for_test(
        serialize_command(lease_command),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    assert_eq!(
        lease_event.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentLanPairingStatusReported,
        "controller lease setup failed: {:?}",
        lease_event.payload
    );
    let route_event = handle_command_text_for_test(
        serialize_command(route_select_command(intent_payload(
            constants::lan_pairing::SELECT_INTENT_ID,
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
        route_event.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentLanPairingStatusReported,
        "route selection failed: {:?}",
        route_event.payload
    );
    runtime
}

pub(crate) fn pairing_command(payload: LogFields) -> AgentCommandEnvelope {
    pairing_command_for_target(constants::lan_pairing::CHILD_DEVICE_ID, payload)
}

pub(crate) fn pairing_command_for_target(
    device_id: impl Display,
    payload: LogFields,
) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentLanPairingProofSubmit,
        local_network_target(device_id),
        payload,
    )
}

pub(crate) fn health_command(payload: LogFields) -> AgentCommandEnvelope {
    health_command_for_target(constants::lan_pairing::CHILD_DEVICE_ID, payload)
}

pub(crate) fn health_command_for_target(
    device_id: impl Display,
    payload: LogFields,
) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentHealthCheck,
        local_network_target(device_id),
        payload,
    )
}

pub(crate) fn route_select_command(payload: LogFields) -> AgentCommandEnvelope {
    route_select_command_for_target(constants::lan_pairing::CHILD_DEVICE_ID, payload)
}

pub(crate) fn route_select_command_for_target(
    device_id: impl Display,
    payload: LogFields,
) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentLanPairingRouteSelect,
        local_network_target(device_id),
        payload,
    )
}

pub(crate) fn status_command(payload: LogFields) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentLanPairingStatusGet,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        payload,
    )
}

pub(crate) fn command_for_target(
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

pub(crate) fn local_network_target(device_id: impl Display) -> AgentMessageTarget {
    let device_id = TestText::from_display(device_id);
    AgentMessageTarget {
        device_id: device_id.to_string(),
        platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
        route: AgentRoute::LocalNetwork,
    }
}

pub(crate) fn proof_payload() -> LogFields {
    proof_payload_for_pairing(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::CHALLENGE_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        constants::lan_pairing::PROOF_DIGEST,
    )
}

pub(crate) fn proof_payload_for_pairing(
    pairing_id: impl Display,
    challenge_id: impl Display,
    child_device_id: impl Display,
    route_id: impl Display,
    proof_digest: impl Display,
) -> LogFields {
    let pairing_id = TestText::from_display(pairing_id);
    let challenge_id = TestText::from_display(challenge_id);
    let child_device_id = TestText::from_display(child_device_id);
    let route_id = TestText::from_display(route_id);
    let proof_digest = TestText::from_display(proof_digest);
    fields_from_pairs(vec![
        (
            constants::field::LAN_PAIRING_ID,
            LogFieldValue::String(pairing_id.to_string()),
        ),
        (
            constants::field::LAN_CHALLENGE_ID,
            LogFieldValue::String(challenge_id.to_string()),
        ),
        (
            constants::field::LAN_CHILD_DEVICE_ID,
            LogFieldValue::String(child_device_id.to_string()),
        ),
        (
            constants::field::LAN_PARENT_DEVICE_ID,
            LogFieldValue::String(constants::lan_pairing::PARENT_DEVICE_ID.to_string()),
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
            constants::field::LAN_EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(constants::lan_pairing::EVIDENCE_REFERENCE_ID.to_string()),
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

pub(crate) fn intent_payload(
    intent_id: impl Display,
    target_device_id: impl Display,
    proof_digest: impl Display,
    expires_at: impl Display,
) -> LogFields {
    intent_payload_for_kind(
        intent_id,
        target_device_id,
        proof_digest,
        expires_at,
        constants::value::LAN_INTENT_HEALTH_QUERY,
    )
}

pub(crate) fn intent_payload_for_kind(
    intent_id: impl Display,
    target_device_id: impl Display,
    proof_digest: impl Display,
    expires_at: impl Display,
    intent_kind: impl Display,
) -> LogFields {
    intent_payload_for_pairing(
        intent_id,
        constants::lan_pairing::PAIRING_ID,
        target_device_id,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        proof_digest,
        expires_at,
        intent_kind,
    )
}

pub(crate) fn intent_payload_for_pairing(
    intent_id: impl Display,
    pairing_id: impl Display,
    target_device_id: impl Display,
    route_id: impl Display,
    proof_digest: impl Display,
    expires_at: impl Display,
    intent_kind: impl Display,
) -> LogFields {
    let intent_id = TestText::from_display(intent_id);
    let pairing_id = TestText::from_display(pairing_id);
    let target_device_id = TestText::from_display(target_device_id);
    let route_id = TestText::from_display(route_id);
    let proof_digest = TestText::from_display(proof_digest);
    let expires_at = TestText::from_display(expires_at);
    let intent_kind = TestText::from_display(intent_kind);
    fields_from_pairs(vec![
        (
            constants::field::LAN_INTENT_ID,
            LogFieldValue::String(intent_id.to_string()),
        ),
        (
            constants::field::LAN_INTENT_KIND,
            LogFieldValue::String(intent_kind.to_string()),
        ),
        (
            constants::field::LAN_PAIRING_ID,
            LogFieldValue::String(pairing_id.to_string()),
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
            constants::field::LAN_EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(constants::lan_pairing::EVIDENCE_REFERENCE_ID.to_string()),
        ),
        (
            constants::field::STARTED_AT,
            LogFieldValue::String(constants::lan_pairing::ISSUED_AT.to_string()),
        ),
        (
            constants::field::STALE_AT,
            LogFieldValue::String(expires_at.to_string()),
        ),
        (
            constants::field::LAN_CONTROLLER_LEASE_ID,
            LogFieldValue::String(constants::lan_pairing::CONTROLLER_LEASE_ID.to_string()),
        ),
        (
            constants::field::LAN_CONTROLLER_DEVICE_ID,
            LogFieldValue::String(constants::lan_pairing::PARENT_DEVICE_ID.to_string()),
        ),
        (
            constants::field::LAN_PARENT_ACTOR_ID,
            LogFieldValue::String(constants::lan_pairing::PARENT_ACTOR_ID.to_string()),
        ),
        (
            constants::field::LAN_PARENT_AUTHORITY,
            LogFieldValue::String(
                constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER.to_string(),
            ),
        ),
        (
            constants::field::LAN_CONTROLLER_LEASE_ISSUED_AT,
            LogFieldValue::String(constants::lan_pairing::ISSUED_AT.to_string()),
        ),
        (
            constants::field::LAN_CONTROLLER_LEASE_EXPIRES_AT,
            LogFieldValue::String(constants::lan_pairing::CONTROLLER_LEASE_EXPIRES_AT.to_string()),
        ),
    ])
}

pub(crate) fn serialize_command(command: AgentCommandEnvelope) -> TestText {
    let command_value = serde_json::to_value(command).unwrap_or_else(|_| std::process::abort());
    let serialized =
        serde_json::to_string(&command_value).unwrap_or_else(|_| std::process::abort());
    TestText::from_display(serialized)
}
