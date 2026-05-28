use std::fs::{read, remove_file};

use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, BrowserBoundaryState, BrowserChannel, BrowserCustodyLabel,
    BrowserExactUrlClaimState, BrowserFamily, BrowserInterventionAction,
    BrowserInterventionCapabilityState, BrowserInterventionDecisionSource,
    BrowserInterventionMechanism, BrowserInterventionOutcome, BrowserInterventionTargetType,
    BrowserQueryVisibilityLabel, BrowserUnmanagedDetectionState, BrowserUnmanagedEnforcementState,
};

use super::{
    browser_intervention_applied_event, ActivityJournal, ActivityStore,
    BrowserInterventionObservation, JournalKey, JOURNAL_KEY_BYTES,
};

#[test]
fn activity_store_reports_typed_browser_intervention_read_model_from_ingested_events() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = browser_intervention_event();

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.latest_event_id, Some(event.event_id));
    assert_eq!(
        read_model.managed_session_intervention_capability,
        BrowserInterventionCapabilityState::Ready
    );
    assert_eq!(
        read_model.unmanaged_browser_enforcement,
        BrowserUnmanagedEnforcementState::RequiresOsAppControl
    );
    let row = &read_model.rows[0];
    assert_eq!(
        row.decision_source,
        BrowserInterventionDecisionSource::ParentRule
    );
    assert_eq!(row.intervention_action, BrowserInterventionAction::Block);
    assert_eq!(
        row.intervention_mechanism,
        BrowserInterventionMechanism::ChromiumCdpFetch
    );
    assert_eq!(
        row.intervention_outcome,
        BrowserInterventionOutcome::Blocked
    );
    assert_eq!(
        row.requested_url.as_deref(),
        Some(constants::activity_store::TEST_BROWSER_URL)
    );
    assert_eq!(
        row.browser_boundary_state,
        BrowserBoundaryState::ManagedSession
    );
    assert_eq!(
        row.exact_url_claim_state,
        BrowserExactUrlClaimState::ExactUrlProven
    );
    assert_eq!(
        row.unmanaged_detection_state,
        BrowserUnmanagedDetectionState::None
    );
}

#[test]
fn activity_store_infers_legacy_managed_url_proof_without_overclaiming_unmanaged_rows() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let mut event = browser_intervention_event();
    remove_browser_claim_fields(&mut event);

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(
        row.browser_boundary_state,
        BrowserBoundaryState::ManagedSession
    );
    assert_eq!(
        row.exact_url_claim_state,
        BrowserExactUrlClaimState::ExactUrlProven
    );
    assert_eq!(
        row.unmanaged_detection_state,
        BrowserUnmanagedDetectionState::None
    );
}

#[test]
fn activity_store_does_not_overclaim_legacy_rows_without_managed_url_proof() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let mut event = browser_intervention_event();
    remove_browser_claim_fields(&mut event);
    event
        .fields
        .remove(constants::field::MANAGED_BROWSER_SESSION_ID);
    event.fields.remove(constants::field::REQUESTED_URL);
    event.fields.remove(constants::field::OBSERVED_URL);

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(row.browser_boundary_state, BrowserBoundaryState::Unknown);
    assert_eq!(
        row.exact_url_claim_state,
        BrowserExactUrlClaimState::NotClaimed
    );
    assert_eq!(
        row.unmanaged_detection_state,
        BrowserUnmanagedDetectionState::Unavailable
    );
}

#[test]
fn activity_store_replays_browser_interventions_from_encrypted_journal() {
    let journal_path = temp_path(
        constants::activity_store::TEST_BROWSER_INTERVENTION_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_BROWSER_INTERVENTION_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &store_path);
    let key = test_key();
    let mut journal = ActivityJournal::open(journal_path.clone(), key.clone())
        .expect(constants::error::JOURNAL_OPENS);
    let event = browser_intervention_event();
    journal
        .append(&event)
        .expect(constants::error::JOURNAL_APPENDS);
    let journal_bytes = read(&journal_path).expect(constants::error::JOURNAL_READS);
    let reader =
        ActivityJournal::open(journal_path.clone(), key).expect(constants::error::JOURNAL_OPENS);
    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);

    let status = store
        .ingest_journal(&reader)
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    cleanup_paths(&journal_path, &store_path);

    assert_eq!(status.events_ingested, 1);
    assert_eq!(
        read_model.rows[0].browser_intervention_id,
        constants::browser::INTERVENTION_ID_PREFIX.to_string() + &0.to_string()
    );
    assert!(!String::from_utf8_lossy(&journal_bytes)
        .contains(constants::activity_store::TEST_BROWSER_URL));
}

#[test]
fn activity_store_reports_empty_browser_intervention_readiness_without_rows() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.managed_session_intervention_capability,
        BrowserInterventionCapabilityState::NeedsManagedSession
    );
    assert_eq!(
        read_model.unmanaged_browser_enforcement,
        BrowserUnmanagedEnforcementState::RequiresOsAppControl
    );
}

fn browser_intervention_event() -> ActivityEvent {
    browser_intervention_applied_event(
        BrowserInterventionObservation {
            browser_family: Some(BrowserFamily::Chrome),
            browser_channel: Some(BrowserChannel::Stable),
            managed_browser_session_id: Some(constants::browser::SESSION_ID_DEV.to_string()),
            profile_id: Some(constants::browser::PROFILE_ID_DEV.to_string()),
            process_id: Some(constants::activity_store::TEST_BROWSER_PROCESS_ID),
            policy_decision_id: Some(
                constants::activity_store::TEST_POLICY_DECISION_ID.to_string(),
            ),
            decision_source: BrowserInterventionDecisionSource::ParentRule,
            intervention_action: BrowserInterventionAction::Block,
            intervention_target_type: BrowserInterventionTargetType::Video,
            intervention_target_value: constants::activity_store::TEST_BROWSER_URL.to_string(),
            requested_url: Some(constants::activity_store::TEST_BROWSER_URL.to_string()),
            observed_url: Some(constants::activity_store::TEST_BROWSER_URL.to_string()),
            intervention_mechanism: BrowserInterventionMechanism::ChromiumCdpFetch,
            intervention_outcome: BrowserInterventionOutcome::Blocked,
            browser_boundary_state: BrowserBoundaryState::ManagedSession,
            exact_url_claim_state: BrowserExactUrlClaimState::ExactUrlProven,
            unmanaged_detection_state: BrowserUnmanagedDetectionState::None,
            managed_session_intervention_capability: BrowserInterventionCapabilityState::Ready,
            unmanaged_browser_enforcement: BrowserUnmanagedEnforcementState::RequiresOsAppControl,
            reason: Some(constants::activity_store::TEST_BROWSER_INTERVENTION_REASON.to_string()),
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    )
}

fn remove_browser_claim_fields(event: &mut ActivityEvent) {
    event
        .fields
        .remove(constants::browser::INTERVENTION_FIELD_BROWSER_BOUNDARY_STATE);
    event
        .fields
        .remove(constants::browser::INTERVENTION_FIELD_EXACT_URL_CLAIM_STATE);
    event
        .fields
        .remove(constants::browser::INTERVENTION_FIELD_UNMANAGED_DETECTION_STATE);
}

fn temp_path(suffix: &str, extension: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension);
    path
}

fn cleanup_paths(journal_path: &std::path::PathBuf, store_path: &std::path::PathBuf) {
    let _ = remove_file(journal_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.clone();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.clone();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([11; JOURNAL_KEY_BYTES])
}
