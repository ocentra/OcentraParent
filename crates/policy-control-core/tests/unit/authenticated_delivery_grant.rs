use super::authenticated_delivery_grant_fixture::{
    assert_durable_milestone_count, durable_milestone_bus, issuer,
    subscribe_issuance_milestone_persistence,
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
use ocentra_policy_control_core::authenticated_delivery_grant::authority::AuthenticatedDeliveryGrantAuthoritySigner;
use ocentra_policy_control_core::authenticated_delivery_grant::issuance_milestone::{
    AuthenticatedDeliveryGrantIssuanceMilestone, AuthenticatedDeliveryGrantIssuanceOutcome,
};
use ocentra_policy_control_core::authenticated_delivery_grant::step_up::ParentStepUpProofSigner;
use ocentra_policy_control_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantIssuance, AuthenticatedDeliveryGrantIssuanceError,
    CanonicalDeliveryGrantAuthorization, DeliveryGrantBindings, DeliveryGrantCapabilityState,
    DeliveryGrantEvidenceState, GrantActionId, GrantCapabilityId, GrantChildProfileId,
    GrantEvidenceDigest, GrantHouseholdId, GrantIssuerActorId, GrantNonce, GrantParentDeviceId,
    GrantPayloadDigest, GrantPolicyDecisionId, GrantPolicyVersion, GrantRevocationVersion,
    GrantTargetDeviceId, ParentStepUpGrantAuthorization,
};
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAssertionSnapshot, AuthenticatedDeliveryGrantCapabilityAssertion,
    AuthenticatedDeliveryGrantEvidenceAssertion,
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
        payload_length: 32,
        dry_run: false,
        nonce: "nonce-1".to_owned(),
        issued_at: "2026-07-28T00:00:00Z".to_owned(),
        expires_at: "2026-07-28T00:05:00Z".to_owned(),
        revocation_version: "revocation-1".to_owned(),
    }
}

fn assertions() -> AuthenticatedDeliveryGrantAssertionSnapshot {
    AuthenticatedDeliveryGrantAssertionSnapshot {
        capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
        evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
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
        },
        target_device_id: test_ok!(
            GrantTargetDeviceId::parse("child-device-1"),
            "target device"
        ),
    }
}

pub(super) struct IssuanceFixture {
    household_authority: HouseholdAuthorityInput,
    policy_decision: PolicyControlDecision,
    policy_authority: PolicyContractAuthorityDecision,
    canonical_authorization: CanonicalDeliveryGrantAuthorization,
    pub(super) parent_step_up: ParentStepUpGrantAuthorization,
    capability_state: DeliveryGrantCapabilityState,
    evidence_state: DeliveryGrantEvidenceState,
    pub(super) bindings: DeliveryGrantBindings,
}

impl IssuanceFixture {
    pub(super) fn new() -> Self {
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

    pub(super) fn request(&self) -> AuthenticatedDeliveryGrantIssuance<'_> {
        let authority_signer =
            AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
        let step_up_signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
        AuthenticatedDeliveryGrantIssuance {
            correlation_id: test_ok!(
                CorrelationId::parse("authenticated-delivery-grant-test-chain-1"),
                "issuance correlation id"
            ),
            household_authority: self.household_authority,
            policy_decision: &self.policy_decision,
            policy_authority: &self.policy_authority,
            canonical_authorization: self.canonical_authorization.clone(),
            parent_step_up: self.parent_step_up.clone(),
            capability_state: self.capability_state,
            evidence_state: self.evidence_state,
            bindings: self.bindings.clone(),
            signed_authority_bindings: test_ok!(
                authority_signer.sign(
                    self.bindings.clone(),
                    assertions(),
                    self.household_authority,
                    self.policy_decision,
                    self.policy_authority.clone(),
                ),
                "signed authority provenance"
            ),
            verified_parent_step_up_proof: test_ok!(
                step_up_signer.sign(
                    self.parent_step_up.validation.clone(),
                    self.bindings.target_device_id.clone(),
                    assertions(),
                ),
                "bounded parent step-up proof"
            ),
        }
    }
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
fn issuer_uses_verified_household_authority_instead_of_caller_claim() -> TestResult {
    let issuer = test_ok!(issuer(), "provenance-configured issuer");
    let fixture = IssuanceFixture::new();
    let authority_signer = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let mut request = fixture.request();

    let mut signed_untrusted_authority = fixture.household_authority;
    signed_untrusted_authority.device_trust_state = DeviceTrustState::Revoked;
    request.signed_authority_bindings = test_ok!(
        authority_signer.sign(
            fixture.bindings.clone(),
            assertions(),
            signed_untrusted_authority,
            fixture.policy_decision,
            fixture.policy_authority.clone(),
        ),
        "signed untrusted authority provenance"
    );
    request.household_authority = authority();

    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentAuthorityRejected)
    );
    Ok(())
}

