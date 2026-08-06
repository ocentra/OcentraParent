use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_family_identity_core::family_identity::{
    ChildCustodyLabel, ChildProfile, ChildProfileId, DeviceId, DeviceRegistration,
    DeviceRouteStateLabel, HouseholdId, HouseholdMembershipState, HouseholdProfile, HouseholdRole,
    ObserverPermission, ObserverPermissionId, ParentControllerLease, ParentControllerLeaseId,
    ParentMember, ParentMemberId, RecoveryId, RecoveryState as RecoveryContractState,
    SetupAuditActionId, SetupAuditEvent, SetupAuditEventId, SetupAuditEvidenceRef,
    SetupAuditTargetId, SetupInvite, SetupInviteId,
};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentControllerLeaseState,
};
use ocentra_family_identity_core::setup_lifecycle::{
    RecoveryKind, RecoveryState as RecoveryWorkflowState, SetupInviteTargetRole,
};

#[test]
fn household_profile_round_trips_with_member_and_child_ids() {
    let profile = HouseholdProfile::new(
        HouseholdId::parse("household-1").expect_value("household id"),
        "Weekend Home",
        "2026-06-27T00:00:00Z",
        vec![ParentMemberId::parse("parent-1").expect_value("parent member id")],
        vec![ChildProfileId::parse("child-1").expect_value("child profile id")],
    )
    .expect_value("household profile");

    let json = serde_json::to_value(&profile).expect_value("serialize household profile");
    let round_trip: HouseholdProfile =
        serde_json::from_value(json).expect_value("deserialize household profile");

    assert_eq!(round_trip, profile);
}

#[test]
fn parent_member_rejects_non_parent_roles() {
    let error = ParentMember::new(
        ParentMemberId::parse("member-1").expect_value("member id"),
        HouseholdId::parse("household-1").expect_value("household id"),
        HouseholdRole::ChildDeviceAgent,
        HouseholdMembershipState::Active,
        "2026-06-27T00:00:00Z",
    )
    .expect_err_value("child device agent is not a parent member");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "family_identity.parent_member.role",
            value: String::from("ChildDeviceAgent"),
        }
    );
}

#[test]
fn parent_member_round_trips_with_observer_membership_state() {
    let member = ParentMember::new(
        ParentMemberId::parse("member-1").expect_value("member id"),
        HouseholdId::parse("household-1").expect_value("household id"),
        HouseholdRole::Observer,
        HouseholdMembershipState::Invited,
        "2026-06-27T00:00:00Z",
    )
    .expect_value("parent member");

    let json = serde_json::to_value(&member).expect_value("serialize parent member");
    let round_trip: ParentMember =
        serde_json::from_value(json).expect_value("deserialize parent member");

    assert_eq!(round_trip, member);
}

#[test]
fn device_registration_rejects_wrong_household_child_binding() {
    let child_profile = ChildProfile::new(
        ChildProfileId::parse("child-1").expect_value("child profile id"),
        HouseholdId::parse("household-1").expect_value("household id"),
        "Avery",
        vec![DeviceId::parse("device-1").expect_value("device id")],
        ChildCustodyLabel::parse("family-custody").expect_value("custody label"),
    )
    .expect_value("child profile");
    let registration = DeviceRegistration::new(
        DeviceId::parse("device-1").expect_value("device id"),
        ChildProfileId::parse("child-1").expect_value("child profile id"),
        HouseholdId::parse("household-2").expect_value("household id"),
        ocentra_family_identity_core::family_identity::DeviceTrustState::Trusted,
        HouseholdRole::ChildDeviceAgent,
        DeviceRouteStateLabel::parse("lan-selected").expect_value("route state"),
        Some(String::from("2026-06-27T02:00:00Z")),
    )
    .expect_value("device registration");

    let error = registration
        .validate_child_profile(&child_profile)
        .expect_err_value("wrong household binding is rejected");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "family_identity.device_registration.household_id",
            value: String::from("household-2"),
        }
    );
}

#[test]
fn child_profile_round_trips_with_device_ids_and_custody_label() {
    let child_profile = ChildProfile::new(
        ChildProfileId::parse("child-1").expect_value("child profile id"),
        HouseholdId::parse("household-1").expect_value("household id"),
        "Avery",
        vec![DeviceId::parse("device-1").expect_value("device id")],
        ChildCustodyLabel::parse("family-custody").expect_value("custody label"),
    )
    .expect_value("child profile");

    let json = serde_json::to_value(&child_profile).expect_value("serialize child profile");
    let round_trip: ChildProfile =
        serde_json::from_value(json).expect_value("deserialize child profile");

    assert_eq!(round_trip, child_profile);
}

#[test]
fn observer_permission_round_trips_with_read_only_scope() {
    let permission = ObserverPermission::new(
        ObserverPermissionId::parse("perm-1").expect_value("permission id"),
        ParentMemberId::parse("member-1").expect_value("member id"),
        HouseholdId::parse("household-1").expect_value("household id"),
        vec![HouseholdAuthorityAction::ViewChildStatus],
        true,
    )
    .expect_value("observer permission");

    let json = serde_json::to_value(&permission).expect_value("serialize observer permission");
    let round_trip: ObserverPermission =
        serde_json::from_value(json).expect_value("deserialize observer permission");

    assert_eq!(round_trip, permission);
}

