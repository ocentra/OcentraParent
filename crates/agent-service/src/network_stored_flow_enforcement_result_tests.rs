use std::fs::remove_file;

use ocentra_parent_agent_core::{network_observation_event, ActivityStore, NetworkObservation};
use ocentra_parent_agent_protocol::{
    constants, policy_constants, ActivityCaptureCapabilityStatus, ActivityEvent,
    ActivityNetworkProtocol, ActivityNetworkTcpState, AgentCommandEnvelope, AgentCommandName,
    AgentEventEnvelope, AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    ChildProfileReference, EnforcementAction, EnforcementResult, FamilyReference,
    LocalAiParentRuleContextRef, LogFieldValue, LogFields, ParentActorReference, ParentActorRole,
    ParentDeviceReference, PolicyAction, PolicyDecisionHandoffState, PolicyPreviewReadModelRow,
    PolicyRule, PolicyTarget, PolicyTargetType, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths};

#[tokio::test]
async fn enforcement_execute_records_network_domain_result_from_stored_flow_policy_refs() {
    let paths = temp_paths(&network_result_temp_suffix());
    cleanup_paths(&paths);
    let preview = stored_network_policy_preview(&paths);
    let event = build_enforcement_audit_report_with_paths(
        command_from_policy_preview_row(&preview.row),
        paths.clone(),
    )
    .await;
    let summary = {
        let store =
            ActivityStore::open(&paths.store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
        store
            .recent_summary(3)
            .expect(constants::error::ACTIVITY_STORE_QUERIES)
    };

    assert_eq!(event.event, AgentEventName::AgentEnforcementAuditReported);
    assert_eq!(preview.row.source_event_id, preview.network_event_id);
    assert_eq!(preview.row.target.target_type, PolicyTargetType::Domain);
    assert_eq!(
        preview.row.target.target_value,
        constants::activity_store::TEST_NETWORK_DOMAIN
    );
    assert_eq!(preview.row.decision.action, PolicyAction::Block);
    assert!(preview.row.decision.dry_run);
    assert_eq!(
        preview.row.decision.enforcement_handoff_state,
        PolicyDecisionHandoffState::Disabled
    );
    assert_eq!(summary.returned, 3);
    assert_eq!(
        event.payload.get(constants::field::POLICY_TARGET_TYPE),
        Some(&LogFieldValue::String(
            policy_constants::TARGET_TYPE_DOMAIN.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::POLICY_TARGET_VALUE),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_NETWORK_DOMAIN.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::ENFORCEMENT_STATUS),
        Some(&LogFieldValue::String(
            constants::enforcement::RESULT_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::EVENTS_STORED),
        Some(&LogFieldValue::Number(3.0))
    );
    assert_network_domain_result_uses_stored_flow_refs(&event, &preview);
    cleanup_paths(&paths);
}

struct StoredNetworkPolicyPreview {
    row: PolicyPreviewReadModelRow,
    network_event_id: String,
}

fn stored_network_policy_preview(paths: &EnforcementJournalPaths) -> StoredNetworkPolicyPreview {
    let store =
        ActivityStore::open(&paths.store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = network_activity_event();
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    store
        .replace_parent_rule_contexts(&[network_parent_rule_context(&event)])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let row = read_model
        .rows
        .into_iter()
        .next()
        .expect(constants::activity_store::TEST_NETWORK_DOMAIN);

    StoredNetworkPolicyPreview {
        row,
        network_event_id: event.event_id,
    }
}

fn network_result_temp_suffix() -> String {
    let mut suffix = String::from(constants::enforcement::ADAPTER_KIND_NETWORK_CONTROL);
    suffix.push(constants::delimiter::HYPHEN);
    suffix.push_str(constants::enforcement::RESULT_ID_PREFIX);
    suffix.push_str(policy_constants::TARGET_TYPE_DOMAIN);
    suffix
}

fn network_activity_event() -> ActivityEvent {
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
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    )
}

fn network_parent_rule_context(event: &ActivityEvent) -> LocalAiParentRuleContextRef {
    LocalAiParentRuleContextRef {
        parent_rule_ref_id: policy_constants::TEST_PARENT_RULE_CONTEXT_REF_ID.to_string(),
        policy_version: policy_constants::TEST_POLICY_VERSION.to_string(),
        family: FamilyReference {
            family_id: policy_constants::TEST_FAMILY_ID.to_string(),
        },
        child_profile: ChildProfileReference {
            child_profile_id: policy_constants::TEST_CHILD_PROFILE_ID.to_string(),
            display_name: policy_constants::TEST_CHILD_PROFILE_DISPLAY_NAME.to_string(),
        },
        device: ParentDeviceReference {
            device_id: policy_constants::TEST_PARENT_DEVICE_ID.to_string(),
            child_profile_id: Some(policy_constants::TEST_CHILD_PROFILE_ID.to_string()),
            label: policy_constants::TEST_PARENT_DEVICE_LABEL.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
        },
        rule: PolicyRule {
            rule_id: policy_constants::TEST_BLOCK_RULE_ID.to_string(),
            target: PolicyTarget {
                target_id: event.subject.subject_id.clone(),
                target_type: PolicyTargetType::Domain,
                target_value: constants::activity_store::TEST_NETWORK_DOMAIN.to_string(),
            },
            action: PolicyAction::Block,
            schedule_id: None,
            priority: 10,
            reason_code: policy_constants::TEST_REASON_PARENT_BLOCK.to_string(),
            created_by: ParentActorReference {
                actor_id: policy_constants::TEST_PARENT_ACTOR_ID.to_string(),
                role: ParentActorRole::Parent,
            },
            enabled: true,
            effective_from: None,
            effective_until: None,
        },
        target_evidence_refs: vec![event.event_id.clone()],
        custody: policy_constants::TEST_PARENT_RULE_CONTEXT_CUSTODY.to_string(),
        updated_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        expires_at: None,
    }
}

fn command_from_policy_preview_row(row: &PolicyPreviewReadModelRow) -> AgentCommandEnvelope {
    let mut command = command();
    command.message_id = row.preview_id.clone();
    command.payload = payload_from_policy_preview_row(row);
    command
}

fn payload_from_policy_preview_row(row: &PolicyPreviewReadModelRow) -> LogFields {
    let mut fields = payload();
    fields.insert(
        constants::field::POLICY_DECISION_ID.to_string(),
        LogFieldValue::String(row.decision.decision_id.clone()),
    );
    fields.insert(
        constants::field::POLICY_ACTION.to_string(),
        LogFieldValue::String(row.decision.action.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(row.target.target_type.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(row.target.target_id.clone()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(row.target.target_value.clone()),
    );
    fields.insert(
        constants::field::POLICY_DRY_RUN.to_string(),
        LogFieldValue::Boolean(false),
    );
    fields.insert(
        constants::field::POLICY_REASON_CODES.to_string(),
        LogFieldValue::String(joined(row.decision.reason_codes.iter().map(String::as_str))),
    );
    fields.insert(
        constants::field::POLICY_RULE_IDS.to_string(),
        LogFieldValue::String(joined(row.decision.rule_ids.iter().map(String::as_str))),
    );
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String(joined(
            row.evidence_references
                .iter()
                .map(|reference| reference.evidence_reference_id.as_str()),
        )),
    );
    fields
}

fn command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_ACTION_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::enforcement::TEST_CHILD_DEVICE_ID.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentEnforcementExecute,
        payload: payload(),
    }
}

fn payload() -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::POLICY_VERSION.to_string(),
        LogFieldValue::String(policy_constants::TEST_POLICY_VERSION.to_string()),
    );
    fields.insert(
        constants::field::REQUESTED_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EVALUATED_AT.to_string()),
    );
    fields.insert(
        constants::field::EXPIRES_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EXPIRES_AT.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_ACTION_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_ACTION_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_RESULT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_RESULT_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
    );
    fields
}

