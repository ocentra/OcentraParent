#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/browser_policy_compiler_assessment.rs"]
mod browser_policy_compiler_assessment;
#[path = "../../src/browser_policy_compiler.rs"]
mod browser_policy_compiler_impl;
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

use crate::browser_policy_compiler::compile_browser_policy;
use crate::browser_policy_runtime_support::{
    base_revision_matches, default_revision_id, next_audit_event_id, next_revision_id,
    preview_revision_id, BrowserPolicyAuditEventId, BrowserPolicyRevisionId,
};
use crate::browser_policy_store::{
    browser_policy_store_path_from_env, BrowserPolicyRevisionRecord, BrowserPolicyStoredState,
};
use crate::test_invariants::{require_ok, require_some};
use crate::test_support::default_browser_policy_for_test;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::policy_constants as policy;

mod browser_policy_compiler {
    #[derive(Clone, Copy)]
    pub struct BrowserPolicyCapabilityRegistryRequest<'a> {
        pub generated_at: &'a str,
    }

    pub fn compile_browser_policy(
        policy: &ocentra_parent_agent_protocol::BrowserPolicyValue,
        revision_id: impl std::fmt::Display,
        compiled_at: impl std::fmt::Display,
    ) -> Result<
        ocentra_parent_agent_protocol::BrowserPolicyEffectivePolicy,
        ocentra_parent_agent_protocol::BrowserPolicyRejectionReason,
    > {
        let revision_id = revision_id.to_string();
        let compiled_at = compiled_at.to_string();
        super::browser_policy_compiler_impl::compile_browser_policy(
            policy,
            super::browser_policy_compiler_impl::BrowserPolicyCompileRequest {
                revision_id: revision_id.as_str(),
                compiled_at: compiled_at.as_str(),
            },
        )
    }

    pub fn browser_policy_capability_registry(
        request: BrowserPolicyCapabilityRegistryRequest<'_>,
    ) -> ocentra_parent_agent_protocol::BrowserPolicyCapabilityRegistry {
        let BrowserPolicyCapabilityRegistryRequest { generated_at } = request;
        super::browser_policy_compiler_impl::browser_policy_capability_registry(
            super::browser_policy_compiler_impl::BrowserPolicyCapabilityRegistryRequest {
                generated_at,
            },
        )
    }
}

#[cfg(test)]
mod clippy_linkage {
    use crate::browser_policy_runtime_support::{
        accepted_response, base_revision_matches, default_policy, default_revision_id,
        next_audit_event_id, next_revision_id, preview_revision_id, rejected_response,
        BrowserPolicyAuditEventId, BrowserPolicyMessage, BrowserPolicyPolicyId,
        BrowserPolicyRequestId, BrowserPolicyRevisionId, BrowserPolicyTimestamp,
    };
    use crate::browser_policy_store::{
        browser_policy_store_path_from_env, BrowserPolicyRevisionRecord, BrowserPolicyStoredState,
    };
    use crate::test_invariants::{require_ok, require_some};
    use crate::test_support::default_browser_policy_for_test;
    use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::policy_constants as policy;

