use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, BrowserActiveTabState, BrowserCapabilityStatus,
    BrowserChannel, BrowserCustodyLabel, BrowserFamily, ParentEvidenceReferenceKind, PolicyAction,
    PolicyDecisionHandoffState, PolicyTargetType,
};

use super::{browser_tab_observation_event, ActivityStore, BrowserBridgeTargetObservation};

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

fn browser_event() -> ocentra_parent_agent_protocol::ActivityEvent {
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
            url: constants::activity_store::TEST_BROWSER_URL.to_string(),
            title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
            capability_status: BrowserCapabilityStatus::TabListOnly,
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET)
}
