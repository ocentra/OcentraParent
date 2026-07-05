use super::parent_storage_settings_apply_flow::{
    parent_storage_settings_apply_flow_known_gaps,
    sample_parent_storage_settings_apply_flow_contract_proof,
    PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION,
};

const PARENT_STORAGE_SETTINGS_APPLY_FLOW_PROOF_JSON_EXPECTATION: &str =
    "parent storage settings apply flow proof json";
const PARENT_STORAGE_SETTINGS_APPLY_FLOW_KNOWN_GAP_SEPARATOR: &str = "\n";
const PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION_TOKEN: &str =
    "__PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION__";
const PARENT_STORAGE_SETTINGS_APPLY_FLOW_KNOWN_GAPS_TOKEN: &str =
    "__PARENT_STORAGE_SETTINGS_APPLY_FLOW_KNOWN_GAPS__";
const PARENT_STORAGE_SETTINGS_APPLY_FLOW_PROOF_JSON_TOKEN: &str =
    "__PARENT_STORAGE_SETTINGS_APPLY_FLOW_PROOF_JSON__";
const PARENT_STORAGE_SETTINGS_APPLY_FLOW_CONTRACTS_TEMPLATE: &str =
    include_str!("parent_storage_settings_apply_flow_contracts.template.txt");
const PARENT_STORAGE_SETTINGS_APPLY_FLOW_RULES_TEMPLATE: &str =
    include_str!("parent_storage_settings_apply_flow_rules.template.txt");

pub fn parent_storage_settings_apply_flow_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_parent_storage_settings_apply_flow_contract_proof()),
        PARENT_STORAGE_SETTINGS_APPLY_FLOW_PROOF_JSON_EXPECTATION,
    );
    let known_gaps = parent_storage_settings_apply_flow_known_gaps()
        .iter()
        .map(|gap| format!("  {:?},", gap))
        .collect::<Vec<_>>()
        .join(PARENT_STORAGE_SETTINGS_APPLY_FLOW_KNOWN_GAP_SEPARATOR);

    PARENT_STORAGE_SETTINGS_APPLY_FLOW_CONTRACTS_TEMPLATE
        .replace(
            PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION_TOKEN,
            PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION,
        )
        .replace(PARENT_STORAGE_SETTINGS_APPLY_FLOW_KNOWN_GAPS_TOKEN, &known_gaps)
        .replace(PARENT_STORAGE_SETTINGS_APPLY_FLOW_PROOF_JSON_TOKEN, &proof_json)
        .replace("{{", "{")
        .replace("}}", "}")
}

pub fn parent_storage_settings_apply_flow_contract_rules_typescript() -> String {
    PARENT_STORAGE_SETTINGS_APPLY_FLOW_RULES_TEMPLATE.to_owned()
}
