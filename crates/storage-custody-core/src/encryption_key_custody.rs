use ocentra_schema::encryption_key_custody as contracts;

#[path = "encryption_key_custody_attempts.rs"]
mod encryption_key_custody_attempts;
#[path = "encryption_key_custody_platform.rs"]
mod encryption_key_custody_platform;
#[path = "encryption_key_custody_proof.rs"]
mod encryption_key_custody_proof;

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
    InvalidContractVersion,
}

pub fn derive_platform_key_custody_row(
    input: PlatformKeyCustodyInput,
) -> Result<contracts::PlatformKeyCustodyRow, EncryptionKeyCustodyDerivationError> {
    encryption_key_custody_platform::derive_platform_key_custody_row(input)
}

pub fn derive_decrypt_attempt_result(
    platform_row: &contracts::PlatformKeyCustodyRow,
    input: DecryptAttemptInput,
) -> contracts::DecryptAttemptResult {
    let encryption_key_custody_attempts::DecryptAttemptOutcome {
        state,
        decrypt_allowed,
        fail_closed,
        manual_required,
        used_recovery_path,
        notes,
    } = encryption_key_custody_attempts::decrypt_attempt_outcome(platform_row, &input);

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

pub fn build_encryption_key_custody_proof(
    key_hierarchy: Vec<contracts::EncryptionKeyHierarchyRow>,
    platform_inputs: Vec<PlatformKeyCustodyInput>,
    attempt_inputs: Vec<DecryptAttemptInput>,
    updated_at: contracts::EncryptionTimestamp,
) -> Result<contracts::EncryptionKeyCustodyContractProof, EncryptionKeyCustodyDerivationError> {
    encryption_key_custody_proof::build_encryption_key_custody_proof(
        key_hierarchy,
        platform_inputs,
        attempt_inputs,
        updated_at,
    )
}
