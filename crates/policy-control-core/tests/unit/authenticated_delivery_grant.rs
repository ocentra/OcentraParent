use ocentra_family_identity_core::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, HouseholdAuthorityInput,
};
use ocentra_policy_control_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantIssuance, AuthenticatedDeliveryGrantIssuanceError,
    AuthenticatedDeliveryGrantIssuer, DeliveryGrantBindings,
};
use ocentra_policy_control_core::policy_authority::{
    PolicyActionAuthorizationState, PolicyControlDecision, PolicyEnforcementExecutionState,
    PolicyManualReviewState,
};
use ocentra_policy_control_core::policy_contract_helpers::authority::{
    PolicyContractAuthorityDecision, PolicyContractAuthoritySource, PolicyContractAuthorityState,
};

fn authority() -> HouseholdAuthorityInput {
    HouseholdAuthorityInput {
        actor_role: HouseholdRole::ParentOwner,
        same_family: true,
        actor_account_state: ActorAccountState::Active,
        membership_state: HouseholdMembershipState::Active,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        device_trust_state: DeviceTrustState::Trusted,
        session_freshness_state: SessionFreshnessState::Fresh,
        capability_granted: true,
        controller_lease_state: None,
        action: HouseholdAuthorityAction::ChangePolicy,
    }
}

fn decision() -> PolicyControlDecision {
    PolicyControlDecision {
        action_authorization_state: PolicyActionAuthorizationState::Authorized,
        enforcement_execution_state: PolicyEnforcementExecutionState::MayExecute,
        manual_review_state: PolicyManualReviewState::NotRequired,
    }
}

fn policy_authority() -> PolicyContractAuthorityDecision {
    PolicyContractAuthorityDecision {
        source: PolicyContractAuthoritySource::ParentPolicy,
        state: PolicyContractAuthorityState::Authorized,
    }
}

fn bindings() -> DeliveryGrantBindings {
    DeliveryGrantBindings {
        issuer_actor_id: "parent-1".to_owned(),
        household_id: "household-1".to_owned(),
        parent_device_id: "parent-device-1".to_owned(),
        child_profile_id: "child-1".to_owned(),
        target_device_id: "child-device-1".to_owned(),
        policy_decision_id: "decision-1".to_owned(),
        policy_version: "1".to_owned(),
        action_id: "action-1".to_owned(),
        capability_id: "process-control".to_owned(),
        evidence_digest: "evidence-1".to_owned(),
        payload_digest: "b".repeat(64),
        dry_run: false,
        nonce: "nonce-1".to_owned(),
        issued_at: "2026-07-28T00:00:00Z".to_owned(),
        expires_at: "2026-07-28T00:05:00Z".to_owned(),
        revocation_version: "revocation-1".to_owned(),
    }
}

#[test]
fn issuer_requires_current_parent_authority_and_produces_verifiable_grant() {
    let issuer = AuthenticatedDeliveryGrantIssuer::from_platform_key("parent-key-1", [3; 32])
        .expect("valid test key id");
    let grant = issuer
        .issue(AuthenticatedDeliveryGrantIssuance {
            household_authority: authority(),
            policy_decision: &decision(),
            policy_authority: &policy_authority(),
            capability_available: true,
            evidence_stable: true,
            bindings: bindings(),
        })
        .expect("current authority can issue");
    assert!(issuer
        .verifying_key()
        .verify_strict(
            &grant.signing_bytes(),
            &ed25519_dalek::Signature::from_slice(&grant.signature).expect("signature bytes")
        )
        .is_ok());
}

#[test]
fn issuer_rejects_untrusted_parent_device_and_dry_run() {
    let issuer = AuthenticatedDeliveryGrantIssuer::from_platform_key("parent-key-1", [3; 32])
        .expect("valid test key id");
    let mut untrusted = authority();
    untrusted.device_trust_state = DeviceTrustState::Revoked;
    assert_eq!(
        issuer.issue(AuthenticatedDeliveryGrantIssuance {
            household_authority: untrusted,
            policy_decision: &decision(),
            policy_authority: &policy_authority(),
            capability_available: true,
            evidence_stable: true,
            bindings: bindings(),
        }),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentAuthorityRejected)
    );
    let mut dry_run = bindings();
    dry_run.dry_run = true;
    assert_eq!(
        issuer.issue(AuthenticatedDeliveryGrantIssuance {
            household_authority: authority(),
            policy_decision: &decision(),
            policy_authority: &policy_authority(),
            capability_available: true,
            evidence_stable: true,
            bindings: dry_run,
        }),
        Err(AuthenticatedDeliveryGrantIssuanceError::DryRunForbidden)
    );
}
