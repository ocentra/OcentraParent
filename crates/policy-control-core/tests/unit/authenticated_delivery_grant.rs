use super::TestResult;
use ocentra_family_identity_core::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, HouseholdAuthorityInput, ParentStepUpAssertionSnapshot,
    ParentStepUpValidationInput,
};
use ocentra_policy_control_core::authenticated_delivery_grant::authority::AuthenticatedDeliveryGrantAuthoritySigner;
use ocentra_policy_control_core::authenticated_delivery_grant::step_up::ParentStepUpProofSigner;
use ocentra_policy_control_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantIssuance, AuthenticatedDeliveryGrantIssuanceError,
    AuthenticatedDeliveryGrantIssuer, CanonicalDeliveryGrantAuthorization, DeliveryGrantBindings,
    DeliveryGrantCapabilityState, DeliveryGrantEvidenceState, GrantActionId, GrantCapabilityId,
    GrantChildProfileId, GrantEvidenceDigest, GrantHouseholdId, GrantIssuerActorId, GrantNonce,
    GrantParentDeviceId, GrantPayloadDigest, GrantPolicyDecisionId, GrantPolicyVersion,
    GrantRevocationVersion, GrantTargetDeviceId, ParentStepUpGrantAuthorization,
};

fn issuer() -> Result<AuthenticatedDeliveryGrantIssuer, AuthenticatedDeliveryGrantIssuanceError> {
    let authority = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let step_up = ParentStepUpProofSigner::from_platform_key([8; 32]);
    AuthenticatedDeliveryGrantIssuer::from_platform_key_with_provenance_verifiers(
        "parent-key-1",
        [3; 32],
        authority.verifying_key(),
        step_up.verifying_key(),
    )
}

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

fn canonical_authorization() -> CanonicalDeliveryGrantAuthorization {
    CanonicalDeliveryGrantAuthorization {
        issuer_actor_id: test_ok!(GrantIssuerActorId::parse("parent-1"), "issuer actor id"),
        household_id: test_ok!(GrantHouseholdId::parse("household-1"), "household id"),
        parent_device_id: test_ok!(
            GrantParentDeviceId::parse("parent-device-1"),
            "parent device"
        ),
        child_profile_id: test_ok!(GrantChildProfileId::parse("child-1"), "child profile"),
        target_device_id: test_ok!(
            GrantTargetDeviceId::parse("child-device-1"),
            "target device"
        ),
        policy_decision_id: test_ok!(GrantPolicyDecisionId::parse("decision-1"), "decision id"),
        policy_version: test_ok!(GrantPolicyVersion::parse("1"), "policy version"),
        action_id: test_ok!(GrantActionId::parse("action-1"), "action id"),
        capability_id: test_ok!(GrantCapabilityId::parse("process-control"), "capability id"),
        evidence_digest: test_ok!(GrantEvidenceDigest::parse("evidence-1"), "evidence digest"),
        payload_digest: test_ok!(GrantPayloadDigest::parse("b".repeat(64)), "payload digest"),
        nonce: test_ok!(GrantNonce::parse("nonce-1"), "nonce"),
        revocation_version: test_ok!(
            GrantRevocationVersion::parse("revocation-1"),
            "revocation version"
        ),
    }
}

fn parent_step_up() -> ParentStepUpGrantAuthorization {
    ParentStepUpGrantAuthorization {
        validation: ParentStepUpValidationInput {
            assertion: Some(ParentStepUpAssertionSnapshot {
                family_id: "household-1".to_owned(),
                parent_account_id: "parent-1".to_owned(),
                action_device_id: "parent-device-1".to_owned(),
                action_device_child_profile_id: Some("child-1".to_owned()),
                target_child_profile_id: Some("child-1".to_owned()),
                action: HouseholdAuthorityAction::ChangePolicy,
                nonce: "nonce-1".to_owned(),
                expires_at: "2026-07-28T00:10:00Z".to_owned(),
            }),
            family_id: "household-1".to_owned(),
            parent_account_id: "parent-1".to_owned(),
            action_device_id: "parent-device-1".to_owned(),
            action_device_child_profile_id: Some("child-1".to_owned()),
            target_child_profile_id: Some("child-1".to_owned()),
            action: HouseholdAuthorityAction::ChangePolicy,
            observed_at: "2026-07-28T00:00:00Z".to_owned(),
            expected_nonce: Some("nonce-1".to_owned()),
        },
        target_device_id: test_ok!(
            GrantTargetDeviceId::parse("child-device-1"),
            "target device"
        ),
    }
}

struct IssuanceFixture {
    household_authority: HouseholdAuthorityInput,
    policy_decision: PolicyControlDecision,
    policy_authority: PolicyContractAuthorityDecision,
    canonical_authorization: CanonicalDeliveryGrantAuthorization,
    parent_step_up: ParentStepUpGrantAuthorization,
    capability_state: DeliveryGrantCapabilityState,
    evidence_state: DeliveryGrantEvidenceState,
    bindings: DeliveryGrantBindings,
}

impl IssuanceFixture {
    fn new() -> Self {
        Self {
            household_authority: authority(),
            policy_decision: decision(),
            policy_authority: policy_authority(),
            canonical_authorization: canonical_authorization(),
            parent_step_up: parent_step_up(),
            capability_state: DeliveryGrantCapabilityState::Available,
            evidence_state: DeliveryGrantEvidenceState::Stable,
            bindings: bindings(),
        }
    }

