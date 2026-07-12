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
fn parent_storage_settings_apply_flow_rules_template() -> String {
    [
        include_str!("parent_storage_settings_apply_flow_rules.core.template.txt"),
        include_str!("parent_storage_settings_apply_flow_rules.modes.template.txt"),
        include_str!("parent_storage_settings_apply_flow_rules.states-early.template.txt"),
        include_str!("parent_storage_settings_apply_flow_rules.states-late.template.txt"),
    ]
    .concat()
}

pub fn parent_storage_settings_apply_flow_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_parent_storage_settings_apply_flow_contract_proof()),
        PARENT_STORAGE_SETTINGS_APPLY_FLOW_PROOF_JSON_EXPECTATION,
    );
    let known_gaps = parent_storage_settings_apply_flow_known_gaps()
        .iter()
        .map(|gap| format!("  '{}',", gap.replace('\'', "\\'")))
        .collect::<Vec<_>>()
        .join(PARENT_STORAGE_SETTINGS_APPLY_FLOW_KNOWN_GAP_SEPARATOR);

    let proof_typescript =
        crate::typescript_literal::json_object_to_typescript_literal(&proof_json);

    PARENT_STORAGE_SETTINGS_APPLY_FLOW_CONTRACTS_TEMPLATE
        .replace(
            PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION_TOKEN,
            PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION,
        )
        .replace(
            PARENT_STORAGE_SETTINGS_APPLY_FLOW_KNOWN_GAPS_TOKEN,
            &known_gaps,
        )
        .replace(
            PARENT_STORAGE_SETTINGS_APPLY_FLOW_PROOF_JSON_TOKEN,
            &proof_typescript,
        )
        .replace("{{", "{")
        .replace("}}", "}")
}

pub fn parent_storage_settings_apply_flow_contract_rules_typescript() -> String {
    parent_storage_settings_apply_flow_rules_template()
}
