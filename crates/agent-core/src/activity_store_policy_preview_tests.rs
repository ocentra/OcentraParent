use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, ParentEvidenceReferenceKind, PolicyAction,
    PolicyDecisionHandoffState, PolicyTarget, PolicyTargetType,
};

use super::{
    activity_store_policy_preview_test_fixture::{
        browser_event, network_flow_event, parent_rule_context,
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
    assert_eq!(row.decision.action, PolicyAction::Block);
    assert_eq!(
        row.decision.reason_codes,
        vec![policy::TEST_REASON_PARENT_BLOCK.to_string()]
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
