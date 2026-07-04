use serde_json::json;

use crate::constants::household_mesh as mesh;
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshAuthenticationState, HouseholdMeshBridgeState, HouseholdMeshLocalRepublish,
    HouseholdMeshPolicyAuthority, HouseholdMeshTransportEnvelope,
};

#[test]
fn household_mesh_transport_envelope_roundtrips_with_expected_wire_shape() {
    let envelope = HouseholdMeshTransportEnvelope::proof_fixture_for(
        mesh::LOCAL_EVENT_AI_RESULT_RETURN,
        mesh::LAN_MESSAGE_AI_RESULT_RETURN,
    );

    let value = serde_json::to_value(&envelope).expect("transport envelope serializes");
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
        serde_json::from_value(value).expect("transport envelope deserializes");
    assert_eq!(decoded, envelope);
}

#[test]
fn household_mesh_enum_wire_strings_match_constants() {
    assert_eq!(
        serde_json::to_string(&HouseholdMeshBridgeState::LocalRepublishRequired)
            .expect("bridge state serializes"),
        format!("\"{}\"", mesh::BRIDGE_STATE_LOCAL_REPUBLISH_REQUIRED)
    );
    assert_eq!(
        serde_json::to_string(&HouseholdMeshAuthenticationState::StaleOrRevoked)
            .expect("authentication state serializes"),
        format!("\"{}\"", mesh::AUTHENTICATION_STALE_OR_REVOKED)
    );
    assert_eq!(
        serde_json::to_string(&HouseholdMeshPolicyAuthority::ParentUiClaimed)
            .expect("policy authority serializes"),
        format!("\"{}\"", mesh::POLICY_AUTHORITY_PARENT_UI_CLAIMED)
    );
}

#[test]
fn household_mesh_local_republish_preserves_child_owned_boundary() {
    let message = HouseholdMeshTransportEnvelope::proof_fixture_for(
        mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION,
        mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION,
    );

    let republish = HouseholdMeshLocalRepublish::from_validated_message(&message);
    assert_eq!(republish.family_id, mesh::TEST_BRIDGE_FAMILY_ID);
    assert_eq!(
        republish.target_child_device_id,
        mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID
    );
    assert_eq!(
        republish.source_peer_id,
        mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID
    );
    assert_eq!(
        republish.local_event_ref,
        mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION
    );
    assert_eq!(
        republish.lan_message_type,
        mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION
    );
    assert_eq!(
        republish.bridge_state,
        HouseholdMeshBridgeState::LocalRepublishRequired
    );
    assert_eq!(
        republish.policy_authority,
        HouseholdMeshPolicyAuthority::ChildAgentOnly
    );
    assert!(republish.validated_before_republish);
    assert!(republish.child_agent_policy_authority_preserved);
}
