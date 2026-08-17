use ocentra_schema::parent_owned_sync_export::ParentOwnedSyncExportDataClass;
use ocentra_schema::parent_storage_settings_apply_flow as contracts;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{ParentStorageApplyDecisionInput, ParentStorageSettingsApplyFlowError};

const APPLY_INTENT_DIGEST_DOMAIN: &[u8] = b"ocentra.parent-storage-settings-apply.intent.v1\0";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ParentStorageApplyIntent<'a> {
    preview_id: &'a contracts::ParentStoragePreviewId,
    household_ref: &'a contracts::ParentStorageHouseholdRef,
    preview_state: contracts::ParentStoragePreviewState,
    created_at: &'a contracts::ParentStorageTimestamp,
    product_version: &'a str,
    schema_version: &'a str,
    household_match: bool,
    device_match: bool,
    data_classes: &'a [ParentOwnedSyncExportDataClass],
    conflicts: &'a [String],
    rejected_sections: &'a [ParentOwnedSyncExportDataClass],
    partial_restore: bool,
    preview_confirmation_required: bool,
    local_truth_authoritative: bool,
    tombstones_preserved: bool,
    preview_manual_required_note: Option<&'a str>,
    apply_id: &'a contracts::ParentStorageApplyId,
    will_change: &'a [ParentOwnedSyncExportDataClass],
    will_not_change: &'a [ParentOwnedSyncExportDataClass],
    preserved_tombstones: &'a [ParentOwnedSyncExportDataClass],
    manual_review_required: &'a [String],
    rollback_available: bool,
    apply_manual_required_note: Option<&'a str>,
}

pub(super) fn derive_parent_storage_apply_intent_digest(
    preview: &contracts::ParentStorageRestorePreview,
    input: &ParentStorageApplyDecisionInput,
) -> Result<contracts::ParentStorageApplyIntentDigest, ParentStorageSettingsApplyFlowError> {
    let intent = ParentStorageApplyIntent {
        preview_id: &preview.preview_id,
        household_ref: &preview.household_ref,
        preview_state: preview.preview_state,
        created_at: &preview.created_at,
        product_version: &preview.product_version,
        schema_version: &preview.schema_version,
        household_match: preview.household_match,
        device_match: preview.device_match,
        data_classes: &preview.data_classes,
        conflicts: &preview.conflicts,
        rejected_sections: &preview.rejected_sections,
        partial_restore: preview.partial_restore,
        preview_confirmation_required: preview.confirmation_required,
        local_truth_authoritative: preview.local_truth_authoritative,
        tombstones_preserved: preview.tombstones_preserved,
        preview_manual_required_note: preview.manual_required_note.as_deref(),
        apply_id: &input.apply_id,
        will_change: &input.will_change,
        will_not_change: &input.will_not_change,
        preserved_tombstones: &input.preserved_tombstones,
        manual_review_required: &input.manual_review_required,
        rollback_available: input.rollback_available,
        apply_manual_required_note: input.manual_required_note.as_deref(),
    };
    let canonical = serde_json::to_vec(&intent)
        .map_err(|_error| ParentStorageSettingsApplyFlowError::ApplyIntentDigestUnavailable)?;
    let mut digest = Sha256::new();
    digest.update(APPLY_INTENT_DIGEST_DOMAIN);
    digest.update(canonical);
    contracts::ParentStorageApplyIntentDigest::parse(format!("{:x}", digest.finalize()))
        .ok_or(ParentStorageSettingsApplyFlowError::ApplyIntentDigestUnavailable)
}
