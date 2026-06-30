use crate::support::ValueOrUnreachable as _;
use ocentra_schema::parent_storage_settings_apply_flow as contracts;
use ocentra_schema::parent_storage_settings_apply_flow_ts::{
    parent_storage_settings_apply_flow_contract_rules_typescript,
    parent_storage_settings_apply_flow_contracts_typescript,
};
use serde_json::json;

fn generated_line<'a>(generated: &'a str, line_start: &str) -> &'a str {
    generated
        .lines()
        .find(|line| line.trim_start().starts_with(line_start))
        .value_or_unreachable("expected generated line to exist")
}

fn line_containing<'a>(generated: &'a str, snippet: &str) -> &'a str {
    generated
        .lines()
        .find(|line| line.contains(snippet))
        .value_or_unreachable("expected generated line to exist")
}

#[test]
fn parent_storage_settings_apply_flow_contract_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_parent_storage_settings_apply_flow_contract_proof();
    let encoded = serde_json::to_value(&proof).value_or_unreachable("proof serializes");

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
        json!("applyRequiresConfirmation")
    );
    assert_eq!(
        encoded["disconnectAction"]["state"],
        json!("disconnect-visible")
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::ParentStorageSettingsApplyFlowContractProof =
        serde_json::from_value(encoded).value_or_unreachable("proof deserializes");
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
        "../../../../packages/schema-domain/src/generated/parent-storage-settings-apply-flow-contracts.ts"
    );
    let generated = parent_storage_settings_apply_flow_contracts_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_line(
            &generated,
            "export interface GeneratedParentStorageSettingsApplyFlowContractProof"
        ),
        "export interface GeneratedParentStorageSettingsApplyFlowContractProof {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export const GeneratedParentStorageKnownGaps = ["
        ),
        "export const GeneratedParentStorageKnownGaps = ["
    );
    assert_eq!(
        generated_line(
            &generated,
            "export const GeneratedParentStorageModeLabels = ["
        ),
        "export const GeneratedParentStorageModeLabels = ["
    );
}

#[test]
fn generated_parent_storage_settings_apply_flow_contract_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/parent-storage-settings-apply-flow-contract-rules.ts"
    );
    let generated = parent_storage_settings_apply_flow_contract_rules_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_line(&generated, "export function parentStorageModeCardIsHonestGenerated("),
        "export function parentStorageModeCardIsHonestGenerated(card: GeneratedParentStorageModeCard): boolean {"
    );
    assert_eq!(
        generated_line(&generated, "export function parentStorageApplyDecisionIsHonestGenerated("),
        "export function parentStorageApplyDecisionIsHonestGenerated(decision: GeneratedParentStorageApplyDecision): boolean {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export function parentStorageSettingsApplyFlowProofIsHonestGenerated("
        ),
        "export function parentStorageSettingsApplyFlowProofIsHonestGenerated("
    );
}

#[test]
fn parent_storage_settings_apply_flow_adapter_stays_thin_and_generated_backed() {
    let adapter = include_str!(
        "../../../../packages/schema-domain/src/parent-storage-settings-apply-flow.ts"
    );

    assert_eq!(
        generated_line(
            adapter,
            "/* thin adapter over Rust-generated parent storage settings apply flow contracts */"
        ),
        "/* thin adapter over Rust-generated parent storage settings apply flow contracts */"
    );
    assert_eq!(
        line_containing(
            adapter,
            "from './generated/parent-storage-settings-apply-flow-contracts'"
        ),
        "} from './generated/parent-storage-settings-apply-flow-contracts';"
    );
    assert_eq!(
        line_containing(
            adapter,
            "from './generated/parent-storage-settings-apply-flow-contract-rules'"
        ),
        "} from './generated/parent-storage-settings-apply-flow-contract-rules';"
    );
}
