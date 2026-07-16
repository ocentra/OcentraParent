#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/json_contract.rs"]
mod json_contract;

#[test]
fn browser_policy_api_harness_links_shared_value_helpers() -> Result<(), serde_json::Error> {
    let json_text = json_contract::serialize_json_string(&serde_json::json!({
        "policy": "linked",
    }));
    let value_from_value = json_contract::serialize_json_value(serde_json::json!({
        "policy": "linked",
    }));
    let value: serde_json::Value = serde_json::from_str(&json_text.0)?;

    assert_eq!(
        value.get("policy").and_then(serde_json::Value::as_str),
        Some("linked")
    );
    assert_eq!(
        value_from_value
            .get("policy")
            .and_then(serde_json::Value::as_str),
        Some("linked")
    );

    Ok(())
}
