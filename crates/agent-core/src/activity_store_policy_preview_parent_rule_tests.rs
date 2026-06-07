use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, PolicyAction, PolicyTarget, PolicyTargetType,
};

use super::{
    activity_store_policy_preview_test_fixture::{
        active_window_event, browser_event, network_flow_event, parent_rule_context,
        parent_rule_context_for_event, parent_rule_context_for_network_flow,
    },
    ActivityStore,
};

#[test]
fn policy_preview_read_model_resolves_local_parent_rule_context_for_matching_evidence() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = browser_event();
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    store
        .replace_parent_rule_contexts(&[parent_rule_context_for_event(&event)])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(row.parent_rule_context_references.len(), 1);
    assert_eq!(
        row.parent_rule_context_references[0].parent_rule_ref_id,
        policy::TEST_PARENT_RULE_CONTEXT_REF_ID
    );
    assert_eq!(
        row.parent_rule_context_references[0].target_evidence_refs,
        vec![event.event_id]
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
}

#[test]
fn policy_preview_read_model_resolves_site_rule_from_browser_url_alias() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = browser_event();
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    store
        .replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
                target_type: PolicyTargetType::Site,
                target_value: constants::activity_store::TEST_BROWSER_URL.to_string(),
            },
            policy::TEST_ASK_PARENT_RULE_ID,
            PolicyAction::AskParent,
            policy::TEST_REASON_PARENT_ASK,
            vec![event.event_id.clone()],
        )])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(row.target.target_type, PolicyTargetType::Domain);
    assert_eq!(
        row.target.target_value,
        constants::activity_store::TEST_BROWSER_DOMAIN
    );
    assert_eq!(row.parent_rule_context_references.len(), 1);
    assert_eq!(row.decision.action, PolicyAction::AskParent);
    assert_eq!(
        row.decision.rule_ids,
        vec![policy::TEST_ASK_PARENT_RULE_ID.to_string()]
    );
}

#[test]
fn policy_preview_read_model_resolves_app_rule_from_active_window_alias() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = active_window_event();
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    store
        .replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: constants::activity_store::TEST_WINDOW_SUBJECT_ID.to_string(),
                target_type: PolicyTargetType::App,
                target_value: constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
            },
            policy::TEST_TIME_LIMIT_RULE_ID,
            PolicyAction::TimeLimit,
            policy::TEST_REASON_PARENT_TIME_LIMIT,
            vec![event.event_id.clone()],
        )])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(row.target.target_type, PolicyTargetType::Window);
    assert_eq!(
        row.target.target_value,
        constants::activity_store::TEST_APP_GAME_WINDOW_TITLE
    );
    assert_eq!(row.parent_rule_context_references.len(), 1);
    assert_eq!(row.decision.action, PolicyAction::TimeLimit);
    assert_eq!(
        row.decision.rule_ids,
        vec![policy::TEST_TIME_LIMIT_RULE_ID.to_string()]
    );
}

#[test]
fn policy_preview_read_model_resolves_network_domain_rule_from_stored_flow_evidence() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = network_flow_event();
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    store
        .replace_parent_rule_contexts(&[parent_rule_context_for_network_flow(&event)])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(row.target.target_type, PolicyTargetType::Domain);
    assert_eq!(
        row.target.target_value,
        constants::activity_store::TEST_NETWORK_DOMAIN
    );
    assert_eq!(row.parent_rule_context_references.len(), 1);
    assert_eq!(
        row.parent_rule_context_references[0].target_evidence_refs,
        vec![event.event_id]
    );
    assert_eq!(row.decision.action, PolicyAction::Block);
    assert_eq!(
        row.decision.rule_ids,
        vec![policy::TEST_BLOCK_RULE_ID.to_string()]
    );
    assert!(row.decision.dry_run);
    assert_eq!(
        row.decision.enforcement_handoff_state,
        ocentra_parent_agent_protocol::PolicyDecisionHandoffState::Disabled
    );
}

#[test]
fn policy_preview_read_model_resolves_process_rule_from_network_flow_alias() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = network_flow_event();
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    store
        .replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: event.subject.subject_id.clone(),
                target_type: PolicyTargetType::Process,
                target_value: constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
            },
            policy::TEST_ASK_PARENT_RULE_ID,
            PolicyAction::AskParent,
            policy::TEST_REASON_PARENT_ASK,
            vec![event.event_id.clone()],
        )])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(row.target.target_type, PolicyTargetType::Domain);
    assert_eq!(
        row.target.target_value,
        constants::activity_store::TEST_NETWORK_DOMAIN
    );
    assert_eq!(row.parent_rule_context_references.len(), 1);
    assert_eq!(row.decision.action, PolicyAction::AskParent);
    assert_eq!(
        row.decision.rule_ids,
        vec![policy::TEST_ASK_PARENT_RULE_ID.to_string()]
    );
    assert!(row.decision.dry_run);
}

#[test]
fn policy_preview_read_model_rejects_partially_grounded_parent_rule_context() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = browser_event();
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    store
        .replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
                target_type: PolicyTargetType::Domain,
                target_value: constants::activity_store::TEST_BROWSER_DOMAIN.to_string(),
            },
            policy::TEST_BLOCK_RULE_ID,
            PolicyAction::Block,
            policy::TEST_REASON_PARENT_BLOCK,
            vec![event.event_id.clone(), policy::TEST_EVIDENCE_ID.to_string()],
        )])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let read_model = store
        .policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(row.parent_rule_context_references.len(), 0);
    assert_eq!(row.decision.action, PolicyAction::Unknown);
    assert_eq!(
        row.decision.reason_codes,
        vec![
            policy::REASON_NO_MATCHING_PARENT_RULE.to_string(),
            policy::REASON_LOCAL_AI_RESULT_MISSING.to_string()
        ]
    );
}
