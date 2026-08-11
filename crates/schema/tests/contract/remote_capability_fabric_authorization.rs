use ocentra_schema::remote_capability_fabric as contracts;

fn paired_parent_grant(
    actor_role: contracts::RemoteActorRole,
    parent_actor_ref: &str,
) -> contracts::RemoteCapabilityGrant {
    contracts::RemoteCapabilityGrant {
        schema_version: contracts::REMOTE_CAPABILITY_FABRIC_SCHEMA_VERSION.to_string(),
        grant_ref: "remote-grant-parent-authorization".to_string(),
        household_ref: "household-alpha".to_string(),
        child_device_ref: "child-device-alpha".to_string(),
        route: contracts::RemoteRoute::LocalNetwork,
        parent_actor_ref: parent_actor_ref.to_string(),
        support_actor_ref: None,
        parent_grant: contracts::RemoteParentGrantState::NotGranted,
        capability_type: contracts::RemoteCapabilityType::LiveView,
        actor_role,
        pairing_state: contracts::RemotePairingState::Paired,
        grant_state: contracts::RemoteGrantState::Active,
        session_state: contracts::RemoteSessionState::Connecting,
        device_trust_state: contracts::RemoteDeviceTrustState::Trusted,
        audit_ref: "remote-audit-parent-authorization".to_string(),
        diagnostic_redaction_state: contracts::RemoteDiagnosticRedactionState::Redacted,
        no_claim: "not-remote-control; not-relay-production-readiness".to_string(),
    }
}

#[test]
fn remote_capability_requires_a_granted_parent_authorization_for_parent_roles() {
    for (role, actor_ref) in [
        (
            contracts::RemoteActorRole::ParentOwner,
            "parent-owner-alpha",
        ),
        (contracts::RemoteActorRole::CoParent, "co-parent-alpha"),
    ] {
        assert_eq!(
            paired_parent_grant(role, actor_ref).authorize_live_view(
                "household-alpha",
                actor_ref,
                "child-device-alpha",
                contracts::RemoteRoute::LocalNetwork,
            ),
            Err(contracts::RemoteCapabilityAuthorizationError::WrongActorRole)
        );
    }
}

#[test]
fn remote_capability_rejects_blank_parent_actor_references() {
    for role in [
        contracts::RemoteActorRole::ParentOwner,
        contracts::RemoteActorRole::CoParent,
    ] {
        for (stored_actor_ref, requesting_actor_ref) in
            [("", ""), (" ", " "), ("parent-owner-alpha", " ")]
        {
            let mut grant = paired_parent_grant(role.clone(), stored_actor_ref);
            grant.parent_grant = contracts::RemoteParentGrantState::Granted;
            assert_eq!(
                grant.authorize_live_view(
                    "household-alpha",
                    requesting_actor_ref,
                    "child-device-alpha",
                    contracts::RemoteRoute::LocalNetwork,
                ),
                Err(contracts::RemoteCapabilityAuthorizationError::WrongParentActor)
            );
        }
    }
}
