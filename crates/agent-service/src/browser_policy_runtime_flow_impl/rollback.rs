use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::constants;

use super::{persist, read_state};
use crate::browser_policy_runtime::BrowserPolicyRuntime;
use crate::browser_policy_runtime_support::{
    next_audit_event_id, BrowserPolicyMessage, BrowserPolicyPolicyId, BrowserPolicyRequestId,
    BrowserPolicyRevisionId, BrowserPolicyTimestamp,
};
use crate::time::timestamp_now;

pub(crate) async fn handle_rollback(
    runtime: &BrowserPolicyRuntime,
    request_id: BrowserPolicyRequestId,
    policy_id: BrowserPolicyPolicyId,
    target_revision_id: BrowserPolicyRevisionId,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    let generated_at = BrowserPolicyTimestamp(timestamp_now());
    let mut state = match read_state(runtime).await {
        Ok(state) => state,
        Err(_) => {
            return persist::rejected_storage_unavailable(
                request_id,
                BrowserPolicyUpdateKind::Rollback,
                generated_at,
            );
        }
    };
    let Some(target) = state.revision_by_id(&target_revision_id).cloned() else {
        return persist::rejected_revision_not_found(
            request_id,
            BrowserPolicyUpdateKind::Rollback,
            generated_at,
        );
    };
    if target.policy.policy_id != policy_id.0 {
        return persist::rejected_revision_not_found(
            request_id,
            BrowserPolicyUpdateKind::Rollback,
            generated_at,
        );
    }
    let audit_event_id = next_audit_event_id(&state);
    state.active_revision_id = Some(target.revision_id.clone());
    state
        .audit_events
        .push(crate::browser_policy_store::BrowserPolicyAuditRecord {
            audit_event_id: audit_event_id.0.clone(),
            request_id: request_id.0.clone(),
            kind: BrowserPolicyUpdateKind::Rollback,
            revision_id: target.revision_id.clone(),
            created_at: generated_at.0.clone(),
        });
    if super::write_state(runtime, &state).await.is_err() {
        return persist::rejected_storage_unavailable(
            request_id,
            BrowserPolicyUpdateKind::Rollback,
            generated_at,
        );
    }
    crate::browser_policy_runtime_support::accepted_response(
        request_id,
        BrowserPolicyUpdateKind::Rollback,
        target.policy,
        target.effective_policy,
        Some(audit_event_id),
        BrowserPolicyMessage(constants::browser_policy::MESSAGE_ROLLBACK_ACCEPTED),
        generated_at,
    )
}