    #[test]
    fn browser_policy_manifest_helpers_are_linked() -> Result<(), serde_json::Error> {
        let policy = default_browser_policy_for_test(
            crate::test_support::default_browser_policy_id_for_test(),
        );
        let effective_policy = crate::test_invariants::require_ok(
            crate::browser_policy_compiler::compile_browser_policy(
                &policy,
                constants::browser_policy::REVISION_ID,
                constants::browser_policy::TEST_SENT_AT,
            ),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let revision_id =
            BrowserPolicyRevisionId(format!("{}1", constants::browser_policy::REVISION_PREFIX));
        let audit_event_id =
            BrowserPolicyAuditEventId(format!("{}1", constants::browser_policy::AUDIT_PREFIX));
        let capability_registry =
            crate::browser_policy_compiler::browser_policy_capability_registry(
                crate::browser_policy_compiler::BrowserPolicyCapabilityRegistryRequest {
                    generated_at: constants::browser_policy::TEST_SENT_AT,
                },
            );
        let state = BrowserPolicyStoredState {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            active_revision_id: Some(revision_id.0.clone()),
            revisions: vec![BrowserPolicyRevisionRecord {
                revision_id: revision_id.0.clone(),
                policy,
                effective_policy,
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
                audit_event_id: audit_event_id.0.clone(),
            }],
            audit_events: vec![crate::browser_policy_store::BrowserPolicyAuditRecord {
                audit_event_id: audit_event_id.0,
                request_id: constants::browser_policy::REQUEST_ID.to_string(),
                kind: BrowserPolicyUpdateKind::Patch,
                revision_id: revision_id.0.clone(),
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
            }],
        };
        let active = require_some(
            state.active_revision(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let revision = require_some(
            state.revision_by_id(&revision_id),
            constants::error::AGENT_EVENT_SERIALIZES,
        );

        assert_eq!(active.revision_id, revision.revision_id);
        assert_eq!(
            state.active_revision_id.as_deref(),
            Some(revision_id.0.as_str())
        );
        assert!(next_revision_id(&state)
            .0
            .starts_with(constants::browser_policy::REVISION_PREFIX));
        assert!(next_audit_event_id(&state)
            .0
            .starts_with(constants::browser_policy::AUDIT_PREFIX));
        assert_eq!(
            default_revision_id().0,
            format!(
                "{}{}",
                constants::browser_policy::REVISION_PREFIX,
                constants::browser_policy::UPDATE_KIND_GET
            )
        );
        assert_eq!(
            preview_revision_id().0,
            format!(
                "{}{}",
                constants::browser_policy::REVISION_PREFIX,
                constants::browser_policy::UPDATE_KIND_PREVIEW
            )
        );
        require_ok(
            base_revision_matches(&state, Some(&revision_id)),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let _ = browser_policy_store_path_from_env();
        let serialized_value = crate::json_contract::serialize_json_value(serde_json::json!({
            "active": state.active_revision_id,
        }));
        let json_text = crate::json_contract::serialize_json_string(&serialized_value);
        let _: serde_json::Value = crate::test_invariants::require_json_decode(
            &json_text.0,
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let log_field = ocentra_parent_agent_protocol::logging::LogFieldValue::String(json_text.0);
        let _ = crate::test_invariants::require_log_string_field(
            Some(&log_field),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        assert_eq!(
            capability_registry.generated_at,
            constants::browser_policy::TEST_SENT_AT
        );
        assert_eq!(capability_registry.capabilities.len(), 8);

        assert_runtime_and_store_linkage(&state, active);
        Ok(())
    }

    fn assert_runtime_and_store_linkage(
        state: &BrowserPolicyStoredState,
        active: &BrowserPolicyRevisionRecord,
    ) {
        let default_policy = default_policy(BrowserPolicyPolicyId(
            constants::browser_policy::POLICY_ID.to_string(),
        ));
        let accepted = accepted_response(
            BrowserPolicyRequestId(constants::browser_policy::REQUEST_ID.to_string()),
            BrowserPolicyUpdateKind::Patch,
            default_policy,
            active.effective_policy.clone(),
            Some(BrowserPolicyAuditEventId(
                constants::browser_policy::AUDIT_EVENT_ID.to_string(),
            )),
            BrowserPolicyMessage(constants::browser_policy::MESSAGE_ACCEPTED),
            BrowserPolicyTimestamp(constants::browser_policy::TEST_SENT_AT.to_string()),
        );
        assert_eq!(
            accepted.status,
            ocentra_parent_agent_protocol::BrowserPolicyUpdateStatus::Accepted
        );
        let rejected = rejected_response(
            BrowserPolicyRequestId(constants::browser_policy::REQUEST_ID.to_string()),
            BrowserPolicyUpdateKind::Patch,
            ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason::StaleRevision,
            BrowserPolicyMessage(constants::browser_policy::MESSAGE_STALE_REVISION),
            BrowserPolicyTimestamp(constants::browser_policy::TEST_SENT_AT.to_string()),
        );
        assert_eq!(
            rejected.status,
            ocentra_parent_agent_protocol::BrowserPolicyUpdateStatus::Rejected
        );

        let path = std::env::temp_dir().join(format!(
            "browser-policy-manifest-{}.json",
            std::process::id()
        ));
        let runtime = crate::test_invariants::require_ok(
            tokio::runtime::Runtime::new(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let write_result = runtime.block_on(async {
            crate::browser_policy_store::write_browser_policy_state(&path, state).await
        });
        let _: () = crate::test_invariants::require_some(
            write_result.ok(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let loaded_result = runtime.block_on(async {
            crate::browser_policy_store::read_browser_policy_state(&path).await
        });
        let loaded = crate::test_invariants::require_some(
            loaded_result.ok(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        assert_eq!(loaded.active_revision_id, state.active_revision_id);
        let _ = std::fs::remove_file(path);
    }
}

#[test]
fn browser_policy_manifest_patch_smoke_uses_store_state_helpers() -> Result<(), serde_json::Error> {
    let policy =
        default_browser_policy_for_test(crate::test_support::default_browser_policy_id_for_test());
    let effective_policy = crate::test_invariants::require_ok(
        compile_browser_policy(
            &policy,
            constants::browser_policy::REVISION_ID,
            constants::browser_policy::TEST_SENT_AT,
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let revision_id =
        BrowserPolicyRevisionId(format!("{}1", constants::browser_policy::REVISION_PREFIX));
    let audit_event_id =
        BrowserPolicyAuditEventId(format!("{}1", constants::browser_policy::AUDIT_PREFIX));
    let capability_registry = crate::browser_policy_compiler::browser_policy_capability_registry(
        crate::browser_policy_compiler::BrowserPolicyCapabilityRegistryRequest {
            generated_at: constants::browser_policy::TEST_SENT_AT,
        },
    );
    let state = BrowserPolicyStoredState {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        active_revision_id: Some(revision_id.0.clone()),
        revisions: vec![BrowserPolicyRevisionRecord {
            revision_id: revision_id.0.clone(),
            policy,
            effective_policy,
            created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
            audit_event_id: audit_event_id.0.clone(),
        }],
        audit_events: vec![crate::browser_policy_store::BrowserPolicyAuditRecord {
            audit_event_id: audit_event_id.0,
            request_id: constants::browser_policy::REQUEST_ID.to_string(),
            kind: BrowserPolicyUpdateKind::Patch,
            revision_id: revision_id.0.clone(),
            created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
        }],
    };

    let active = require_some(
        state.active_revision(),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let revision = require_some(
        state.revision_by_id(&revision_id),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(active.revision_id, revision.revision_id);
    assert_eq!(
        state.active_revision_id.as_deref(),
        Some(revision_id.0.as_str())
    );
    assert!(next_revision_id(&state)
        .0
        .starts_with(constants::browser_policy::REVISION_PREFIX));
    assert!(next_audit_event_id(&state)
        .0
        .starts_with(constants::browser_policy::AUDIT_PREFIX));
    assert_eq!(
        default_revision_id().0,
        format!(
            "{}{}",
            constants::browser_policy::REVISION_PREFIX,
            constants::browser_policy::UPDATE_KIND_GET
        )
    );
    assert_eq!(
        preview_revision_id().0,
        format!(
            "{}{}",
            constants::browser_policy::REVISION_PREFIX,
            constants::browser_policy::UPDATE_KIND_PREVIEW
        )
    );
    require_ok(
        base_revision_matches(&state, Some(&revision_id)),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let _ = browser_policy_store_path_from_env();
    let json_text = crate::json_contract::serialize_json_string(&serde_json::json!({
        "active": state.active_revision_id,
    }));
    let _: serde_json::Value = serde_json::from_str(&json_text.0)?;
    assert_eq!(
        capability_registry.generated_at,
        constants::browser_policy::TEST_SENT_AT
    );
    assert_eq!(capability_registry.capabilities.len(), 8);

    Ok(())
}