fn joined<'a>(values: impl Iterator<Item = &'a str>) -> String {
    let mut output = String::new();
    for value in values {
        if !output.is_empty() {
            output.push(constants::delimiter::LIST);
        }
        output.push_str(value);
    }
    output
}

fn assert_network_domain_result_uses_stored_flow_refs(
    event: &AgentEventEnvelope,
    preview: &StoredNetworkPolicyPreview,
) {
    let action = payload_string(&event.payload, constants::field::ENFORCEMENT_ACTION)
        .and_then(|text| serde_json::from_str::<EnforcementAction>(text).ok())
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let result = payload_string(&event.payload, constants::field::ENFORCEMENT_RESULT)
        .and_then(|text| serde_json::from_str::<EnforcementResult>(text).ok())
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        action.adapter_kind.as_protocol_str(),
        constants::enforcement::ADAPTER_KIND_NETWORK_CONTROL
    );
    assert!(!action.dry_run);
    assert_eq!(
        evidence_reference_ids(&action.evidence_references),
        evidence_reference_ids(&preview.row.evidence_references)
    );
    assert_eq!(
        evidence_reference_kinds(&action.evidence_references),
        evidence_reference_kinds(&preview.row.evidence_references)
    );
    assert_eq!(
        payload_string(&event.payload, constants::field::EVIDENCE_REFERENCE_IDS),
        Some(preview.network_event_id.as_str())
    );
    assert_manual_or_unavailable_result(&result);
}

