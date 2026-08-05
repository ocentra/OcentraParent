use ocentra_eventing::expect_value::ExpectValue;
use ocentra_family_identity_core::family_identity::{
    household_authority_handoff::{
        evaluate_household_authority_handoff, HouseholdAuthorityHandoffRedactionState,
        HouseholdAuthorityHandoffRequest, HOUSEHOLD_AUTHORITY_HANDOFF_SCHEMA_VERSION,
    },
    ActorAccountState, ChildCustodyLabel, ChildProfile, ChildProfileId, DeviceId,
    DeviceRegistration, DeviceRouteStateLabel, DeviceTrustState, HouseholdAuthorityEvaluationId,
    HouseholdId, HouseholdMembershipState, HouseholdRole, ParentControllerLease,
    ParentControllerLeaseId, ParentMember, ParentMemberId, SessionFreshnessState,
};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, HouseholdAuthorizationFailureReason, HouseholdAuthorizationState,
    ParentControllerLeaseState,
};

fn handoff_request(action: HouseholdAuthorityAction) -> HouseholdAuthorityHandoffRequest {
    HouseholdAuthorityHandoffRequest {
        evaluation_id: HouseholdAuthorityEvaluationId::parse("authority-evaluation-1")
            .expect_value("evaluation id"),
        parent_member: ParentMember::new(
            ParentMemberId::parse("parent-1").expect_value("parent member id"),
            HouseholdId::parse("household-1").expect_value("household id"),
            HouseholdRole::ParentOwner,
            HouseholdMembershipState::Active,
            "2026-08-05T00:00:00Z",
        )
        .expect_value("parent member"),
        child_profile: ChildProfile::new(
            ChildProfileId::parse("child-1").expect_value("child id"),
            HouseholdId::parse("household-1").expect_value("household id"),
            "child display name is not emitted by the handoff",
            vec![DeviceId::parse("device-1").expect_value("device id")],
            ChildCustodyLabel::parse("family-custody").expect_value("custody label"),
        )
        .expect_value("child profile"),
        device_registration: DeviceRegistration::new(
            DeviceId::parse("device-1").expect_value("device id"),
            ChildProfileId::parse("child-1").expect_value("child id"),
            HouseholdId::parse("household-1").expect_value("household id"),
            DeviceTrustState::Trusted,
            HouseholdRole::ChildDeviceAgent,
            DeviceRouteStateLabel::parse("lan-selected").expect_value("route state"),
            None,
        )
        .expect_value("device registration"),
        actor_account_state: ActorAccountState::Active,
        session_freshness_state: SessionFreshnessState::Fresh,
        capability_granted: true,
        controller_lease: None,
        action,
    }
}

#[test]
fn record_derived_handoff_is_versioned_correlated_and_identifier_only() {
    let decision = evaluate_household_authority_handoff(handoff_request(
        HouseholdAuthorityAction::ChangePolicy,
    ));

    assert_eq!(
        decision.schema_version,
        HOUSEHOLD_AUTHORITY_HANDOFF_SCHEMA_VERSION
    );
    assert_eq!(
        decision.evaluation_id,
        HouseholdAuthorityEvaluationId::parse("authority-evaluation-1").expect_value("id")
    );
    assert_eq!(
        decision.decision.authorization_state,
        HouseholdAuthorizationState::Authorized
    );
    assert_eq!(
        decision.redaction_state,
        HouseholdAuthorityHandoffRedactionState::IdentifiersOnly
    );

    let encoded = serde_json::to_value(&decision).expect_value("serialize handoff");
    assert_eq!(
        encoded,
        serde_json::json!({
            "schema_version": HOUSEHOLD_AUTHORITY_HANDOFF_SCHEMA_VERSION,
            "evaluation_id": "authority-evaluation-1",
            "household_id": "household-1",
            "parent_member_id": "parent-1",
            "child_profile_id": "child-1",
            "device_id": "device-1",
            "action": "change-policy",
            "decision": {
                "authorization_state": "authorized",
                "audit_requirement_state": "required",
                "elevated_confirmation_state": "not-required",
                "failure_reason": null,
            },
            "redaction_state": "identifiers-only",
        })
    );
}

#[test]
fn cross_household_records_are_rejected_before_authorization() {
    let mut request = handoff_request(HouseholdAuthorityAction::ChangePolicy);
    request.child_profile.household_id = HouseholdId::parse("household-2").expect_value("id");

    let decision = evaluate_household_authority_handoff(request);

    assert_eq!(
        decision.decision.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        decision.decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::ExternalHousehold)
    );
}

#[test]
fn unbound_or_untrusted_child_device_records_are_rejected() {
    let mut unbound = handoff_request(HouseholdAuthorityAction::ViewChildStatus);
    unbound.device_registration.child_id = ChildProfileId::parse("child-2").expect_value("id");
    let unbound_decision = evaluate_household_authority_handoff(unbound);
    assert_eq!(
        unbound_decision.decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::ChildProfileNotBound)
    );

    let mut untrusted = handoff_request(HouseholdAuthorityAction::ViewChildStatus);
    untrusted.device_registration.trust_state = DeviceTrustState::Revoked;
    let untrusted_decision = evaluate_household_authority_handoff(untrusted);
    assert_eq!(
        untrusted_decision.decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::DeviceNotTrusted)
    );
}

#[test]
fn stale_session_and_mismatched_controller_lease_are_rejected() {
    let mut stale = handoff_request(HouseholdAuthorityAction::StartRemoteControl);
    stale.session_freshness_state = SessionFreshnessState::Stale;
    let stale_decision = evaluate_household_authority_handoff(stale);
    assert_eq!(
        stale_decision.decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::SessionNotFresh)
    );

    let mut mismatched_lease = handoff_request(HouseholdAuthorityAction::StartRemoteControl);
    mismatched_lease.controller_lease = Some(
        ParentControllerLease::new(
            ParentControllerLeaseId::parse("lease-1").expect_value("lease id"),
            ParentMemberId::parse("parent-2").expect_value("parent member id"),
            DeviceId::parse("device-1").expect_value("device id"),
            "2026-08-05T00:00:00Z",
            "2026-08-05T01:00:00Z",
            ParentControllerLeaseState::Active,
        )
        .expect_value("lease"),
    );
    let lease_decision = evaluate_household_authority_handoff(mismatched_lease);
    assert_eq!(
        lease_decision.decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::ControllerLeaseRequired)
    );
}
