use std::collections::BTreeSet;

use ocentra_schema::parent_storage_settings_apply_flow as contracts;

use super::{
    derive_parent_storage_apply_decision, derive_parent_storage_delete_action_row,
    derive_parent_storage_disconnect_row, derive_parent_storage_mode_card,
    derive_parent_storage_restore_preview, ParentStorageApplyDecisionInput,
    ParentStorageDeleteActionInput, ParentStorageDisconnectInput, ParentStorageModeCardInput,
    ParentStorageRestorePreviewInput, ParentStorageSettingsApplyFlowError,
};

pub(super) fn build_parent_storage_settings_apply_flow_proof(
    mode_card_input: ParentStorageModeCardInput,
    preview_input: ParentStorageRestorePreviewInput,
    apply_input: ParentStorageApplyDecisionInput,
    delete_action_inputs: Vec<ParentStorageDeleteActionInput>,
    disconnect_input: ParentStorageDisconnectInput,
    updated_at: contracts::ParentStorageTimestamp,
) -> Result<
    contracts::ParentStorageSettingsApplyFlowContractProof,
    ParentStorageSettingsApplyFlowError,
> {
    let mode_card = derive_parent_storage_mode_card(mode_card_input)?;
    let preview = derive_parent_storage_restore_preview(preview_input)?;
    let apply_decision = derive_parent_storage_apply_decision(&preview, apply_input)?;
    let disconnect_action = derive_parent_storage_disconnect_row(disconnect_input)?;

    let mut seen_delete_kinds = BTreeSet::new();
    let mut delete_actions = Vec::new();
    for input in delete_action_inputs {
        if !seen_delete_kinds.insert(input.action_kind.as_str().to_owned()) {
            return Err(
                ParentStorageSettingsApplyFlowError::DuplicateDeleteActionKind(input.action_kind),
            );
        }
        delete_actions.push(derive_parent_storage_delete_action_row(input)?);
    }

    if seen_delete_kinds.len() != contracts::required_parent_storage_delete_action_kinds().len() {
        return Err(ParentStorageSettingsApplyFlowError::DeleteActionCoverageIncomplete);
    }

    if disconnect_action.provider_delete_requested_separately
        && delete_actions
            .iter()
            .any(|row| !row.separate_from_disconnect)
    {
        return Err(
            ParentStorageSettingsApplyFlowError::DeleteActionMustStaySeparateFromDisconnect,
        );
    }

    Ok(contracts::ParentStorageSettingsApplyFlowContractProof {
        schema_version: contracts::PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION.to_string(),
        contract_version: contracts::ParentStorageContractVersion::parse("v0.6")
            .ok_or(ParentStorageSettingsApplyFlowError::InvalidContractVersion)?,
        mode_card,
        restore_preview: preview,
        apply_decision,
        delete_actions,
        disconnect_action,
        claim_safe_copy: contracts::sample_parent_storage_settings_apply_flow_contract_proof()
            .claim_safe_copy,
        no_claims: contracts::required_parent_storage_no_claims(),
        updated_at,
    })
}
