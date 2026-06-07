use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, ActivityCaptureCapabilityStatus, ActivityEvent,
    ActivityNetworkProtocol, ActivityNetworkTcpState, BrowserActiveProofSource,
    BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel,
    BrowserFamily, BrowserQueryVisibilityLabel, ChildProfileReference, FamilyReference,
    LocalAiParentRuleContextRef, ParentActorReference, ParentActorRole, ParentDeviceReference,
    PolicyAction, PolicyRule, PolicyTarget, PolicyTargetType,
};

use super::{
    browser_tab_observation_event, foreground_window_observation_event, network_observation_event,
    BrowserBridgeTargetObservation, ForegroundWindowObservation, NetworkObservation,
};

pub(crate) fn browser_event() -> ActivityEvent {
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
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET)
}

pub(crate) fn active_window_event() -> ActivityEvent {
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

pub(crate) fn network_flow_event() -> ActivityEvent {
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
            pid: Some(4242),
            process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
            associated_pid_count: 1,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    )
}

pub(crate) fn parent_rule_context_for_event(event: &ActivityEvent) -> LocalAiParentRuleContextRef {
    parent_rule_context(
        PolicyTarget {
            target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
            target_type: PolicyTargetType::Domain,
            target_value: constants::activity_store::TEST_BROWSER_DOMAIN.to_string(),
        },
        policy::TEST_BLOCK_RULE_ID,
        PolicyAction::Block,
        policy::TEST_REASON_PARENT_BLOCK,
        vec![event.event_id.clone()],
    )
}

pub(crate) fn parent_rule_context_for_network_flow(
    event: &ActivityEvent,
) -> LocalAiParentRuleContextRef {
    parent_rule_context(
        PolicyTarget {
            target_id: event.subject.subject_id.clone(),
            target_type: PolicyTargetType::Domain,
            target_value: constants::activity_store::TEST_NETWORK_DOMAIN.to_string(),
        },
        policy::TEST_BLOCK_RULE_ID,
        PolicyAction::Block,
        policy::TEST_REASON_PARENT_BLOCK,
        vec![event.event_id.clone()],
    )
}

pub(crate) fn parent_rule_context(
    target: PolicyTarget,
    rule_id: &str,
    action: PolicyAction,
    reason_code: &str,
    target_evidence_refs: Vec<String>,
) -> LocalAiParentRuleContextRef {
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
            device_id: policy::TEST_PARENT_DEVICE_ID.to_string(),
            child_profile_id: Some(policy::TEST_CHILD_PROFILE_ID.to_string()),
            label: policy::TEST_PARENT_DEVICE_LABEL.to_string(),
            platform: policy::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
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
        target_evidence_refs,
        custody: policy::TEST_PARENT_RULE_CONTEXT_CUSTODY.to_string(),
        updated_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        expires_at: None,
    }
}
