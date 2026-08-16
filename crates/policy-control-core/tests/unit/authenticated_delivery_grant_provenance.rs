use super::authenticated_delivery_grant_fixture::{
    aggregate_id, executable_conflict_decision, issuer as durable_issuer,
    issuer_with_current_state as durable_issuer_with_current_state,
};
use super::TestResult;
use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, HouseholdAuthorityInput, ParentStepUpAssertionSnapshot,
    ParentStepUpValidationInput,
};
use ocentra_family_identity_core::household_authority_proof::{
    HouseholdAuthorityCurrentState, HouseholdAuthorityProofIdentityBinding,
    HouseholdAuthorityProofSigner,
};
use ocentra_family_identity_core::parent_step_up_proof::{
    authorization_digest, ParentStepUpAuthorizationBinding, ParentStepUpProofError,
    ParentStepUpProofSigner, ParentStepUpProofVerifier,
};
use ocentra_policy_control_core::authenticated_delivery_grant::authority::{
    AuthenticatedDeliveryGrantAuthoritySigner, AuthenticatedDeliveryGrantAuthorityVerifier,
};
use ocentra_policy_control_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantIssuance, AuthenticatedDeliveryGrantIssuanceError,
    AuthenticatedDeliveryGrantIssuer, CanonicalDeliveryGrantAuthorization, DeliveryGrantBindings,
    DeliveryGrantCapabilityState, DeliveryGrantEvidenceState, GrantActionId, GrantCapabilityId,
    GrantChildProfileId, GrantEvidenceDigest, GrantHouseholdId, GrantIssuerActorId, GrantNonce,
    GrantParentDeviceId, GrantPayloadDigest, GrantPolicyDecisionId, GrantPolicyVersion,
    GrantRevocationVersion, GrantTargetDeviceId, ParentStepUpGrantAuthorization,
};
use ocentra_policy_control_core::policy_authority::{
    PolicyActionAuthorizationState, PolicyConflictDecision, PolicyConflictResolutionState,
    PolicyControlDecision, PolicyEnforcementExecutionState, PolicyManualReviewState,
};
use ocentra_policy_control_core::policy_authority_resolved_decision::ResolvedPolicyDecision;
use ocentra_policy_control_core::policy_contract_helpers::authority::{
    PolicyContractAuthorityDecision, PolicyContractAuthoritySource, PolicyContractAuthorityState,
};
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAssertionSnapshot, AuthenticatedDeliveryGrantCapabilityAssertion,
    AuthenticatedDeliveryGrantEvidenceAssertion,
};
use std::sync::{Arc, Mutex};

pub(super) struct ProvenanceFixture {
    pub(super) authority: HouseholdAuthorityInput,
    pub(super) decision: PolicyControlDecision,
    pub(super) contract_authority: PolicyContractAuthorityDecision,
    pub(super) bindings: DeliveryGrantBindings,
    canonical: CanonicalDeliveryGrantAuthorization,
    step_up: ParentStepUpGrantAuthorization,
}

