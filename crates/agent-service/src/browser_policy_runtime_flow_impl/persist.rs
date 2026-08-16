use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyValue;
use ocentra_parent_agent_protocol::constants;

use crate::browser_policy_compiler::compile_browser_policy;
use crate::browser_policy_runtime::BrowserPolicyRuntime;
use crate::browser_policy_runtime_support::{
    accepted_response, next_audit_event_id, next_revision_id, rejected_response,
    BrowserPolicyMessage, BrowserPolicyRequestId, BrowserPolicyTimestamp,
};
use crate::browser_policy_store::{
    BrowserPolicyAuditRecord, BrowserPolicyRevisionRecord, BrowserPolicyStoreError,
    BrowserPolicyStoredState,
};

pub(crate) async fn persist_revision(
    runtime: &BrowserPolicyRuntime,
    mut state: BrowserPolicyStoredState,
    request_id: BrowserPolicyRequestId,
    kind: BrowserPolicyUpdateKind,
    policy: BrowserPolicyValue,
    generated_at: BrowserPolicyTimestamp,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    let revision_id = next_revision_id(&state);
    let audit_event_id = next_audit_event_id(&state);
    let effective_policy = match compile_browser_policy(
        &policy,
        crate::browser_policy_compiler::BrowserPolicyCompileRequest {
            revision_id: &revision_id.0,
            compiled_at: &generated_at.0,
        },
    ) {
        Ok(effective_policy) => effective_policy,
        Err(reason) => return rejected_invalid_policy(request_id, kind, reason, generated_at),
    };
    state.active_revision_id = Some(revision_id.0.clone());
    state.revisions.push(BrowserPolicyRevisionRecord {
        revision_id: revision_id.0.clone(),
        policy: policy.clone(),
        effective_policy: effective_policy.clone(),
        created_at: generated_at.0.clone(),
        audit_event_id: audit_event_id.0.clone(),
    });
    state.audit_events.push(BrowserPolicyAuditRecord {
        audit_event_id: audit_event_id.0.clone(),
        request_id: request_id.0.clone(),
        kind,
        revision_id: revision_id.0,
        created_at: generated_at.0.clone(),
    });
    if write_state(runtime, &state).await.is_err() {
        return rejected_storage_unavailable(request_id, kind, generated_at);
    }
    accepted_response(
        request_id,
        kind,
        policy,
        effective_policy,
        Some(audit_event_id),
        BrowserPolicyMessage(constants::browser_policy::MESSAGE_ACCEPTED),
        generated_at,
    )
}

pub(crate) fn rejected_storage_unavailable(
    request_id: BrowserPolicyRequestId,
    kind: BrowserPolicyUpdateKind,
    generated_at: BrowserPolicyTimestamp,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    rejected_response(
        request_id,
        kind,
        BrowserPolicyRejectionReason::StorageUnavailable,
        BrowserPolicyMessage(constants::browser_policy::MESSAGE_STORAGE_UNAVAILABLE),
        generated_at,
    )
}

pub(crate) fn rejected_stale_revision(
    request_id: BrowserPolicyRequestId,
    kind: BrowserPolicyUpdateKind,
    generated_at: BrowserPolicyTimestamp,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    rejected_response(
        request_id,
        kind,
        BrowserPolicyRejectionReason::StaleRevision,
        BrowserPolicyMessage(constants::browser_policy::MESSAGE_STALE_REVISION),
        generated_at,
    )
}

pub(crate) fn rejected_revision_not_found(
    request_id: BrowserPolicyRequestId,
    kind: BrowserPolicyUpdateKind,
    generated_at: BrowserPolicyTimestamp,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    rejected_response(
        request_id,
        kind,
        BrowserPolicyRejectionReason::RevisionNotFound,
        BrowserPolicyMessage(constants::browser_policy::MESSAGE_REVISION_NOT_FOUND),
        generated_at,
    )
}

pub(crate) fn rejected_invalid_policy(
    request_id: BrowserPolicyRequestId,
    kind: BrowserPolicyUpdateKind,
    reason: BrowserPolicyRejectionReason,
    generated_at: BrowserPolicyTimestamp,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    rejected_response(
        request_id,
        kind,
        reason,
        BrowserPolicyMessage(constants::browser_policy::MESSAGE_INVALID_POLICY),
        generated_at,
    )
}

pub(crate) fn rejected_base_revision(
    request_id: BrowserPolicyRequestId,
    kind: BrowserPolicyUpdateKind,
    reason: BrowserPolicyRejectionReason,
    generated_at: BrowserPolicyTimestamp,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    rejected_response(
        request_id,
        kind,
        reason,
        BrowserPolicyMessage(match reason {
            BrowserPolicyRejectionReason::RevisionNotFound => {
                constants::browser_policy::MESSAGE_REVISION_NOT_FOUND
            }
            BrowserPolicyRejectionReason::StaleRevision => {
                constants::browser_policy::MESSAGE_STALE_REVISION
            }
            _ => constants::browser_policy::MESSAGE_STALE_REVISION,
        }),
        generated_at,
    )
}

async fn write_state(
    runtime: &BrowserPolicyRuntime,
    state: &BrowserPolicyStoredState,
) -> Result<(), BrowserPolicyStoreError> {
    match &runtime.persistence {
        crate::browser_policy_runtime::BrowserPolicyPersistence::LocalJson(path) => {
            crate::browser_policy_store::write_browser_policy_state(path, state).await
        }
    }
}
