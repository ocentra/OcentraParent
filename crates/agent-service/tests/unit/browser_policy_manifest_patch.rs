#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/test_text.rs"]
mod test_text;

use crate::browser_policy_compiler::compile_browser_policy;
use crate::browser_policy_runtime_support::{
    base_revision_matches, default_revision_id, next_audit_event_id, next_revision_id,
    preview_revision_id,
};
use crate::browser_policy_store::{
    browser_policy_store_path_from_env, BrowserPolicyRevisionRecord, BrowserPolicyStoredState,
};
use crate::test_invariants::{require_ok, require_some};
use crate::test_support::default_browser_policy_for_test;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::policy_constants as policy;

#[path = "../support/browser_policy_test_support.rs"]
pub mod test_support;

#[path = "../../src/browser_policy_compiler_assessment.rs"]
mod browser_policy_compiler_assessment;
#[path = "../../src/browser_policy_compiler.rs"]
mod browser_policy_compiler_impl;
#[path = "browser_policy_manifest_patch_tests.rs"]
mod browser_policy_manifest_patch_tests;
#[path = "../../src/browser_policy_runtime_support.rs"]
mod browser_policy_runtime_support;
#[path = "../../src/browser_policy_store.rs"]
mod browser_policy_store;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../support/test_invariants.rs"]
mod test_invariants;

mod browser_policy_compiler {
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
        super::browser_policy_compiler_impl::browser_policy_capability_registry(
            super::browser_policy_compiler_impl::BrowserPolicyCapabilityRegistryRequest {
                generated_at: request.generated_at,
            },
        )
    }
}

#[cfg(test)]
mod clippy_linkage {
    use crate::browser_policy_runtime_support::{
        base_revision_matches, default_revision_id, next_audit_event_id, next_revision_id,
        preview_revision_id,
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
    fn browser_policy_manifest_helpers_are_linked() {
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
        let revision_id = format!("{}1", constants::browser_policy::REVISION_PREFIX);
        let audit_event_id = format!("{}1", constants::browser_policy::AUDIT_PREFIX);
        let state = BrowserPolicyStoredState {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            active_revision_id: Some(revision_id.clone()),
            revisions: vec![BrowserPolicyRevisionRecord {
                revision_id: revision_id.clone(),
                policy,
                effective_policy,
                created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
                audit_event_id: audit_event_id.clone(),
            }],
            audit_events: vec![crate::browser_policy_store::BrowserPolicyAuditRecord {
                audit_event_id,
                request_id: constants::browser_policy::REQUEST_ID.to_string(),
                kind: BrowserPolicyUpdateKind::Patch,
                revision_id: revision_id.clone(),
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
            Some(revision_id.as_str())
        );
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
        require_ok(
            base_revision_matches(&state, Some(&revision_id)),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let _ = browser_policy_store_path_from_env();
        let _ = crate::json_contract::serialize_json_value(serde_json::json!({
            "active": state.active_revision_id,
        }));
    }
}

#[test]
fn browser_policy_manifest_patch_smoke_uses_store_state_helpers() {
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
    let revision_id = format!("{}1", constants::browser_policy::REVISION_PREFIX);
    let audit_event_id = format!("{}1", constants::browser_policy::AUDIT_PREFIX);
    let state = BrowserPolicyStoredState {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        active_revision_id: Some(revision_id.clone()),
        revisions: vec![BrowserPolicyRevisionRecord {
            revision_id: revision_id.clone(),
            policy,
            effective_policy,
            created_at: constants::browser_policy::TEST_SENT_AT.to_string(),
            audit_event_id: audit_event_id.clone(),
        }],
        audit_events: vec![crate::browser_policy_store::BrowserPolicyAuditRecord {
            audit_event_id,
            request_id: constants::browser_policy::REQUEST_ID.to_string(),
            kind: BrowserPolicyUpdateKind::Patch,
            revision_id: revision_id.clone(),
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
        Some(revision_id.as_str())
    );
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
    require_ok(
        base_revision_matches(&state, Some(&revision_id)),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let _ = browser_policy_store_path_from_env();
}
