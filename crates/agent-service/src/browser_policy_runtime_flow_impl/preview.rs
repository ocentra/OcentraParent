use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyValue;
use ocentra_parent_agent_protocol::constants;

use crate::browser_policy_compiler::compile_browser_policy;
use crate::browser_policy_runtime_support::{
    accepted_response, preview_revision_id, rejected_response, BrowserPolicyMessage,
    BrowserPolicyRequestId, BrowserPolicyTimestamp,
};
use crate::time::timestamp_now;

pub(crate) async fn handle_preview(
    request_id: BrowserPolicyRequestId,
    policy: BrowserPolicyValue,
) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
    let generated_at = BrowserPolicyTimestamp(timestamp_now());
    let revision_id = preview_revision_id();
    match compile_browser_policy(
        &policy,
        crate::browser_policy_compiler::BrowserPolicyCompileRequest {
            revision_id: &revision_id.0,
            compiled_at: &generated_at.0,
        },
    ) {
        Ok(effective_policy) => accepted_response(
            request_id,
            BrowserPolicyUpdateKind::Preview,
            policy,
            effective_policy,
            None,
            BrowserPolicyMessage(constants::browser_policy::MESSAGE_PREVIEWED),
            generated_at,
        ),
        Err(reason) => rejected_response(
            request_id,
            BrowserPolicyUpdateKind::Preview,
            reason,
            BrowserPolicyMessage(constants::browser_policy::MESSAGE_INVALID_POLICY),
            generated_at,
        ),
    }
}
