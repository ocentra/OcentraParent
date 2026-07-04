use crate::support::ValueOrUnreachable as _;
use ocentra_schema::export_import_backup_recovery as contracts;
use serde_json::json;

#[test]
fn export_import_backup_recovery_contract_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_export_import_backup_recovery_contract_proof();
    let encoded = serde_json::to_value(&proof)
        .value_or_unreachable(crate::assert_context!("proof serializes"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION)
    );
    assert_eq!(encoded["bundle"]["manifest"]["bundleType"], json!("backup"));
    assert_eq!(encoded["bundle"]["sections"][0]["encrypted"], json!(true));
    assert_eq!(encoded["importPreflight"]["state"], json!("partialPreview"));
    assert_eq!(encoded["restoreApply"]["state"], json!("partial"));
    assert_eq!(
        encoded["supportDefaultChildEvidenceDecryption"],
        json!(false)
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::ExportImportBackupRecoveryContractProof =
        serde_json::from_value(encoded)
            .value_or_unreachable(crate::assert_context!("proof deserializes"));
    assert_eq!(decoded, proof);
}

#[test]
fn export_import_backup_recovery_sample_proof_covers_required_negatives_and_redaction() {
    let proof = contracts::sample_export_import_backup_recovery_contract_proof();

    assert_eq!(
        proof
            .negative_preflights
            .iter()
            .map(|preflight| preflight.state.as_str())
            .collect::<Vec<_>>(),
        contracts::required_export_import_negative_preflight_states()
            .iter()
            .map(contracts::ExportImportPreflightState::as_str)
            .collect::<Vec<_>>()
    );
    assert!(proof
        .bundle
        .sections
        .iter()
        .all(|section| section.encrypted && !section.support_default_decryptable));
    assert!(proof.bundle.human_summary.raw_payload_redacted);
    assert!(proof.bundle.human_summary.support_safe);
    assert_eq!(
        proof.import_preflight.migration_state,
        contracts::ExportImportMigrationState::RequiredSupported
    );
}
