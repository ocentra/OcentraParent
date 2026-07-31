use super::authenticated_delivery_grant::IssuanceFixture;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::ids::EventType;
use ocentra_eventing::journal::policy::{JournalPolicy, JournalSelector};
use ocentra_eventing::journal::production_file::ProductionFileEventJournal;
use ocentra_family_identity_core::household_authority::HouseholdAuthorityInput;
use ocentra_family_identity_core::household_authority_proof::{
    HouseholdAuthorityCurrentState, HouseholdAuthorityProofIdentityBinding,
    HouseholdAuthorityProofSigner,
};
use ocentra_family_identity_core::parent_step_up_proof::ParentStepUpProofSigner;
use ocentra_policy_control_core::authenticated_delivery_grant::authority::AuthenticatedDeliveryGrantAuthoritySigner;
use ocentra_policy_control_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantIssuanceError, AuthenticatedDeliveryGrantIssuer,
    DeliveryGrantBindings,
};
use ocentra_policy_control_core::policy_authority::{
    PolicyConflictDecision, PolicyConflictResolutionState, PolicyControlDecision,
    PolicyManualReviewState,
};
use ocentra_policy_control_core::policy_authority_resolved_decision::ResolvedPolicyDecision;
use std::sync::atomic::{AtomicU64, Ordering};

static JOURNAL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn unique_journal_path(label: &str) -> std::path::PathBuf {
    let sequence = JOURNAL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "ocentra-policy-issuance-{label}-{}-{sequence}.journal",
        std::process::id()
    ))
}

pub(super) fn issuer_without_milestone_publisher(
) -> Result<AuthenticatedDeliveryGrantIssuer, AuthenticatedDeliveryGrantIssuanceError> {
    let authority = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let household_authority = HouseholdAuthorityProofSigner::from_platform_key([6; 32]);
    let step_up = ParentStepUpProofSigner::from_platform_key([8; 32]);
    AuthenticatedDeliveryGrantIssuer::from_platform_key_with_provenance_verifiers(
        "parent-key-1",
        [3; 32],
        authority.verifying_key(),
        household_authority.verifying_key(),
        current_household_authority_state(),
        step_up.verifying_key(),
    )
    .map(|issuer| issuer.with_trusted_issuance_now_for_debug_test("2026-07-28T00:01:00Z"))
}

pub(super) fn issuer(
) -> Result<AuthenticatedDeliveryGrantIssuer, AuthenticatedDeliveryGrantIssuanceError> {
    let event_type = EventType::parse("authenticated-delivery-grant.issuance.milestone")
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed)?;
    let event_bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
        ProductionFileEventJournal::new(unique_journal_path("issuer")).shared(),
    );
    issuer_without_milestone_publisher()
        .and_then(|issuer| {
            issuer
                .with_event_bus_issuance_publisher(event_bus)
                .map_err(|_error| {
                    AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed
                })
        })
        .map(|issuer| issuer.with_trusted_issuance_now_for_debug_test("2026-07-28T00:01:00Z"))
}

pub(super) fn household_authority_proof(
    authority: HouseholdAuthorityInput,
) -> ocentra_family_identity_core::household_authority_proof::HouseholdAuthorityProof {
    test_ok!(
        HouseholdAuthorityProofSigner::from_platform_key([6; 32]).sign_bound_at(
            &HouseholdAuthorityCurrentState {
                authority,
                family_revocation_epoch: 1,
            },
            HouseholdAuthorityProofIdentityBinding {
                household_id: "household-1".to_owned(),
                parent_actor_id: "parent-1".to_owned(),
                parent_device_id: "parent-device-1".to_owned(),
                child_profile_id: "child-1".to_owned(),
                target_device_id: "child-device-1".to_owned(),
            },
            "2026-07-28T00:00:00Z",
            "2026-07-28T00:05:00Z",
        ),
        "family identity authority proof"
    )
}

pub(super) fn current_household_authority_state() -> HouseholdAuthorityCurrentState {
    HouseholdAuthorityCurrentState {
        authority: HouseholdAuthorityInput {
            actor_role: ocentra_family_identity_core::family_identity::HouseholdRole::ParentOwner,
            same_family: true,
            actor_account_state: ocentra_family_identity_core::family_identity::ActorAccountState::Active,
            membership_state: ocentra_family_identity_core::family_identity::HouseholdMembershipState::Active,
            child_profile_binding_state: ocentra_family_identity_core::family_identity::ChildProfileBindingState::Bound,
            device_ownership_scope: ocentra_family_identity_core::family_identity::DeviceOwnershipScope::ChildProfileDevice,
            device_trust_state: ocentra_family_identity_core::family_identity::DeviceTrustState::Trusted,
            session_freshness_state: ocentra_family_identity_core::family_identity::SessionFreshnessState::Fresh,
            capability_granted: true,
            controller_lease_state: None,
            action: ocentra_family_identity_core::household_authority::HouseholdAuthorityAction::ChangePolicy,
        },
        family_revocation_epoch: 1,
    }
}

pub(super) fn resolved_decision(
    bindings: &DeliveryGrantBindings,
    decision: PolicyControlDecision,
) -> ResolvedPolicyDecision {
    test_ok!(
        ResolvedPolicyDecision::for_delivery_grant(
            aggregate_id(&bindings.target_device_id, &bindings.action_id),
            bindings.policy_decision_id.clone(),
            decision,
            executable_conflict_decision(),
        ),
        "resolved policy decision identity"
    )
}

pub(super) fn aggregate_id(target_device_id: &str, action_id: &str) -> String {
    format!(
        "policy-control-aggregate:target:{}:{}:action:{}:{}",
        target_device_id.len(),
        target_device_id,
        action_id.len(),
        action_id
    )
}

pub(super) fn executable_conflict_decision() -> PolicyConflictDecision {
    PolicyConflictDecision {
        resolution_state: PolicyConflictResolutionState::UseParentPolicy,
        manual_review_state: PolicyManualReviewState::NotRequired,
    }
}

pub(super) fn durable_milestone_bus(
    journal_path: &std::path::Path,
) -> Result<EventBus, ocentra_eventing::error::EventingError> {
    let event_type = EventType::parse("authenticated-delivery-grant.issuance.milestone")?;
    Ok(EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
        ProductionFileEventJournal::new(journal_path).shared(),
    ))
}

pub(super) fn issuance_fixture_with_expiry(
    mut fixture: IssuanceFixture,
    expires_at: &str,
) -> Result<IssuanceFixture, String> {
    fixture.bindings.expires_at = expires_at.to_owned();
    let assertion = fixture
        .parent_step_up
        .validation
        .assertion
        .as_mut()
        .ok_or_else(|| "fixture is missing its parent step-up assertion".to_owned())?;
    assertion.expires_at = expires_at.to_owned();
    Ok(fixture)
}