impl ProvenanceFixture {
    pub(super) fn new() -> Self {
        let bindings = DeliveryGrantBindings {
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
            payload_length: 32,
            dry_run: false,
            nonce: "nonce-1".to_owned(),
            issued_at: "2026-07-28T00:00:00Z".to_owned(),
            expires_at: "2026-07-28T00:05:00Z".to_owned(),
            revocation_version: "revocation-1".to_owned(),
        };
        let authority = HouseholdAuthorityInput {
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
        };
        let step_up_validation = ParentStepUpValidationInput {
            assertion: Some(ParentStepUpAssertionSnapshot {
                family_id: "household-1".to_owned(),
                parent_account_id: "parent-1".to_owned(),
                action_device_id: "parent-device-1".to_owned(),
                action_device_child_profile_id: None,
                target_child_profile_id: Some("child-1".to_owned()),
                action: HouseholdAuthorityAction::ChangePolicy,
                nonce: "nonce-1".to_owned(),
                expires_at: "2026-07-28T00:05:00Z".to_owned(),
            }),
            family_id: "household-1".to_owned(),
            parent_account_id: "parent-1".to_owned(),
            action_device_id: "parent-device-1".to_owned(),
            action_device_child_profile_id: None,
            target_child_profile_id: Some("child-1".to_owned()),
            action: HouseholdAuthorityAction::ChangePolicy,
            observed_at: "2026-07-28T00:00:00Z".to_owned(),
            expected_nonce: Some("nonce-1".to_owned()),
        };
        Self {
            authority,
            decision: authorized_decision(),
            contract_authority: authorized_contract_authority(),
            canonical: CanonicalDeliveryGrantAuthorization {
                issuer_actor_id: test_ok!(GrantIssuerActorId::parse("parent-1"), "issuer"),
                household_id: test_ok!(GrantHouseholdId::parse("household-1"), "household"),
                parent_device_id: test_ok!(
                    GrantParentDeviceId::parse("parent-device-1"),
                    "parent device"
                ),
                child_profile_id: test_ok!(GrantChildProfileId::parse("child-1"), "child"),
                target_device_id: test_ok!(GrantTargetDeviceId::parse("child-device-1"), "target"),
                policy_decision_id: test_ok!(
                    GrantPolicyDecisionId::parse("decision-1"),
                    "decision"
                ),
                policy_version: test_ok!(GrantPolicyVersion::parse("1"), "version"),
                action_id: test_ok!(GrantActionId::parse("action-1"), "action"),
                capability_id: test_ok!(GrantCapabilityId::parse("process-control"), "capability"),
                evidence_digest: test_ok!(GrantEvidenceDigest::parse("evidence-1"), "evidence"),
                payload_digest: test_ok!(GrantPayloadDigest::parse("b".repeat(64)), "payload"),
                nonce: test_ok!(GrantNonce::parse("nonce-1"), "nonce"),
                revocation_version: test_ok!(
                    GrantRevocationVersion::parse("revocation-1"),
                    "revocation"
                ),
            },
            bindings,
            step_up: ParentStepUpGrantAuthorization {
                validation: step_up_validation,
                target_device_id: test_ok!(GrantTargetDeviceId::parse("child-device-1"), "target"),
            },
        }
    }

    fn request(&self) -> AuthenticatedDeliveryGrantIssuance<'_> {
        let authority_signer =
            AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
        let step_up_signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
        let assertions = AuthenticatedDeliveryGrantAssertionSnapshot {
            capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
            evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
        };
        AuthenticatedDeliveryGrantIssuance {
            correlation_id: test_ok!(
                CorrelationId::parse("authenticated-delivery-grant-provenance-1"),
                "issuance correlation id"
            ),
            household_authority: self.authority,
            policy_decision: &self.decision,
            policy_authority: &self.contract_authority,
            canonical_authorization: self.canonical.clone(),
            parent_step_up: self.step_up.clone(),
            capability_state: DeliveryGrantCapabilityState::Available,
            evidence_state: DeliveryGrantEvidenceState::Stable,
            bindings: self.bindings.clone(),
            signed_authority_bindings: test_ok!(
                authority_signer.sign(
                    self.bindings.clone(),
                    assertions.clone(),
                    household_authority_proof(self.authority),
                    resolved_decision(&self.bindings, self.decision),
                    self.contract_authority.clone(),
                ),
                "authority provenance"
            ),
            verified_parent_step_up_proof: test_ok!(
                step_up_signer.sign_bound(
                    self.step_up.validation.clone(),
                    self.bindings.target_device_id.clone(),
                    assertions,
                    authorization_digest(ParentStepUpAuthorizationBinding {
                        household_id: &self.bindings.household_id,
                        parent_actor_id: &self.bindings.issuer_actor_id,
                        parent_device_id: &self.bindings.parent_device_id,
                        child_profile_id: &self.bindings.child_profile_id,
                        target_device_id: &self.bindings.target_device_id,
                        action_id: &self.bindings.action_id,
                        capability_id: &self.bindings.capability_id,
                        evidence_digest: &self.bindings.evidence_digest,
                        payload_digest: &self.bindings.payload_digest,
                    }),
                ),
                "bounded parent step-up proof"
            ),
        }
    }
}

