use ocentra_schema::parent_storage_settings_apply_flow as contracts;

use super::{
    ParentStorageDeleteActionInput, ParentStorageDisconnectInput,
    ParentStorageSettingsApplyFlowError,
};

pub(super) fn derive_parent_storage_delete_action_row(
    input: ParentStorageDeleteActionInput,
) -> Result<contracts::ParentStorageDeleteActionRow, ParentStorageSettingsApplyFlowError> {
    if input.notes.trim().is_empty() {
        return Err(ParentStorageSettingsApplyFlowError::DeleteActionNotesMustStayVisible);
    }

    Ok(contracts::ParentStorageDeleteActionRow {
        action_id: input.action_id,
        action_kind: input.action_kind,
        state: input.state,
        separate_from_disconnect: true,
        proof_required: true,
        notes: input.notes,
    })
}

pub(super) fn derive_parent_storage_disconnect_row(
    input: ParentStorageDisconnectInput,
) -> Result<contracts::ParentStorageDisconnectRow, ParentStorageSettingsApplyFlowError> {
    if input.notes.trim().is_empty() {
        return Err(ParentStorageSettingsApplyFlowError::DisconnectNotesMustStayVisible);
    }

    Ok(contracts::ParentStorageDisconnectRow {
        action_id: input.action_id,
        state: input.state,
        existing_files_may_remain: true,
        provider_delete_requested_separately: true,
        notes: input.notes,
    })
}
