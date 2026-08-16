use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::constants;

use super::read_state;
use crate::browser_policy_compiler::compile_browser_policy;
use crate::browser_policy_runtime::BrowserPolicyRuntime;
use crate::browser_policy_runtime_support::{
    accepted_response, default_policy, default_revision_id, rejected_response,
    BrowserPolicyMessage, BrowserPolicyPolicyId, BrowserPolicyRequestId, BrowserPolicyTimestamp,
};
use crate::time::timestamp_now;

pub(crate) async fn handle_get(
    runtime: &BrowserPolicyRuntime,
    request_id: BrowserPolicyRequestId,
    policy_id: BrowserPolicyPolicyId,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    let generated_at = BrowserPolicyTimestamp(timestamp_now());
    let state = match read_state(runtime).await {
        Ok(state) => state,
        Err(_) => {
            return rejected_response(
                request_id,
                BrowserPolicyUpdateKind::Get,
                BrowserPolicyRejectionReason::StorageUnavailable,
                BrowserPolicyMessage(constants::browser_policy::MESSAGE_STORAGE_UNAVAILABLE),
                generated_at,
            );
        }
    };
    if let Some(revision) = state.active_revision() {
        if revision.policy.policy_id == policy_id.0 {
            return accepted_response(
                request_id,
                BrowserPolicyUpdateKind::Get,
                revision.policy.clone(),
                revision.effective_policy.clone(),
                None,
                BrowserPolicyMessage(constants::browser_policy::MESSAGE_REPORTED),
                generated_at,
            );
        }
    }
    let revision_id = default_revision_id();
    let policy = default_policy(policy_id);
    match compile_browser_policy(
        &policy,
        crate::browser_policy_compiler::BrowserPolicyCompileRequest {
            revision_id: &revision_id.0,
            compiled_at: &generated_at.0,
        },
    ) {
        Ok(effective_policy) => accepted_response(
            request_id,
            BrowserPolicyUpdateKind::Get,
            policy,
            effective_policy,
            None,
            BrowserPolicyMessage(constants::browser_policy::MESSAGE_REPORTED),
            generated_at,
        ),
        Err(reason) => rejected_response(
            request_id,
            BrowserPolicyUpdateKind::Get,
            reason,
            BrowserPolicyMessage(constants::browser_policy::MESSAGE_INVALID_POLICY),
            generated_at,
        ),
    }
}
