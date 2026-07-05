use super::report_query_custody::{
    report_query_custody_known_gaps, sample_report_query_custody_contract_proof,
    REPORT_QUERY_CUSTODY_SCHEMA_VERSION,
};

const REPORT_QUERY_CUSTODY_PROOF_JSON_EXPECT: &str = "report query custody proof json";
const REPORT_QUERY_CUSTODY_KNOWN_GAPS_SEPARATOR: &str = "\n";
const REPORT_QUERY_CUSTODY_SCHEMA_VERSION_TOKEN: &str = "__REPORT_QUERY_CUSTODY_SCHEMA_VERSION__";
const REPORT_QUERY_CUSTODY_PROOF_JSON_TOKEN: &str = "__REPORT_QUERY_CUSTODY_PROOF_JSON__";
const REPORT_QUERY_CUSTODY_KNOWN_GAPS_TOKEN: &str = "__REPORT_QUERY_CUSTODY_KNOWN_GAPS__";
const REPORT_QUERY_CUSTODY_CONTRACTS_TEMPLATE: &str =
    include_str!("report_query_custody_contracts.template.txt");
const REPORT_QUERY_CUSTODY_CONTRACT_RULES_TEMPLATE: &str =
    include_str!("report_query_custody_contract_rules.template.txt");

pub fn report_query_custody_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_report_query_custody_contract_proof()),
        REPORT_QUERY_CUSTODY_PROOF_JSON_EXPECT,
    );
    let known_gaps = report_query_custody_known_gaps()
        .iter()
        .map(|gap| format!("  {:?},", gap))
        .collect::<Vec<_>>()
        .join(REPORT_QUERY_CUSTODY_KNOWN_GAPS_SEPARATOR);

    REPORT_QUERY_CUSTODY_CONTRACTS_TEMPLATE
        .replace(
            REPORT_QUERY_CUSTODY_SCHEMA_VERSION_TOKEN,
            REPORT_QUERY_CUSTODY_SCHEMA_VERSION,
        )
        .replace(REPORT_QUERY_CUSTODY_PROOF_JSON_TOKEN, &proof_json)
        .replace(REPORT_QUERY_CUSTODY_KNOWN_GAPS_TOKEN, &known_gaps)
        .replace("{{", "{")
        .replace("}}", "}")
}

pub fn report_query_custody_contract_rules_typescript() -> String {
    REPORT_QUERY_CUSTODY_CONTRACT_RULES_TEMPLATE.to_owned()
}
