use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    child_tracking_config_updated_event_from_parent, default_tracking_config_update_request,
    parent_tracking_config_updated_event_from_command, tracking_config_audit_entry_committed_event,
    tracking_config_change_approved_event, tracking_config_change_rejected_event,
    tracking_config_change_requested_event, tracking_config_policy_decision_completed_event,
    tracking_config_policy_evaluation_requested_event,
    tracking_config_portal_read_model_updated_event,
    tracking_config_update_applied_event_from_child, TrackingConfigAuditOutcome,
    TrackingConfigEffectiveState, TrackingConfigPolicyDecisionState,
    TrackingConfigPortalUpdateKind, TrackingConfigUpdateEventName,
    TrackingConfigUpdateResponseState, TRACKING_CONFIG_UPDATE_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingPolicyRuleRef;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingDurableSettingsPersistenceState;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole,
    AgentRoute, AGENT_TRANSPORT_SCHEMA_VERSION,
};

#[test]
fn tracking_config_update_event_names_serialize_exact_contract_text() {
    assert_eq!(
        serde_json::to_value(TrackingConfigUpdateEventName::Parent)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        constants::tracking_config_update::PARENT_EVENT_TYPE
    );
    assert_eq!(
        serde_json::to_value(TrackingConfigUpdateEventName::Child)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        constants::tracking_config_update::CHILD_EVENT_TYPE
    );
    assert_eq!(
        serde_json::to_value(TrackingConfigUpdateEventName::Applied)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        constants::tracking_config_update::APPLIED_EVENT_TYPE
    );
}

#[test]
fn tracking_config_update_applied_event_serializes_durable_child_runtime_result() {
    let request = default_tracking_config_update_request();
    let command = command_envelope(&request);
    let parent_event = parent_tracking_config_updated_event_from_command(&command, request);
    let child_event = child_tracking_config_updated_event_from_parent(&parent_event);
    let applied_event = tracking_config_update_applied_event_from_child(
        &child_event,
        TrackingConfigUpdateResponseState::Applied,
        TrackingConfigEffectiveState::Enabled,
        7,
        TrackingDurableSettingsPersistenceState::Persisted,
    );
    let serialized =
        serde_json::to_value(&applied_event).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        applied_event
            .contract()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .event_type
            .as_str(),
        constants::tracking_config_update::APPLIED_EVENT_TYPE
    );
    assert_eq!(
        serialized["parentEventType"],
        constants::tracking_config_update::PARENT_EVENT_TYPE
    );
    assert_eq!(
        serialized["childEventType"],
        constants::tracking_config_update::CHILD_EVENT_TYPE
    );
    assert_eq!(
        serialized["responseState"],
        constants::tracking_config_update::RESPONSE_STATE_APPLIED
    );
    assert_eq!(
        serialized["effectiveTrackingState"],
        constants::tracking_config_update::EFFECTIVE_STATE_ENABLED
    );
    assert_eq!(
        serialized["target"]["scope"],
        constants::tracking_config_update::TARGET_SCOPE_CHILD_DEVICE
    );
    assert_eq!(serialized["localServiceStateRevision"], 7);
    assert_eq!(serialized["durableSettingsPersistenceState"], "persisted");
}

