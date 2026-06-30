use crate::support::ValueOrUnreachable as _;
use ocentra_schema::report_query_custody as contracts;
use ocentra_schema::report_query_custody_ts::{
    report_query_custody_contract_rules_typescript, report_query_custody_contracts_typescript,
};
use serde_json::json;

fn generated_line<'a>(generated: &'a str, line_start: &str) -> &'a str {
    generated
        .lines()
        .find(|line| line.trim_start().starts_with(line_start))
        .value_or_unreachable("expected generated line to exist")
}

fn line_containing<'a>(generated: &'a str, snippet: &str) -> &'a str {
    generated
        .lines()
        .find(|line| line.contains(snippet))
        .value_or_unreachable("expected generated line to exist")
}

#[test]
fn report_query_custody_contract_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_report_query_custody_contract_proof();
    let encoded = serde_json::to_value(&proof).value_or_unreachable("proof serializes");

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::REPORT_QUERY_CUSTODY_SCHEMA_VERSION)
    );
    assert_eq!(encoded["rows"][0]["state"], json!("derivedFresh"));
    assert_eq!(encoded["rows"][3]["tombstoneState"], json!("written"));
    assert_eq!(
        encoded["rows"][6]["rateLimitedUntilAt"],
        json!("2026-06-28T16:05:00.000Z")
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::ReportQueryCustodyContractProof =
        serde_json::from_value(encoded).value_or_unreachable("proof deserializes");
    assert_eq!(decoded, proof);
}

#[test]
fn generated_report_query_custody_contracts_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/report-query-custody-contracts.ts"
    );
    let generated = report_query_custody_contracts_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_line(
            &generated,
            "export interface GeneratedReportQueryCustodyContractProof"
        ),
        "export interface GeneratedReportQueryCustodyContractProof {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export interface GeneratedReportQueryCustodyRow"
        ),
        "export interface GeneratedReportQueryCustodyRow {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export const GeneratedReportQueryCustodyKnownGaps = ["
        ),
        "export const GeneratedReportQueryCustodyKnownGaps = ["
    );
    assert_eq!(
        generated_line(
            &generated,
            "export const GeneratedReportQueryCustodyStates = ["
        ),
        "export const GeneratedReportQueryCustodyStates = ["
    );
}

#[test]
fn generated_report_query_custody_contract_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/report-query-custody-contract-rules.ts"
    );
    let generated = report_query_custody_contract_rules_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_line(&generated, "export function reportQueryCustodyRequestIsHonestGenerated("),
        "export function reportQueryCustodyRequestIsHonestGenerated(request: GeneratedReportQueryCustodyRequest): boolean {"
    );
    assert_eq!(
        generated_line(&generated, "export function reportQueryCustodyRowIsHonestGenerated("),
        "export function reportQueryCustodyRowIsHonestGenerated(row: GeneratedReportQueryCustodyRow): boolean {"
    );
    assert_eq!(
        generated_line(&generated, "export function reportQueryCustodyProofIsHonestGenerated("),
        "export function reportQueryCustodyProofIsHonestGenerated(proof: GeneratedReportQueryCustodyContractProof): boolean {"
    );
}

#[test]
fn report_query_custody_adapter_stays_thin_and_generated_backed() {
    let adapter = include_str!("../../../../packages/schema-domain/src/report-query-custody.ts");

    assert_eq!(
        generated_line(
            adapter,
            "/* thin adapter over Rust-generated report query custody contracts */"
        ),
        "/* thin adapter over Rust-generated report query custody contracts */"
    );
    assert_eq!(
        line_containing(adapter, "from './generated/report-query-custody-contracts'"),
        "} from './generated/report-query-custody-contracts';"
    );
    assert_eq!(
        line_containing(
            adapter,
            "from './generated/report-query-custody-contract-rules'"
        ),
        "} from './generated/report-query-custody-contract-rules';"
    );
}