#[test]
fn parent_step_up_proof_rejects_oversized_fields_before_signing_or_verification() -> TestResult {
    let signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
    let fixture = ProvenanceFixture::new();
    let assertions = AuthenticatedDeliveryGrantAssertionSnapshot {
        capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
        evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
    };
    let valid_proof = test_ok!(
        signer.sign(
            fixture.step_up.validation.clone(),
            fixture.bindings.target_device_id.clone(),
            assertions.clone(),
        ),
        "bounded valid parent step-up proof"
    );
    let verifier = ParentStepUpProofVerifier::new(signer.verifying_key());
    let verified = test_ok!(
        verifier.verify(&valid_proof),
        "verified valid parent step-up proof"
    );
    assert_eq!(verified.0, fixture.step_up.validation);
    assert_eq!(verified.1, fixture.bindings.target_device_id);
    assert_eq!(verified.2, assertions);

    let mut oversized_validation = fixture.step_up.validation;
    oversized_validation.expected_nonce = Some("x".repeat(513));
    assert_eq!(
        signer.sign(oversized_validation, verified.1.clone(), verified.2),
        Err(ParentStepUpProofError::Rejected)
    );

    let mut oversized_proof = valid_proof;
    test_some!(
        oversized_proof.validation.assertion.as_mut(),
        "signed step-up assertion"
    )
    .nonce = "x".repeat(513);
    assert_eq!(
        verifier.verify(&oversized_proof),
        Err(ParentStepUpProofError::Rejected)
    );
    Ok(())
}

#[test]
fn parent_step_up_proof_rejects_fractional_lifetimes_over_five_minutes_at_signing_and_verification(
) -> TestResult {
    let signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
    let fixture = ProvenanceFixture::new();
    let assertions = AuthenticatedDeliveryGrantAssertionSnapshot {
        capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
        evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
    };
    let mut overlong_validation = fixture.step_up.validation.clone();
    test_some!(
        overlong_validation.assertion.as_mut(),
        "step-up assertion must exist"
    )
    .expires_at = "2026-07-28T00:05:00.001Z".to_owned();
    assert_eq!(
        signer.sign(
            overlong_validation,
            fixture.bindings.target_device_id.clone(),
            assertions.clone(),
        ),
        Err(ParentStepUpProofError::Rejected),
        "signing must reject a parent-presence proof longer than five minutes, including a fractional overage"
    );

    let mut proof = test_ok!(
        signer.sign(
            fixture.step_up.validation.clone(),
            fixture.bindings.target_device_id,
            assertions,
        ),
        "five-minute step-up proof signs"
    );
    test_some!(
        proof.validation.assertion.as_mut(),
        "signed step-up assertion"
    )
    .expires_at = "2026-07-28T00:05:00.001Z".to_owned();
    let verifier = ParentStepUpProofVerifier::new(signer.verifying_key());
    assert_eq!(
        verifier.verify(&proof),
        Err(ParentStepUpProofError::Rejected),
        "verification must reject an overlong signed step-up proof before signature acceptance, including a fractional overage"
    );
    Ok(())
}