#[test]
fn tracking_config_change_approval_chain_serializes_named_contract_and_refs() {
    let request = default_tracking_config_update_request();
    let command = command_envelope(&request);
    let parent_event = parent_tracking_config_updated_event_from_command(&command, request);
    let change_requested = tracking_config_change_requested_event(
        "event.parent-controller.parent-action.received.1",
        &parent_event,
    );
    let evaluation = tracking_config_policy_evaluation_requested_event(
        &change_requested,
        vec![
            TrackingPolicyRuleRef::parse(
                constants::tracking_config_update::POLICY_RULE_LOCAL_CHILD_RUNTIME,
            )
            .expect_value(constants::tracking_config_update::POLICY_RULE_LOCAL_CHILD_RUNTIME),
            TrackingPolicyRuleRef::parse(
                constants::tracking_config_update::POLICY_RULE_REMOTE_SYNC_DISABLED,
            )
            .expect_value(constants::tracking_config_update::POLICY_RULE_REMOTE_SYNC_DISABLED),
        ],
        false,
    );
    let decision = tracking_config_policy_decision_completed_event(
        &evaluation,
        TrackingConfigPolicyDecisionState::Approved,
        true,
    );
    let approved = tracking_config_change_approved_event(&decision);
    let audit = tracking_config_audit_entry_committed_event(
        &decision,
        approved.change_approved_event_ref.clone(),
        TrackingConfigAuditOutcome::Committed,
    );
    let portal = tracking_config_portal_read_model_updated_event(
        &audit,
        TrackingConfigPortalUpdateKind::TrackingConfigState,
        false,
        false,
    );

    assert_eq!(
        change_requested
            .contract()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .event_type
            .as_str(),
        constants::tracking_config_update::CHANGE_REQUESTED_EVENT_TYPE
    );
    assert_eq!(
        evaluation
            .contract()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .event_type
            .as_str(),
        constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED
    );
    assert_eq!(
        decision
            .contract()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .event_type
            .as_str(),
        constants::network_flow::EVENT_POLICY_DECISION_COMPLETED
    );
    assert_eq!(
        approved
            .contract()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .event_type
            .as_str(),
        constants::tracking_config_update::CHANGE_APPROVED_EVENT_TYPE
    );
    assert_eq!(
        audit
            .contract()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .event_type
            .as_str(),
        constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED
    );
    assert_eq!(
        portal
            .contract()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .event_type
            .as_str(),
        constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED
    );

    let serialized_decision =
        serde_json::to_value(&decision).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let serialized_portal =
        serde_json::to_value(&portal).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let serialized_requested = serde_json::to_value(&change_requested)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized_decision["decisionState"], "approved");
    assert_eq!(serialized_decision["childRuntimePublishRequired"], true);
    assert_eq!(
        serialized_requested["config"]["runtimeConfig"]["trackingEnabledState"],
        constants::tracking_config_update::EFFECTIVE_STATE_ENABLED
    );
    assert_eq!(serialized_portal["updateKind"], "tracking-config-state");
    assert_eq!(serialized_portal["visibleManualRequired"], false);
    assert_eq!(serialized_portal["visibleUnavailable"], false);
}

#[test]
fn tracking_config_change_rejection_chain_serializes_manual_required_surface_state() {
    let request = default_tracking_config_update_request();
    let command = command_envelope(&request);
    let parent_event = parent_tracking_config_updated_event_from_command(&command, request);
    let change_requested = tracking_config_change_requested_event(
        "event.parent-controller.parent-action.received.1",
        &parent_event,
    );
    let evaluation = tracking_config_policy_evaluation_requested_event(
        &change_requested,
        vec![TrackingPolicyRuleRef::parse(
            constants::tracking_config_update::POLICY_RULE_LOCAL_CHILD_RUNTIME,
        )
        .expect_value(constants::tracking_config_update::POLICY_RULE_LOCAL_CHILD_RUNTIME)],
        false,
    );
    let decision = tracking_config_policy_decision_completed_event(
        &evaluation,
        TrackingConfigPolicyDecisionState::Rejected,
        false,
    );
    let rejected = tracking_config_change_rejected_event(
        &decision,
        constants::tracking_config_update::REJECTION_REASON_CHILD_RUNTIME_DISPATCH_BLOCKED,
    );
    let audit = tracking_config_audit_entry_committed_event(
        &decision,
        rejected.change_rejected_event_ref.clone(),
        TrackingConfigAuditOutcome::Failed,
    );
    let portal = tracking_config_portal_read_model_updated_event(
        &audit,
        TrackingConfigPortalUpdateKind::ManualRequiredState,
        true,
        true,
    );

    let serialized_rejected =
        serde_json::to_value(&rejected).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let serialized_audit =
        serde_json::to_value(&audit).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let serialized_portal =
        serde_json::to_value(&portal).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized_rejected["rejectionReasonCode"],
        constants::tracking_config_update::REJECTION_REASON_CHILD_RUNTIME_DISPATCH_BLOCKED
    );
    assert_eq!(serialized_audit["auditOutcome"], "failed");
    assert_eq!(serialized_portal["updateKind"], "manual-required-state");
    assert_eq!(serialized_portal["visibleManualRequired"], true);
    assert_eq!(serialized_portal["visibleUnavailable"], true);
}

fn command_envelope(
    request: &crate::tracking::config_update_event::TrackingConfigUpdateRequest,
) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_TRANSPORT_SCHEMA_VERSION,
        message_id: request.command_id.clone(),
        sent_at: constants::tracking_retention_settings_write::ACCEPTED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityTrackingRetentionSettingsWrite,
        payload: LogFields::new(),
    }
}
