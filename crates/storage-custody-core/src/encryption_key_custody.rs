use std::collections::BTreeSet;

use ocentra_schema::encryption_key_custody as contracts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformKeyCustodyInput {
    pub surface: contracts::PlatformKeyCustodySurface,
    pub key_store: contracts::PlatformKeyStoreKind,
    pub decrypt_authority: contracts::PlatformDecryptAuthority,
    pub manual_required: bool,
    pub device_proof_required: bool,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecryptAttemptInput {
    pub attempt_id: contracts::EncryptionAttemptId,
    pub household_id: contracts::EncryptionHouseholdId,
    pub device_id: Option<contracts::EncryptionDeviceId>,
    pub surface: contracts::PlatformKeyCustodySurface,
    pub requested_scope: contracts::EncryptionUnlockScope,
    pub key_state: contracts::KeyCustodyState,
    pub recovery_mode: contracts::RecoveryMode,
    pub household_match: bool,
    pub device_match: bool,
    pub device_proof_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncryptionKeyCustodyDerivationError {
    DuplicateKeyClass(contracts::EncryptionKeyClass),
    DuplicateSurface(contracts::PlatformKeyCustodySurface),
    DuplicateAttempt(contracts::EncryptionAttemptId),
    MissingPlatformRowForAttempt(contracts::PlatformKeyCustodySurface),
    HostedPortalCannotDecrypt,
    UniversalDecryptForbidden(contracts::EncryptionKeyHolder),
    LinuxMustStayManualRequired,
    MobileProofGapMustStayManualRequired(contracts::PlatformKeyCustodySurface),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DecryptAttemptOutcome {
    state: contracts::DecryptDecisionState,
    decrypt_allowed: bool,
    fail_closed: bool,
    manual_required: bool,
    used_recovery_path: bool,
    notes: String,
}

fn option_or_unreachable<T>(value: Option<T>, context: &str) -> T {
    match value {
        Some(value) => value,
        None => unreachable!("{context}"),
    }
}

pub fn derive_platform_key_custody_row(
    input: PlatformKeyCustodyInput,
) -> Result<contracts::PlatformKeyCustodyRow, EncryptionKeyCustodyDerivationError> {
    match input.surface {
        contracts::PlatformKeyCustodySurface::Linux => {
            if !input.manual_required
                || input.decrypt_authority != contracts::PlatformDecryptAuthority::ManualRequired
            {
                return Err(EncryptionKeyCustodyDerivationError::LinuxMustStayManualRequired);
            }
        }
        contracts::PlatformKeyCustodySurface::Android
        | contracts::PlatformKeyCustodySurface::IOS
        | contracts::PlatformKeyCustodySurface::ParentMobile
        | contracts::PlatformKeyCustodySurface::ChildMobile => {
            if !input.manual_required || !input.device_proof_required {
                return Err(
                    EncryptionKeyCustodyDerivationError::MobileProofGapMustStayManualRequired(
                        input.surface,
                    ),
                );
            }
        }
        contracts::PlatformKeyCustodySurface::WebPortal => {
            if input.decrypt_authority != contracts::PlatformDecryptAuthority::NotDecryptRoot {
                return Err(EncryptionKeyCustodyDerivationError::HostedPortalCannotDecrypt);
            }
        }
        _ => {}
    }

    Ok(contracts::PlatformKeyCustodyRow {
        surface: input.surface,
        key_store: input.key_store,
        decrypt_authority: input.decrypt_authority,
        manual_required: input.manual_required,
        device_proof_required: input.device_proof_required,
        wrong_household_fails_closed: true,
        wrong_device_fails_closed: true,
        revoked_key_fails_closed: true,
        notes: input.notes,
    })
}

pub fn derive_decrypt_attempt_result(
    platform_row: &contracts::PlatformKeyCustodyRow,
    input: DecryptAttemptInput,
) -> contracts::DecryptAttemptResult {
    let DecryptAttemptOutcome {
        state,
        decrypt_allowed,
        fail_closed,
        manual_required,
        used_recovery_path,
        notes,
    } = decrypt_attempt_outcome(platform_row, &input);

    contracts::DecryptAttemptResult {
        attempt_id: input.attempt_id,
        surface: input.surface,
        requested_scope: input.requested_scope,
        state,
        decrypt_allowed,
        fail_closed,
        manual_required,
        used_recovery_path,
        notes,
    }
}

fn decrypt_attempt_outcome(
    platform_row: &contracts::PlatformKeyCustodyRow,
    input: &DecryptAttemptInput,
) -> DecryptAttemptOutcome {
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

pub fn build_encryption_key_custody_proof(
    key_hierarchy: Vec<contracts::EncryptionKeyHierarchyRow>,
    platform_inputs: Vec<PlatformKeyCustodyInput>,
    attempt_inputs: Vec<DecryptAttemptInput>,
    updated_at: contracts::EncryptionTimestamp,
) -> Result<contracts::EncryptionKeyCustodyContractProof, EncryptionKeyCustodyDerivationError> {
    let mut seen_key_classes = BTreeSet::new();
    for row in &key_hierarchy {
        if !seen_key_classes.insert(row.key_class.as_str().to_owned()) {
            return Err(EncryptionKeyCustodyDerivationError::DuplicateKeyClass(
                row.key_class,
            ));
        }
        if matches!(
            row.default_holder,
            contracts::EncryptionKeyHolder::ProviderConnection
                | contracts::EncryptionKeyHolder::SupportFlow
                | contracts::EncryptionKeyHolder::HostedPortal
        ) && (row.may_decrypt_child_evidence || row.may_decrypt_parent_exports)
        {
            return Err(
                EncryptionKeyCustodyDerivationError::UniversalDecryptForbidden(row.default_holder),
            );
        }
    }

    let mut platform_rows = Vec::with_capacity(platform_inputs.len());
    let mut seen_surfaces = BTreeSet::new();
    for input in platform_inputs {
        if !seen_surfaces.insert(input.surface.as_str().to_owned()) {
            return Err(EncryptionKeyCustodyDerivationError::DuplicateSurface(
                input.surface,
            ));
        }
        platform_rows.push(derive_platform_key_custody_row(input)?);
    }

    let mut attempts = Vec::with_capacity(attempt_inputs.len());
    let mut seen_attempts = BTreeSet::new();
    for input in attempt_inputs {
        if !seen_attempts.insert(input.attempt_id.as_str().to_owned()) {
            return Err(EncryptionKeyCustodyDerivationError::DuplicateAttempt(
                input.attempt_id,
            ));
        }
        let row = platform_rows
            .iter()
            .find(|row| row.surface == input.surface)
            .ok_or(
                EncryptionKeyCustodyDerivationError::MissingPlatformRowForAttempt(input.surface),
            )?;
        attempts.push(derive_decrypt_attempt_result(row, input));
    }

    Ok(contracts::EncryptionKeyCustodyContractProof {
        schema_version: contracts::ENCRYPTION_KEY_CUSTODY_SCHEMA_VERSION.to_string(),
        contract_version: option_or_unreachable(
            contracts::EncryptionKeyContractVersion::parse("v0.2"),
            "contract version",
        ),
        key_hierarchy,
        platform_matrix: platform_rows,
        attempts,
        non_claims: contracts::required_key_custody_non_claims(),
        universal_ocentra_key_present: false,
        hosted_portal_decrypt_root: false,
        updated_at,
    })
}
