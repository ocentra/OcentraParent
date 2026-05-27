use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    fields::fields_from_pairs, lan_pairing::LanPairingRuntime,
    websocket::handle_command_text_for_test,
};

pub(crate) async fn paired_runtime() -> LanPairingRuntime {
    let runtime = LanPairingRuntime::empty();
    let _ = handle_command_text_for_test(
        &serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let _ = handle_command_text_for_test(
        &serialize_command(route_select_command(intent_payload(
            constants::lan_pairing::SELECT_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    runtime
}

pub(crate) fn pairing_command(payload: LogFields) -> AgentCommandEnvelope {
    pairing_command_for_target(constants::lan_pairing::CHILD_DEVICE_ID, payload)
}

pub(crate) fn pairing_command_for_target(
    device_id: &str,
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
    device_id: &str,
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

pub(crate) fn route_revoke_command(payload: LogFields) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentLanPairingRouteRevoke,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        payload,
    )
}

pub(crate) fn route_select_command_for_target(
    device_id: &str,
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

pub(crate) fn local_network_target(device_id: &str) -> AgentMessageTarget {
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

pub(crate) fn second_proof_payload() -> LogFields {
    proof_payload_for_pairing(
        constants::lan_pairing::SECOND_PAIRING_ID,
        constants::lan_pairing::SECOND_CHALLENGE_ID,
        constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK,
        constants::lan_pairing::SECOND_PROOF_DIGEST,
    )
}

pub(crate) fn proof_payload_for_pairing(
    pairing_id: &str,
    challenge_id: &str,
    child_device_id: &str,
    route_id: &str,
    proof_digest: &str,
) -> LogFields {
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
    intent_id: &str,
    target_device_id: &str,
    proof_digest: &str,
    expires_at: &str,
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
    intent_id: &str,
    target_device_id: &str,
    proof_digest: &str,
    expires_at: &str,
    intent_kind: &str,
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
    intent_id: &str,
    pairing_id: &str,
    target_device_id: &str,
    route_id: &str,
    proof_digest: &str,
    expires_at: &str,
    intent_kind: &str,
) -> LogFields {
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
            constants::field::LAN_CONTROLLER_LEASE_ISSUED_AT,
            LogFieldValue::String(constants::lan_pairing::ISSUED_AT.to_string()),
        ),
        (
            constants::field::LAN_CONTROLLER_LEASE_EXPIRES_AT,
            LogFieldValue::String(constants::lan_pairing::CONTROLLER_LEASE_EXPIRES_AT.to_string()),
        ),
    ])
}

pub(crate) fn serialize_command(command: AgentCommandEnvelope) -> String {
    serde_json::to_string(&command).expect(constants::error::AGENT_EVENT_SERIALIZES)
}
