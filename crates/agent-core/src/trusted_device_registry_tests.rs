use std::fs::remove_file;

use ocentra_parent_agent_protocol::{
    constants, policy_constants, LanPairingDeviceRef, LanPairingIntentKind, LanPairingProof,
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
fn trusted_device_registry_rejects_stale_expired_and_replayed_intents() {
    let mut registry = TrustedDeviceRegistry::empty();
    let active_proof = proof(constants::lan_pairing::EXPIRES_AT);
    registry.accept_pairing_proof(
        &active_proof,
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

    let mut expired_registry = TrustedDeviceRegistry::empty();
    let expired_proof = proof(constants::lan_pairing::EXPIRED_AT);
    expired_registry.accept_pairing_proof(
        &expired_proof,
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );
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
    assert_eq!(missing.entries().len(), 0);
}

fn proof(expires_at: &str) -> LanPairingProof {
    LanPairingProof {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: constants::lan_pairing::PAIRING_ID.to_string(),
        challenge_id: constants::lan_pairing::CHALLENGE_ID.to_string(),
        child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        proof_digest: constants::lan_pairing::PROOF_DIGEST.to_string(),
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
    LanParentIntentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: intent_id.to_string(),
        intent_kind: LanPairingIntentKind::RuleQuery,
        target_child_device_id: target_child_device_id.to_string(),
        route_id: route_id.to_string(),
        pairing_id: constants::lan_pairing::PAIRING_ID.to_string(),
        proof_digest: proof_digest.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: expires_at.to_string(),
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
