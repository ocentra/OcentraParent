use ocentra_family_identity_core::family_identity::DeviceTrustState;
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot, ParentStepUpValidationInput,
};
use ocentra_family_identity_core::parent_step_up_proof::{
    authorization_digest, ParentDeviceTrustCurrentState, ParentStepUpAuthorizationBinding,
    ParentStepUpProofError, ParentStepUpProofSigner, ParentStepUpProofVerifier,
};
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAssertionSnapshot, AuthenticatedDeliveryGrantCapabilityAssertion,
    AuthenticatedDeliveryGrantEvidenceAssertion,
};

fn validation() -> ParentStepUpValidationInput {
    ParentStepUpValidationInput {
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
    }
}

fn assertions() -> AuthenticatedDeliveryGrantAssertionSnapshot {
    AuthenticatedDeliveryGrantAssertionSnapshot {
        capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
        evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
    }
}

fn authorization_binding<'a>(
    household_id: &'a str,
    parent_actor_id: &'a str,
) -> ParentStepUpAuthorizationBinding<'a> {
    ParentStepUpAuthorizationBinding {
        household_id,
        parent_actor_id,
        parent_device_id: "parent-device-1",
        child_profile_id: "child-1",
        target_device_id: "child-device-1",
        action_id: "action-1",
        capability_id: "process-control",
        evidence_digest: "evidence-1",
        payload_digest: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    }
}

#[test]
fn authorization_digest_distinguishes_fields_that_collide_under_separator_joining() {
    let left = authorization_digest(authorization_binding("household\u{1f}parent", "actor"));
    let right = authorization_digest(authorization_binding("household", "parent\u{1f}actor"));

    assert_ne!(left, right);
    assert_eq!(left.len(), "sha256:".len() + 64);
    assert!(left.starts_with("sha256:"));
}

#[test]
fn bound_step_up_proof_requires_canonical_bounded_sha256_digest_before_signing_and_verification(
) -> Result<(), ParentStepUpProofError> {
    let signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
    let canonical_digest = authorization_digest(authorization_binding("household-1", "parent-1"));
    let expected_validation = validation();
    let proof = signer.sign_bound(
        expected_validation.clone(),
        "child-device-1".to_owned(),
        assertions(),
        canonical_digest,
    )?;
    let verifier = ParentStepUpProofVerifier::new(signer.verifying_key());
    let verified = verifier.verify(&proof)?;
    assert_eq!(verified.0, expected_validation);
    assert_eq!(verified.1, "child-device-1");
    assert_eq!(verified.2, assertions());

    let uppercase_digest = format!("sha256:{}", "A".repeat(64));
    assert_eq!(
        signer.sign_bound(
            validation(),
            "child-device-1".to_owned(),
            assertions(),
            uppercase_digest,
        ),
        Err(ParentStepUpProofError::Rejected)
    );
    assert_eq!(
        signer.sign_bound(
            validation(),
            "child-device-1".to_owned(),
            assertions(),
            format!("sha256:{}", "a".repeat(65)),
        ),
        Err(ParentStepUpProofError::Rejected)
    );

    let mut malformed_proof = proof;
    malformed_proof.authorization_digest = format!("sha256:{}", "A".repeat(64));
    assert_eq!(
        verifier.verify(&malformed_proof),
        Err(ParentStepUpProofError::Rejected)
    );
    Ok(())
}

#[test]
fn current_device_trust_epoch_is_signed_and_revocation_or_epoch_tampering_is_rejected(
) -> Result<(), ParentStepUpProofError> {
    let signer = ParentStepUpProofSigner::from_platform_key([8; 32]);
    let verifier = ParentStepUpProofVerifier::new(signer.verifying_key());
    let current_state = ParentDeviceTrustCurrentState {
        parent_device_id: "parent-device-1".to_owned(),
        trust_state: DeviceTrustState::Trusted,
        revocation_epoch: 7,
    };
    let proof = signer.sign_bound_for_current_device_trust_state(
        validation(),
        "child-device-1".to_owned(),
        assertions(),
        authorization_digest(authorization_binding("household-1", "parent-1")),
        &current_state,
    )?;

    assert_eq!(proof.parent_device_trust_revocation_epoch, 7);
    assert_eq!(
        verifier.verify_against_current_device_trust_state(&proof, &current_state)?,
        (validation(), "child-device-1".to_owned(), assertions())
    );

    let mut epoch_tampered = proof.clone();
    epoch_tampered.parent_device_trust_revocation_epoch = 8;
    let advanced_trusted_state = ParentDeviceTrustCurrentState {
        revocation_epoch: 8,
        ..current_state.clone()
    };
    assert_eq!(
        verifier
            .verify_against_current_device_trust_state(&epoch_tampered, &advanced_trusted_state),
        Err(ParentStepUpProofError::Rejected)
    );
    assert_eq!(
        verifier.verify_against_current_device_trust_state(&proof, &advanced_trusted_state),
        Err(ParentStepUpProofError::Rejected)
    );
    let revoked_state = ParentDeviceTrustCurrentState {
        trust_state: DeviceTrustState::Revoked,
        ..current_state
    };
    assert_eq!(
        verifier.verify_against_current_device_trust_state(&proof, &revoked_state),
        Err(ParentStepUpProofError::Rejected)
    );
    Ok(())
}
