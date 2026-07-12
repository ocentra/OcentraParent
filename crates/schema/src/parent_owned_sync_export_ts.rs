use super::parent_owned_sync_export::{
    parent_owned_sync_export_known_gaps, sample_parent_owned_sync_export_contract_proof,
    PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION,
};

const PARENT_OWNED_SYNC_EXPORT_PROOF_JSON_EXPECT_MESSAGE: &str =
    "parent owned sync export proof json";
const PARENT_OWNED_SYNC_EXPORT_TYPESCRIPT_LINE_BREAK: &str = "\n";
const PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION_TOKEN: &str =
    "__PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION__";
const PARENT_OWNED_SYNC_EXPORT_PROOF_JSON_TOKEN: &str = "__PARENT_OWNED_SYNC_EXPORT_PROOF_JSON__";
const PARENT_OWNED_SYNC_EXPORT_KNOWN_GAPS_TOKEN: &str = "__PARENT_OWNED_SYNC_EXPORT_KNOWN_GAPS__";
const PARENT_OWNED_SYNC_EXPORT_CONTRACTS_TEMPLATE: &str =
    include_str!("parent_owned_sync_export_contracts.template.txt");
const PARENT_OWNED_SYNC_EXPORT_CONTRACT_RULES_TEMPLATE: &str =
    include_str!("parent_owned_sync_export_contract_rules.template.txt");

pub fn parent_owned_sync_export_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_parent_owned_sync_export_contract_proof()),
        PARENT_OWNED_SYNC_EXPORT_PROOF_JSON_EXPECT_MESSAGE,
    );
    let known_gaps = parent_owned_sync_export_known_gaps()
        .iter()
        .map(|gap| format!("  '{}',", gap.replace('\'', "\\'")))
        .collect::<Vec<_>>()
        .join(PARENT_OWNED_SYNC_EXPORT_TYPESCRIPT_LINE_BREAK);

    let proof_typescript =
        crate::typescript_literal::json_object_to_typescript_literal(&proof_json);

    PARENT_OWNED_SYNC_EXPORT_CONTRACTS_TEMPLATE
        .replace(
            PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION_TOKEN,
            PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION,
        )
        .replace(PARENT_OWNED_SYNC_EXPORT_PROOF_JSON_TOKEN, &proof_typescript)
        .replace(PARENT_OWNED_SYNC_EXPORT_KNOWN_GAPS_TOKEN, &known_gaps)
        .replace("{{", "{")
        .replace("}}", "}")
}

pub fn parent_owned_sync_export_contract_rules_typescript() -> String {
    PARENT_OWNED_SYNC_EXPORT_CONTRACT_RULES_TEMPLATE.to_owned()
}
