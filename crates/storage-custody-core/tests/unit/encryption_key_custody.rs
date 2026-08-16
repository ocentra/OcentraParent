use crate::support::StorageCustodyTestValueExt;

use ocentra_schema::encryption_key_custody as contracts;
use ocentra_storage_custody_core::encryption_key_custody::{
    build_encryption_key_custody_proof, derive_decrypt_attempt_result,
    derive_platform_key_custody_row, DecryptAttemptInput, EncryptionKeyCustodyDerivationError,
    PlatformKeyCustodyInput,
};

#[test]
fn encryption_key_custody_derives_platform_matrix_with_linux_and_mobile_manual_required() {
    let linux = derive_platform_key_custody_row(PlatformKeyCustodyInput {
        surface: contracts::PlatformKeyCustodySurface::Linux,
        key_store: contracts::PlatformKeyStoreKind::LinuxSecretStoreUndecided,
        decrypt_authority: contracts::PlatformDecryptAuthority::ManualRequired,
        manual_required: true,
        device_proof_required: false,
        notes: "Linux undecided".to_string(),
    })
    .assume_ok();
    assert!(linux.manual_required);

    let ios = derive_platform_key_custody_row(PlatformKeyCustodyInput {
        surface: contracts::PlatformKeyCustodySurface::IOS,
        key_store: contracts::PlatformKeyStoreKind::IOSKeychain,
        decrypt_authority: contracts::PlatformDecryptAuthority::ManualRequired,
        manual_required: true,
        device_proof_required: true,
        notes: "iOS limited".to_string(),
    })
    .assume_ok();
    assert!(ios.manual_required && ios.device_proof_required);
}

#[test]
fn encryption_key_custody_wrong_household_and_wrong_device_fail_closed() {
    let child_service = child_service_row();

    let wrong_household = derive_decrypt_attempt_result(
        &child_service,
        attempt_input(DecryptAttemptFixture {
            attempt_id: contracts::EncryptionAttemptId::parse("attempt-wrong-household")
                .assume_ok(),
            household_id: contracts::EncryptionHouseholdId::parse("family-key-custody-proof-1")
                .assume_ok(),
            device_id: Some(
                contracts::EncryptionDeviceId::parse("device-key-custody-proof-1").assume_ok(),
            ),
            surface: contracts::PlatformKeyCustodySurface::ChildService,
            requested_scope: contracts::EncryptionUnlockScope::ChildEvidenceLocal,
            key_state: contracts::KeyCustodyState::KeyAvailable,
            household_match: false,
            device_match: true,
            device_proof_present: true,
        }),
    );
    assert_eq!(
        wrong_household.state,
        contracts::DecryptDecisionState::WrongHouseholdDenied
    );
    assert!(wrong_household.fail_closed);

    let wrong_device = derive_decrypt_attempt_result(
        &child_service,
        attempt_input(DecryptAttemptFixture {
            attempt_id: contracts::EncryptionAttemptId::parse("attempt-wrong-device").assume_ok(),
            household_id: contracts::EncryptionHouseholdId::parse("family-key-custody-proof-1")
                .assume_ok(),
            device_id: Some(
                contracts::EncryptionDeviceId::parse("device-key-custody-proof-1").assume_ok(),
            ),
            surface: contracts::PlatformKeyCustodySurface::ChildService,
            requested_scope: contracts::EncryptionUnlockScope::ChildEvidenceLocal,
            key_state: contracts::KeyCustodyState::KeyAvailable,
            household_match: true,
            device_match: false,
            device_proof_present: true,
        }),
    );
    assert_eq!(
        wrong_device.state,
        contracts::DecryptDecisionState::WrongDeviceDenied
    );
    assert!(wrong_device.fail_closed);
}

