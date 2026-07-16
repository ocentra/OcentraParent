use super::data_custody_source_of_truth::{
    data_custody_source_of_truth_known_gaps, sample_data_custody_source_of_truth_contract_proof,
    DATA_CUSTODY_SOURCE_OF_TRUTH_SCHEMA_VERSION,
};

const DATA_CUSTODY_SOURCE_OF_TRUTH_PROOF_JSON_EXPECT_MESSAGE: &str =
    "data custody source-of-truth proof json";
const DATA_CUSTODY_SOURCE_OF_TRUTH_TYPESCRIPT_LINE_BREAK: &str = "\n";
const DATA_CUSTODY_SOURCE_OF_TRUTH_SCHEMA_VERSION_PLACEHOLDER: &str = "__SCHEMA_VERSION__";
const DATA_CUSTODY_SOURCE_OF_TRUTH_KNOWN_GAPS_PLACEHOLDER: &str = "__KNOWN_GAPS__";
const DATA_CUSTODY_SOURCE_OF_TRUTH_PROOF_JSON_PLACEHOLDER: &str = "__PROOF_JSON__";
const DATA_CUSTODY_SOURCE_OF_TRUTH_TEMPLATE: &str =
    include_str!("data_custody_source_of_truth.template.txt");

pub fn data_custody_source_of_truth_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_data_custody_source_of_truth_contract_proof()),
        DATA_CUSTODY_SOURCE_OF_TRUTH_PROOF_JSON_EXPECT_MESSAGE,
    );
    let known_gaps = data_custody_source_of_truth_known_gaps()
        .iter()
        .map(|gap| format!("  '{}',", gap.replace('\'', "\\'")))
        .collect::<Vec<_>>()
        .join(DATA_CUSTODY_SOURCE_OF_TRUTH_TYPESCRIPT_LINE_BREAK);

    let proof_typescript =
        crate::typescript_literal::json_object_to_typescript_literal(&proof_json);

    DATA_CUSTODY_SOURCE_OF_TRUTH_TEMPLATE
        .replace(
            DATA_CUSTODY_SOURCE_OF_TRUTH_SCHEMA_VERSION_PLACEHOLDER,
            DATA_CUSTODY_SOURCE_OF_TRUTH_SCHEMA_VERSION,
        )
        .replace(
            DATA_CUSTODY_SOURCE_OF_TRUTH_KNOWN_GAPS_PLACEHOLDER,
            &known_gaps,
        )
        .replace(
            DATA_CUSTODY_SOURCE_OF_TRUTH_PROOF_JSON_PLACEHOLDER,
            &proof_typescript,
        )
}
