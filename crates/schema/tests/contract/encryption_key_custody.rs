use crate::support::ValueOrUnreachable as _;
use ocentra_schema::encryption_key_custody as contracts;
use serde_json::json;

#[test]
fn encryption_key_custody_contract_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_encryption_key_custody_contract_proof();
    let encoded = serde_json::to_value(&proof)
        .value_or_unreachable(crate::assert_context!("proof serializes"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::ENCRYPTION_KEY_CUSTODY_SCHEMA_VERSION)
    );
    assert_eq!(
        encoded["keyHierarchy"][0]["keyClass"],
        json!("child-device-local-key")
    );
    assert_eq!(encoded["platformMatrix"][2]["surface"], json!("linux"));
    assert_eq!(
        encoded["attempts"][1]["state"],
        json!("wrongHouseholdDenied")
    );
    assert_eq!(encoded["universalOcentraKeyPresent"], json!(false));
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::EncryptionKeyCustodyContractProof = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("proof deserializes"));
    assert_eq!(decoded, proof);
}

#[test]
fn encryption_key_custody_sample_proof_covers_required_rows_and_non_claims() {
    let proof = contracts::sample_encryption_key_custody_contract_proof();

    assert_eq!(
        proof
            .key_hierarchy
            .iter()
            .map(|row| row.key_class.as_str())
            .collect::<Vec<_>>(),
        contracts::required_encryption_key_classes()
            .iter()
            .map(contracts::EncryptionKeyClass::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        proof
            .platform_matrix
            .iter()
            .map(|row| row.surface.as_str())
            .collect::<Vec<_>>(),
        contracts::required_platform_key_surfaces()
            .iter()
            .map(contracts::PlatformKeyCustodySurface::as_str)
            .collect::<Vec<_>>()
    );
    assert!(proof.platform_matrix.iter().any(|row| {
        row.surface == contracts::PlatformKeyCustodySurface::Linux && row.manual_required
    }));
    assert!(!proof.universal_ocentra_key_present);
    assert!(!proof.hosted_portal_decrypt_root);
}
