use ocentra_schema::encryption_key_custody as contracts;

use super::DecryptAttemptOutcome;

pub(super) fn scope_is_authorized(
    authority: contracts::PlatformDecryptAuthority,
    scope: contracts::EncryptionUnlockScope,
) -> bool {
    match authority {
        contracts::PlatformDecryptAuthority::ChildLocalEvidenceOnly => {
            scope == contracts::EncryptionUnlockScope::ChildEvidenceLocal
        }
        contracts::PlatformDecryptAuthority::ParentOwnedBundlesOnly => {
            scope == contracts::EncryptionUnlockScope::ParentOwnedBundle
        }
        contracts::PlatformDecryptAuthority::ParentCacheReportsAndBundles => matches!(
            scope,
            contracts::EncryptionUnlockScope::ParentOwnedBundle
                | contracts::EncryptionUnlockScope::ParentCacheReports
        ),
        contracts::PlatformDecryptAuthority::HouseholdRecoveryBundlesOnly => {
            scope == contracts::EncryptionUnlockScope::HouseholdRecoveryBundle
        }
        contracts::PlatformDecryptAuthority::NotDecryptRoot
        | contracts::PlatformDecryptAuthority::ManualRequired => false,
    }
}

pub(super) fn unauthorized_scope_outcome() -> DecryptAttemptOutcome {
    DecryptAttemptOutcome {
        state: contracts::DecryptDecisionState::UnauthorizedScopeDenied,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: false,
        used_recovery_path: false,
        notes: "Requested decrypt scope is outside this surface's explicit authority.".to_string(),
    }
}