#[test]
fn encryption_key_custody_revoked_and_lost_key_states_are_explicit() {
    let parent_desktop = parent_desktop_row();

    let revoked = derive_decrypt_attempt_result(
        &parent_desktop,
        attempt_input(DecryptAttemptFixture {
            attempt_id: contracts::EncryptionAttemptId::parse("attempt-revoked").assume_ok(),
            household_id: contracts::EncryptionHouseholdId::parse("family-key-custody-proof-1")
                .assume_ok(),
            device_id: Some(
                contracts::EncryptionDeviceId::parse("device-key-custody-proof-1").assume_ok(),
            ),
            surface: contracts::PlatformKeyCustodySurface::ParentDesktop,
            requested_scope: contracts::EncryptionUnlockScope::ParentOwnedBundle,
            key_state: contracts::KeyCustodyState::KeyRevoked,
            household_match: true,
            device_match: true,
            device_proof_present: true,
        }),
    );
    assert_eq!(
        revoked.state,
        contracts::DecryptDecisionState::RevokedKeyDenied
    );

    let lost = derive_decrypt_attempt_result(
        &parent_desktop,
        attempt_input(DecryptAttemptFixture {
            attempt_id: contracts::EncryptionAttemptId::parse("attempt-lost").assume_ok(),
            household_id: contracts::EncryptionHouseholdId::parse("family-key-custody-proof-1")
                .assume_ok(),
            device_id: Some(
                contracts::EncryptionDeviceId::parse("device-key-custody-proof-1").assume_ok(),
            ),
            surface: contracts::PlatformKeyCustodySurface::ParentDesktop,
            requested_scope: contracts::EncryptionUnlockScope::ParentOwnedBundle,
            key_state: contracts::KeyCustodyState::KeyUnavailable,
            household_match: true,
            device_match: true,
            device_proof_present: true,
        }),
    );
    assert_eq!(
        lost.state,
        contracts::DecryptDecisionState::LostKeyManualRequired
    );
    assert!(lost.manual_required);
}

#[test]
fn encryption_key_custody_rejects_universal_key_and_hosted_portal_decrypt_root() {
    let universal = build_encryption_key_custody_proof(
        vec![contracts::EncryptionKeyHierarchyRow {
            key_class: contracts::EncryptionKeyClass::ProviderAuthToken,
            default_holder: contracts::EncryptionKeyHolder::ProviderConnection,
            unlock_scope: contracts::EncryptionUnlockScope::ProviderApiOnly,
            may_decrypt_child_evidence: true,
            may_decrypt_parent_exports: true,
            default_by_product: false,
            notes: "forbidden".to_string(),
        }],
        vec![PlatformKeyCustodyInput {
            surface: contracts::PlatformKeyCustodySurface::WebPortal,
            key_store: contracts::PlatformKeyStoreKind::NoDecryptRoot,
            decrypt_authority: contracts::PlatformDecryptAuthority::NotDecryptRoot,
            manual_required: true,
            device_proof_required: false,
            notes: "portal".to_string(),
        }],
        vec![],
        contracts::EncryptionTimestamp::parse("2026-06-28T18:55:00.000Z").assume_ok(),
    );
    assert_eq!(
        universal,
        Err(
            EncryptionKeyCustodyDerivationError::UniversalDecryptForbidden(
                contracts::EncryptionKeyHolder::ProviderConnection
            )
        )
    );

    let hosted_root = derive_platform_key_custody_row(PlatformKeyCustodyInput {
        surface: contracts::PlatformKeyCustodySurface::WebPortal,
        key_store: contracts::PlatformKeyStoreKind::NoDecryptRoot,
        decrypt_authority: contracts::PlatformDecryptAuthority::ParentOwnedBundlesOnly,
        manual_required: true,
        device_proof_required: false,
        notes: "forbidden".to_string(),
    });
    assert_eq!(
        hosted_root,
        Err(EncryptionKeyCustodyDerivationError::HostedPortalCannotDecrypt)
    );
}

