use std::time::Duration;

use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision,
};
use ocentra_parent_agent_protocol::transport::{
    AgentEventEnvelope, AgentEventName, PolicyRequestAssistantPreviewConfirmAction,
    PolicyRequestAssistantPreviewConfirmActorRole, PolicyRequestAssistantPreviewConfirmActorState,
    PolicyRequestAssistantPreviewConfirmRequest, PolicyRequestAssistantPreviewConfirmRequestKind,
    PolicyRequestAssistantPreviewConfirmTargetKind, PolicyRequestParentResolutionDecision,
    PolicyRequestParentResolutionRequest,
};
use ocentra_parent_agent_protocol::{constants, AGENT_PROTOCOL_SCHEMA_VERSION};
use ocentra_schema::parent_ui_bridge::ParentPolicyPreviewConfirmationContext;
use serde_json::json;

use super::common::events::responses::{
    policy_preview_confirmed_response_event, policy_preview_response_event,
    policy_request_assistant_preview_confirmed_response_event,
    policy_request_parent_resolution_resolved_response_event,
};
use super::common::helpers::{require_result_live_activity, require_some, TestContext};
use super::tests_support::{
    lan_event, require_ok, sample_lan_read_model, start_local_server_with_capture_responses,
    with_agent_addr, CapturedLanRequest,
};
use super::{dispatch_parent_ui_action, ParentRouteId, ParentUiAction, ParentUiActionKind};

#[test]
fn policy_preview_confirm_action_consumes_staged_handle_and_relays_typed_request() {
    let confirm_payload = stage_policy_preview_draft();
    let (address, capture) = start_local_server_with_capture_responses(vec![
        active_controller_status_event(),
        policy_preview_with_confirmation_context(),
        policy_request_assistant_preview_confirmed_response_event(),
        policy_preview_confirmed_response_event(),
    ]);
    let action = ParentUiAction {
        action: ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested,
        route: ParentRouteId::PolicyNetwork,
        command: None,
        payload: confirm_payload,
        context: None,
    };

    let result = with_agent_addr(&address, || dispatch_parent_ui_action(&action));
    let requests = capture_requests(
        &capture,
        &[
            "agent.lan-pairing.status.get",
            "agent.policy.preview.read-model.get",
            "agent.policy.request.assistant-preview.confirm",
            "agent.policy.preview.read-model.get",
        ],
    );
    let request = parse_confirm_request(require_some(
        requests.get(2),
        TestContext("typed policy preview confirm request was captured"),
    ));

    assert!(result.accepted);
    assert_eq!(
        result.message,
        "parent Rust facade relayed the typed policy preview confirmation"
    );
    assert_confirm_request(&request);
    assert_confirmed_preview_snapshot(&result);
}

#[test]
fn policy_parent_resolution_action_builds_request_from_controller_and_preview_authority() {
    let request_payload = json!({
        "policyRequestParentResolutionRequest": { "decision": "grant" }
    });
    let (address, capture) = start_local_server_with_capture_responses(vec![
        active_controller_status_event(),
        policy_preview_with_confirmation_context(),
        policy_request_parent_resolution_resolved_response_event(),
        policy_preview_confirmed_response_event(),
    ]);
    let action = ParentUiAction {
        action: ParentUiActionKind::PolicyRequestParentResolutionRequested,
        route: ParentRouteId::PolicyNetwork,
        command: None,
        payload: request_payload,
        context: None,
    };

    let result = with_agent_addr(&address, || dispatch_parent_ui_action(&action));
    let requests = capture_requests(
        &capture,
        &[
            "agent.lan-pairing.status.get",
            "agent.policy.preview.read-model.get",
            "agent.policy.request.parent-resolution.resolve",
            "agent.policy.preview.read-model.get",
        ],
    );
    let request = parse_resolution_request(require_some(
        requests.get(2),
        TestContext("typed parent resolution request was captured"),
    ));

    assert!(result.accepted);
    assert_eq!(
        result.message,
        "parent Rust facade relayed the typed parent resolution"
    );
    assert_resolution_request(&request);
}

