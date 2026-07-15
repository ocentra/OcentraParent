use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyPatch;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;

use super::{persist, read_state};
use crate::browser_policy_request::apply_browser_policy_patches;
use crate::browser_policy_runtime::BrowserPolicyRuntime;
use crate::browser_policy_runtime_support::{
    BrowserPolicyPolicyId, BrowserPolicyRequestId, BrowserPolicyRevisionId, BrowserPolicyTimestamp,
};
use crate::time::timestamp_now;

pub(crate) async fn handle_patch(
    runtime: &BrowserPolicyRuntime,
    request_id: BrowserPolicyRequestId,
    policy_id: BrowserPolicyPolicyId,
    base_revision_id: BrowserPolicyRevisionId,
    patches: Vec<BrowserPolicyPatch>,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    let generated_at = BrowserPolicyTimestamp(timestamp_now());
    let state = match read_state(runtime).await {
        Ok(state) => state,
        Err(_) => {
            return persist::rejected_storage_unavailable(
                request_id,
                BrowserPolicyUpdateKind::Patch,
                generated_at,
            );
        }
    };
    let Some(active) = state.active_revision() else {
        return persist::rejected_revision_not_found(
            request_id,
            BrowserPolicyUpdateKind::Patch,
            generated_at,
        );
    };
    if active.revision_id != base_revision_id.0 || active.policy.policy_id != policy_id.0 {
        return persist::rejected_stale_revision(
            request_id,
            BrowserPolicyUpdateKind::Patch,
            generated_at,
        );
    }
    let policy = match apply_browser_policy_patches(active.policy.clone(), &patches) {
        Ok(policy) => policy,
        Err(reason) => {
            return persist::rejected_invalid_policy(
                request_id,
                BrowserPolicyUpdateKind::Patch,
                reason,
                generated_at,
            );
        }
    };
    persist::persist_revision(
        runtime,
        state,
        request_id,
        BrowserPolicyUpdateKind::Patch,
        policy,
        generated_at,
    )
    .await
}
