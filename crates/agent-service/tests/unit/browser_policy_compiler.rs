#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

use std::sync::atomic::{AtomicU64, Ordering};

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

use crate::browser_policy_runtime_support::{
    accepted_response, base_revision_matches, default_revision_id, next_audit_event_id,
    next_revision_id, preview_revision_id, rejected_response, BrowserPolicyAuditEventId,
    BrowserPolicyMessage, BrowserPolicyRequestId, BrowserPolicyRevisionId, BrowserPolicyTimestamp,
};
use crate::browser_policy_store::{
    browser_policy_store_path_from_env, read_browser_policy_state, write_browser_policy_state,
    BrowserPolicyAuditRecord, BrowserPolicyRevisionRecord, BrowserPolicyStoredState,
};
use crate::test_invariants::{require_ok, require_some};
use crate::test_support::default_browser_policy_for_test;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::{
    BrowserPolicyEffectivePolicy, BrowserPolicyUpdateStatus, BrowserPolicyValue,
};

static TEST_POLICY_STORE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[cfg(test)]
mod clippy_linkage {
    use crate::browser_policy_runtime_support::{
        BrowserPolicyAuditEventId, BrowserPolicyRevisionId,
    };
    use crate::browser_policy_store::{
        browser_policy_store_path_from_env, BrowserPolicyAuditRecord, BrowserPolicyRevisionRecord,
        BrowserPolicyStoredState,
    };
    use crate::test_support::default_browser_policy_for_test;
    use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::policy_constants as policy;

