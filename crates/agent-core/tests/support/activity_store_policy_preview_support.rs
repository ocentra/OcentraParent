use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_core::browser_bridge_event::{
    browser_tab_observation_event, BrowserBridgeTargetObservation,
};
use ocentra_parent_agent_core::network_capture::NetworkObservation;
use ocentra_parent_agent_core::network_capture_event::network_observation_event;
use ocentra_parent_agent_core::window_capture::ForegroundWindowObservation;
use ocentra_parent_agent_core::window_capture_event::foreground_window_observation_event;
use ocentra_parent_agent_protocol::activity::policy::ParentActorReference;
use ocentra_parent_agent_protocol::activity::policy::ParentActorRole;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyRule;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::activity::policy_context::ChildProfileReference;
use ocentra_parent_agent_protocol::activity::policy_context::FamilyReference;
use ocentra_parent_agent_protocol::activity::policy_context::LocalAiParentRuleContextRef;
use ocentra_parent_agent_protocol::activity::policy_context::ParentDeviceReference;
use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::browser::{
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants as policy;
use std::fmt::Display;

use crate::test_text::TestText;

pub fn browser_event() -> ActivityEvent {
    browser_tab_observation_event(
        BrowserBridgeTargetObservation {
            browser_family: BrowserFamily::Edge,
            browser_channel: BrowserChannel::Stable,
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: 4242,
            target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
            tab_id: Some(constants::activity_store::TEST_BROWSER_TAB_ID.to_string()),
            window_id: None,
            active_state: BrowserActiveTabState::Unknown,
            active_proof_source: BrowserActiveProofSource::TargetListOnly,
            url: constants::activity_store::TEST_BROWSER_URL.to_string(),
            title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
            capability_status: BrowserCapabilityStatus::TabListOnly,
            degraded_reason: None,
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect_value(constants::error::BROWSER_BRIDGE_MAPS_TARGET)
}

pub fn active_window_event() -> ActivityEvent {
    foreground_window_observation_event(
        ForegroundWindowObservation::active(
            4242,
            constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
            constants::activity_store::TEST_APP_GAME_PROCESS_PATH.to_string(),
            constants::activity_store::TEST_APP_GAME_WINDOW_TITLE.to_string(),
            constants::activity_store::TEST_WINDOW_ID.to_string(),
        ),
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
}

pub fn network_flow_event() -> ActivityEvent {
    network_flow_event_at(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0)
}

pub fn network_flow_event_at(observed_at: impl Display, sequence_index: usize) -> ActivityEvent {
    let observed_at = TestText::from_display(observed_at);
    let observed_at = observed_at.to_string();
    network_observation_event(
        NetworkObservation {
            status: ActivityCaptureCapabilityStatus::Available,
            protocol: Some(ActivityNetworkProtocol::Tcp),
            local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
            destination_ip: Some(
                constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string(),
            ),
            destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
            destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
            tcp_state: Some(ActivityNetworkTcpState::Established),
            pid: Some(constants::activity_store::TEST_BROWSER_PROCESS_ID),
            process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
            associated_pid_count: 1,
        },
        observed_at.as_str(),
        sequence_index,
    )
}

pub fn network_retention_deleted_event(deleted_event_id: impl Display) -> ActivityEvent {
    network_retention_deleted_event_at(
        deleted_event_id,
        constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT,
    )
}

pub fn network_retention_deleted_event_at(
    deleted_event_id: impl Display,
    observed_at: impl Display,
) -> ActivityEvent {
    let deleted_event_id = TestText::from_display(deleted_event_id);
    let observed_at = TestText::from_display(observed_at);
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String(deleted_event_id.to_string()),
    );
    fields.insert(
        constants::field::DELETED_AT.to_string(),
        LogFieldValue::String(observed_at.to_string()),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: constants::activity_store::TEST_NETWORK_RETENTION_DELETE_EVENT_ID.to_string(),
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::AgentService,
            source_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        },
        kind: ActivityEventKind::NetworkRetentionDeleted,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Retention,
            subject_id: deleted_event_id.to_string(),
            display_name: None,
        },
        fields,
        evidence: vec![ActivityEvidenceRef {
            evidence_id: deleted_event_id.to_string(),
            kind: ActivityEvidenceKind::JournalEntry,
            digest: None,
            uri: None,
        }],
    }
}

pub fn parent_rule_context_for_event(event: &ActivityEvent) -> LocalAiParentRuleContextRef {
    parent_rule_context(
        PolicyTarget {
            target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
            target_type: PolicyTargetType::Domain,
            target_value: constants::activity_store::TEST_BROWSER_DOMAIN.to_string(),
        },
        policy::TEST_BLOCK_RULE_ID,
        PolicyAction::Block,
        policy::TEST_REASON_PARENT_BLOCK,
        vec![TestText::from_display(event.event_id.clone())],
    )
}

pub fn parent_rule_context(
    target: PolicyTarget,
    rule_id: impl Display,
    action: PolicyAction,
    reason_code: impl Display,
    target_evidence_refs: Vec<TestText>,
) -> LocalAiParentRuleContextRef {
    let rule_id = TestText::from_display(rule_id);
    let reason_code = TestText::from_display(reason_code);
    LocalAiParentRuleContextRef {
        parent_rule_ref_id: policy::TEST_PARENT_RULE_CONTEXT_REF_ID.to_string(),
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        family: FamilyReference {
            family_id: policy::TEST_FAMILY_ID.to_string(),
        },
        child_profile: ChildProfileReference {
            child_profile_id: policy::TEST_CHILD_PROFILE_ID.to_string(),
            display_name: policy::TEST_CHILD_PROFILE_DISPLAY_NAME.to_string(),
        },
        device: ParentDeviceReference {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            child_profile_id: Some(policy::TEST_CHILD_PROFILE_ID.to_string()),
            label: policy::TEST_PARENT_DEVICE_LABEL.to_string(),
            platform: std::env::consts::OS.to_string(),
        },
        rule: PolicyRule {
            rule_id: rule_id.to_string(),
            target,
            action,
            schedule_id: None,
            priority: 10,
            reason_code: reason_code.to_string(),
            created_by: ParentActorReference {
                actor_id: policy::TEST_PARENT_ACTOR_ID.to_string(),
                role: ParentActorRole::Parent,
            },
            enabled: true,
            effective_from: None,
            effective_until: None,
        },
        target_evidence_refs: target_evidence_refs
            .into_iter()
            .map(|value| value.to_string())
            .collect(),
        custody: policy::TEST_PARENT_RULE_CONTEXT_CUSTODY.to_string(),
        updated_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        expires_at: None,
    }
}