#[test]
fn observer_permission_rejects_write_scope() {
    let error = ObserverPermission::new(
        ObserverPermissionId::parse("perm-1").expect_value("permission id"),
        ParentMemberId::parse("member-1").expect_value("member id"),
        HouseholdId::parse("household-1").expect_value("household id"),
        vec![HouseholdAuthorityAction::ChangePolicy],
        true,
    )
    .expect_err_value("write scope is blocked for observers");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "family_identity.observer_permission.granted_scopes",
            value: String::from("ChangePolicy"),
        }
    );
}

#[test]
fn active_controller_lease_round_trips_and_stays_reusable() {
    let lease = ParentControllerLease::new(
        ParentControllerLeaseId::parse("lease-1").expect_value("lease id"),
        ParentMemberId::parse("member-1").expect_value("member id"),
        DeviceId::parse("device-1").expect_value("device id"),
        "2026-06-27T01:00:00Z",
        "2026-06-27T02:00:00Z",
        vec![HouseholdAuthorityAction::StartRemoteView],
        ParentControllerLeaseState::Active,
    )
    .expect_value("active lease record");

    let json = serde_json::to_value(&lease).expect_value("serialize controller lease");
    assert_eq!(
        json.get("granted_actions"),
        Some(&serde_json::json!(["start-remote-view"]))
    );
    let round_trip: ParentControllerLease =
        serde_json::from_value(json).expect_value("deserialize controller lease");

    assert_eq!(round_trip, lease);
    lease
        .ensure_reusable()
        .expect_value("active lease remains reusable");
}

#[test]
fn revoked_controller_lease_cannot_be_reused() {
    let lease = ParentControllerLease::new(
        ParentControllerLeaseId::parse("lease-1").expect_value("lease id"),
        ParentMemberId::parse("member-1").expect_value("member id"),
        DeviceId::parse("device-1").expect_value("device id"),
        "2026-06-27T01:00:00Z",
        "2026-06-27T02:00:00Z",
        vec![HouseholdAuthorityAction::StartRemoteControl],
        ParentControllerLeaseState::Revoked,
    )
    .expect_value("revoked lease record");

    let error = lease
        .ensure_reusable()
        .expect_err_value("revoked lease reuse is rejected");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "family_identity.parent_controller_lease.revocation_state",
            value: String::from("Revoked"),
        }
    );
}

#[test]
fn setup_invite_and_audit_event_round_trip_through_serde_json() {
    let invite = SetupInvite::new(
        SetupInviteId::parse("invite-1").expect_value("invite id"),
        HouseholdId::parse("household-1").expect_value("household id"),
        "parent@example.com",
        SetupInviteTargetRole::Observer,
        "2026-06-28T00:00:00Z",
    )
    .expect_value("setup invite");
    let invite_json = serde_json::to_value(&invite).expect_value("serialize setup invite");
    let invite_round_trip: SetupInvite =
        serde_json::from_value(invite_json).expect_value("deserialize setup invite");
    assert_eq!(invite_round_trip, invite);

    let audit_event = SetupAuditEvent::new(
        SetupAuditEventId::parse("audit-1").expect_value("audit id"),
        HouseholdId::parse("household-1").expect_value("household id"),
        ParentMemberId::parse("member-1").expect_value("member id"),
        SetupAuditTargetId::parse("child-1").expect_value("target id"),
        SetupAuditActionId::parse("household-invite-issued").expect_value("action id"),
        "2026-06-27T03:00:00Z",
        Some(SetupAuditEvidenceRef::parse("evidence-1").expect_value("evidence ref")),
    )
    .expect_value("audit event");
    let audit_json = serde_json::to_value(&audit_event).expect_value("serialize setup audit event");
    let audit_round_trip: SetupAuditEvent =
        serde_json::from_value(audit_json).expect_value("deserialize setup audit event");
    assert_eq!(audit_round_trip, audit_event);
}

#[test]
fn recovery_record_stays_distinct_from_recovery_state_enum() {
    let record = RecoveryContractState::new(
        RecoveryId::parse("recovery-1").expect_value("recovery id"),
        DeviceId::parse("device-1").expect_value("device id"),
        RecoveryKind::HouseholdTransfer,
        true,
    )
    .expect_value("recovery record");

    assert_eq!(record.reason, RecoveryKind::HouseholdTransfer);
    assert!(record.parent_action_required);
    let json = serde_json::to_value(&record).expect_value("serialize recovery contract state");
    let round_trip: RecoveryContractState =
        serde_json::from_value(json).expect_value("deserialize recovery contract state");

    assert_eq!(round_trip, record);
    assert_eq!(
        RecoveryWorkflowState::Approved,
        RecoveryWorkflowState::Approved
    );
}

#[test]
fn recovery_contract_state_rejects_missing_parent_action_for_transfer() {
    let error = RecoveryContractState::new(
        RecoveryId::parse("recovery-1").expect_value("recovery id"),
        DeviceId::parse("device-1").expect_value("device id"),
        RecoveryKind::HouseholdTransfer,
        false,
    )
    .expect_err_value("household transfer must keep parent action required");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "family_identity.recovery_record.parent_action_required",
            value: String::from("false"),
        }
    );
}