    #[tokio::test]
    async fn browser_policy_compiler_helpers_are_linked() -> Result<(), serde_json::Error> {
        let policy = default_browser_policy_for_test(
            crate::test_support::default_browser_policy_id_for_test(),
        );
        let effective_policy = crate::test_invariants::require_ok(
            crate::browser_policy_compiler::compile_browser_policy(
                &policy,
                crate::browser_policy_compiler::BrowserPolicyCompileRequest {
                    revision_id: constants::browser_policy::REVISION_ID,
                    compiled_at: constants::browser_policy::TEST_SENT_AT,
                },
            ),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let capability_registry =
            crate::browser_policy_compiler::browser_policy_capability_registry(
                crate::browser_policy_compiler::BrowserPolicyCapabilityRegistryRequest {
                    generated_at: constants::browser_policy::TEST_SENT_AT,
                },
            );
        let revision_id =
            BrowserPolicyRevisionId(format!("{}1", constants::browser_policy::REVISION_PREFIX));
        let audit_event_id =
            BrowserPolicyAuditEventId(format!("{}1", constants::browser_policy::AUDIT_PREFIX));
        let state = BrowserPolicyStoredState {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            active_revision_id: Some(revision_id.0.clone()),
            revisions: vec![BrowserPolicyRevisionRecord {
                revision_id: revision_id.0.clone(),
                policy: policy.clone(),
                effective_policy: effective_policy.clone(),
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
                audit_event_id: audit_event_id.0.clone(),
            }],
            audit_events: vec![BrowserPolicyAuditRecord {
                audit_event_id: audit_event_id.0.clone(),
                request_id: constants::browser_policy::REQUEST_ID.to_string(),
                kind: BrowserPolicyUpdateKind::Preview,
                revision_id: revision_id.0.clone(),
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
            }],
        };
        let revision = crate::test_invariants::require_some(
            state.revision_by_id(&revision_id),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let active = crate::test_invariants::require_some(
            state.active_revision(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let (roundtrip, accepted_status, rejected_status, serialized) =
            crate::test_invariants::require_ok(
                super::browser_policy_compiler_roundtrip_helpers(
                    &policy,
                    &effective_policy,
                    &state,
                )
                .await,
                constants::error::AGENT_EVENT_SERIALIZES,
            );
        assert_serialized_log_field_is_linked(&serialized)?;
        assert_eq!(
            capability_registry.generated_at,
            constants::browser_policy::TEST_SENT_AT
        );
        assert_eq!(capability_registry.capabilities.len(), 8);

        super::assert_browser_policy_revision_helpers(&state, revision, active);
        super::assert_browser_policy_roundtrip_helpers(
            &roundtrip,
            &state,
            accepted_status,
            rejected_status,
            &serialized,
        );
        let _ = browser_policy_store_path_from_env();

        Ok(())
    }

    fn assert_serialized_log_field_is_linked(
        serialized: &serde_json::Value,
    ) -> Result<(), serde_json::Error> {
        let encoded = crate::json_contract::serialize_json_string(serialized);
        let _: serde_json::Value = crate::test_invariants::require_json_decode(
            &encoded.0,
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let field = ocentra_parent_agent_protocol::logging::LogFieldValue::String(encoded.0);
        let _ = crate::test_invariants::require_log_string_field(
            Some(&field),
            constants::error::AGENT_EVENT_SERIALIZES,
        );

        Ok(())
    }
}

async fn browser_policy_compiler_roundtrip_helpers(
    policy: &BrowserPolicyValue,
    effective_policy: &BrowserPolicyEffectivePolicy,
    state: &BrowserPolicyStoredState,
) -> Result<
    (
        BrowserPolicyStoredState,
        BrowserPolicyUpdateStatus,
        BrowserPolicyUpdateStatus,
        serde_json::Value,
    ),
    Box<dyn std::error::Error>,
> {
    let sequence = TEST_POLICY_STORE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let store_path = std::env::temp_dir().join(format!(
        "ocentra-browser-policy-compiler-smoke-{}-{}-{sequence}.json",
        std::process::id(),
        constants::browser_policy::REVISION_PREFIX
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
        BrowserPolicyRequestId(constants::browser_policy::REQUEST_ID.to_string()),
        BrowserPolicyUpdateKind::Preview,
        policy.clone(),
        effective_policy.clone(),
        Some(BrowserPolicyAuditEventId(format!(
            "{}1",
            constants::browser_policy::AUDIT_PREFIX
        ))),
        BrowserPolicyMessage("accepted"),
        BrowserPolicyTimestamp(constants::browser_policy::TEST_SENT_AT.to_string()),
    );
    let rejected = rejected_response(
        BrowserPolicyRequestId(constants::browser_policy::REQUEST_ID.to_string()),
        BrowserPolicyUpdateKind::Patch,
        ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason::RevisionNotFound,
        BrowserPolicyMessage("rejected"),
        BrowserPolicyTimestamp(constants::browser_policy::TEST_SENT_AT.to_string()),
    );
    let serialized = crate::json_contract::serialize_json_value(serde_json::json!({
        "accepted": accepted.status,
        "rejected": rejected.status,
        "roundtrip_revision": roundtrip.active_revision_id.as_deref(),
    }));
    let json_text = crate::json_contract::serialize_json_string(&serialized);
    let _: serde_json::Value = serde_json::from_str(&json_text.0)?;

    Ok((roundtrip, accepted.status, rejected.status, serialized))
}

fn assert_browser_policy_revision_helpers(
    state: &BrowserPolicyStoredState,
    revision: &BrowserPolicyRevisionRecord,
    active: &BrowserPolicyRevisionRecord,
) {
    assert_eq!(revision.revision_id, active.revision_id);
    assert_eq!(
        state.active_revision_id.as_deref(),
        Some(active.revision_id.as_str())
    );
    assert_eq!(
        base_revision_matches(
            state,
            Some(&BrowserPolicyRevisionId(active.revision_id.clone())),
        ),
        Ok(())
    );
    assert!(next_revision_id(state)
        .0
        .starts_with(constants::browser_policy::REVISION_PREFIX));
    assert!(next_audit_event_id(state)
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
}

fn assert_browser_policy_roundtrip_helpers(
    roundtrip: &BrowserPolicyStoredState,
    state: &BrowserPolicyStoredState,
    accepted_status: BrowserPolicyUpdateStatus,
    rejected_status: BrowserPolicyUpdateStatus,
    serialized: &serde_json::Value,
) {
    assert_eq!(accepted_status, BrowserPolicyUpdateStatus::Accepted);
    assert_eq!(rejected_status, BrowserPolicyUpdateStatus::Rejected);
    assert_eq!(roundtrip, state);
    assert!(serialized.is_object());
}

#[tokio::test]
async fn browser_policy_compiler_smoke_uses_runtime_support_store_and_json_helpers() {
    let policy =
        default_browser_policy_for_test(crate::test_support::default_browser_policy_id_for_test());
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
    let audit_event_id =
        BrowserPolicyAuditEventId(format!("{}1", constants::browser_policy::AUDIT_PREFIX));
    let empty_state = BrowserPolicyStoredState::empty();
    let state = BrowserPolicyStoredState {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        active_revision_id: Some(revision_id.0.clone()),
        revisions: vec![BrowserPolicyRevisionRecord {
            revision_id: revision_id.0.clone(),
            policy: policy.clone(),
            effective_policy: effective_policy.clone(),
            created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
            audit_event_id: audit_event_id.0.clone(),
        }],
        audit_events: vec![BrowserPolicyAuditRecord {
            audit_event_id: audit_event_id.0.clone(),
            request_id: constants::browser_policy::REQUEST_ID.to_string(),
            kind: BrowserPolicyUpdateKind::Preview,
            revision_id: revision_id.0.clone(),
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
            browser_policy_compiler_roundtrip_helpers(&policy, &effective_policy, &state).await,
            constants::error::AGENT_EVENT_SERIALIZES,
        );
    let _: crate::browser_policy_store::BrowserPolicyStoreError =
        crate::browser_policy_store::BrowserPolicyStoreError::Unavailable;

    assert_browser_policy_revision_helpers(&state, revision, active);
    assert_browser_policy_roundtrip_helpers(
        &roundtrip,
        &state,
        accepted_status,
        rejected_status,
        &serialized,
    );
    let _ = browser_policy_store_path_from_env();
    assert!(empty_state.active_revision().is_none());
    assert!(empty_state.revision_by_id(&revision_id).is_none());
}
