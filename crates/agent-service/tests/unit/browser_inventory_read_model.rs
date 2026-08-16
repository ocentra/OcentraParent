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
    use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateStatus;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::LogFieldValue;
    use ocentra_parent_agent_protocol::policy_constants as policy;
    use ocentra_parent_agent_protocol::{BrowserPolicyBudgets, BrowserPolicyEffectivePolicy};

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
        let _: String = crate::time::timestamp_after_epoch_seconds(0, 1);

        assert_eq!(active.revision_id, revision.revision_id);
        assert_eq!(
            state.active_revision_id.as_deref(),
            Some(revision_id.0.as_str())
        );
        assert_eq!(string_value.as_str(), encoded.as_str());
        assert!(parsed.is_object());
    }

    #[test]
    fn browser_policy_compiler_helpers_are_linked() {
        let registry = crate::browser_policy_compiler::browser_policy_capability_registry(
            crate::browser_policy_compiler::BrowserPolicyCapabilityRegistryRequest {
                generated_at: constants::browser_policy::TEST_SENT_AT,
            },
        );

        assert_eq!(
            registry.generated_at,
            constants::browser_policy::TEST_SENT_AT
        );
        assert_eq!(registry.capabilities.len(), 8);
    }

    #[test]
    fn browser_policy_runtime_support_state_helpers_are_linked() {
        let request_id =
            crate::browser_policy_runtime_support::BrowserPolicyRequestId("request-1".to_string());
        let _store_path = crate::browser_policy_runtime_support::BrowserPolicyStorePath(
            std::path::PathBuf::from("browser-policy.json"),
        );

        let empty_state = crate::browser_policy_store::BrowserPolicyStoredState::empty();
        assert!(
            crate::browser_policy_runtime_support::base_revision_matches(&empty_state, None)
                .is_ok()
        );
        assert_eq!(
            crate::browser_policy_runtime_support::base_revision_matches(
                &empty_state,
                Some(&crate::browser_policy_runtime_support::BrowserPolicyRevisionId(
                    "missing".to_string()
                ))
            ),
            Err(ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason::RevisionNotFound)
        );
        assert_eq!(
            crate::browser_policy_runtime_support::next_revision_id(&empty_state).0,
            format!("{}1", constants::browser_policy::REVISION_PREFIX)
        );
        assert_eq!(
            crate::browser_policy_runtime_support::next_audit_event_id(&empty_state).0,
            format!("{}1", constants::browser_policy::AUDIT_PREFIX)
        );
        assert_eq!(
            crate::browser_policy_runtime_support::preview_revision_id().0,
            format!(
                "{}{}",
                constants::browser_policy::REVISION_PREFIX,
                constants::browser_policy::UPDATE_KIND_PREVIEW
            )
        );
        assert_eq!(
            crate::browser_policy_runtime_support::default_revision_id().0,
            format!(
                "{}{}",
                constants::browser_policy::REVISION_PREFIX,
                constants::browser_policy::UPDATE_KIND_GET
            )
        );
        let _ = crate::browser_policy_runtime_support::browser_policy_store_path_from_env();
        let _ = request_id;
    }

    #[test]
    fn browser_policy_runtime_support_response_helpers_are_linked() {
        let request_id =
            crate::browser_policy_runtime_support::BrowserPolicyRequestId("request-1".to_string());
        let audit_event_id =
            crate::browser_policy_runtime_support::BrowserPolicyAuditEventId("audit-1".to_string());
        let timestamp = crate::browser_policy_runtime_support::BrowserPolicyTimestamp(
            "2026-07-13T00:00:00.000Z".to_string(),
        );
        let message =
            crate::browser_policy_runtime_support::BrowserPolicyMessage("browser policy ready");
        let policy = default_browser_policy_for_test(
            crate::test_support::default_browser_policy_id_for_test(),
        );
        let effective_policy = BrowserPolicyEffectivePolicy {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            policy_id: policy.policy_id.clone(),
            revision_id: "revision-1".to_string(),
            compiled_hash: format!(
                "{}{}",
                constants::browser_policy::COMPILED_HASH_PREFIX,
                "revision-1"
            ),
            compiled_at: constants::browser_policy::TEST_SENT_AT.to_string(),
            execution_mode: policy.execution_mode,
            default_posture: policy.default_posture,
            fallback_posture: policy.fallback_posture,
            discovery: policy.discovery.clone(),
            budgets: BrowserPolicyBudgets {
                enabled: policy.budgets.enabled,
                default_daily_minutes: policy.budgets.default_daily_minutes,
                counting_mode: policy.budgets.counting_mode,
            },
            rules: Vec::new(),
        };
        let accepted = crate::browser_policy_runtime_support::accepted_response(
            request_id.clone(),
            BrowserPolicyUpdateKind::Preview,
            policy,
            effective_policy,
            Some(audit_event_id),
            message,
            timestamp.clone(),
        );
        assert_eq!(accepted.status, BrowserPolicyUpdateStatus::Accepted);
        assert_eq!(accepted.request_id, "request-1");
        assert_eq!(accepted.audit_event_id, Some("audit-1".to_string()));

        let rejected = crate::browser_policy_runtime_support::rejected_response(
            request_id,
            BrowserPolicyUpdateKind::Preview,
            ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason::InvalidRequest,
            crate::browser_policy_runtime_support::BrowserPolicyMessage(
                "browser policy rejected",
            ),
            timestamp,
        );
        assert_eq!(rejected.status, BrowserPolicyUpdateStatus::Rejected);
        assert_eq!(rejected.request_id, "request-1");
        assert_eq!(
            rejected.rejection_reason,
            Some(ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason::InvalidRequest)
        );
        assert_eq!(
            rejected
                .capability_registry
                .map(|registry| registry.generated_at),
            Some("2026-07-13T00:00:00.000Z".to_string())
        );
    }

    #[test]
    fn browser_policy_store_helpers_round_trip_state() {
        let _ = crate::browser_policy_store::browser_policy_store_path_from_env();
        let mut path = std::env::temp_dir();
        path.push(format!(
            "ocentra-browser-policy-store-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0)
        ));

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap_or_else(|_| std::process::abort());

        runtime.block_on(async {
            let state = crate::browser_policy_store::BrowserPolicyStoredState::empty();
            crate::browser_policy_store::write_browser_policy_state(&path, &state)
                .await
                .unwrap_or_else(|_| std::process::abort());
            let loaded = crate::browser_policy_store::read_browser_policy_state(&path)
                .await
                .unwrap_or_else(|_| std::process::abort());

            assert_eq!(loaded, state);
        });

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn json_contract_helpers_are_linked() {
        let json_text = crate::json_contract::serialize_json_string(&serde_json::json!({
            "browser": "policy",
        }));
        assert_eq!(json_text.0, "{\"browser\":\"policy\"}");

        let json_value = crate::json_contract::serialize_json_value(serde_json::json!({
            "browser": "policy",
        }));
        assert_eq!(json_value["browser"], "policy");
    }

    #[test]
    fn timestamp_helpers_are_linked() {
        let now: String = crate::time::timestamp_now();
        let zero: String = crate::time::timestamp_from_epoch_seconds(0);
        let plus_one: String = crate::time::timestamp_after_epoch_seconds(0, 1);

        assert_eq!(zero, "1970-01-01T00:00:00.000Z");
        assert_eq!(plus_one, "1970-01-01T00:00:01.000Z");
        assert_eq!(now.len(), 24);
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
