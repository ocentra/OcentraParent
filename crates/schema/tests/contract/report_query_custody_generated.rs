use crate::support::ValueOrUnreachable;
use ocentra_schema::report_query_custody as contracts;
use ocentra_schema::report_query_custody_ts::{
    report_query_custody_contract_rules_typescript, report_query_custody_contracts_typescript,
};
use serde_json::json;

pub(super) fn assert_report_query_custody_contracts() {
    let proof = contracts::sample_report_query_custody_contract_proof();
    let encoded = serde_json::to_value(&proof)
        .value_or_unreachable(crate::assert_context!("proof serializes"));

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

    let decoded: contracts::ReportQueryCustodyContractProof = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("proof deserializes"));
    assert_eq!(decoded, proof);
    assert_eq!(
        decoded.schema_version,
        contracts::REPORT_QUERY_CUSTODY_SCHEMA_VERSION
    );
    assert_eq!(
        decoded.rows.len(),
        contracts::required_report_query_custody_states().len()
    );
}

pub(super) fn assert_generated_report_query_custody_contracts() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-report-query-custody-contracts.ts"
    );
    let generated_source = report_query_custody_contracts_typescript();
    let generated = crate::contract_text!(&generated_source);
    assert_eq!(checked_in, generated.0);
}

pub(super) fn assert_generated_report_query_custody_contract_rules() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-report-query-custody-contract-rules.ts"
    );
    let generated_source = report_query_custody_contract_rules_typescript();
    let generated = crate::contract_text!(&generated_source);
    assert_eq!(checked_in, generated.0);
}

pub(super) fn assert_report_query_custody_non_claims() {
    let non_claims = contracts::required_report_query_custody_non_claims();
    assert_eq!(
        non_claims,
        vec![
            contracts::ReportQueryCustodyNonClaim::SecondTruthStore,
            contracts::ReportQueryCustodyNonClaim::PortalUi,
            contracts::ReportQueryCustodyNonClaim::RawChildEvidence,
            contracts::ReportQueryCustodyNonClaim::UnboundedPagination,
            contracts::ReportQueryCustodyNonClaim::ProviderRouting,
            contracts::ReportQueryCustodyNonClaim::OcentraHostedFamilyDataCustody,
        ]
    );
}

pub(super) fn assert_raw_report_query_custody_proof_is_untrusted() {
    let mut raw = contracts::sample_report_query_custody_contract_proof();
    raw.rows.clear();
    raw.report_runtime_claimed = true;

    let encoded = serde_json::to_value(&raw)
        .value_or_unreachable(crate::assert_context!("untrusted proof serializes"));
    let decoded: contracts::ReportQueryCustodyContractProof = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("untrusted proof deserializes"));

    assert!(decoded.rows.is_empty());
    assert!(decoded.report_runtime_claimed);
}

pub(super) fn assert_required_report_query_custody_states_are_explicit() {
    assert_eq!(
        contracts::required_report_query_custody_states()
            .iter()
            .map(|state| state.as_str())
            .collect::<Vec<_>>(),
        vec![
            "derivedFresh",
            "derivedStale",
            "partiallyRedacted",
            "deletedSource",
            "syncConflict",
            "cursorExpired",
            "rateLimited",
        ]
    );
}
