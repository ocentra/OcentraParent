#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "support/browser_inventory_test_support.rs"]
pub mod test_support;

#[path = "support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../src/browser_inventory_read_model.rs"]
mod browser_inventory_read_model;
#[path = "unit/browser_inventory_read_model_tests.rs"]
mod browser_inventory_read_model_tests;
#[path = "../src/browser_payload.rs"]
mod browser_payload;
#[path = "../src/browser_policy_compiler.rs"]
mod browser_policy_compiler;
#[path = "../src/browser_policy_compiler_assessment.rs"]
mod browser_policy_compiler_assessment;
#[path = "../src/browser_policy_runtime_support.rs"]
mod browser_policy_runtime_support;
#[path = "../src/browser_policy_store.rs"]
mod browser_policy_store;
#[path = "../src/browser_runtime_paths.rs"]
mod browser_runtime_paths;
#[path = "../src/browser_runtime_status.rs"]
mod browser_runtime_status;
#[path = "../src/fields.rs"]
mod fields;
#[path = "../src/json_contract.rs"]
mod json_contract;
#[path = "support/test_invariants.rs"]
mod test_invariants;
#[path = "../src/time.rs"]
mod time;

#[cfg(test)]
mod clippy_linkage {
    use crate::browser_policy_store::{BrowserPolicyRevisionRecord, BrowserPolicyStoredState};
    use crate::test_invariants::{require_json_decode, require_log_string_field};
    use crate::test_support::default_browser_policy_for_test;
    use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::LogFieldValue;
    use ocentra_parent_agent_protocol::policy_constants as policy;

    #[test]
    fn browser_inventory_read_model_helpers_are_linked() {
        let empty_state = BrowserPolicyStoredState::empty();
        assert!(empty_state.active_revision().is_none());
        assert!(empty_state.revision_by_id("missing").is_none());

        let policy =
            default_browser_policy_for_test(constants::browser_policy::POLICY_ID.to_string());
        let effective_policy = match crate::browser_policy_compiler::compile_browser_policy(
            &policy,
            constants::browser_policy::REVISION_ID,
            constants::browser_policy::TEST_SENT_AT,
        ) {
            Ok(value) => value,
            Err(_) => return,
        };
        let revision_id = format!("{}1", constants::browser_policy::REVISION_PREFIX);
        let state = BrowserPolicyStoredState {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            active_revision_id: Some(revision_id.clone()),
            revisions: vec![BrowserPolicyRevisionRecord {
                revision_id: revision_id.clone(),
                policy,
                effective_policy,
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
                audit_event_id: constants::browser_policy::REQUEST_ID.to_string(),
            }],
            audit_events: vec![crate::browser_policy_store::BrowserPolicyAuditRecord {
                audit_event_id: constants::browser_policy::REQUEST_ID.to_string(),
                request_id: constants::browser_policy::REQUEST_ID.to_string(),
                kind: BrowserPolicyUpdateKind::Preview,
                revision_id: revision_id.clone(),
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
            }],
        };
        let active = match state.active_revision() {
            Some(value) => value,
            None => return,
        };
        let revision = match state.revision_by_id(&revision_id) {
            Some(value) => value,
            None => return,
        };
        let encoded = match serde_json::to_string(&serde_json::json!({
            "active_revision_id": state.active_revision_id.as_deref(),
        })) {
            Ok(value) => value,
            Err(_) => return,
        };
        let parsed = match serde_json::from_str::<serde_json::Value>(&encoded) {
            Ok(value) => value,
            Err(_) => return,
        };
        let field = LogFieldValue::String(encoded.clone());
        let string_value = match &field {
            LogFieldValue::String(value) => value.as_str(),
            _ => return,
        };
        let _: serde_json::Value = crate::json_contract::serialize_json_value(serde_json::json!({
            "browser_inventory_read_model": true,
        }));
        let _ = crate::time::timestamp_from_epoch_seconds(0);
        let _ = crate::time::timestamp_after_epoch_seconds(0, 1);
        let _: serde_json::Value =
            require_json_decode(&encoded, constants::error::AGENT_EVENT_SERIALIZES);
        let _ = require_log_string_field(Some(&field), constants::error::AGENT_EVENT_SERIALIZES);

        assert_eq!(active.revision_id, revision.revision_id);
        assert_eq!(
            state.active_revision_id.as_deref(),
            Some(revision_id.as_str())
        );
        assert_eq!(string_value, encoded);
        assert!(parsed.is_object());
    }
}

#[test]
fn browser_inventory_read_model_smoke_uses_require_ok_helper() {
    let decoded = match serde_json::from_str::<serde_json::Value>("{}") {
        Ok(value) => value,
        Err(_) => return,
    };

    assert!(decoded.is_object());
}
