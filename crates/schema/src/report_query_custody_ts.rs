use super::report_query_custody::{
    report_query_custody_known_gaps, sample_report_query_custody_contract_proof,
    REPORT_QUERY_CUSTODY_MAX_PAGE_SIZE, REPORT_QUERY_CUSTODY_SCHEMA_VERSION,
};

const REPORT_QUERY_CUSTODY_PROOF_JSON_EXPECT: &str = "report query custody proof json";
const REPORT_QUERY_CUSTODY_KNOWN_GAPS_SEPARATOR: &str = "\n";
const REPORT_QUERY_CUSTODY_SCHEMA_VERSION_TOKEN: &str = "__REPORT_QUERY_CUSTODY_SCHEMA_VERSION__";
const REPORT_QUERY_CUSTODY_MAX_PAGE_SIZE_TOKEN: &str = "__REPORT_QUERY_CUSTODY_MAX_PAGE_SIZE__";
const REPORT_QUERY_CUSTODY_PROOF_JSON_TOKEN: &str = "__REPORT_QUERY_CUSTODY_PROOF_JSON__";
const REPORT_QUERY_CUSTODY_KNOWN_GAPS_TOKEN: &str = "__REPORT_QUERY_CUSTODY_KNOWN_GAPS__";
const REPORT_QUERY_CUSTODY_CONTRACTS_TEMPLATE: &str =
    include_str!("report_query_custody_contracts.template.txt");
fn report_query_custody_contract_rules_template() -> String {
    assemble_template_fragments(&[
        include_str!("report_query_custody_contract_rules.request.template.txt"),
        include_str!("report_query_custody_contract_rules.state.template.txt"),
        include_str!("report_query_custody_contract_rules.proof.template.txt"),
    ])
}

fn assemble_template_fragments(fragments: &[&str]) -> String {
    format!(
        "{}\n",
        fragments
            .iter()
            .map(|fragment| fragment.trim_end_matches('\n'))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

pub fn report_query_custody_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_report_query_custody_contract_proof()),
        REPORT_QUERY_CUSTODY_PROOF_JSON_EXPECT,
    );
    let known_gaps = report_query_custody_known_gaps()
        .iter()
        .map(|gap| format!("  '{}',", gap.replace('\'', "\\'")))
        .collect::<Vec<_>>()
        .join(REPORT_QUERY_CUSTODY_KNOWN_GAPS_SEPARATOR);

    let proof_typescript =
        crate::typescript_literal::json_object_to_typescript_literal(&proof_json);

    REPORT_QUERY_CUSTODY_CONTRACTS_TEMPLATE
        .replace(
            REPORT_QUERY_CUSTODY_SCHEMA_VERSION_TOKEN,
            REPORT_QUERY_CUSTODY_SCHEMA_VERSION,
        )
        .replace(
            REPORT_QUERY_CUSTODY_MAX_PAGE_SIZE_TOKEN,
            &REPORT_QUERY_CUSTODY_MAX_PAGE_SIZE.to_string(),
        )
        .replace(REPORT_QUERY_CUSTODY_PROOF_JSON_TOKEN, &proof_typescript)
        .replace(REPORT_QUERY_CUSTODY_KNOWN_GAPS_TOKEN, &known_gaps)
        .replace("{{", "{")
        .replace("}}", "}")
}

pub fn report_query_custody_contract_rules_typescript() -> String {
    report_query_custody_contract_rules_template()
}