#[test]
fn encryption_key_custody_recovery_and_mobile_proof_gaps_stay_manual_required() {
    let ios = derive_platform_key_custody_row(PlatformKeyCustodyInput {
        surface: contracts::PlatformKeyCustodySurface::IOS,
        key_store: contracts::PlatformKeyStoreKind::IOSKeychain,
        decrypt_authority: contracts::PlatformDecryptAuthority::ManualRequired,
        manual_required: true,
        device_proof_required: true,
        notes: "ios".to_string(),
    })
    .assume_ok();

    let limited = derive_decrypt_attempt_result(
        &ios,
        attempt_input(DecryptAttemptFixture {
            attempt_id: contracts::EncryptionAttemptId::parse("attempt-ios-limited").assume_ok(),
            household_id: contracts::EncryptionHouseholdId::parse("family-key-custody-proof-1")
                .assume_ok(),
            device_id: Some(
                contracts::EncryptionDeviceId::parse("device-key-custody-proof-1").assume_ok(),
            ),
            surface: contracts::PlatformKeyCustodySurface::IOS,
            requested_scope: contracts::EncryptionUnlockScope::ParentOwnedBundle,
            key_state: contracts::KeyCustodyState::KeyAvailable,
            household_match: true,
            device_match: true,
            device_proof_present: false,
        }),
    );
    assert_eq!(
        limited.state,
        contracts::DecryptDecisionState::LimitedUntilDeviceProof
    );
    assert!(limited.manual_required);

    let recovery = derive_decrypt_attempt_result(
        &parent_desktop_row(),
        DecryptAttemptInput {
            recovery_mode: contracts::RecoveryMode::ParentOwnedRecovery,
            key_state: contracts::KeyCustodyState::RecoveryAvailable,
            ..attempt_input(DecryptAttemptFixture {
                attempt_id: contracts::EncryptionAttemptId::parse("attempt-recovery").assume_ok(),
                household_id: contracts::EncryptionHouseholdId::parse("family-key-custody-proof-1")
                    .assume_ok(),
                device_id: Some(
                    contracts::EncryptionDeviceId::parse("device-key-custody-proof-1").assume_ok(),
                ),
                surface: contracts::PlatformKeyCustodySurface::ParentDesktop,
                requested_scope: contracts::EncryptionUnlockScope::ParentOwnedBundle,
                key_state: contracts::KeyCustodyState::KeyAvailable,
                household_match: true,
                device_match: true,
                device_proof_present: true,
            })
        },
    );
    assert_eq!(
        recovery.state,
        contracts::DecryptDecisionState::RecoveryAvailableManualRequired
    );
    assert!(recovery.manual_required && recovery.used_recovery_path);
}

fn parent_desktop_row() -> contracts::PlatformKeyCustodyRow {
    derive_platform_key_custody_row(PlatformKeyCustodyInput {
        surface: contracts::PlatformKeyCustodySurface::ParentDesktop,
        key_store: contracts::PlatformKeyStoreKind::ParentDesktopLocalKeyPath,
        decrypt_authority: contracts::PlatformDecryptAuthority::ParentOwnedBundlesOnly,
        manual_required: false,
        device_proof_required: false,
        notes: "parent desktop".to_string(),
    })
    .assume_ok()
}

fn child_service_row() -> contracts::PlatformKeyCustodyRow {
    derive_platform_key_custody_row(PlatformKeyCustodyInput {
        surface: contracts::PlatformKeyCustodySurface::ChildService,
        key_store: contracts::PlatformKeyStoreKind::ChildServiceLocalKeyPath,
        decrypt_authority: contracts::PlatformDecryptAuthority::ChildLocalEvidenceOnly,
        manual_required: false,
        device_proof_required: false,
        notes: "child service".to_string(),
    })
    .assume_ok()
}

struct DecryptAttemptFixture {
    attempt_id: contracts::EncryptionAttemptId,
    household_id: contracts::EncryptionHouseholdId,
    device_id: Option<contracts::EncryptionDeviceId>,
    surface: contracts::PlatformKeyCustodySurface,
    requested_scope: contracts::EncryptionUnlockScope,
    key_state: contracts::KeyCustodyState,
    household_match: bool,
    device_match: bool,
    device_proof_present: bool,
}

fn attempt_input(fixture: DecryptAttemptFixture) -> DecryptAttemptInput {
    DecryptAttemptInput {
        attempt_id: fixture.attempt_id,
        household_id: fixture.household_id,
        device_id: fixture.device_id,
        surface: fixture.surface,
        requested_scope: fixture.requested_scope,
        key_state: fixture.key_state,
        recovery_mode: contracts::RecoveryMode::NotSupported,
        household_match: fixture.household_match,
        device_match: fixture.device_match,
        device_proof_present: fixture.device_proof_present,
    }
}
