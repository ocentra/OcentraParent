use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyValue;

use super::{persist, read_state};
use crate::browser_policy_runtime::BrowserPolicyRuntime;
use crate::browser_policy_runtime_support::{
    base_revision_matches, BrowserPolicyRequestId, BrowserPolicyRevisionId, BrowserPolicyTimestamp,
};
use crate::time::timestamp_now;

pub(crate) async fn handle_replace(
    runtime: &BrowserPolicyRuntime,
    request_id: BrowserPolicyRequestId,
    base_revision_id: Option<BrowserPolicyRevisionId>,
    policy: BrowserPolicyValue,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    let generated_at = BrowserPolicyTimestamp(timestamp_now());
    let state = match read_state(runtime).await {
        Ok(state) => state,
        Err(_) => {
            return persist::rejected_storage_unavailable(
                request_id,
                BrowserPolicyUpdateKind::Replace,
                generated_at,
            );
        }
    };
    if let Err(reason) = base_revision_matches(&state, base_revision_id.as_ref()) {
        return persist::rejected_base_revision(
            request_id,
            BrowserPolicyUpdateKind::Replace,
            reason,
            generated_at,
        );
    }
    persist::persist_revision(
        runtime,
        state,
        request_id,
        BrowserPolicyUpdateKind::Replace,
        policy,
        generated_at,
    )
    .await
}