#[test]
fn parent_step_up_proof_rejects_fractional_expiry_and_accepts_exact_fractional_boundaries(
) -> TestResult {
    let signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
    let fixture = ProvenanceFixture::new();
    let assertions = AuthenticatedDeliveryGrantAssertionSnapshot {
        capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
        evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
    };
    let mut zero_lifetime = fixture.step_up.validation.clone();
    zero_lifetime.observed_at = "2026-07-28T00:00:00.500Z".to_owned();
    test_some!(
        zero_lifetime.assertion.as_mut(),
        "step-up assertion must exist"
    )
    .expires_at = "2026-07-28T00:00:00.500Z".to_owned();
    let verifier = ParentStepUpProofVerifier::new(signer.verifying_key());
    let zero_proof = test_ok!(
        signer.sign(
            zero_lifetime,
            fixture.bindings.target_device_id.clone(),
            assertions.clone(),
        ),
        "zero fractional lifetime signs"
    );
    let _zero_verified = test_ok!(
        verifier.verify(&zero_proof),
        "an exact zero fractional lifetime remains valid"
    );

    let mut expired_validation = fixture.step_up.validation.clone();
    expired_validation.observed_at = "2026-07-28T00:00:00.500Z".to_owned();
    test_some!(
        expired_validation.assertion.as_mut(),
        "step-up assertion must exist"
    )
    .expires_at = "2026-07-28T00:00:00.499Z".to_owned();
    assert_eq!(
        signer.sign(
            expired_validation,
            fixture.bindings.target_device_id.clone(),
            assertions.clone(),
        ),
        Err(ParentStepUpProofError::Rejected),
        "signing must reject a proof that expired one millisecond before observation"
    );

    let mut maximum_lifetime = fixture.step_up.validation.clone();
    maximum_lifetime.observed_at = "2026-07-28T00:00:00.500Z".to_owned();
    test_some!(
        maximum_lifetime.assertion.as_mut(),
        "step-up assertion must exist"
    )
    .expires_at = "2026-07-28T00:05:00.500Z".to_owned();
    let maximum_proof = test_ok!(
        signer.sign(
            maximum_lifetime,
            fixture.bindings.target_device_id,
            assertions,
        ),
        "exact five-minute fractional lifetime signs"
    );
    let _maximum_verified = test_ok!(
        verifier.verify(&maximum_proof),
        "an exact five-minute fractional lifetime remains valid"
    );

    let mut expired = zero_proof;
    test_some!(
        expired.validation.assertion.as_mut(),
        "signed step-up assertion"
    )
    .expires_at = "2026-07-28T00:00:00.499Z".to_owned();
    assert_eq!(
        verifier.verify(&expired),
        Err(ParentStepUpProofError::Rejected),
        "verification must reject a proof that expired one millisecond before observation"
    );

    let mut overlong = maximum_proof;
    test_some!(
        overlong.validation.assertion.as_mut(),
        "signed step-up assertion"
    )
    .expires_at = "2026-07-28T00:05:00.501Z".to_owned();
    assert_eq!(
        verifier.verify(&overlong),
        Err(ParentStepUpProofError::Rejected),
        "verification must reject a proof one millisecond over the five-minute boundary"
    );
    Ok(())
}

#[test]
fn issuer_rejects_signed_step_up_target_that_differs_from_canonical_target() -> TestResult {
    let issuer = test_ok!(durable_issuer(), "provenance-configured issuer");
    let fixture = ProvenanceFixture::new();
    let step_up_signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
    let mut request = fixture.request();
    request.verified_parent_step_up_proof = test_ok!(
        step_up_signer.sign_bound(
            fixture.step_up.validation.clone(),
            "other-child-device".to_owned(),
            AuthenticatedDeliveryGrantAssertionSnapshot {
                capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
                evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
            },
            authorization_digest(ParentStepUpAuthorizationBinding {
                household_id: &fixture.bindings.household_id,
                parent_actor_id: &fixture.bindings.issuer_actor_id,
                parent_device_id: &fixture.bindings.parent_device_id,
                child_profile_id: &fixture.bindings.child_profile_id,
                target_device_id: &fixture.bindings.target_device_id,
                action_id: &fixture.bindings.action_id,
                capability_id: &fixture.bindings.capability_id,
                evidence_digest: &fixture.bindings.evidence_digest,
                payload_digest: &fixture.bindings.payload_digest,
            }),
        ),
        "signed mismatched step-up target"
    );

    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
    );
    Ok(())
}

#[test]
fn issuer_rejects_step_up_proof_reused_for_a_different_authorization() -> TestResult {
    let issuer = test_ok!(durable_issuer(), "provenance-configured issuer");
    let authorization_a = ProvenanceFixture::new();
    let proof_for_a = authorization_a.request().verified_parent_step_up_proof;
    let mut authorization_b = ProvenanceFixture::new();
    authorization_b.bindings.action_id = "action-2".to_owned();
    authorization_b.canonical.action_id = test_ok!(GrantActionId::parse("action-2"), "action B");
    let mut request_for_b = authorization_b.request();
    request_for_b.verified_parent_step_up_proof = proof_for_a;
    assert_eq!(
        issuer.issue(request_for_b),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch),
        "a family-signed step-up proof for authorization A cannot authorize B"
    );
    let matching = authorization_a.request();
    let _matching_grant = test_ok!(
        issuer.issue(matching),
        "a matching authorization digest remains issuable"
    );
    Ok(())
}