fn stage_policy_preview_draft() -> serde_json::Value {
    let payload = json!({
        "policyPreviewAuthoringDraft": json!({
            "targetValue": "example.test",
            "requestedAction": "block"
        })
        .to_string()
    });
    let (address, capture) = start_local_server_with_capture_responses(vec![
        active_controller_status_event(),
        policy_preview_with_confirmation_context(),
        policy_preview_with_confirmation_context(),
    ]);
    let action = ParentUiAction {
        action: ParentUiActionKind::PolicyPreviewAuthoringDraftStaged,
        route: ParentRouteId::PolicyNetwork,
        command: None,
        payload,
        context: None,
    };
    let result = with_agent_addr(&address, || dispatch_parent_ui_action(&action));
    capture_requests(
        &capture,
        &[
            "agent.lan-pairing.status.get",
            "agent.policy.preview.read-model.get",
            "agent.policy.preview.read-model.get",
        ],
    );

    assert!(result.accepted);
    assert_eq!(
        result.message,
        "parent Rust facade staged a bounded policy preview draft"
    );
    let authoring = require_some(
        require_some(
            require_some(
                require_some(
                    result.snapshot,
                    TestContext("staged policy preview returns a route snapshot"),
                )
                .live_activity,
                TestContext("staged policy preview returns live activity"),
            )
            .policy_preview_panel,
            TestContext("staged policy preview returns its panel"),
        )
        .authoring,
        TestContext("staged policy preview returns authoring state"),
    );
    let confirm_action = require_some(
        authoring.confirm_action,
        TestContext("staged policy preview exposes a confirm action"),
    );
    let confirm_payload = require_some(
        confirm_action.payload,
        TestContext("staged policy preview exposes a bounded opaque handle payload"),
    );
    let handle = require_some(
        confirm_payload
            .get("policyPreviewAuthoringHandle")
            .and_then(serde_json::Value::as_str),
        TestContext("staged policy preview handle payload is a string"),
    );
    assert_opaque_handle(handle, "ppah-");
    json!({ "policyPreviewAuthoringHandle": handle })
}

fn active_controller_status_event() -> AgentEventEnvelope {
    let mut read_model = sample_lan_read_model();
    read_model
        .household_device_decisions
        .push(LanHouseholdDeviceDecision {
            schema_version: 1,
            action_id: "household-device-decision-1".to_string(),
            action_kind: LanHouseholdDeviceActionKind::Trust,
            canonical_device_id: "network-neighbor-1".to_string(),
            child_profile_id: Some("child-profile-1".to_string()),
            display_name: Some("Study Laptop".to_string()),
            device_kind: Some("windows".to_string()),
            parent_actor_id: "parent-1".to_string(),
            decided_at: "2026-06-18T00:04:00Z".to_string(),
            revoked_at: None,
        });
    lan_event(AgentEventName::AgentLanPairingStatusReported, &read_model)
}