#[test]
fn issuer_rejects_action_device_bound_to_a_different_child() -> TestResult {
    let issuer = test_ok!(issuer(), "provenance-configured issuer");
    let fixture = IssuanceFixture::new();
    let step_up_signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
    let mut request = fixture.request();
    request
        .parent_step_up
        .validation
        .action_device_child_profile_id = Some("other-child".to_owned());
    test_some!(
        request.parent_step_up.validation.assertion.as_mut(),
        "signed step-up assertion"
    )
    .action_device_child_profile_id = Some("other-child".to_owned());
    request.verified_parent_step_up_proof = test_ok!(
        step_up_signer.sign(
            request.parent_step_up.validation.clone(),
            request.bindings.target_device_id.clone(),
            assertions(),
        ),
        "bounded parent step-up proof"
    );

    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
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
    request.signed_authority_bindings = test_ok!(
        authority_signed_by_another_key.sign(
            fixture.bindings.clone(),
            assertions(),
            fixture.household_authority,
            fixture.policy_decision,
            fixture.policy_authority.clone(),
        ),
        "unconfigured authority provenance"
    );
    request.verified_parent_step_up_proof = test_ok!(
        step_up_signed_by_another_key.sign(
            fixture.parent_step_up.validation.clone(),
            fixture.bindings.target_device_id.clone(),
            assertions(),
        ),
        "bounded parent step-up proof"
    );

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
fn issuer_rejects_manual_review() -> TestResult {
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
    Ok(())
}

#[test]
fn issuer_rejects_malformed_grant_timestamp() -> TestResult {
    let issuer = test_ok!(issuer(), "valid test key id");
    let mut malformed_timestamp = IssuanceFixture::new();
    malformed_timestamp.bindings.issued_at = "not-a-timestamp".to_owned();
    assert_eq!(
        issuer.issue(malformed_timestamp.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp)
    );
    Ok(())
}

#[test]
fn issuer_rejects_expired_step_up_timestamp_with_offset() -> TestResult {
    let issuer = test_ok!(issuer(), "valid test key id");
    let offset_expired = IssuanceFixture::new();
    let mut request = offset_expired.request();
    test_some!(
        request
            .verified_parent_step_up_proof
            .validation
            .assertion
            .as_mut(),
        "step-up assertion"
    )
    .expires_at = "2026-07-28T04:00:00+05:00".to_owned();
    request.verified_parent_step_up_proof.validation.observed_at =
        "2026-07-28T03:00:00Z".to_owned();
    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
    );
    Ok(())
}

#[test]
fn issuer_rejects_storage_unrepresentable_expiry_that_outlives_bounded_step_up_proof() -> TestResult
{
    let issuer = test_ok!(issuer(), "valid test key id");
    let mut storage_unrepresentable_expiry = IssuanceFixture::new();
    storage_unrepresentable_expiry.bindings.expires_at = "2500-01-01T00:00:00Z".to_owned();
    assert_eq!(
        issuer.issue(storage_unrepresentable_expiry.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
    );
    Ok(())
}

#[test]
fn issuer_rejects_future_grant_issuance_timestamp() -> TestResult {
    let issuer = test_ok!(issuer(), "valid test key id");
    let mut future_issued_at = IssuanceFixture::new();
    future_issued_at.bindings.issued_at = "2026-07-28T00:01:01Z".to_owned();
    future_issued_at.parent_step_up.validation.observed_at = "2026-07-28T00:01:01Z".to_owned();
    test_some!(
        future_issued_at
            .parent_step_up
            .validation
            .assertion
            .as_mut(),
        "step-up assertion"
    )
    .expires_at = "2026-07-28T00:06:01Z".to_owned();
    assert_eq!(
        issuer.issue(future_issued_at.request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp)
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

#[test]
fn issuer_uses_dually_signed_assertions_instead_of_caller_claims() -> TestResult {
    let issuer = test_ok!(issuer(), "provenance-configured issuer");
    let fixture = IssuanceFixture::new();
    let authority_signer = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let step_up_signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
    let unavailable = AuthenticatedDeliveryGrantAssertionSnapshot {
        capability: AuthenticatedDeliveryGrantCapabilityAssertion::Unavailable,
        evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
    };
    let mut request = fixture.request();
    request.signed_authority_bindings = test_ok!(
        authority_signer.sign(
            fixture.bindings.clone(),
            unavailable.clone(),
            fixture.household_authority,
            fixture.policy_decision,
            fixture.policy_authority.clone(),
        ),
        "unavailable capability provenance"
    );
    request.verified_parent_step_up_proof = test_ok!(
        step_up_signer.sign(
            fixture.parent_step_up.validation.clone(),
            fixture.bindings.target_device_id.clone(),
            unavailable,
        ),
        "bounded parent step-up proof"
    );
    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::CapabilityUnavailable)
    );
    Ok(())
}

#[test]
fn issuer_rejects_mismatched_dually_signed_assertions() -> TestResult {
    let issuer = test_ok!(issuer(), "provenance-configured issuer");
    let fixture = IssuanceFixture::new();
    let step_up_signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
    let mut request = fixture.request();
    request.verified_parent_step_up_proof = test_ok!(
        step_up_signer.sign(
            fixture.parent_step_up.validation.clone(),
            fixture.bindings.target_device_id.clone(),
            AuthenticatedDeliveryGrantAssertionSnapshot {
                capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
                evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Unstable,
            },
        ),
        "bounded parent step-up proof"
    );
    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch)
    );
    Ok(())
}