fn authorized_decision() -> PolicyControlDecision {
    PolicyControlDecision {
        action_authorization_state: PolicyActionAuthorizationState::Authorized,
        enforcement_execution_state: PolicyEnforcementExecutionState::MayExecute,
        manual_review_state: PolicyManualReviewState::NotRequired,
    }
}

fn authorized_contract_authority() -> PolicyContractAuthorityDecision {
    PolicyContractAuthorityDecision {
        source: PolicyContractAuthoritySource::ParentPolicy,
        state: PolicyContractAuthorityState::Authorized,
    }
}

pub(super) fn household_authority_proof(
    authority: HouseholdAuthorityInput,
) -> ocentra_family_identity_core::household_authority_proof::HouseholdAuthorityProof {
    test_ok!(
        HouseholdAuthorityProofSigner::from_platform_key([6; 32]).sign_bound_at(
            &HouseholdAuthorityCurrentState {
                authority,
                identity_binding: HouseholdAuthorityProofIdentityBinding {
                    household_id: "household-1".to_owned(),
                    parent_actor_id: "parent-1".to_owned(),
                    parent_device_id: "parent-device-1".to_owned(),
                    child_profile_id: "child-1".to_owned(),
                    target_device_id: "child-device-1".to_owned(),
                },
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

#[test]
fn issuer_rejects_household_authority_proof_transplanted_between_grant_identities() -> TestResult {
    let mut fixture = ProvenanceFixture::new();
    fixture.bindings.household_id = "household-2".to_owned();
    let request = fixture.request();
    let authority = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let household_authority = HouseholdAuthorityProofSigner::from_platform_key([6; 32]);
    let verifier = AuthenticatedDeliveryGrantAuthorityVerifier::new(
        authority.verifying_key(),
        household_authority.verifying_key(),
    );
    assert_eq!(
        verifier.verify(&request.signed_authority_bindings),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected),
        "a family-signed household authority proof for identity A must not authorize grant identity B"
    );
    Ok(())
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

fn issuer_with_current_state<F>(
    current_state: F,
) -> Result<AuthenticatedDeliveryGrantIssuer, AuthenticatedDeliveryGrantIssuanceError>
where
    F: Fn() -> HouseholdAuthorityCurrentState + Send + Sync + 'static,
{
    let authority = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let household_authority = HouseholdAuthorityProofSigner::from_platform_key([6; 32]);
    let step_up = ParentStepUpProofSigner::from_platform_key([8; 32]);
    AuthenticatedDeliveryGrantIssuer::from_platform_key_with_provenance_verifiers(
        "parent-key-1",
        [3; 32],
        authority.verifying_key(),
        household_authority.verifying_key(),
        current_state,
        step_up.verifying_key(),
        super::authenticated_delivery_grant_fixture::current_parent_device_trust_state,
    )
    .map(|issuer| issuer.with_trusted_issuance_now_for_debug_test("2026-07-28T00:01:00Z"))
}

#[test]
fn issuer_reloads_changed_household_state_after_construction() -> TestResult {
    let fixture = ProvenanceFixture::new();
    let current_state = Arc::new(Mutex::new(HouseholdAuthorityCurrentState {
        authority: fixture.authority,
        identity_binding: HouseholdAuthorityProofIdentityBinding {
            household_id: "household-1".to_owned(),
            parent_actor_id: "parent-1".to_owned(),
            parent_device_id: "parent-device-1".to_owned(),
            child_profile_id: "child-1".to_owned(),
            target_device_id: "child-device-1".to_owned(),
        },
        family_revocation_epoch: 1,
    }));
    let resolver_state = Arc::clone(&current_state);
    let issuer = test_ok!(
        durable_issuer_with_current_state(move || {
            resolver_state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .clone()
        }),
        "live household state issuer"
    )
    .with_trusted_issuance_now_for_debug_test("2026-07-28T00:01:00Z");
    test_ok!(
        issuer.issue(fixture.request()),
        "issuer accepts a proof matching current state before revocation"
    );
    current_state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .family_revocation_epoch = 2;
    assert_eq!(
        issuer.issue(fixture.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected),
        "issuer must obtain current household state for every issuance rather than retain a constructor snapshot"
    );
    Ok(())
}

#[test]
fn issuer_rejects_authority_proof_revoked_after_mint() -> TestResult {
    let fixture = ProvenanceFixture::new();
    let issuer = test_ok!(
        durable_issuer_with_current_state(move || HouseholdAuthorityCurrentState {
            authority: fixture.authority,
            identity_binding: HouseholdAuthorityProofIdentityBinding {
                household_id: "household-1".to_owned(),
                parent_actor_id: "parent-1".to_owned(),
                parent_device_id: "parent-device-1".to_owned(),
                child_profile_id: "child-1".to_owned(),
                target_device_id: "child-device-1".to_owned(),
            },
            family_revocation_epoch: 2,
        }),
        "durable revoked authority issuer"
    );
    assert_eq!(
        issuer.issue(fixture.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
    );
    Ok(())
}

#[test]
fn issuer_rejects_expired_current_authority_proof() -> TestResult {
    let fixture = ProvenanceFixture::new();
    let issuer = test_ok!(
        durable_issuer_with_current_state(move || HouseholdAuthorityCurrentState {
            authority: fixture.authority,
            identity_binding: HouseholdAuthorityProofIdentityBinding {
                household_id: "household-1".to_owned(),
                parent_actor_id: "parent-1".to_owned(),
                parent_device_id: "parent-device-1".to_owned(),
                child_profile_id: "child-1".to_owned(),
                target_device_id: "child-device-1".to_owned(),
            },
            family_revocation_epoch: 1,
        }),
        "durable expired authority issuer"
    )
    .with_trusted_issuance_now_for_debug_test("2026-07-28T00:05:00Z");
    assert_eq!(
        issuer.issue(fixture.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
    );
    Ok(())
}

#[test]
fn issuer_uses_verifier_backed_policy_decision_and_contract_authority() -> TestResult {
    let issuer = test_ok!(durable_issuer(), "provenance-configured issuer");
    let fixture = ProvenanceFixture::new();
    let authority_signer = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let blocked_decision = PolicyControlDecision {
        action_authorization_state: PolicyActionAuthorizationState::Blocked,
        enforcement_execution_state: PolicyEnforcementExecutionState::MustNotExecute,
        manual_review_state: PolicyManualReviewState::Required,
    };
    let mut forged_caller_decision = fixture.request();
    forged_caller_decision.signed_authority_bindings = test_ok!(
        authority_signer.sign(
            fixture.bindings.clone(),
            AuthenticatedDeliveryGrantAssertionSnapshot {
                capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
                evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
            },
            household_authority_proof(fixture.authority),
            resolved_decision(&fixture.bindings, blocked_decision),
            fixture.contract_authority.clone(),
        ),
        "blocked policy decision provenance"
    );
    assert_eq!(
        issuer.issue(forged_caller_decision),
        Err(AuthenticatedDeliveryGrantIssuanceError::PolicyNotExecutable)
    );
    let evidence_only_authority = PolicyContractAuthorityDecision {
        source: PolicyContractAuthoritySource::ActivityEvidence,
        state: PolicyContractAuthorityState::EvidenceOnly,
    };
    let mut forged_caller_authority = fixture.request();
    forged_caller_authority.signed_authority_bindings = test_ok!(
        authority_signer.sign(
            fixture.bindings.clone(),
            AuthenticatedDeliveryGrantAssertionSnapshot {
                capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
                evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
            },
            household_authority_proof(fixture.authority),
            resolved_decision(&fixture.bindings, fixture.decision),
            evidence_only_authority,
        ),
        "evidence-only policy authority provenance"
    );
    assert_eq!(
        issuer.issue(forged_caller_authority),
        Err(AuthenticatedDeliveryGrantIssuanceError::PolicyNotExecutable)
    );
    Ok(())
}

#[test]
fn issuer_rejects_a_validly_signed_decision_for_a_different_policy_id() -> TestResult {
    let issuer = test_ok!(durable_issuer(), "provenance-configured issuer");
    let fixture = ProvenanceFixture::new();
    let policy_signer = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let mut request = fixture.request();
    request.signed_authority_bindings = test_ok!(
        policy_signer.sign(
            fixture.bindings.clone(),
            AuthenticatedDeliveryGrantAssertionSnapshot {
                capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
                evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
            },
            household_authority_proof(fixture.authority),
            test_ok!(
                ResolvedPolicyDecision::for_delivery_grant(
                    aggregate_id(
                        &fixture.bindings.target_device_id,
                        &fixture.bindings.action_id
                    ),
                    "different-decision",
                    fixture.decision,
                    executable_conflict_decision(),
                ),
                "separate resolved policy decision"
            ),
            fixture.contract_authority.clone(),
        ),
        "valid policy signature with mismatched decision identity"
    );
    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch),
        "the signed resolved decision identity must equal the grant binding"
    );
    Ok(())
}

#[test]
fn issuer_rejects_a_validly_signed_decision_transplanted_from_another_aggregate() -> TestResult {
    let issuer = test_ok!(durable_issuer(), "provenance-configured issuer");
    let fixture = ProvenanceFixture::new();
    let signer = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let mut request = fixture.request();
    request.signed_authority_bindings = test_ok!(
        signer.sign(
            fixture.bindings.clone(),
            AuthenticatedDeliveryGrantAssertionSnapshot {
                capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
                evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
            },
            household_authority_proof(fixture.authority),
            test_ok!(
                ResolvedPolicyDecision::for_delivery_grant(
                    "policy-control-aggregate:other-device:action-1",
                    fixture.bindings.policy_decision_id.clone(),
                    fixture.decision,
                    executable_conflict_decision(),
                ),
                "separate aggregate resolved policy decision"
            ),
            fixture.contract_authority.clone(),
        ),
        "valid policy signature with transplanted aggregate"
    );
    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch),
        "a signed decision for another target/action aggregate must not issue this grant"
    );

    let mut fixture = ProvenanceFixture::new();
    fixture.bindings.target_device_id = "device".to_owned();
    fixture.bindings.action_id = "a:b".to_owned();
    fixture.canonical.target_device_id = test_ok!(GrantTargetDeviceId::parse("device"), "target");
    fixture.canonical.action_id = test_ok!(GrantActionId::parse("a:b"), "action");
    fixture.step_up.target_device_id = test_ok!(GrantTargetDeviceId::parse("device"), "target");
    let alternate_identity = HouseholdAuthorityProofIdentityBinding {
        household_id: "household-1".to_owned(),
        parent_actor_id: "parent-1".to_owned(),
        parent_device_id: "parent-device-1".to_owned(),
        child_profile_id: "child-1".to_owned(),
        target_device_id: "device".to_owned(),
    };

    let signer = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let mut request = fixture.request();
    request.signed_authority_bindings = test_ok!(
        signer.sign(
            fixture.bindings.clone(),
            AuthenticatedDeliveryGrantAssertionSnapshot {
                capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
                evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
            },
            test_ok!(
                HouseholdAuthorityProofSigner::from_platform_key([6; 32]).sign_bound_at(
                    &HouseholdAuthorityCurrentState {
                        authority: fixture.authority,
                        identity_binding: alternate_identity.clone(),
                        family_revocation_epoch: 1,
                    },
                    alternate_identity.clone(),
                    "2026-07-28T00:00:00Z",
                    "2026-07-28T00:05:00Z",
                ),
                "family identity authority proof for colon-collision target"
            ),
            test_ok!(
                ResolvedPolicyDecision::for_delivery_grant(
                    aggregate_id("device:a", "b"),
                    fixture.bindings.policy_decision_id.clone(),
                    fixture.decision,
                    executable_conflict_decision(),
                ),
                "colon-colliding resolved policy decision"
            ),
            fixture.contract_authority.clone(),
        ),
        "signed colon-colliding authority provenance"
    );

    let alternate_issuer = test_ok!(
        durable_issuer_with_current_state(move || HouseholdAuthorityCurrentState {
            authority: ProvenanceFixture::new().authority,
            identity_binding: alternate_identity.clone(),
            family_revocation_epoch: 1,
        }),
        "alternate current household state issuer"
    );
    assert_eq!(
        alternate_issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch),
        "a decision for device:a plus b must not authorize device plus a:b"
    );
    let matching = ProvenanceFixture::new();
    assert!(
        issuer.issue(matching.request()).is_ok(),
        "the typed aggregate encoding remains compatible with a matching authorization"
    );
    Ok(())
}

#[test]
fn issuer_rejects_signed_conflict_provenance_that_forbids_execution() -> TestResult {
    let issuer = test_ok!(durable_issuer(), "provenance-configured issuer");
    let fixture = ProvenanceFixture::new();
    let signer = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let mut request = fixture.request();
    request.signed_authority_bindings = test_ok!(
        signer.sign(
            fixture.bindings.clone(),
            AuthenticatedDeliveryGrantAssertionSnapshot {
                capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
                evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
            },
            household_authority_proof(fixture.authority),
            test_ok!(
                ResolvedPolicyDecision::for_delivery_grant(
                    aggregate_id(
                        &fixture.bindings.target_device_id,
                        &fixture.bindings.action_id
                    ),
                    fixture.bindings.policy_decision_id.clone(),
                    fixture.decision,
                    PolicyConflictDecision {
                        resolution_state: PolicyConflictResolutionState::ObserveOnly,
                        manual_review_state: PolicyManualReviewState::NotRequired,
                    },
                ),
                "non-executable conflict provenance"
            ),
            fixture.contract_authority.clone(),
        ),
        "signed conflict provenance"
    );
    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::PolicyNotExecutable)
    );
    Ok(())
}

