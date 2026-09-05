use crate::support::ValueOrUnreachable as _;
use ocentra_schema::parent_storage_settings_apply_flow as contracts;
use ocentra_schema::parent_storage_settings_apply_flow_ts::{
    parent_storage_settings_apply_flow_contract_rules_typescript,
    parent_storage_settings_apply_flow_contracts_typescript,
};
use serde_json::json;

fn generated_line<'a>(generated: &'a [u8], line_start: &[u8]) -> &'a [u8] {
    generated
        .split(|byte| *byte == b'\n')
        .find(|line| {
            let line = std::str::from_utf8(line)
                .value_or_unreachable(crate::assert_context!("generated line is valid utf-8"));
            line.trim_start().as_bytes().starts_with(line_start)
        })
        .value_or_unreachable(crate::assert_context!("expected generated line to exist"))
}

#[test]
fn parent_storage_settings_apply_flow_contract_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_parent_storage_settings_apply_flow_contract_proof();
    let encoded = serde_json::to_value(&proof)
        .value_or_unreachable(crate::assert_context!("proof serializes"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION)
    );
    assert_eq!(
        encoded["modeCard"]["currentModeLabel"],
        json!("manual-required")
    );
    assert_eq!(
        encoded["restorePreview"]["previewState"],
        json!("partialRestore")
    );
    assert_eq!(
        encoded["applyDecision"]["applyState"],
        json!("blockedManualRequired")
    );
    assert_eq!(
        encoded["disconnectAction"]["state"],
        json!("disconnect-visible")
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::ParentStorageSettingsApplyFlowContractProof =
        serde_json::from_value(encoded)
            .value_or_unreachable(crate::assert_context!("proof deserializes"));
    assert_eq!(decoded, proof);
}

#[test]
fn parent_storage_settings_apply_flow_sample_proof_covers_required_modes_actions_and_no_claims() {
    let proof = contracts::sample_parent_storage_settings_apply_flow_contract_proof();

    assert_eq!(
        contracts::required_parent_storage_mode_labels()
            .iter()
            .map(contracts::ParentStorageModeLabel::as_str)
            .collect::<Vec<_>>(),
        vec![
            "local-only",
            "local-plus-encrypted-backup",
            "local-plus-encrypted-provider-sync",
            "provider-disconnected",
            "provider-error",
            "manual-required",
            "disabled",
        ]
    );
    assert_eq!(
        proof
            .delete_actions
            .iter()
            .map(|row| row.action_kind.as_str())
            .collect::<Vec<_>>(),
        contracts::required_parent_storage_delete_action_kinds()
            .iter()
            .map(contracts::ParentStorageDeleteActionKind::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        proof
            .claim_safe_copy
            .iter()
            .map(|row| row.copy_key.as_str())
            .collect::<Vec<_>>(),
        contracts::required_parent_storage_copy_keys()
            .iter()
            .map(contracts::ParentStorageCopyKey::as_str)
            .collect::<Vec<_>>()
    );
    assert!(proof.restore_preview.confirmation_required);
    assert!(proof.restore_preview.tombstones_preserved);
    assert!(proof
        .delete_actions
        .iter()
        .all(|row| row.separate_from_disconnect && row.proof_required));
    assert!(proof.disconnect_action.existing_files_may_remain);
    assert!(proof.disconnect_action.provider_delete_requested_separately);
    assert_eq!(
        proof.no_claims,
        contracts::required_parent_storage_no_claims()
    );
}

#[test]
fn generated_parent_storage_settings_apply_flow_contracts_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-parent-storage-settings-apply-flow-contracts.ts"
    );
    let generated = parent_storage_settings_apply_flow_contracts_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_line(
            generated.as_bytes(),
            b"export interface GeneratedParentStorageSettingsApplyFlowContractProof"
        ),
        b"export interface GeneratedParentStorageSettingsApplyFlowContractProof {"
    );
    assert_eq!(
        generated_line(
            generated.as_bytes(),
            b"export const GeneratedParentStorageKnownGaps = ["
        ),
        b"export const GeneratedParentStorageKnownGaps = ["
    );
    assert_eq!(
        generated_line(
            generated.as_bytes(),
            b"export const GeneratedParentStorageModeLabels = ["
        ),
        b"export const GeneratedParentStorageModeLabels = ["
    );
}

#[test]
fn generated_parent_storage_settings_apply_flow_contract_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-parent-storage-settings-apply-flow-contract-rules.ts"
    );
    let generated = parent_storage_settings_apply_flow_contract_rules_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_line(
            generated.as_bytes(),
            b"export function parentStorageModeCardIsHonestGenerated("
        ),
        b"export function parentStorageModeCardIsHonestGenerated(card: GeneratedParentStorageModeCard): boolean {"
    );
    assert_eq!(
        generated_line(
            generated.as_bytes(),
            b"export function parentStorageApplyDecisionIsHonestGenerated("
        ),
        b"export function parentStorageApplyDecisionIsHonestGenerated(decision: GeneratedParentStorageApplyDecision): boolean {"
    );
    assert_eq!(
        generated_line(
            generated.as_bytes(),
            b"export function parentStorageSettingsApplyFlowProofIsHonestGenerated("
        ),
        b"export function parentStorageSettingsApplyFlowProofIsHonestGenerated("
    );
}
