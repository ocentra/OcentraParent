use ocentra_eventing::expect_value::ExpectValue;
use serde_json::json;

use crate::constants::household_mesh as mesh;
use ocentra_parent_agent_protocol::household_mesh::household_mesh_bridge_input::HouseholdMeshBridgeInboundEnvelope;
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshAuthenticationState, HouseholdMeshBridgeRejectionReason, HouseholdMeshBridgeState,
    HouseholdMeshPolicyAuthority, HouseholdMeshTransportEnvelope,
};

#[test]
fn household_mesh_transport_envelope_roundtrips_with_expected_wire_shape() {
    let envelope = transport_envelope(
        mesh::LOCAL_EVENT_AI_RESULT_RETURN,
        mesh::LAN_MESSAGE_AI_RESULT_RETURN,
    );

    let value = serde_json::to_value(&envelope).expect_value("transport envelope serializes");
    assert_eq!(value["schemaVersion"], json!(mesh::EVENT_SCHEMA_VERSION));
    assert_eq!(
        value["messageId"],
        json!(mesh::TEST_BRIDGE_INBOUND_MESSAGE_ID)
    );
    assert_eq!(
        value["idempotencyKey"],
        json!(mesh::TEST_BRIDGE_IDEMPOTENCY_KEY)
    );
    assert_eq!(value["familyId"], json!(mesh::TEST_BRIDGE_FAMILY_ID));
    assert_eq!(
        value["targetChildDeviceId"],
        json!(mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID)
    );
    assert_eq!(
        value["localEventRef"],
        json!(mesh::LOCAL_EVENT_AI_RESULT_RETURN)
    );
    assert_eq!(
        value["lanMessageType"],
        json!(mesh::LAN_MESSAGE_AI_RESULT_RETURN)
    );
    assert_eq!(
        value["bridgeState"],
        json!(mesh::BRIDGE_STATE_EXPORT_SELECTED)
    );
    assert_eq!(
        value["authenticationState"],
        json!(mesh::AUTHENTICATION_PAIRED_TRUSTED_DEVICE)
    );
    assert_eq!(
        value["policyAuthority"],
        json!(mesh::POLICY_AUTHORITY_CHILD_AGENT_ONLY)
    );

    let decoded: HouseholdMeshTransportEnvelope =
        serde_json::from_value(value).expect_value("transport envelope deserializes");
    assert_eq!(decoded, envelope);
}

#[test]
fn household_mesh_enum_wire_strings_match_constants() {
    assert_eq!(
        serde_json::to_string(&HouseholdMeshBridgeState::LocalRepublishRequired)
            .expect_value("bridge state serializes"),
        format!("\"{}\"", mesh::BRIDGE_STATE_LOCAL_REPUBLISH_REQUIRED)
    );
    assert_eq!(
        serde_json::to_string(&HouseholdMeshAuthenticationState::StaleOrRevoked)
            .expect_value("authentication state serializes"),
        format!("\"{}\"", mesh::AUTHENTICATION_STALE_OR_REVOKED)
    );
    assert_eq!(
        serde_json::to_string(&HouseholdMeshPolicyAuthority::ParentUiClaimed)
            .expect_value("policy authority serializes"),
        format!("\"{}\"", mesh::POLICY_AUTHORITY_PARENT_UI_CLAIMED)
    );
}

#[test]
fn household_mesh_validation_preserves_child_owned_boundary() {
    let message = transport_envelope(
        mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION,
        mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION,
    );

    let validated = HouseholdMeshBridgeInboundEnvelope::for_structural_validation(
        message,
        mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
        mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID.to_string(),
        mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
        Vec::new(),
        Vec::new(),
    )
    .validate_structure()
    .expect_value("household mesh envelope validates");
    let validated_message = validated.message();
    assert_eq!(validated_message.family_id, mesh::TEST_BRIDGE_FAMILY_ID);
    assert_eq!(
        validated_message.target_child_device_id,
        mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID
    );
    assert_eq!(
        validated_message.source_peer_id,
        mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID
    );
    assert_eq!(
        validated_message.local_event_ref,
        mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION
    );
    assert_eq!(
        validated_message.lan_message_type,
        mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION
    );
    assert_eq!(
        validated_message.bridge_state,
        HouseholdMeshBridgeState::ExportSelected
    );
    assert_eq!(
        validated_message.policy_authority,
        HouseholdMeshPolicyAuthority::ChildAgentOnly
    );
}

#[test]
fn household_mesh_validation_rejects_an_unselected_local_event() {
    let mut message = transport_envelope(
        mesh::LOCAL_EVENT_AI_RESULT_RETURN,
        mesh::LAN_MESSAGE_AI_RESULT_RETURN,
    );
    message.local_event_ref = mesh::LOCAL_EVENT_RAW_CAPTURE_INTERNAL.to_string();

    let validation = HouseholdMeshBridgeInboundEnvelope::for_structural_validation(
        message,
        mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
        mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID.to_string(),
        mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
        Vec::new(),
        Vec::new(),
    )
    .validate_structure();
    assert_eq!(
        validation
            .err()
            .and_then(|rejection| rejection.rejection_reason),
        Some(HouseholdMeshBridgeRejectionReason::UnselectedEvent)
    );
}

fn transport_envelope(
    local_event_ref: &str,
    lan_message_type: &str,
) -> HouseholdMeshTransportEnvelope {
    HouseholdMeshTransportEnvelope {
        schema_version: mesh::EVENT_SCHEMA_VERSION,
        message_id: mesh::TEST_BRIDGE_INBOUND_MESSAGE_ID.to_string(),
        idempotency_key: mesh::TEST_BRIDGE_IDEMPOTENCY_KEY.to_string(),
        family_id: mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
        target_child_device_id: mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID.to_string(),
        source_peer_id: mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID.to_string(),
        local_event_ref: local_event_ref.to_string(),
        lan_message_type: lan_message_type.to_string(),
        bridge_state: HouseholdMeshBridgeState::ExportSelected,
        authentication_state: HouseholdMeshAuthenticationState::PairedTrustedDevice,
        policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
        direct_remote_publish_requested: false,
        raw_payload_included: false,
        sent_at_epoch_seconds: mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS,
        stale_after_seconds: mesh::TEST_BRIDGE_STALE_AFTER_SECONDS,
    }
}
