use crate::support::ValueOrUnreachable as _;
use ocentra_schema::export_import_backup_recovery as contracts;
use serde_json::json;

#[test]
fn runtime_contract_keeps_backup_integrity_and_tombstone_fields_typed() {
    let proof = contracts::sample_export_import_backup_recovery_contract_proof();
    let encoded = serde_json::to_value(&proof)
        .value_or_unreachable(crate::assert_context!("runtime proof serializes"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION)
    );
    assert_eq!(
        encoded["bundle"]["manifest"]["encryptionMode"],
        json!("per-class-envelope-encrypted")
    );
    assert_eq!(
        encoded["bundle"]["manifest"]["payloadIntegrityMode"],
        json!("manifest-and-payload-hashes")
    );
    assert_eq!(
        encoded["bundle"]["manifest"]["tombstoneCursor"],
        json!("tombstone-cursor-proof-7")
    );
    assert_eq!(
        encoded["bundle"]["humanSummary"]["rawPayloadRedacted"],
        json!(true)
    );
    assert_eq!(
        encoded["bundle"]["humanSummary"]["supportSafe"],
        json!(true)
    );
    assert_eq!(
        encoded["importPreflight"]["localTruthMutated"],
        json!(false)
    );
    assert_eq!(
        encoded["importPreflight"]["tombstonesPreserved"],
        json!(true)
    );
    assert_eq!(
        encoded["importPreflight"]["noDefaultSupportDecrypt"],
        json!(true)
    );
    assert!(encoded.get("schema_version").is_none());
    assert!(encoded["bundle"]["manifest"]
        .get("tombstone_cursor")
        .is_none());
}

#[test]
fn runtime_contract_exposes_all_negative_states_without_claiming_external_owners() {
    let proof = contracts::sample_export_import_backup_recovery_contract_proof();

    let negative_states = proof
        .negative_preflights
        .iter()
        .map(|preflight| preflight.state)
        .collect::<Vec<_>>();
    assert_eq!(
        negative_states,
        contracts::required_export_import_negative_preflight_states()
    );
    assert!(proof
        .negative_preflights
        .iter()
        .all(|preflight| !preflight.local_truth_mutated));
    assert!(proof
        .negative_preflights
        .iter()
        .all(|preflight| preflight.tombstones_preserved));
    assert_eq!(
        proof.non_claims,
        contracts::required_export_import_non_claims()
    );
    assert!(!proof.provider_runtime_claimed);
    assert!(!proof.support_default_child_evidence_decryption);
    assert!(!proof.ts_business_owner_claimed);
}

#[test]
fn runtime_contract_round_trip_preserves_partial_restore_and_redaction_boundaries() {
    let proof = contracts::sample_export_import_backup_recovery_contract_proof();
    let encoded = serde_json::to_vec(&proof)
        .value_or_unreachable(crate::assert_context!("runtime proof encodes"));
    let decoded: contracts::ExportImportBackupRecoveryContractProof =
        serde_json::from_slice(&encoded)
            .value_or_unreachable(crate::assert_context!("runtime proof decodes"));

    assert_eq!(decoded, proof);
    assert_eq!(
        decoded.restore_apply.state,
        contracts::ExportImportRestoreApplyState::Partial
    );
    assert!(decoded.restore_apply.tombstones_preserved);
    assert!(decoded.restore_apply.idempotent);
    assert!(!decoded.restore_apply.duplicates_created);
    assert!(decoded.restore_apply.no_default_support_decrypt);
    assert!(decoded
        .bundle
        .sections
        .iter()
        .all(|section| section.encrypted && !section.support_default_decryptable));
}
