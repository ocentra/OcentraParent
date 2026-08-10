use crate::support::result_or_unreachable;
use ocentra_schema::remote_capability_fabric as contracts;
use serde_json::json;

fn paired_live_view_grant() -> contracts::RemoteCapabilityGrant {
    contracts::RemoteCapabilityGrant {
        schema_version: contracts::REMOTE_CAPABILITY_FABRIC_SCHEMA_VERSION.to_string(),
        grant_ref: "remote-grant-alpha".to_string(),
        household_ref: "household-alpha".to_string(),
        child_device_ref: "child-device-alpha".to_string(),
        route: contracts::RemoteRoute::LocalNetwork,
        parent_actor_ref: "parent-owner-alpha".to_string(),
        parent_grant: contracts::RemoteParentGrantState::NotGranted,
        capability_type: contracts::RemoteCapabilityType::LiveView,
        actor_role: contracts::RemoteActorRole::ParentOwner,
        pairing_state: contracts::RemotePairingState::Paired,
        grant_state: contracts::RemoteGrantState::Active,
        session_state: contracts::RemoteSessionState::Connecting,
        device_trust_state: contracts::RemoteDeviceTrustState::Trusted,
        audit_ref: "remote-audit-alpha".to_string(),
        diagnostic_redaction_state: "redacted".to_string(),
        no_claim: "not-remote-control; not-relay-production-readiness".to_string(),
    }
}

#[test]
fn remote_capability_paired_access_round_trips_the_rust_owned_contract() {
    let grant = paired_live_view_grant();
    let encoded = result_or_unreachable(
        serde_json::to_value(&grant),
        crate::assert_context!("grant serializes"),
    );

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::REMOTE_CAPABILITY_FABRIC_SCHEMA_VERSION)
    );
    assert_eq!(encoded["capabilityType"], json!("live-view"));
    assert_eq!(encoded["pairingState"], json!("paired"));
    assert_eq!(encoded["route"], json!("local-network"));
    assert_eq!(encoded["diagnosticRedactionState"], json!("redacted"));
    assert!(encoded.get("schema_version").is_none());
    assert_eq!(
        result_or_unreachable(
            serde_json::from_value::<contracts::RemoteCapabilityGrant>(encoded),
            crate::assert_context!("grant deserializes"),
        ),
        grant
    );
}

#[test]
fn remote_capability_live_view_authorizes_only_the_paired_trusted_household_grant() {
    assert_eq!(
        paired_live_view_grant().authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Ok(())
    );
    assert_eq!(
        paired_live_view_grant().authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::CloudRelay,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::WrongRoute)
    );
}

#[test]
fn remote_capability_rejects_deferred_control_wrong_household_role_pairing_trust_and_terminal_grants(
) {
    let mut grant = paired_live_view_grant();
    grant.capability_type = contracts::RemoteCapabilityType::RemoteControlDeferred;
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::CapabilityDeferred)
    );

    let mut grant = paired_live_view_grant();
    assert_eq!(
        grant.authorize_live_view(
            "household-other",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::WrongHousehold)
    );
    grant.actor_role = contracts::RemoteActorRole::SupportAdmin;
    grant.parent_grant = contracts::RemoteParentGrantState::NotGranted;
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::WrongActorRole)
    );

    let grant = paired_live_view_grant();
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-other",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::WrongParentActor)
    );

    let grant = paired_live_view_grant();
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-other",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::WrongChildDevice)
    );

    let mut grant = paired_live_view_grant();
    grant.pairing_state = contracts::RemotePairingState::Requested;
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::PairingRequired)
    );

    let mut grant = paired_live_view_grant();
    grant.device_trust_state = contracts::RemoteDeviceTrustState::Missing;
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::DeviceTrustRequired)
    );

    let mut grant = paired_live_view_grant();
    grant.grant_state = contracts::RemoteGrantState::Revoked;
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::Revoked)
    );

    let mut grant = paired_live_view_grant();
    grant.grant_state = contracts::RemoteGrantState::Removed;
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::DeviceRemoved)
    );
}

#[test]
fn remote_capability_allows_only_parent_granted_support_live_view() {
    let mut grant = paired_live_view_grant();
    grant.actor_role = contracts::RemoteActorRole::SupportAdmin;
    grant.parent_grant = contracts::RemoteParentGrantState::Granted;
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Ok(())
    );
}

#[test]
fn remote_capability_v1_payload_is_deserializable_but_rejected_by_version() {
    let mut encoded = result_or_unreachable(
        serde_json::to_value(paired_live_view_grant()),
        crate::assert_context!("grant serializes"),
    );
    encoded["schemaVersion"] = json!("remote-capability-fabric-v1");
    encoded.as_object_mut().map(|object| {
        object.remove("route");
        object.remove("parentGrant");
    });
    let restored = result_or_unreachable(
        serde_json::from_value::<contracts::RemoteCapabilityGrant>(encoded),
        crate::assert_context!("legacy grant deserializes with defaults"),
    );
    assert_eq!(restored.route, contracts::RemoteRoute::Localhost);
    assert_eq!(
        restored.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::Localhost,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::UnsupportedSchemaVersion)
    );
}

#[test]
fn remote_capability_rejects_unknown_schema_version_and_missing_audit_reference() {
    let mut grant = paired_live_view_grant();
    grant.schema_version = "remote-capability-fabric-v3".to_string();
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::UnsupportedSchemaVersion)
    );

    let mut grant = paired_live_view_grant();
    grant.audit_ref = "   ".to_string();
    assert_eq!(
        grant.authorize_live_view(
            "household-alpha",
            "parent-owner-alpha",
            "child-device-alpha",
            contracts::RemoteRoute::LocalNetwork,
        ),
        Err(contracts::RemoteCapabilityAuthorizationError::MissingAuditRef)
    );
}
