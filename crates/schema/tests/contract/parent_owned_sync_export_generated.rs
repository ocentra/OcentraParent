use crate::support::{assert_generated_line_containing_eq, assert_generated_line_eq, ContractLine};
use ocentra_schema::parent_owned_sync_export as contracts;
use ocentra_schema::parent_owned_sync_export_ts::{
    parent_owned_sync_export_contract_rules_typescript,
    parent_owned_sync_export_contracts_typescript,
};
use serde_json::json;

pub(super) fn assert_parent_owned_sync_export_contracts() {
    let proof = contracts::sample_parent_owned_sync_export_contract_proof();
    let encoded = serde_json::to_value(&proof).expect("proof serializes");

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION)
    );
    assert_eq!(
        encoded["providerStatuses"][1]["providerStatus"],
        json!("manual-required")
    );
    assert_eq!(
        encoded["syncStates"][6]["manifestIntegrityState"],
        json!("corrupt")
    );
    assert_eq!(
        encoded["tombstones"][3]["propagationState"],
        json!("blocked")
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::ParentOwnedSyncExportContractProof =
        serde_json::from_value(encoded).expect("proof deserializes");
    assert_eq!(decoded, proof);

    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-parent-owned-sync-export-contracts.ts"
    );
    let generated = parent_owned_sync_export_contracts_typescript();
    let generated = crate::contract_text!(&generated);

    assert_eq!(checked_in, generated.0);
    assert_generated_line_eq(
        generated,
        crate::contract_text!("export interface GeneratedParentOwnedSyncExportContractProof"),
        ContractLine("export interface GeneratedParentOwnedSyncExportContractProof {"),
    );
    assert_generated_line_eq(
        generated,
        crate::contract_text!("export interface GeneratedParentOwnedSyncProviderStatusRow"),
        ContractLine("export interface GeneratedParentOwnedSyncProviderStatusRow {"),
    );
    assert_generated_line_eq(
        generated,
        crate::contract_text!("export const GeneratedParentOwnedSyncExportKnownGaps = ["),
        ContractLine("export const GeneratedParentOwnedSyncExportKnownGaps = ["),
    );
    assert_generated_line_eq(
        generated,
        crate::contract_text!("export const GeneratedParentOwnedSyncStates = ["),
        ContractLine("export const GeneratedParentOwnedSyncStates = ["),
    );

    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-parent-owned-sync-export-contract-rules.ts"
    );
    let generated = parent_owned_sync_export_contract_rules_typescript();
    let generated = crate::contract_text!(&generated);

    assert_eq!(checked_in, generated.0);
    assert_generated_line_eq(
        generated,
        crate::contract_text!("export function syncExportManifestItemIsHonestGenerated("),
        ContractLine("export function syncExportManifestItemIsHonestGenerated(item: GeneratedParentOwnedSyncExportManifestItem): boolean {"),
    );
    assert_generated_line_eq(
        generated,
        crate::contract_text!("export function syncExportProviderStatusRowIsHonestGenerated("),
        ContractLine("export function syncExportProviderStatusRowIsHonestGenerated(row: GeneratedParentOwnedSyncProviderStatusRow): boolean {"),
    );
    assert_generated_line_eq(
        generated,
        crate::contract_text!("export function syncExportContractProofIsHonestGenerated("),
        ContractLine("export function syncExportContractProofIsHonestGenerated(proof: GeneratedParentOwnedSyncExportContractProof): boolean {"),
    );

    let adapter =
        include_str!("../../../../packages/schema-domain/src/parent-owned-sync-export.ts");

    assert_generated_line_eq(
        crate::contract_text!(adapter),
        crate::contract_text!(
            "/* thin adapter over Rust-generated parent-owned sync export contracts */"
        ),
        ContractLine("/* thin adapter over Rust-generated parent-owned sync export contracts */"),
    );
    assert_generated_line_containing_eq(
        crate::contract_text!(adapter),
        crate::contract_text!("from './generated-parent-owned-sync-export-contracts'"),
        ContractLine("} from './generated-parent-owned-sync-export-contracts';"),
    );
    assert_generated_line_containing_eq(
        crate::contract_text!(adapter),
        crate::contract_text!("from './generated-parent-owned-sync-export-contract-rules'"),
        ContractLine("} from './generated-parent-owned-sync-export-contract-rules';"),
    );
}
