#[path = "report_query_custody_generated.rs"]
mod generated;

use generated::assert_report_query_custody_contracts;

#[test]
fn report_query_custody_contract_round_trips_through_rust_owned_shape() {
    assert_report_query_custody_contracts();
}

#[test]
fn generated_report_query_custody_contracts_stay_checked_in() {
    generated::assert_generated_report_query_custody_contracts();
}

#[test]
fn generated_report_query_custody_contract_rules_stay_checked_in() {
    generated::assert_generated_report_query_custody_contract_rules();
}

#[test]
fn report_query_custody_non_claims_remain_explicit() {
    generated::assert_report_query_custody_non_claims();
}

#[test]
fn report_query_custody_raw_proof_is_only_wire_shape_not_a_validated_snapshot() {
    generated::assert_raw_report_query_custody_proof_is_untrusted();
}

#[test]
fn report_query_custody_requires_every_explicit_outcome_state() {
    generated::assert_required_report_query_custody_states_are_explicit();
}
