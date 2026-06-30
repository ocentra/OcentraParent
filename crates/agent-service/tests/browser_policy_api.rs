#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "support/command_dispatch_test_support.rs"]
pub mod test_support;

#[path = "unit/browser_policy_api_tests.rs"]
mod browser_policy_api_tests;
#[path = "../src/json_contract.rs"]
mod json_contract;
#[path = "support/test_invariants.rs"]
mod test_invariants;

#[test]
fn browser_policy_api_harness_links_shared_value_helpers() {
    let value = json_contract::serialize_json_value(serde_json::json!({"policy": "linked"}));
    assert_eq!(
        test_invariants::require_some(value.get("policy"), "policy helper value"),
        "linked"
    );
}
