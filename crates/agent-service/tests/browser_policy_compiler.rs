#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

use crate::browser_policy_runtime_support::{
    accepted_response, base_revision_matches, default_revision_id, next_audit_event_id,
    next_revision_id, preview_revision_id, rejected_response,
};
use crate::browser_policy_store::{
    browser_policy_store_path_from_env, read_browser_policy_state, write_browser_policy_state,
    BrowserPolicyAuditRecord, BrowserPolicyRevisionRecord, BrowserPolicyStoredState,
};
use crate::test_invariants::{require_ok, require_some};
use crate::test_support::default_browser_policy_for_test;
use ocentra_parent_agent_protocol::browser_policy::{
    BrowserPolicyUpdateKind, BrowserPolicyUpdateStatus,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::policy_constants as policy;

#[path = "support/browser_policy_test_support.rs"]
pub mod test_support;

#[path = "../src/browser_policy_compiler.rs"]
mod browser_policy_compiler;
#[path = "../src/browser_policy_compiler_assessment.rs"]
mod browser_policy_compiler_assessment;
#[path = "unit/browser_policy_compiler_tests.rs"]
mod browser_policy_compiler_tests;
#[path = "../src/browser_policy_runtime_support.rs"]
mod browser_policy_runtime_support;
#[path = "../src/browser_policy_store.rs"]
mod browser_policy_store;
#[path = "../src/json_contract.rs"]
mod json_contract;
#[path = "support/test_invariants.rs"]
mod test_invariants;

#[cfg(test)]
mod clippy_linkage {
    use crate::browser_policy_runtime_support::{
        accepted_response, base_revision_matches, default_revision_id, next_audit_event_id,
        next_revision_id, preview_revision_id, rejected_response,
    };
    use crate::browser_policy_store::{
        browser_policy_store_path_from_env, read_browser_policy_state, write_browser_policy_state,
        BrowserPolicyAuditRecord, BrowserPolicyRevisionRecord, BrowserPolicyStoredState,
    };
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
    };
    use crate::test_support::default_browser_policy_for_test;
    use ocentra_parent_agent_protocol::browser_policy::{
        BrowserPolicyUpdateKind, BrowserPolicyUpdateStatus,
    };
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::policy_constants as policy;

    #[tokio::test]
    async fn browser_policy_compiler_helpers_are_linked() {
        let policy =
            default_browser_policy_for_test(constants::browser_policy::POLICY_ID.to_string());
        let effective_policy = require_ok(
            crate::browser_policy_compiler::compile_browser_policy(
                &policy,
                constants::browser_policy::REVISION_ID,
                constants::browser_policy::TEST_SENT_AT,
            ),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let revision_id = format!("{}1", constants::browser_policy::REVISION_PREFIX);
        let audit_event_id = format!("{}1", constants::browser_policy::AUDIT_PREFIX);
        let state = BrowserPolicyStoredState {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            active_revision_id: Some(revision_id.clone()),
            revisions: vec![BrowserPolicyRevisionRecord {
                revision_id: revision_id.clone(),
                policy: policy.clone(),
                effective_policy: effective_policy.clone(),
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
                audit_event_id: audit_event_id.clone(),
            }],
            audit_events: vec![BrowserPolicyAuditRecord {
                audit_event_id: audit_event_id.clone(),
                request_id: constants::browser_policy::REQUEST_ID.to_string(),
                kind: BrowserPolicyUpdateKind::Preview,
                revision_id: revision_id.clone(),
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
            }],
        };
        let revision = require_some(
            state.revision_by_id(&revision_id),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let active = require_some(
            state.active_revision(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let store_path = std::env::temp_dir().join(format!(
            "ocentra-browser-policy-compiler-linkage-{}-{}.json",
            std::process::id(),
            revision_id
        ));
        require_ok(
            write_browser_policy_state(&store_path, &state).await,
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let roundtrip = require_ok(
            read_browser_policy_state(&store_path).await,
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let accepted = accepted_response(
            constants::browser_policy::REQUEST_ID.to_string(),
            BrowserPolicyUpdateKind::Preview,
            policy.clone(),
            effective_policy.clone(),
            Some(audit_event_id.clone()),
            "accepted",
            constants::browser_policy::TEST_SENT_AT,
        );
        let rejected = rejected_response(
            constants::browser_policy::REQUEST_ID.to_string(),
            BrowserPolicyUpdateKind::Patch,
            ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason::RevisionNotFound,
            "rejected",
            constants::browser_policy::TEST_SENT_AT,
        );
        let serialized = crate::json_contract::serialize_json_value(serde_json::json!({
            "accepted": accepted.status,
            "rejected": rejected.status,
            "roundtrip_revision": roundtrip.active_revision_id.as_deref(),
        }));
        let encoded = crate::test_invariants::serialize_test_json(&serialized);
        let _: serde_json::Value =
            require_json_decode(&encoded, constants::error::AGENT_EVENT_SERIALIZES);
        let field = ocentra_parent_agent_protocol::logging::LogFieldValue::String(encoded);
        let _ = require_log_string_field(Some(&field), constants::error::AGENT_EVENT_SERIALIZES);

        assert_eq!(revision.revision_id, active.revision_id);
        assert_eq!(
            state.active_revision_id.as_deref(),
            Some(revision_id.as_str())
        );
        assert_eq!(base_revision_matches(&state, Some(&revision_id)), Ok(()));
        assert!(next_revision_id(&state).starts_with(constants::browser_policy::REVISION_PREFIX));
        assert!(next_audit_event_id(&state).starts_with(constants::browser_policy::AUDIT_PREFIX));
        assert_eq!(
            default_revision_id(),
            format!(
                "{}{}",
                constants::browser_policy::REVISION_PREFIX,
                constants::browser_policy::UPDATE_KIND_GET
            )
        );
        assert_eq!(
            preview_revision_id(),
            format!(
                "{}{}",
                constants::browser_policy::REVISION_PREFIX,
                constants::browser_policy::UPDATE_KIND_PREVIEW
            )
        );
        assert_eq!(accepted.status, BrowserPolicyUpdateStatus::Accepted);
        assert_eq!(rejected.status, BrowserPolicyUpdateStatus::Rejected);
        assert_eq!(roundtrip, state);
        assert!(serialized.is_object());
        let _ = browser_policy_store_path_from_env();
    }
}

async fn browser_policy_compiler_roundtrip_helpers(
    policy: &ocentra_parent_agent_protocol::browser_policy::BrowserPolicy,
    effective_policy: &ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyEffectivePolicy,
    state: &BrowserPolicyStoredState,
    revision_id: &str,
    audit_event_id: &str,
) -> Result<
    (
        BrowserPolicyStoredState,
        BrowserPolicyUpdateStatus,
        BrowserPolicyUpdateStatus,
        serde_json::Value,
    ),
    Box<dyn std::error::Error>,
> {
    let store_path = std::env::temp_dir().join(format!(
        "ocentra-browser-policy-compiler-smoke-{}-{}.json",
        std::process::id(),
        revision_id
    ));
    crate::test_invariants::require_ok(
        write_browser_policy_state(&store_path, state).await,
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let roundtrip = crate::test_invariants::require_ok(
        read_browser_policy_state(&store_path).await,
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let accepted = accepted_response(
        constants::browser_policy::REQUEST_ID.to_string(),
        BrowserPolicyUpdateKind::Preview,
        policy.clone(),
        effective_policy.clone(),
        Some(audit_event_id.to_string()),
        "accepted",
        constants::browser_policy::TEST_SENT_AT,
    );
    let rejected = rejected_response(
        constants::browser_policy::REQUEST_ID.to_string(),
        BrowserPolicyUpdateKind::Patch,
        ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason::RevisionNotFound,
        "rejected",
        constants::browser_policy::TEST_SENT_AT,
    );
    let serialized = crate::json_contract::serialize_json_value(serde_json::json!({
        "accepted": accepted.status,
        "rejected": rejected.status,
        "roundtrip_revision": roundtrip.active_revision_id.as_deref(),
    }));
    let _: serde_json::Value = crate::test_invariants::require_json_decode(
        &crate::test_invariants::serialize_test_json(&serialized),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    Ok((roundtrip, accepted.status, rejected.status, serialized))
}

#[tokio::test]
async fn browser_policy_compiler_smoke_uses_runtime_support_store_and_json_helpers() {
    let policy = default_browser_policy_for_test(constants::browser_policy::POLICY_ID.to_string());
    let effective_policy = require_ok(
        crate::browser_policy_compiler::compile_browser_policy(
            &policy,
            constants::browser_policy::REVISION_ID,
            constants::browser_policy::TEST_SENT_AT,
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let revision_id = format!("{}1", constants::browser_policy::REVISION_PREFIX);
    let audit_event_id = format!("{}1", constants::browser_policy::AUDIT_PREFIX);
    let empty_state = BrowserPolicyStoredState::empty();
    let state = BrowserPolicyStoredState {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        active_revision_id: Some(revision_id.clone()),
        revisions: vec![BrowserPolicyRevisionRecord {
            revision_id: revision_id.clone(),
            policy: policy.clone(),
            effective_policy: effective_policy.clone(),
            created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
            audit_event_id: audit_event_id.clone(),
        }],
        audit_events: vec![BrowserPolicyAuditRecord {
            audit_event_id: audit_event_id.clone(),
            request_id: constants::browser_policy::REQUEST_ID.to_string(),
            kind: BrowserPolicyUpdateKind::Preview,
            revision_id: revision_id.clone(),
            created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
        }],
    };
    let revision = require_some(
        state.revision_by_id(&revision_id),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let active = require_some(
        state.active_revision(),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let (roundtrip, accepted_status, rejected_status, serialized) =
        crate::test_invariants::require_ok(
            browser_policy_compiler_roundtrip_helpers(
                &policy,
                &effective_policy,
                &state,
                &revision_id,
                &audit_event_id,
            )
            .await,
            constants::error::AGENT_EVENT_SERIALIZES,
        );
    let _: crate::browser_policy_store::BrowserPolicyStoreError =
        crate::browser_policy_store::BrowserPolicyStoreError::Unavailable;

    assert_eq!(revision.revision_id, active.revision_id);
    assert_eq!(
        state.active_revision_id.as_deref(),
        Some(revision_id.as_str())
    );
    assert_eq!(base_revision_matches(&state, Some(&revision_id)), Ok(()));
    assert!(next_revision_id(&state).starts_with(constants::browser_policy::REVISION_PREFIX));
    assert!(next_audit_event_id(&state).starts_with(constants::browser_policy::AUDIT_PREFIX));
    assert_eq!(
        default_revision_id(),
        format!(
            "{}{}",
            constants::browser_policy::REVISION_PREFIX,
            constants::browser_policy::UPDATE_KIND_GET
        )
    );
    assert_eq!(
        preview_revision_id(),
        format!(
            "{}{}",
            constants::browser_policy::REVISION_PREFIX,
            constants::browser_policy::UPDATE_KIND_PREVIEW
        )
    );
    assert_eq!(accepted_status, BrowserPolicyUpdateStatus::Accepted);
    assert_eq!(rejected_status, BrowserPolicyUpdateStatus::Rejected);
    assert_eq!(roundtrip, state);
    assert!(serialized.is_object());
    let _ = browser_policy_store_path_from_env();
    assert!(empty_state.active_revision().is_none());
    assert!(empty_state.revision_by_id(&revision_id).is_none());
}
