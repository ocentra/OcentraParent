use std::fs::remove_file;

use ocentra_parent_agent_protocol::{
    constants, policy_constants, LanPairingAuthenticationState, LanPairingDeviceReachability,
    LanPairingDeviceRef, LanPairingIntentKind, LanPairingParentAuthority, LanPairingProof,
    LanPairingRejectionReason, LanParentIntentEnvelope,
};

use crate::TrustedDeviceRegistry;

#[test]
fn trusted_device_registry_accepts_pairing_and_validates_intent() {
    let mut registry = TrustedDeviceRegistry::empty();
    let active_proof = proof(constants::lan_pairing::EXPIRES_AT);
    registry.accept_pairing_proof(
        &active_proof,
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );
    registry
        .select_pairing(
            constants::lan_pairing::PAIRING_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
            constants::lan_pairing::EXPIRES_AT,
        )
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    registry
        .select_pairing(
            constants::lan_pairing::PAIRING_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
            constants::lan_pairing::EXPIRES_AT,
        )
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    let result = registry.validate_intent(
        &intent(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ),
        Some(constants::lan_pairing::ALLOWED_ORIGIN),
        constants::lan_pairing::OBSERVED_AT,
    );

    assert_eq!(result, Ok(()));
}

#[test]
fn trusted_device_registry_rejects_anonymous_wrong_origin_wrong_device_and_revoked() {
    let mut registry = TrustedDeviceRegistry::empty();
    let proof = proof(constants::lan_pairing::EXPIRES_AT);
    registry.accept_pairing_proof(
        &proof,
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );

    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            None,
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::WrongOrigin)
    );
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::peer::LOCAL_DEV_AGENT,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::WrongDevice)
    );
    assert_eq!(
        registry.validate_intent(
            &intent_with_route(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::ROUTE_ID_UNSUPPORTED,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::UnsupportedRoute)
    );
    assert!(registry.revoke_pairing(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::OBSERVED_AT
    ));
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Revoked)
    );
}

#[test]
fn trusted_device_registry_rejects_stale_and_replayed_intents() {
    let mut registry = selected_registry(constants::lan_pairing::EXPIRES_AT);
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRED_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Stale)
    );
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::REPLAYED_INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Ok(())
    );
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::REPLAYED_INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Replayed)
    );
}

#[test]
fn trusted_device_registry_rejects_expired_pairings() {
    let mut expired_registry = selected_registry(constants::lan_pairing::EXPIRED_AT);
    assert_eq!(
        expired_registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Expired)
    );
}

#[test]
fn trusted_device_registry_requires_selected_device_for_multi_device_control() {
    let mut registry = TrustedDeviceRegistry::empty();
    registry.accept_pairing_proof(
        &proof(constants::lan_pairing::EXPIRES_AT),
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );
    registry.accept_pairing_proof(
        &proof_for(
            constants::lan_pairing::SECOND_PAIRING_ID,
            constants::lan_pairing::SECOND_CHALLENGE_ID,
            constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
            constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK,
            constants::lan_pairing::SECOND_PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ),
        second_child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );

    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::UnselectedDevice)
    );

    let selected = registry
        .select_pairing(
            constants::lan_pairing::PAIRING_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
            constants::lan_pairing::EXPIRES_AT,
        )
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        selected.selected_child_device_id,
        constants::lan_pairing::CHILD_DEVICE_ID
    );
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Ok(())
    );
    assert_eq!(
        registry.validate_intent(
            &intent_for_pairing(
                constants::lan_pairing::SECOND_INTENT_ID,
                constants::lan_pairing::SECOND_PAIRING_ID,
                constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
                constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK,
                constants::lan_pairing::SECOND_PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::UnselectedDevice)
    );
}

#[test]
fn trusted_device_registry_reports_revoked_stale_and_offline_selected_state() {
    let mut stale_registry = selected_registry(constants::lan_pairing::EXPIRES_AT);
    assert!(stale_registry.mark_selected_stale(constants::lan_pairing::EXPIRED_AT));
    let stale_target = stale_registry
        .selected_target_at(constants::lan_pairing::OBSERVED_AT)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let mut offline_registry = selected_registry(constants::lan_pairing::EXPIRES_AT);
    assert!(offline_registry.mark_selected_offline(constants::lan_pairing::OBSERVED_AT));
    let offline_target = offline_registry
        .selected_target_at(constants::lan_pairing::OBSERVED_AT)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let mut revoked_registry = selected_registry(constants::lan_pairing::EXPIRES_AT);

    assert_eq!(
        stale_target.reachability,
        LanPairingDeviceReachability::Stale
    );
    assert_eq!(
        stale_registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Stale)
    );
    assert_eq!(
        offline_target.reachability,
        LanPairingDeviceReachability::Offline
    );
    assert_eq!(
        offline_registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Offline)
    );
    assert!(revoked_registry.revoke_pairing(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::OBSERVED_AT
    ));
    assert_eq!(revoked_registry.selected_target(), None);
    assert_eq!(revoked_registry.trusted_device_count(), 0);
    assert_eq!(
        revoked_registry.revoked_device_ids(),
        vec![constants::lan_pairing::CHILD_DEVICE_ID.to_string()]
    );
}

