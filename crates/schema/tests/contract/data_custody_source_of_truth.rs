use crate::support::{extract_typescript_block, ValueOrUnreachable as _};
use ocentra_schema::data_custody_source_of_truth as contracts;
use ocentra_schema::data_custody_source_of_truth_ts::data_custody_source_of_truth_contracts_typescript;
use serde_json::json;

#[test]
fn data_custody_source_of_truth_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_data_custody_source_of_truth_contract_proof();
    let encoded = serde_json::to_value(&proof)
        .value_or_unreachable(crate::assert_context!("proof serializes"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::DATA_CUSTODY_SOURCE_OF_TRUTH_SCHEMA_VERSION)
    );
    assert_eq!(
        encoded["rows"][0]["classId"],
        json!("account-identity-metadata")
    );
    assert_eq!(
        encoded["rows"][15]["sourceOfTruth"]["kind"],
        json!("derived-from-data-classes")
    );
    assert_eq!(
        encoded["rows"][27]["classId"],
        json!("universal-decrypt-keys")
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::DataCustodySourceOfTruthContractProof = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("proof deserializes"));
    assert_eq!(decoded, proof);
}

#[test]
fn generated_data_custody_source_of_truth_contracts_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-data-custody-source-of-truth-contracts.ts"
    );
    let generated = data_custody_source_of_truth_contracts_typescript();

    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(checked_in),
            crate::text_boundary!(
                "export const GeneratedDataCustodyKnownGaps = ",
                " as const;"
            )
        ),
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const GeneratedDataCustodyKnownGaps = ",
                " as const;"
            )
        )
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(checked_in),
            crate::text_boundary!(
                "export const GeneratedDataCustodySourceOfTruthContractProof =",
                " as const satisfies GeneratedDataCustodySourceOfTruthContractProof;"
            )
        ),
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const GeneratedDataCustodySourceOfTruthContractProof =",
                " as const satisfies GeneratedDataCustodySourceOfTruthContractProof;"
            )
        )
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(checked_in),
            crate::text_boundary!(
                "export const GeneratedDataCustodyClassIds = [",
                "] as const satisfies readonly GeneratedDataCustodyClassId[];"
            )
        ),
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const GeneratedDataCustodyClassIds = [",
                "] as const satisfies readonly GeneratedDataCustodyClassId[];"
            )
        )
    );
    assert_generated_data_custody_source_of_truth_contracts(
        crate::contract_text!(checked_in),
        crate::contract_text!(&generated),
    );
}

fn assert_generated_data_custody_source_of_truth_contracts(
    checked_in: crate::support::ContractText<'_>,
    generated: crate::support::ContractText<'_>,
) {
    let checked_in_lines: Vec<&str> = checked_in.0.lines().collect();
    let generated_lines: Vec<&str> = generated.0.lines().collect();

    assert_eq!(
        checked_in_lines.first().copied(),
        Some("/* generated from crates/schema/src/data_custody_source_of_truth.rs */")
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export interface GeneratedDataCustodySourceOfTruthRow {")
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line
                == "export interface GeneratedDataCustodySourceOfTruthContractProof {")
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export const GeneratedDataCustodyKnownGaps = [")
            .count(),
        1
    );
    let schema_version_line = format!(
        "  SchemaVersion: '{}',",
        contracts::DATA_CUSTODY_SOURCE_OF_TRUTH_SCHEMA_VERSION
    );
    assert_eq!(
        generated_lines
            .iter()
            .find(|line| **line == schema_version_line.as_str()),
        Some(&schema_version_line.as_str())
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export const GeneratedDataCustodyClassIds = [")
            .count(),
        1
    );
}