fn evidence_reference_ids(
    references: &[ocentra_parent_agent_protocol::ParentEvidenceReference],
) -> Vec<String> {
    references
        .iter()
        .map(|reference| reference.evidence_reference_id.clone())
        .collect()
}

fn evidence_reference_kinds(
    references: &[ocentra_parent_agent_protocol::ParentEvidenceReference],
) -> Vec<String> {
    references
        .iter()
        .map(|reference| reference.kind.as_protocol_str().to_string())
        .collect()
}

fn payload_string<'a>(payload: &'a LogFields, field: &str) -> Option<&'a str> {
    match payload.get(field) {
        Some(LogFieldValue::String(value)) => Some(value.as_str()),
        _ => None,
    }
}

fn assert_manual_or_unavailable_result(result: &EnforcementResult) {
    #[cfg(windows)]
    {
        assert_eq!(
            result.capability.capability_state.as_protocol_str(),
            constants::enforcement::CAPABILITY_MANUAL_REQUIRED
        );
        assert_eq!(
            result
                .unavailable_status
                .as_ref()
                .map(|status| status.unavailable_reason.as_protocol_str()),
            Some(constants::enforcement::UNAVAILABLE_MANUAL_REQUIRED)
        );
    }
    #[cfg(not(windows))]
    assert_eq!(
        result.capability.capability_state.as_protocol_str(),
        constants::enforcement::CAPABILITY_UNAVAILABLE
    );
}

fn temp_paths(suffix: &str) -> EnforcementJournalPaths {
    EnforcementJournalPaths {
        journal_path: temp_path(
            suffix,
            constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX,
            constants::journal::FILE_EXTENSION,
        ),
        key_path: temp_path(
            suffix,
            constants::activity_store::TEST_CAPTURE_KEY_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        store_path: temp_path(
            suffix,
            constants::activity_store::TEST_STORE_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        timer_state_path: temp_path(
            suffix,
            constants::enforcement::TIMER_STATE_ID_PREFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
    }
}

fn temp_path(suffix: &str, role: &str, extension: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::journal::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(role);
    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension);
    path
}

fn cleanup_paths(paths: &EnforcementJournalPaths) {
    let _ = remove_file(&paths.journal_path);
    let _ = remove_file(&paths.key_path);
    let _ = remove_file(&paths.store_path);
    let _ = remove_file(&paths.timer_state_path);
    for index in 1..=3 {
        let mut rotated_path = paths.journal_path.clone();
        let mut extension = index.to_string();
        extension.push(constants::delimiter::DOT);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = remove_file(rotated_path);
    }
    let mut wal_path = paths.store_path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = paths.store_path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