fn policy_preview_with_confirmation_context() -> AgentEventEnvelope {
    let context = ParentPolicyPreviewConfirmationContext {
        request_id: Some("policy-request-1".to_string()),
        submission_key: Some("policy-request-submission-1".to_string()),
        household_id: Some("household-1".to_string()),
        child_profile_id: Some("child-profile-1".to_string()),
        device_id: Some("device-1".to_string()),
        source_document_id: Some("source-document-1".to_string()),
        policy_version: Some(7),
        target_reference_id: Some("example.test".to_string()),
        rule_id: Some("rule-1".to_string()),
        requested_at: Some("2026-06-18T00:05:00Z".to_string()),
        expires_at: Some("2026-06-18T00:20:00Z".to_string()),
        assistant_preview_id: Some("policy-preview.network.1".to_string()),
        audit_reference_ids: Some("audit.policy-request.confirmed".to_string()),
        actor_id: Some("parent-1".to_string()),
        actor_role: Some("parent".to_string()),
        actor_state: Some("active".to_string()),
        confirmation_audit_reference_id: Some("audit.policy-request.confirmed".to_string()),
    };
    let mut event = policy_preview_response_event();
    event.payload.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String("site".to_string()),
    );
    event.payload.insert(
        constants::field::POLICY_APPROVAL_ID.to_string(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String(
            "policy-approval-1".to_string(),
        ),
    );
    event.payload.insert(
        constants::field::POLICY_PREVIEW_CONFIRMATION_CONTEXT.to_string(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String(require_ok(
            serde_json::to_string(&context),
            "policy preview confirmation context serializes",
        )),
    );
    event
}

fn capture_requests(
    capture: &std::sync::mpsc::Receiver<CapturedLanRequest>,
    expected_commands: &[&str],
) -> Vec<CapturedLanRequest> {
    expected_commands
        .iter()
        .map(|expected_command| {
            let request = require_ok(
                capture.recv_timeout(Duration::from_secs(1)),
                "captured policy lifecycle request arrives",
            );
            assert_eq!(request.command["command"], json!(expected_command));
            request
        })
        .collect()
}

fn parse_confirm_request(
    request: &CapturedLanRequest,
) -> PolicyRequestAssistantPreviewConfirmRequest {
    let text = require_some(
        request.command["payload"]["policyRequestAssistantPreviewConfirmRequest"].as_str(),
        TestContext("policy preview confirm payload is a serialized typed request"),
    );
    require_ok(
        serde_json::from_str(text),
        "policy preview confirm payload parses through the Rust protocol contract",
    )
}

fn parse_resolution_request(request: &CapturedLanRequest) -> PolicyRequestParentResolutionRequest {
    let text = require_some(
        request.command["payload"]["policyRequestParentResolutionRequest"].as_str(),
        TestContext("parent resolution payload is a serialized typed request"),
    );
    require_ok(
        serde_json::from_str(text),
        "parent resolution payload parses through the Rust protocol contract",
    )
}

fn assert_confirm_request(request: &PolicyRequestAssistantPreviewConfirmRequest) {
    assert_opaque_command_id(&request.command_id, "policy-preview-confirm-ppah-");
    assert_eq!(request.schema_version, AGENT_PROTOCOL_SCHEMA_VERSION);
    assert_eq!(request.request_id, "policy-request-1");
    assert_eq!(request.submission_key, "policy-request-submission-1");
    assert_eq!(request.household_id, "household-1");
    assert_eq!(request.child_profile_id, "child-profile-1");
    assert_eq!(request.device_id.as_deref(), Some("device-1"));
    assert_eq!(request.source_document_id, "source-document-1");
    assert_eq!(request.policy_version, 7);
    assert_eq!(
        request.request_kind,
        PolicyRequestAssistantPreviewConfirmRequestKind::AskParent
    );
    assert_eq!(
        request.target_kind,
        PolicyRequestAssistantPreviewConfirmTargetKind::Site
    );
    assert_eq!(request.target_reference_id, "example.test");
    assert_eq!(
        request.requested_action,
        PolicyRequestAssistantPreviewConfirmAction::Block
    );
    assert_eq!(request.rule_id.as_deref(), Some("rule-1"));
    assert_eq!(request.requested_bonus_minutes, None);
    assert_eq!(request.requested_at, "2026-06-18T00:05:00Z");
    assert_eq!(request.expires_at, "2026-06-18T00:20:00Z");
    assert_eq!(request.origin, PolicyRequestOrigin::AssistantDraft);
    assert_eq!(request.assistant_preview_id, "policy-preview.network.1");
    assert_eq!(
        request.assistant_confirmation_state,
        PolicyAssistantConfirmationState::ParentConfirmationRequired
    );
    assert_eq!(request.request_status, PolicyRequestStatus::PreviewOnly);
    assert_eq!(
        request.audit_reference_ids,
        vec!["audit.policy-request.confirmed"]
    );
    assert_eq!(request.confirmation_actor_id, "parent-1");
    assert_eq!(
        request.confirmation_actor_role,
        PolicyRequestAssistantPreviewConfirmActorRole::Parent
    );
    assert_eq!(
        request.confirmation_actor_state,
        PolicyRequestAssistantPreviewConfirmActorState::Active
    );
    assert_eq!(
        request.confirmation_audit_reference_id,
        "audit.policy-request.confirmed"
    );
    assert_runtime_timestamp_not_before(&request.confirmed_at, "2026-06-18T00:05:00Z");
}

fn assert_resolution_request(request: &PolicyRequestParentResolutionRequest) {
    assert_opaque_command_id(&request.command_id, "policy-parent-resolution-pprh-");
    assert_eq!(request.schema_version, AGENT_PROTOCOL_SCHEMA_VERSION);
    assert_eq!(
        request.confirmed_audit_reference_id,
        "audit.policy-request.confirmed"
    );
    assert_eq!(request.approval_id, "policy-approval-1");
    assert_eq!(request.parent_actor_id, "parent-1");
    assert_eq!(
        request.parent_actor_role,
        PolicyRequestAssistantPreviewConfirmActorRole::Parent
    );
    assert_eq!(
        request.parent_actor_state,
        PolicyRequestAssistantPreviewConfirmActorState::Active
    );
    assert_eq!(
        request.decision,
        PolicyRequestParentResolutionDecision::Grant
    );
    assert_eq!(
        request.approved_action,
        Some(PolicyRequestAssistantPreviewConfirmAction::Block)
    );
    assert_eq!(request.approved_bonus_minutes, None);
    assert_eq!(
        request.override_expires_at.as_deref(),
        Some("2026-06-18T00:20:00Z")
    );
    assert_runtime_timestamp_not_before(&request.decided_at, "2026-06-18T00:05:00Z");
    assert_eq!(
        request.approval_audit_reference_id,
        "audit.policy-request.confirmed"
    );
    let binding = require_some(
        request.delivery_binding.as_ref(),
        TestContext("granted parent resolution includes a delivery identity binding"),
    );
    assert_eq!(binding.household_id, "household-1");
    assert_eq!(binding.child_profile_id, "child-profile-1");
    assert_eq!(binding.device_id.as_deref(), Some("device-1"));
    assert_eq!(binding.source_document_id, "source-document-1");
    assert_eq!(binding.policy_version, 7);
}

fn assert_confirmed_preview_snapshot(result: &super::ParentUiActionResult) {
    let live_activity = require_result_live_activity(
        result,
        TestContext("policy preview confirm action returns snapshot"),
        TestContext("policy preview confirm action returns live activity snapshot"),
    );
    let policy_preview = require_some(
        live_activity.policy_preview_panel.as_ref(),
        TestContext("policy preview confirm action returns policy preview panel"),
    );
    let details = require_some(
        policy_preview.cards.first(),
        TestContext("policy preview panel keeps first preview card"),
    )
    .details
    .as_slice();
    let value = |label: &str| {
        details
            .iter()
            .find(|detail| detail.label == label)
            .map(|detail| detail.value.as_str())
    };
    assert_eq!(value("Request origin"), Some("Assistant draft"));
    assert_eq!(value("Assistant confirmation"), Some("Parent confirmed"));
    assert_eq!(value("Request status"), Some("Pending parent review"));
    assert_eq!(
        value("Audit reference"),
        Some("audit.policy-request.confirmed")
    );
}

fn assert_opaque_command_id(value: &str, prefix: &str) {
    let suffix = require_some(
        value.strip_prefix(prefix),
        TestContext("command id carries its typed opaque-handle prefix"),
    );
    assert_opaque_handle(suffix, "");
}

fn assert_opaque_handle(value: &str, prefix: &str) {
    let suffix = require_some(
        value.strip_prefix(prefix),
        TestContext("opaque handle carries its owned prefix"),
    );
    assert_eq!(suffix.len(), 48);
    assert!(suffix.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

fn assert_runtime_timestamp_not_before(value: &str, lower_bound: &str) {
    let timestamp = require_ok(
        chrono::DateTime::parse_from_rfc3339(value),
        "runtime-generated timestamp is valid RFC3339",
    );
    let lower_bound = require_ok(
        chrono::DateTime::parse_from_rfc3339(lower_bound),
        "test lower-bound timestamp is valid RFC3339",
    );
    assert!(timestamp >= lower_bound);
    assert_eq!(timestamp.offset().local_minus_utc(), 0);
}