    fn request(&self) -> AuthenticatedDeliveryGrantIssuance<'_> {
        let authority_signer =
            AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
        let step_up_signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
        AuthenticatedDeliveryGrantIssuance {
            household_authority: self.household_authority,
            policy_decision: &self.policy_decision,
            policy_authority: &self.policy_authority,
            canonical_authorization: self.canonical_authorization.clone(),
            parent_step_up: self.parent_step_up.clone(),
            capability_state: self.capability_state,
            evidence_state: self.evidence_state,
            bindings: self.bindings.clone(),
            signed_authority_bindings: authority_signer.sign(self.bindings.clone()),
            verified_parent_step_up_proof: step_up_signer
                .sign(self.parent_step_up.validation.clone()),
        }
    }
}

#[test]
fn issuer_requires_current_parent_authority_and_produces_verifiable_grant() -> TestResult {
    let issuer = test_ok!(issuer(), "valid test key id");
    let grant = test_ok!(
        issuer.issue(IssuanceFixture::new().request()),
        "current authority can issue"
    );
    let signature = test_ok!(
        ed25519_dalek::Signature::from_slice(&grant.signature),
        "signature bytes"
    );
    assert!(issuer
        .verifying_key()
        .verify_strict(&grant.signing_bytes(), &signature)
        .is_ok());
    Ok(())
}

#[test]
fn issuer_rejects_untrusted_parent_device_and_dry_run() -> TestResult {
    let issuer = test_ok!(issuer(), "valid test key id");
    let mut untrusted = IssuanceFixture::new();
    untrusted.household_authority.device_trust_state = DeviceTrustState::Revoked;
    assert_eq!(
        issuer.issue(untrusted.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentAuthorityRejected)
    );
    let mut dry_run = IssuanceFixture::new();
    dry_run.bindings.dry_run = true;
    assert_eq!(
        issuer.issue(dry_run.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::DryRunForbidden)
    );
    Ok(())
}

#[test]
fn issuer_rejects_forged_or_substituted_signed_provenance() -> TestResult {
    let issuer = test_ok!(issuer(), "provenance-configured issuer");
    let fixture = IssuanceFixture::new();

    let mut forged_authority = fixture.request();
    forged_authority.signed_authority_bindings.signature[0] ^= 1;
    assert_eq!(
        issuer.issue(forged_authority),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
    );

    let mut substituted_binding = fixture.request();
    substituted_binding
        .signed_authority_bindings
        .bindings
        .target_device_id = "other-device".to_owned();
    assert_eq!(
        issuer.issue(substituted_binding),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
    );

    let mut forged_step_up = fixture.request();
    forged_step_up.verified_parent_step_up_proof.signature[0] ^= 1;
    assert_eq!(
        issuer.issue(forged_step_up),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
    );
    Ok(())
}

#[test]
fn issuer_rejects_valid_signatures_from_unconfigured_provenance_keys() -> TestResult {
    let issuer = test_ok!(issuer(), "provenance-configured issuer");
    let fixture = IssuanceFixture::new();
    let authority_signed_by_another_key =
        AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([9; 32]);
    let step_up_signed_by_another_key = ParentStepUpProofSigner::from_platform_key([10; 32]);
    let mut request = fixture.request();
    request.signed_authority_bindings =
        authority_signed_by_another_key.sign(fixture.bindings.clone());
    request.verified_parent_step_up_proof =
        step_up_signed_by_another_key.sign(fixture.parent_step_up.validation.clone());

    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
    );
    Ok(())
}

#[test]
fn issuer_rejects_step_up_or_canonical_authorization_that_does_not_match_signed_bindings(
) -> TestResult {
    let issuer = test_ok!(issuer(), "valid test key id");
    let mut mismatched = IssuanceFixture::new();
    mismatched.canonical_authorization.target_device_id = test_ok!(
        GrantTargetDeviceId::parse("different-device"),
        "mismatched target device"
    );
    assert_eq!(
        issuer.issue(mismatched.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch)
    );

    let mut wrong_step_up = IssuanceFixture::new();
    wrong_step_up.parent_step_up.validation.expected_nonce = Some("wrong-nonce".to_owned());
    assert_eq!(
        issuer.issue(wrong_step_up.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
    );
    Ok(())
}

#[test]
fn issuer_rejects_manual_review_and_invalid_or_chronologically_expired_timestamps() -> TestResult {
    let issuer = test_ok!(issuer(), "valid test key id");
    let mut manual_review = IssuanceFixture::new();
    manual_review.policy_decision = PolicyControlDecision {
        manual_review_state: PolicyManualReviewState::Required,
        ..decision()
    };
    assert_eq!(
        issuer.issue(manual_review.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::ManualReviewRequired)
    );

    let mut malformed_timestamp = IssuanceFixture::new();
    malformed_timestamp.bindings.issued_at = "not-a-timestamp".to_owned();
    assert_eq!(
        issuer.issue(malformed_timestamp.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp)
    );

    let mut offset_expired = IssuanceFixture::new();
    offset_expired.parent_step_up.validation.observed_at = "2026-07-28T03:00:00Z".to_owned();
    offset_expired.bindings.issued_at = "2026-07-28T03:00:00Z".to_owned();
    test_some!(
        offset_expired.parent_step_up.validation.assertion.as_mut(),
        "step-up assertion"
    )
    .expires_at = "2026-07-28T04:00:00+05:00".to_owned();
    assert_eq!(
        issuer.issue(offset_expired.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
    );
    Ok(())
}

#[test]
fn issuer_rejects_a_grant_that_outlives_the_signed_parent_step_up_proof() -> TestResult {
    let issuer = test_ok!(issuer(), "provenance-configured issuer");
    let mut fixture = IssuanceFixture::new();
    fixture.bindings.expires_at = "2026-07-28T00:10:01Z".to_owned();

    assert_eq!(
        issuer.issue(fixture.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
    );
    Ok(())
}
