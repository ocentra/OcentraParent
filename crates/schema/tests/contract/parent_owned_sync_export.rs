use crate::support::ValueOrUnreachable as _;
use ocentra_schema::parent_owned_sync_export as contracts;
use ocentra_schema::parent_owned_sync_export_ts::{
    parent_owned_sync_export_contract_rules_typescript,
    parent_owned_sync_export_contracts_typescript,
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
fn parent_owned_sync_export_contract_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_parent_owned_sync_export_contract_proof();
    let encoded = serde_json::to_value(&proof).value_or_unreachable("proof serializes");

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
        serde_json::from_value(encoded).value_or_unreachable("proof deserializes");
    assert_eq!(decoded, proof);
}

#[test]
fn generated_parent_owned_sync_export_contracts_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/parent-owned-sync-export-contracts.ts"
    );
    let generated = parent_owned_sync_export_contracts_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_line(
            &generated,
            "export interface GeneratedParentOwnedSyncExportContractProof"
        ),
        "export interface GeneratedParentOwnedSyncExportContractProof {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export interface GeneratedParentOwnedSyncProviderStatusRow"
        ),
        "export interface GeneratedParentOwnedSyncProviderStatusRow {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export const GeneratedParentOwnedSyncExportKnownGaps = ["
        ),
        "export const GeneratedParentOwnedSyncExportKnownGaps = ["
    );
    assert_eq!(
        generated_line(
            &generated,
            "export const GeneratedParentOwnedSyncStates = ["
        ),
        "export const GeneratedParentOwnedSyncStates = ["
    );
}

#[test]
fn generated_parent_owned_sync_export_contract_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/parent-owned-sync-export-contract-rules.ts"
    );
    let generated = parent_owned_sync_export_contract_rules_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_line(&generated, "export function syncExportManifestItemIsHonestGenerated("),
        "export function syncExportManifestItemIsHonestGenerated(item: GeneratedParentOwnedSyncExportManifestItem): boolean {"
    );
    assert_eq!(
        generated_line(&generated, "export function syncExportProviderStatusRowIsHonestGenerated("),
        "export function syncExportProviderStatusRowIsHonestGenerated(row: GeneratedParentOwnedSyncProviderStatusRow): boolean {"
    );
    assert_eq!(
        generated_line(&generated, "export function syncExportContractProofIsHonestGenerated("),
        "export function syncExportContractProofIsHonestGenerated(proof: GeneratedParentOwnedSyncExportContractProof): boolean {"
    );
}

#[test]
fn parent_owned_sync_export_adapter_stays_thin_and_generated_backed() {
    let adapter =
        include_str!("../../../../packages/schema-domain/src/parent-owned-sync-export.ts");

    assert_eq!(
        generated_line(
            adapter,
            "/* thin adapter over Rust-generated parent-owned sync export contracts */"
        ),
        "/* thin adapter over Rust-generated parent-owned sync export contracts */"
    );
    assert_eq!(
        line_containing(
            adapter,
            "from './generated/parent-owned-sync-export-contracts'"
        ),
        "} from './generated/parent-owned-sync-export-contracts';"
    );
    assert_eq!(
        line_containing(
            adapter,
            "from './generated/parent-owned-sync-export-contract-rules'"
        ),
        "} from './generated/parent-owned-sync-export-contract-rules';"
    );
}
