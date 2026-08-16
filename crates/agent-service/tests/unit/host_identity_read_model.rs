#![forbid(unsafe_code)]

#[path = "../../src/host_identity_read_model.rs"]
mod host_identity_read_model;
#[path = "host_identity_read_model_tests.rs"]
mod host_identity_read_model_tests;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../support/test_invariants.rs"]
mod test_invariants;

#[test]
fn host_identity_harness_links_shared_json_invariants() {
    let encoded = test_invariants::serialize_test_json(&serde_json::json!({"host": "linked"}));
    let decoded: serde_json::Value =
        test_invariants::require_json_decode(&encoded, "host identity helper JSON decodes");
    assert_eq!(decoded["host"], "linked");

    let value = json_contract::serialize_json_value(serde_json::json!({"identity": "linked"}));
    assert_eq!(value["identity"], "linked");

    let field = ocentra_parent_agent_protocol::logging::LogFieldValue::String("linked".to_string());
    assert_eq!(
        test_invariants::require_log_string_field(
            Some(&field),
            "host identity log helper reads strings"
        ),
        "linked"
    );
}
