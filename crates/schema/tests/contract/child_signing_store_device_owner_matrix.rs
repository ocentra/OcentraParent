use crate::support::{ErrorOrUnreachable as _, ValueOrUnreachable as _};
use ocentra_schema::child_signing_store_device_owner_matrix as contracts;
use ocentra_schema::child_signing_store_device_owner_matrix_ts::child_signing_store_device_owner_matrix_contracts_typescript;
use serde_json::json;

#[test]
fn child_signing_store_device_owner_matrix_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_child_signing_store_device_owner_matrix_proof();
    let encoded = serde_json::to_value(&proof)
        .value_or_unreachable(crate::assert_context!("proof serializes"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION)
    );
    assert_eq!(encoded["rows"][0]["platform"], json!("windows"));
    assert_eq!(encoded["rows"][0]["signingState"], json!("unsigned"));
    assert_eq!(
        encoded["rows"][3]["deviceOwnerState"],
        json!("manual-required")
    );
    assert_eq!(
        encoded["rows"][4]["supervisionState"],
        json!("device-proof-required")
    );
    assert_eq!(
        encoded["claimBoundaries"]["parentParity"],
        json!(
            "child artifact matrix does not imply parent-client parity, hidden daemons, or broader child runtime readiness"
        )
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::ChildSigningStoreDeviceOwnerMatrixProof =
        serde_json::from_value(encoded)
            .value_or_unreachable(crate::assert_context!("proof deserializes"));
    assert_eq!(decoded, proof);
}

#[test]
fn child_signing_store_device_owner_matrix_keeps_manual_required_boundaries_explicit() {
    let proof = contracts::sample_child_signing_store_device_owner_matrix_proof();

    assert_eq!(proof.rows.len(), 5);
    assert_eq!(proof.rows[0].device_owner_state.as_str(), "not-applicable");
    assert_eq!(
        proof.rows[3].managed_profile_state.as_str(),
        "manual-required"
    );
    assert_eq!(
        proof.rows[4].supervision_state.as_str(),
        "device-proof-required"
    );
    assert_eq!(
        proof.claim_boundaries.management_parity.as_str(),
        "device-owner, managed-profile, and supervision states stay platform-specific and manual-required, device-proof-required, or not-applicable unless a row proves otherwise"
    );
}

#[test]
fn child_signing_store_device_owner_matrix_rejects_missing_claim_boundary() {
    let mut encoded =
        serde_json::to_value(contracts::sample_child_signing_store_device_owner_matrix_proof())
            .value_or_unreachable(crate::assert_context!("proof serializes"));
    encoded["rows"][1]
        .as_object_mut()
        .value_or_unreachable(crate::assert_context!("row object"))
        .remove("claimBoundary");

    let decoded =
        serde_json::from_value::<contracts::ChildSigningStoreDeviceOwnerMatrixProof>(encoded);
    assert_eq!(
        decoded
            .error_or_unreachable(crate::assert_context!("missing claimBoundary should fail"))
            .to_string(),
        "missing field `claimBoundary`"
    );
}

#[test]
fn generated_child_signing_store_device_owner_matrix_contracts_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-child-signing-store-device-owner-matrix-contracts.ts"
    );
    let generated = child_signing_store_device_owner_matrix_contracts_typescript();
    let generated_lines: Vec<&str> = generated.lines().collect();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line
                == "export interface GeneratedChildSigningStoreDeviceOwnerMatrixProof {")
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export interface GeneratedChildArtifactMatrixRow {")
            .count(),
        1
    );
    assert!(generated_lines.iter().any(|line| {
        *line
            == format!(
                "export type GeneratedChildSigningStoreDeviceOwnerMatrixSchemaVersion = '{}';",
                contracts::CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION
            )
    }));
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export const GeneratedChildArtifactMatrixPlatforms = [")
            .count(),
        1
    );
}
