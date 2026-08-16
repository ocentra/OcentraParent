#[path = "parent_owned_sync_export_generated.rs"]
mod generated;

use generated::assert_parent_owned_sync_export_contracts;

#[test]
fn parent_owned_sync_export_contract_round_trips_through_rust_owned_shape() {
    assert_parent_owned_sync_export_contracts();
}

#[test]
fn generated_parent_owned_sync_export_contracts_stay_checked_in() {
    assert_parent_owned_sync_export_contracts();
}

#[test]
fn generated_parent_owned_sync_export_contract_rules_stay_checked_in() {
    assert_parent_owned_sync_export_contracts();
}