#[test]
fn trusted_device_registry_can_survive_restart_or_fail_closed_when_missing() {
    let path = temp_registry_path();
    let _ = remove_file(&path);
    let mut registry = TrustedDeviceRegistry::empty();
    registry.accept_pairing_proof(
        &proof(constants::lan_pairing::EXPIRES_AT),
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );
    registry
        .save_json(&path)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    let loaded = TrustedDeviceRegistry::load_json(&path);
    let missing_path = temp_registry_path();
    let _ = remove_file(&path);
    let _ = remove_file(&missing_path);
    let missing = TrustedDeviceRegistry::load_json(&missing_path);

    assert_eq!(loaded.entries().len(), 1);
    assert_eq!(loaded.selected_target(), None);
    assert_eq!(
        loaded.authentication_state(),
        LanPairingAuthenticationState::Unpaired
    );
    assert_eq!(missing.entries().len(), 0);
}

fn proof(expires_at: &str) -> LanPairingProof {
    proof_for(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::CHALLENGE_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        constants::lan_pairing::PROOF_DIGEST,
        expires_at,
    )
}

fn selected_registry(expires_at: &str) -> TrustedDeviceRegistry {
    let mut registry = TrustedDeviceRegistry::empty();
    let active_proof = proof(expires_at);
    registry.accept_pairing_proof(
        &active_proof,
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );
    registry
        .select_pairing(
            constants::lan_pairing::PAIRING_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
            constants::lan_pairing::EXPIRES_AT,
        )
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    registry
}

fn proof_for(
    pairing_id: &str,
    challenge_id: &str,
    child_device_id: &str,
    route_id: &str,
    proof_digest: &str,
    expires_at: &str,
) -> LanPairingProof {
    LanPairingProof {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: pairing_id.to_string(),
        challenge_id: challenge_id.to_string(),
        child_device_id: child_device_id.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        route_id: route_id.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        proof_digest: proof_digest.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: expires_at.to_string(),
    }
}

fn intent(
    intent_id: &str,
    target_child_device_id: &str,
    proof_digest: &str,
    expires_at: &str,
) -> LanParentIntentEnvelope {
    intent_with_route(
        intent_id,
        target_child_device_id,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        proof_digest,
        expires_at,
    )
}

fn intent_with_route(
    intent_id: &str,
    target_child_device_id: &str,
    route_id: &str,
    proof_digest: &str,
    expires_at: &str,
) -> LanParentIntentEnvelope {
    intent_for_pairing(
        intent_id,
        constants::lan_pairing::PAIRING_ID,
        target_child_device_id,
        route_id,
        proof_digest,
        expires_at,
    )
}

fn intent_for_pairing(
    intent_id: &str,
    pairing_id: &str,
    target_child_device_id: &str,
    route_id: &str,
    proof_digest: &str,
    expires_at: &str,
) -> LanParentIntentEnvelope {
    LanParentIntentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: intent_id.to_string(),
        intent_kind: LanPairingIntentKind::RuleQuery,
        target_child_device_id: target_child_device_id.to_string(),
        route_id: route_id.to_string(),
        pairing_id: pairing_id.to_string(),
        proof_digest: proof_digest.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: expires_at.to_string(),
        controller_lease_id: constants::lan_pairing::CONTROLLER_LEASE_ID.to_string(),
        controller_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        parent_actor_id: constants::lan_pairing::PARENT_ACTOR_ID.to_string(),
        parent_authority: LanPairingParentAuthority::ActiveController,
        controller_lease_issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        controller_lease_expires_at: constants::lan_pairing::CONTROLLER_LEASE_EXPIRES_AT
            .to_string(),
        evidence_references: Vec::new(),
    }
}

fn child_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef {
        device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        child_profile_id: Some(policy_constants::TEST_CHILD_PROFILE_ID.to_string()),
        label: policy_constants::TEST_PARENT_DEVICE_LABEL.to_string(),
        platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
    }
}

fn second_child_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef {
        device_id: constants::lan_pairing::SECOND_CHILD_DEVICE_ID.to_string(),
        child_profile_id: Some(policy_constants::TEST_CHILD_PROFILE_ID.to_string()),
        label: policy_constants::TEST_PARENT_DEVICE_LABEL.to_string(),
        platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
    }
}

fn parent_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef {
        device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        child_profile_id: None,
        label: policy_constants::TEST_PARENT_DEVICE_LABEL.to_string(),
        platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
    }
}

fn temp_registry_path() -> std::path::PathBuf {
    let mut name = String::from(constants::lan_pairing::REGISTRY_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::lan_pairing::REGISTRY_FILE_EXTENSION);
    path
}
