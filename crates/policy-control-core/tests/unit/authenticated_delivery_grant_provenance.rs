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
use ocentra_policy_control_core::policy_authority::{
    PolicyActionAuthorizationState, PolicyControlDecision, PolicyEnforcementExecutionState,
    PolicyManualReviewState,
};
use ocentra_policy_control_core::policy_contract_helpers::authority::{
    PolicyContractAuthorityDecision, PolicyContractAuthoritySource, PolicyContractAuthorityState,
};
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAssertionSnapshot, AuthenticatedDeliveryGrantCapabilityAssertion,
    AuthenticatedDeliveryGrantEvidenceAssertion,
};

struct ProvenanceFixture {
    authority: HouseholdAuthorityInput,
    decision: PolicyControlDecision,
    contract_authority: PolicyContractAuthorityDecision,
    bindings: DeliveryGrantBindings,
    canonical: CanonicalDeliveryGrantAuthorization,
    step_up: ParentStepUpGrantAuthorization,
}

impl ProvenanceFixture {
    fn new() -> Self {
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
                expires_at: "2026-07-28T00:10:00Z".to_owned(),
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
                    self.authority,
                    self.decision,
                    self.contract_authority.clone(),
                ),
                "authority provenance"
            ),
            verified_parent_step_up_proof: step_up_signer
                .sign(self.step_up.validation.clone(), assertions),
        }
    }
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

#[test]
fn issuer_uses_verifier_backed_policy_decision_and_contract_authority() -> TestResult {
    let issuer = test_ok!(issuer(), "provenance-configured issuer");
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
            fixture.authority,
            blocked_decision,
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
            fixture.authority,
            fixture.decision,
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
fn issuer_rejects_oversized_signed_authority_binding_before_verification() -> TestResult {
    let issuer = test_ok!(issuer(), "provenance-configured issuer");
    let fixture = ProvenanceFixture::new();
    let mut request = fixture.request();
    request.signed_authority_bindings.bindings.target_device_id = "x".repeat(513);
    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
    );
    Ok(())
}
