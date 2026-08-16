use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventName, PolicyPreviewAssistantConfirmationStateValue,
    PolicyPreviewRequestStatusValue, PolicyRequestAssistantPreviewConfirmClaimState,
    PolicyRequestAssistantPreviewConfirmResult, PolicyRequestAssistantPreviewConfirmResultState,
    PolicyRequestParentResolutionResult, PolicyRequestParentResolutionResultState,
};

#[test]
fn policy_request_assistant_preview_confirm_command_and_event_names_serialize_to_contract_shape() {
    let command = serde_json::to_value(AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm)
        .expect_value("policy request confirm command serializes: {error}");
    let event =
        serde_json::to_value(AgentEventName::AgentPolicyRequestAssistantPreviewConfirmReported)
            .expect_value("policy request confirm event serializes: {error}");

    assert_eq!(command, "agent.policy.request.assistant-preview.confirm");
    assert_eq!(
        event,
        "agent.policy.request.assistant-preview.confirm.reported"
    );
}

#[test]
fn policy_request_parent_resolution_command_and_event_names_serialize_to_contract_shape() {
    let command = serde_json::to_value(AgentCommandName::AgentPolicyRequestParentResolutionResolve)
        .expect_value("policy request parent resolution command serializes: {error}");
    let event = serde_json::to_value(AgentEventName::AgentPolicyRequestParentResolutionResolved)
        .expect_value("policy request parent resolution event serializes: {error}");

    assert_eq!(command, "agent.policy.request.parent-resolution.resolve");
    assert_eq!(event, "agent.policy.request.parent-resolution.resolved");
}

#[test]
fn policy_request_parent_resolution_result_preserves_unclaimed_delivery_boundary() {
    let result = PolicyRequestParentResolutionResult {
        schema_version: 1,
        command_id: "parent-resolution-command".to_string(),
        confirmed_audit_reference_id: "audit.policy-request.confirmed".to_string(),
        request_id: Some("policy-request-1".to_string()),
        result_state: PolicyRequestParentResolutionResultState::Resolved,
        policy_request_status: PolicyPreviewRequestStatusValue::Approved,
        resolved_approval_id: Some("approval-1".to_string()),
        temporary_override_id: Some("policy-override:approval-1".to_string()),
        resolved_at: Some("2026-06-18T00:10:00Z".to_string()),
        rejection_reason: None,
        command_transport_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        service_validation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        activity_store_lookup_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        policy_resolution_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        notification_handoff_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        child_device_delivery_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        product_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
    };

    let serialized = serde_json::to_value(result)
        .expect_value("policy request parent resolution result serializes: {error}");

    assert_eq!(serialized["resultState"], "resolved");
    assert_eq!(serialized["policyRequestStatus"], "approved");
    assert_eq!(serialized["childDeviceDeliveryClaimState"], "unclaimed");
    assert_eq!(serialized["productClaimState"], "unclaimed");
}

#[test]
fn policy_request_assistant_preview_confirm_result_serializes_without_product_overclaims() {
    let result = PolicyRequestAssistantPreviewConfirmResult {
        schema_version: 1,
        command_id: "policy-request-assistant-preview-confirm-command".to_string(),
        request_id: "policy-request-1".to_string(),
        assistant_preview_id: Some("assistant-preview-1".to_string()),
        result_state: PolicyRequestAssistantPreviewConfirmResultState::Confirmed,
        policy_request_status: PolicyPreviewRequestStatusValue::PendingParentReview,
        policy_assistant_confirmation_state:
            PolicyPreviewAssistantConfirmationStateValue::ParentConfirmed,
        policy_audit_reference_id: Some("audit.policy-request.confirmed".to_string()),
        confirmed_at: Some("2026-06-18T00:05:00Z".to_string()),
        rejection_reason: None,
        command_transport_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        service_validation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        activity_store_mutation_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        read_model_projection_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        portal_writable_ui_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        child_device_delivery_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        provider_delivery_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        platform_enforcement_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        product_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
    };

    let serialized = serde_json::to_value(result)
        .expect_value("policy request confirm result serializes: {error}");

    assert_eq!(serialized["resultState"], "confirmed");
    assert_eq!(serialized["policyRequestStatus"], "pending-parent-review");
    assert_eq!(
        serialized["policyAssistantConfirmationState"],
        "parent-confirmed"
    );
    assert_eq!(serialized["commandTransportClaimState"], "claimed");
    assert_eq!(serialized["serviceValidationClaimState"], "claimed");
    assert_eq!(serialized["productClaimState"], "unclaimed");
}
