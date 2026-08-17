use ocentra_schema::parent_storage_settings_apply_flow as contracts;

use super::{
    ParentStorageConfirmationReceipt, ParentStorageConfirmationReceiptStatus,
    ParentStorageSettingsApplyFlowError,
};

pub(super) fn validate_confirmation_receipt(
    preview: &contracts::ParentStorageRestorePreview,
    receipt: &ParentStorageConfirmationReceipt,
) -> Result<(), ParentStorageSettingsApplyFlowError> {
    if receipt.confirmation_ref.as_str().trim().is_empty()
        || !is_canonical_utc_timestamp(receipt.issued_at.as_str())
        || !is_canonical_utc_timestamp(receipt.expires_at.as_str())
    {
        return Err(ParentStorageSettingsApplyFlowError::ConfirmationReceiptInvalid);
    }
    if receipt.preview_id != preview.preview_id {
        return Err(ParentStorageSettingsApplyFlowError::ConfirmationPreviewMismatch);
    }
    if receipt.household_ref != preview.household_ref {
        return Err(ParentStorageSettingsApplyFlowError::ConfirmationHouseholdMismatch);
    }
    if receipt.issued_at.as_str() >= receipt.expires_at.as_str() {
        return Err(ParentStorageSettingsApplyFlowError::ConfirmationWindowInvalid);
    }
    match receipt.status {
        ParentStorageConfirmationReceiptStatus::Issued => Ok(()),
        ParentStorageConfirmationReceiptStatus::Expired => {
            Err(ParentStorageSettingsApplyFlowError::ConfirmationExpired)
        }
        ParentStorageConfirmationReceiptStatus::Replayed => {
            Err(ParentStorageSettingsApplyFlowError::ConfirmationReplayed)
        }
        ParentStorageConfirmationReceiptStatus::WrongHousehold => {
            Err(ParentStorageSettingsApplyFlowError::ConfirmationHouseholdMismatch)
        }
    }
}

fn is_canonical_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() < 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
    {
        return false;
    }

    if bytes.len() == 20 {
        return bytes[19] == b'Z';
    }
    if bytes[19] != b'.' || *bytes.last().unwrap_or(&0) != b'Z' {
        return false;
    }
    bytes[20..bytes.len() - 1]
        .iter()
        .all(|byte| byte.is_ascii_digit())
}