#[test]
fn issuer_rejects_signed_conflict_provenance_requiring_manual_review() -> TestResult {
    let issuer = test_ok!(durable_issuer(), "provenance-configured issuer");
    let fixture = ProvenanceFixture::new();
    let signer = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let mut request = fixture.request();
    request.signed_authority_bindings = test_ok!(
        signer.sign(
            fixture.bindings.clone(),
            AuthenticatedDeliveryGrantAssertionSnapshot {
                capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
                evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
            },
            household_authority_proof(fixture.authority),
            test_ok!(
                ResolvedPolicyDecision::for_delivery_grant(
                    aggregate_id(
                        &fixture.bindings.target_device_id,
                        &fixture.bindings.action_id
                    ),
                    fixture.bindings.policy_decision_id.clone(),
                    fixture.decision,
                    PolicyConflictDecision {
                        resolution_state: PolicyConflictResolutionState::UseParentPolicy,
                        manual_review_state: PolicyManualReviewState::Required,
                    },
                ),
                "manual-review conflict provenance"
            ),
            fixture.contract_authority.clone(),
        ),
        "signed manual-review conflict provenance"
    );
    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::PolicyNotExecutable)
    );
    Ok(())
}

#[test]
fn issuer_rejects_oversized_signed_authority_binding_before_verification() -> TestResult {
    let issuer = test_ok!(durable_issuer(), "provenance-configured issuer");
    let fixture = ProvenanceFixture::new();
    let mut request = fixture.request();
    request.signed_authority_bindings.bindings.target_device_id = "x".repeat(513);
    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
    );
    Ok(())
}

