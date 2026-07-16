use crate::support::ValueOrUnreachable as _;
use ocentra_schema::retention_delete_tombstone as contracts;
use serde_json::json;

#[test]
fn retention_delete_tombstone_contract_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_retention_delete_tombstone_contract_proof();
    let encoded = serde_json::to_value(&proof)
        .value_or_unreachable(crate::assert_context!("proof serializes"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::RETENTION_DELETE_TOMBSTONE_SCHEMA_VERSION)
    );
    assert_eq!(encoded["rows"][0]["state"], json!("deleteRequested"));
    assert_eq!(encoded["rows"][2]["tombstoneWritten"], json!(true));
    assert_eq!(encoded["rows"][8]["hardDeleted"], json!(true));
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::RetentionDeleteTombstoneContractProof = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("proof deserializes"));
    assert_eq!(decoded, proof);
}

#[test]
fn retention_delete_tombstone_sample_proof_covers_required_states_and_matrix() {
    let proof = contracts::sample_retention_delete_tombstone_contract_proof();

    assert_eq!(proof.retention_matrix.len(), 11);
    assert_eq!(
        proof
            .rows
            .iter()
            .map(|row| row.state.as_str())
            .collect::<Vec<_>>(),
        contracts::required_retention_delete_states()
            .iter()
            .map(contracts::RetentionDeleteState::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        proof.rows[7].retention_class,
        contracts::RetentionDeleteRetentionClass::AuditMinimal
    );
    assert_eq!(
        proof.rows[8].retention_class,
        contracts::RetentionDeleteRetentionClass::HardDeleted
    );
}