#[test]
fn issuer_requires_durable_receipt_and_awaits_safely_inside_an_entered_tokio_runtime() -> TestResult
{
    let runtime = test_ok!(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        "event runtime"
    );
    let journal_path = std::env::temp_dir().join(format!(
        "ocentra-policy-control-issuance-runtime-{}.ndjson",
        ocentra_eventing::ids::EventId::generated().as_str()
    ));
    let event_bus = test_ok!(
        durable_milestone_bus(&journal_path),
        "durable issuance milestone event bus"
    );
    runtime.block_on(async {
        test_ok!(
            subscribe_issuance_milestone_persistence(&event_bus).await,
            "durable issuance subscriber registers"
        );
    });
    let issuer = test_ok!(issuer(), "provenance-configured issuer")
        .with_event_bus_issuance_publisher(event_bus.clone())
        .map_err(|error| format!("event publisher: {error:?}"))?;

    let grant = test_ok!(
        issuer.issue(IssuanceFixture::new().request()),
        "issuance outside Tokio runtime retains its milestone through the enabled runtime"
    );
    assert_eq!(grant.issuer_key_id, "parent-key-1");
    assert_eq!(grant.target_device_id, "child-device-1");
    let journal = runtime.block_on(event_bus.journal());
    assert_eq!(
        journal.len(),
        2,
        "accepted issuance must retain prepare and terminal milestones"
    );
    let milestone = journal[0].decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
    assert_eq!(
        milestone.payload.outcome,
        AuthenticatedDeliveryGrantIssuanceOutcome::Prepared
    );
    assert_eq!(milestone.payload.rejection, None);
    assert!(milestone.payload.redaction_state);
    let terminal = journal[1].decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
    assert_eq!(
        terminal.payload.outcome,
        AuthenticatedDeliveryGrantIssuanceOutcome::Accepted
    );

    runtime.block_on(async {
        let grant = test_ok!(
            issuer.issue_async(IssuanceFixture::new().request()).await,
            "entered runtime issuance must await its durable audit milestone without blocking the executor"
        );
        assert_eq!(grant.issuer_key_id, "parent-key-1");
        let journal = event_bus.journal().await;
        assert_eq!(
            journal.len(),
            4,
            "issuance must not return before its prepare and terminal milestones are retained"
        );
        let milestone = journal[3].decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
        assert_eq!(
            milestone.payload.outcome,
            AuthenticatedDeliveryGrantIssuanceOutcome::Accepted
        );
        assert_eq!(milestone.payload.rejection, None);
        assert!(milestone.payload.redaction_state);
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;
    drop(runtime);

    let inspection_runtime = test_ok!(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        "post-shutdown journal inspection runtime"
    );
    let journal = inspection_runtime.block_on(event_bus.journal());
    assert_eq!(
        journal.len(),
        4,
        "runtime shutdown must not erase retained milestones"
    );
    assert_durable_milestone_count(
        &journal_path,
        4,
        "both sync and async issuance calls must have durable prepare and terminal receipts",
    )
}
