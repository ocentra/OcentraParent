mod confirmation_types;
mod constants;
mod enums;
mod identifiers;
mod proof_types;
mod sample;
mod text_types;

pub const PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION: &str =
    constants::PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION;

pub type ParentStorageContractVersion = text_types::ParentStorageContractVersion;
pub type ParentStorageSettingsRowId = text_types::ParentStorageSettingsRowId;
pub type ParentStoragePreviewId = text_types::ParentStoragePreviewId;
pub type ParentStorageHouseholdRef = confirmation_types::ParentStorageHouseholdRef;
pub type ParentStorageConfirmationRef = confirmation_types::ParentStorageConfirmationRef;
pub type ParentStorageApplyId = text_types::ParentStorageApplyId;
pub type ParentStorageActionId = text_types::ParentStorageActionId;
pub type ParentStorageTimestamp = text_types::ParentStorageTimestamp;
pub type ParentStorageModeLabel = enums::ParentStorageModeLabel;
pub type ParentStorageUiState = enums::ParentStorageUiState;
pub type ParentStorageEncryptionStatus = enums::ParentStorageEncryptionStatus;
pub type ParentStorageKeyStatus = enums::ParentStorageKeyStatus;
pub type ParentStoragePreviewState = enums::ParentStoragePreviewState;
pub type ParentStorageApplyState = enums::ParentStorageApplyState;
pub type ParentStorageDeleteActionKind = enums::ParentStorageDeleteActionKind;
pub type ParentStorageCopyKey = enums::ParentStorageCopyKey;
pub type ParentStorageNoClaim = enums::ParentStorageNoClaim;
pub type ParentStorageModeCard = proof_types::ParentStorageModeCard;
pub type ParentStorageRestorePreview = proof_types::ParentStorageRestorePreview;
pub type ParentStorageApplyDecision = proof_types::ParentStorageApplyDecision;
pub type ParentStorageDeleteActionRow = proof_types::ParentStorageDeleteActionRow;
pub type ParentStorageDisconnectRow = proof_types::ParentStorageDisconnectRow;
pub type ParentStorageClaimSafeCopyRow = proof_types::ParentStorageClaimSafeCopyRow;
pub type ParentStorageSettingsApplyFlowContractProof =
    proof_types::ParentStorageSettingsApplyFlowContractProof;

pub fn required_parent_storage_mode_labels() -> Vec<ParentStorageModeLabel> {
    sample::required_parent_storage_mode_labels()
}

pub fn required_parent_storage_delete_action_kinds() -> Vec<ParentStorageDeleteActionKind> {
    sample::required_parent_storage_delete_action_kinds()
}

pub fn required_parent_storage_copy_keys() -> Vec<ParentStorageCopyKey> {
    sample::required_parent_storage_copy_keys()
}

pub fn required_parent_storage_no_claims() -> Vec<ParentStorageNoClaim> {
    sample::required_parent_storage_no_claims()
}

pub fn parent_storage_settings_apply_flow_known_gaps() -> [&'static str; 4] {
    sample::parent_storage_settings_apply_flow_known_gaps()
}

pub fn sample_parent_storage_settings_apply_flow_contract_proof(
) -> ParentStorageSettingsApplyFlowContractProof {
    sample::sample_parent_storage_settings_apply_flow_contract_proof()
}
