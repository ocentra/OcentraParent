#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/browser_policy_compiler.rs"]
mod browser_policy_compiler;
#[path = "../../src/browser_policy_compiler_assessment.rs"]
mod browser_policy_compiler_assessment;
#[path = "../../src/browser_policy_runtime_support.rs"]
mod browser_policy_runtime_support;
#[path = "../../src/browser_policy_store.rs"]
mod browser_policy_store;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../support/browser_policy_test_support.rs"]
mod test_support;
#[path = "../support/test_text.rs"]
mod test_text;
#[path = "../../src/time.rs"]
mod time;

use ocentra_parent_agent_protocol::constants;

#[cfg(test)]
mod clippy_linkage {
    use crate::browser_policy_runtime_support::BrowserPolicyRevisionId;
    use crate::browser_policy_store::{BrowserPolicyRevisionRecord, BrowserPolicyStoredState};
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
    };
    use crate::test_support::default_browser_policy_for_test;
    use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::LogFieldValue;
    use ocentra_parent_agent_protocol::policy_constants as policy;

    #[test]
    fn browser_inventory_read_model_helpers_are_linked() {
        let empty_state = BrowserPolicyStoredState::empty();
        assert!(empty_state.active_revision().is_none());
        assert!(empty_state
            .revision_by_id(&BrowserPolicyRevisionId("missing".to_string()))
            .is_none());

        let (revision_id, state) = linked_browser_policy_state();
        let active = require_some(
            state.active_revision(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let revision = require_some(
            state.revision_by_id(&revision_id),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let encoded = require_ok(
            serde_json::to_string(&serde_json::json!({
                "active_revision_id": state.active_revision_id.as_deref(),
            })),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let parsed: serde_json::Value =
            require_json_decode(&encoded, constants::error::AGENT_EVENT_SERIALIZES);
        let field = LogFieldValue::String(encoded.clone());
        let string_value =
            require_log_string_field(Some(&field), constants::error::AGENT_EVENT_SERIALIZES);
        let _: serde_json::Value = crate::json_contract::serialize_json_value(serde_json::json!({
            "browser_inventory_read_model": true,
        }));
        let _: String = crate::time::timestamp_from_epoch_seconds(0);
        let _: String = crate::time::timestamp_after_epoch_seconds(0, 1);

        assert_eq!(active.revision_id, revision.revision_id);
        assert_eq!(
            state.active_revision_id.as_deref(),
            Some(revision_id.0.as_str())
        );
        assert_eq!(string_value.as_str(), encoded.as_str());
        assert!(parsed.is_object());
    }

    fn linked_browser_policy_state() -> (BrowserPolicyRevisionId, BrowserPolicyStoredState) {
        let policy = default_browser_policy_for_test(
            crate::test_support::default_browser_policy_id_for_test(),
        );
        let effective_policy = require_ok(
            crate::browser_policy_compiler::compile_browser_policy(
                &policy,
                crate::browser_policy_compiler::BrowserPolicyCompileRequest {
                    revision_id: constants::browser_policy::REVISION_ID,
                    compiled_at: constants::browser_policy::TEST_SENT_AT,
                },
            ),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let revision_id =
            BrowserPolicyRevisionId(format!("{}1", constants::browser_policy::REVISION_PREFIX));
        let state = BrowserPolicyStoredState {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            active_revision_id: Some(revision_id.0.clone()),
            revisions: vec![BrowserPolicyRevisionRecord {
                revision_id: revision_id.0.clone(),
                policy,
                effective_policy,
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
                audit_event_id: constants::browser_policy::REQUEST_ID.to_string(),
            }],
            audit_events: vec![crate::browser_policy_store::BrowserPolicyAuditRecord {
                audit_event_id: constants::browser_policy::REQUEST_ID.to_string(),
                request_id: constants::browser_policy::REQUEST_ID.to_string(),
                kind: BrowserPolicyUpdateKind::Preview,
                revision_id: revision_id.0.clone(),
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
            }],
        };

        (revision_id, state)
    }
}

#[test]
fn browser_inventory_read_model_smoke_uses_require_ok_helper() {
    let decoded = crate::test_invariants::require_ok(
        serde_json::from_str::<serde_json::Value>("{}"),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert!(decoded.is_object());
}
