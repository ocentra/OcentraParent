use std::time::Duration;

use serde_json::json;

use super::common::events::responses::{
    policy_preview_confirmed_response_event,
    policy_request_parent_resolution_resolved_response_event,
};
use super::tests_support::{
    require_ok, start_local_server_with_capture_responses, with_agent_addr,
};
use super::{dispatch_parent_ui_action, ParentRouteId, ParentUiAction, ParentUiActionKind};

#[test]
fn policy_parent_resolution_action_dispatches_rust_owned_command_and_reloads_snapshot() {
    let request_payload = json!({
        "policyRequestParentResolutionRequest": "{\"schemaVersion\":1,\"commandId\":\"cmd-policy-request-resolution-1\",\"approvalAuditReferenceId\":\"audit.policy-request.confirmed\",\"decision\":\"grant\"}"
    });
    let (address, capture) = start_local_server_with_capture_responses(vec![
        policy_request_parent_resolution_resolved_response_event(),
        policy_preview_confirmed_response_event(),
    ]);
    let action = ParentUiAction {
        action: ParentUiActionKind::PolicyRequestParentResolutionRequested,
        route: ParentRouteId::Approvals,
        command: None,
        payload: request_payload.clone(),
        context: None,
    };
    let result = with_agent_addr(&address, || dispatch_parent_ui_action(&action));
    let resolution_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured policy parent resolution command arrives",
    );
    let _policy_preview_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured policy preview reload after parent resolution",
    );

    assert!(result.accepted);
    assert_eq!(
        result.message,
        "parent Rust facade requested parent policy request resolution"
    );
    assert_eq!(
        resolution_request.command["command"],
        json!("agent.policy.request.parent-resolution.resolve")
    );
    assert_eq!(
        resolution_request.command["payload"]["policyRequestParentResolutionRequest"],
        request_payload["policyRequestParentResolutionRequest"]
    );
}
