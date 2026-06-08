use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, ParentEvidenceReferenceKind, PolicyAction,
    PolicyDecisionHandoffState, PolicyPreviewReadModelRow, PolicyTarget, PolicyTargetType,
};

use super::{
    activity_store_policy_preview_test_fixture::{
        browser_event, network_flow_event, network_flow_event_at, network_retention_deleted_event,
        network_retention_deleted_event_at, parent_rule_context,
    },
    ActivityStore,
};

#[test]
fn policy_preview_read_model_evaluates_stored_browser_evidence_without_enforcement() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = browser_event();
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 1);
    assert_eq!(
        read_model.capability_status,
        policy::PREVIEW_CAPABILITY_READY
    );
    let row = &read_model.rows[0];
    assert_eq!(row.source_event_id, event.event_id);
    assert_eq!(row.target.target_type, PolicyTargetType::Domain);
    assert_eq!(
        row.target.target_value,
        constants::activity_store::TEST_BROWSER_DOMAIN
    );
    assert_eq!(row.evidence_references.len(), 1);
    assert_eq!(
        row.evidence_references[0].kind,
        ParentEvidenceReferenceKind::ActivityEvent
    );
    assert_eq!(row.parent_rule_context_references.len(), 0);
    assert_eq!(row.network_evidence_mapping, None);
    assert_eq!(row.decision.action, PolicyAction::Unknown);
    assert_eq!(
        row.decision.reason_codes,
        vec![
            policy::REASON_NO_MATCHING_PARENT_RULE.to_string(),
            policy::REASON_LOCAL_AI_RESULT_MISSING.to_string()
        ]
    );
    assert!(row.decision.dry_run);
    assert_eq!(
        row.decision.enforcement_handoff_state,
        PolicyDecisionHandoffState::Disabled
    );
}

#[test]
fn policy_preview_read_model_evaluates_stored_network_flow_evidence_without_enforcement() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = network_flow_event();
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    store
        .replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: event.subject.subject_id.clone(),
                target_type: PolicyTargetType::Domain,
                target_value: constants::activity_store::TEST_NETWORK_DOMAIN.to_string(),
            },
            policy::TEST_BLOCK_RULE_ID,
            PolicyAction::Block,
            policy::TEST_REASON_PARENT_BLOCK,
            vec![event.event_id.clone()],
        )])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 1);
    assert_eq!(
        read_model.capability_status,
        policy::PREVIEW_CAPABILITY_READY
    );
    let row = &read_model.rows[0];
    assert_eq!(row.source_event_id, event.event_id);
    assert_eq!(row.target.target_id, event.subject.subject_id);
    assert_eq!(row.target.target_type, PolicyTargetType::Domain);
    assert_eq!(
        row.target.target_value,
        constants::activity_store::TEST_NETWORK_DOMAIN
    );
    assert_eq!(row.evidence_references.len(), 1);
    assert_eq!(
        row.evidence_references[0].kind,
        ParentEvidenceReferenceKind::ActivityEvent
    );
    assert_eq!(
        row.evidence_references[0].evidence_reference_id,
        row.source_event_id
    );
    assert_eq!(row.parent_rule_context_references.len(), 1);
    assert_eq!(
        row.parent_rule_context_references[0].target_evidence_refs,
        vec![row.source_event_id.clone()]
    );
    assert_grade_b_network_mapping(row);
    assert_eq!(row.decision.action, PolicyAction::AskParent);
    assert_eq!(
        row.decision.reason_codes,
        vec![
            policy::TEST_REASON_PARENT_BLOCK.to_string(),
            policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW.to_string()
        ]
    );
    assert_eq!(
        row.decision.rule_ids,
        vec![policy::TEST_BLOCK_RULE_ID.to_string()]
    );
    assert!(row.decision.dry_run);
    assert_eq!(
        row.decision.enforcement_handoff_state,
        PolicyDecisionHandoffState::Disabled
    );
}

fn assert_grade_b_network_mapping(row: &PolicyPreviewReadModelRow) {
    let network_mapping = row
        .network_evidence_mapping
        .as_ref()
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    assert_eq!(
        network_mapping.evidence_grade,
        policy::NETWORK_EVIDENCE_GRADE_B
    );
    assert_eq!(network_mapping.requested_action, policy::ACTION_BLOCK);
    assert_eq!(network_mapping.mapped_action, policy::ACTION_ASK_PARENT);
    assert_eq!(
        network_mapping.mode,
        policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW
    );
    assert!(!network_mapping.adapter_action_authorized);
    assert!(!network_mapping.enforcement_command_authorized);
}

#[test]
fn policy_preview_read_model_excludes_network_flow_deleted_by_retention_tombstone() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = network_flow_event();
    let deleted_event_id = event.event_id.clone();
    store
        .ingest_events(&[event, network_retention_deleted_event(&deleted_event_id)])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    store
        .replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: constants::activity_store::TEST_NETWORK_EVENT_ID.to_string(),
                target_type: PolicyTargetType::Domain,
                target_value: constants::activity_store::TEST_NETWORK_DOMAIN.to_string(),
            },
            policy::TEST_BLOCK_RULE_ID,
            PolicyAction::Block,
            policy::TEST_REASON_PARENT_BLOCK,
            vec![deleted_event_id],
        )])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.capability_status,
        policy::PREVIEW_CAPABILITY_NO_EVIDENCE
    );
}

#[test]
fn policy_preview_read_model_applies_retention_tombstones_before_limit() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = network_flow_event_at(constants::activity_store::TEST_THIRD_OBSERVED_AT, 1);
    let deleted_event_id = event.event_id.clone();
    let tombstone = network_retention_deleted_event_at(
        &deleted_event_id,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    store
        .ingest_events(&[tombstone, event])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    store
        .replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: constants::activity_store::TEST_NETWORK_EVENT_ID.to_string(),
                target_type: PolicyTargetType::Domain,
                target_value: constants::activity_store::TEST_NETWORK_DOMAIN.to_string(),
            },
            policy::TEST_BLOCK_RULE_ID,
            PolicyAction::Block,
            policy::TEST_REASON_PARENT_BLOCK,
            vec![deleted_event_id],
        )])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let read_model = store
        .policy_preview_read_model(1, constants::activity_store::TEST_THIRD_OBSERVED_AT)
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.capability_status,
        policy::PREVIEW_CAPABILITY_NO_EVIDENCE
    );
}

#[test]
fn policy_preview_read_model_reports_empty_store_without_inventing_rows() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.capability_status,
        policy::PREVIEW_CAPABILITY_NO_EVIDENCE
    );
}
