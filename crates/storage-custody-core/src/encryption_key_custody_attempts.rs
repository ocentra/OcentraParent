use ocentra_schema::encryption_key_custody as contracts;

use super::DecryptAttemptInput;

#[path = "encryption_key_custody_scope.rs"]
mod encryption_key_custody_scope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DecryptAttemptOutcome {
    pub state: contracts::DecryptDecisionState,
    pub decrypt_allowed: bool,
    pub fail_closed: bool,
    pub manual_required: bool,
    pub used_recovery_path: bool,
    pub notes: String,
}

pub(super) fn decrypt_attempt_outcome(
    platform_row: &contracts::PlatformKeyCustodyRow,
    input: &DecryptAttemptInput,
) -> DecryptAttemptOutcome {
    if platform_row.surface != input.surface {
        return encryption_key_custody_scope::surface_mismatch_outcome();
    }
    if !input.household_match || input.key_state == contracts::KeyCustodyState::WrongHousehold {
        return wrong_household_outcome();
    }
    if !input.device_match || input.key_state == contracts::KeyCustodyState::WrongDevice {
        return wrong_device_outcome();
    }
    if input.key_state == contracts::KeyCustodyState::KeyRevoked {
        return revoked_key_outcome();
    }
    if platform_row.surface == contracts::PlatformKeyCustodySurface::WebPortal {
        return hosted_portal_outcome();
    }
    if platform_row.surface == contracts::PlatformKeyCustodySurface::Linux {
        return linux_manual_required_outcome();
    }
    if platform_row.device_proof_required && !input.device_proof_present {
        return limited_until_device_proof_outcome();
    }
    if input.key_state == contracts::KeyCustodyState::KeyUnavailable
        || input.key_state == contracts::KeyCustodyState::ReinstallRequired
        || input.key_state == contracts::KeyCustodyState::RecoveryNotSupported
    {
        return lost_key_manual_required_outcome();
    }
    if input.key_state == contracts::KeyCustodyState::RecoveryAvailable
        || input.recovery_mode == contracts::RecoveryMode::ParentOwnedRecovery
    {
        return recovery_available_manual_required_outcome();
    }
    if !encryption_key_custody_scope::scope_is_authorized(
        platform_row.decrypt_authority,
        input.requested_scope,
    ) {
        return encryption_key_custody_scope::unauthorized_scope_outcome();
    }

    allowed_outcome()
}

fn wrong_household_outcome() -> DecryptAttemptOutcome {
    DecryptAttemptOutcome {
        state: contracts::DecryptDecisionState::WrongHouseholdDenied,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: false,
        used_recovery_path: false,
        notes: "Wrong-household decrypt requests fail closed.".to_string(),
    }
}

fn wrong_device_outcome() -> DecryptAttemptOutcome {
    DecryptAttemptOutcome {
        state: contracts::DecryptDecisionState::WrongDeviceDenied,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: false,
        used_recovery_path: false,
        notes: "Wrong-device decrypt requests fail closed.".to_string(),
    }
}

fn revoked_key_outcome() -> DecryptAttemptOutcome {
    DecryptAttemptOutcome {
        state: contracts::DecryptDecisionState::RevokedKeyDenied,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: false,
        used_recovery_path: false,
        notes: "Revoked keys fail closed.".to_string(),
    }
}

fn hosted_portal_outcome() -> DecryptAttemptOutcome {
    DecryptAttemptOutcome {
        state: contracts::DecryptDecisionState::NotDecryptRootDenied,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: false,
        used_recovery_path: false,
        notes: "Hosted portal is never the decrypt root.".to_string(),
    }
}

fn linux_manual_required_outcome() -> DecryptAttemptOutcome {
    DecryptAttemptOutcome {
        state: contracts::DecryptDecisionState::PlatformManualRequired,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: true,
        used_recovery_path: false,
        notes: "Linux remains manual-required until a real secret-store decision exists."
            .to_string(),
    }
}

fn limited_until_device_proof_outcome() -> DecryptAttemptOutcome {
    DecryptAttemptOutcome {
        state: contracts::DecryptDecisionState::LimitedUntilDeviceProof,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: true,
        used_recovery_path: false,
        notes: "Mobile/device-limited custody remains manual-required until proof exists."
            .to_string(),
    }
}

fn lost_key_manual_required_outcome() -> DecryptAttemptOutcome {
    DecryptAttemptOutcome {
        state: contracts::DecryptDecisionState::LostKeyManualRequired,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: true,
        used_recovery_path: false,
        notes: "Lost or unavailable key material stays manual-required.".to_string(),
    }
}

fn recovery_available_manual_required_outcome() -> DecryptAttemptOutcome {
    DecryptAttemptOutcome {
        state: contracts::DecryptDecisionState::RecoveryAvailableManualRequired,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: true,
        used_recovery_path: true,
        notes: "Recovery is explicit and parent-owned, never automatic.".to_string(),
    }
}

fn allowed_outcome() -> DecryptAttemptOutcome {
    DecryptAttemptOutcome {
        state: contracts::DecryptDecisionState::Allowed,
        decrypt_allowed: true,
        fail_closed: false,
        manual_required: false,
        used_recovery_path: false,
        notes: "Decrypt authority is explicit for this surface and scope.".to_string(),
    }
}
