use std::collections::BTreeSet;

use ocentra_schema::encryption_key_custody as contracts;

use super::{
    derive_decrypt_attempt_result, derive_platform_key_custody_row, DecryptAttemptInput,
    EncryptionKeyCustodyDerivationError, PlatformKeyCustodyInput,
};

pub(super) fn build_encryption_key_custody_proof(
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
        contract_version: contracts::EncryptionKeyContractVersion::parse("v0.2")
            .ok_or(EncryptionKeyCustodyDerivationError::InvalidContractVersion)?,
        key_hierarchy,
        platform_matrix: platform_rows,
        attempts,
        non_claims: contracts::required_key_custody_non_claims(),
        universal_ocentra_key_present: false,
        hosted_portal_decrypt_root: false,
        updated_at,
    })
}