#[test]
fn issuer_fails_closed_without_a_durable_milestone_publisher() -> TestResult {
    let fixture = ProvenanceFixture::new();
    let issuer = test_ok!(
        issuer_with_current_state(move || HouseholdAuthorityCurrentState {
            authority: fixture.authority,
            identity_binding: HouseholdAuthorityProofIdentityBinding {
                household_id: "household-1".to_owned(),
                parent_actor_id: "parent-1".to_owned(),
                parent_device_id: "parent-device-1".to_owned(),
                child_profile_id: "child-1".to_owned(),
                target_device_id: "child-device-1".to_owned(),
            },
            family_revocation_epoch: 1,
        }),
        "provenance-configured issuer"
    );
    let fixture = ProvenanceFixture::new();
    assert_eq!(
        issuer.issue(fixture.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed)
    );
    Ok(())
}

#[test]
fn issuer_ignores_an_oversized_untrusted_correlation_before_publication() -> TestResult {
    let issuer = test_ok!(durable_issuer(), "provenance-configured issuer");
    let fixture = ProvenanceFixture::new();
    let mut request = fixture.request();
    request.correlation_id = test_ok!(
        CorrelationId::parse("c".repeat(513)),
        "oversized eventing correlation remains syntactically valid"
    );
    assert!(
        issuer.issue(request).is_ok(),
        "verified authority, rather than caller context, owns issuance correlation"
    );
    Ok(())
}
