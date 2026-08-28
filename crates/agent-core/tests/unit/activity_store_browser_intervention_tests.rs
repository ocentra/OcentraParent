use std::fs::{read, remove_file};
use std::path::Path;

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserCustodyLabel, BrowserFamily};
use ocentra_parent_agent_protocol::browser_intervention_values::{
    BrowserBoundaryState, BrowserExactUrlClaimState, BrowserInterventionAction,
    BrowserInterventionCapabilityState, BrowserInterventionDecisionSource,
    BrowserInterventionDeliveryState, BrowserInterventionMechanism, BrowserInterventionOutcome,
    BrowserInterventionTargetType, BrowserUnmanagedDetectionState,
    BrowserUnmanagedFallbackActionState,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::browser_unmanaged_enforcement::BrowserUnmanagedEnforcementState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{
    browser_intervention_applied_event, ActivityJournal, ActivityStore,
    BrowserInterventionObservation, JournalKey, JOURNAL_KEY_BYTES,
};

#[test]
fn activity_store_reports_typed_browser_intervention_read_model_from_ingested_events() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let event = browser_intervention_event();

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

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
    assert_eq!(
        read_model.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Unavailable
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
    assert_eq!(
        row.intervention_action_id.as_deref(),
        Some(constants::activity_store::TEST_BROWSER_INTERVENTION_ACTION_ID)
    );
    assert_eq!(
        row.intervention_audit_id.as_deref(),
        Some(constants::activity_store::TEST_BROWSER_INTERVENTION_AUDIT_ID)
    );
    assert_eq!(
        row.evidence_reference_ids,
        vec![constants::activity_store::TEST_BROWSER_INTERVENTION_EVIDENCE_ID.to_string()]
    );
    assert_eq!(
        row.child_delivery_state,
        BrowserInterventionDeliveryState::BlockPageRendered
    );
    assert_eq!(
        row.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Unavailable
    );
}

#[test]
fn activity_store_infers_legacy_managed_url_proof_without_overclaiming_unmanaged_rows() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let mut event = browser_intervention_event();
    remove_browser_claim_fields(&mut event);
    remove_browser_intervention_proof_fields(&mut event);

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

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
    assert_eq!(row.intervention_action_id, None);
    assert_eq!(row.intervention_audit_id, None);
    assert_eq!(row.evidence_reference_ids.len(), 0);
    assert_eq!(
        row.child_delivery_state,
        BrowserInterventionDeliveryState::NotDelivered
    );
    assert_eq!(
        row.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Unavailable
    );
}

#[test]
fn activity_store_does_not_overclaim_legacy_rows_without_managed_url_proof() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let mut event = browser_intervention_event();
    remove_browser_claim_fields(&mut event);
    remove_browser_intervention_proof_fields(&mut event);
    event.fields = event
        .fields
        .into_inner()
        .into_iter()
        .filter(|(key, _)| {
            key != constants::field::MANAGED_BROWSER_SESSION_ID
                && key != constants::field::REQUESTED_URL
                && key != constants::field::OBSERVED_URL
        })
        .collect();

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

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
    assert_eq!(
        row.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Unavailable
    );
}

#[test]
fn activity_store_reconstructs_unmanaged_fallback_action_state_without_url_claims() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let event = unmanaged_browser_terminate_event();

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(
        row.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::TerminateProcess
    );
    assert_eq!(
        read_model.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::TerminateProcess
    );
    assert_eq!(row.requested_url, None);
    assert_eq!(row.observed_url, None);
    assert_eq!(
        row.exact_url_claim_state,
        BrowserExactUrlClaimState::NotClaimed
    );
}

#[test]
fn activity_store_defaults_invalid_capability_and_enforcement_to_fail_closed_states() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let mut event = browser_intervention_event();
    set_field(
        &mut event,
        constants::field::MANAGED_SESSION_INTERVENTION_CAPABILITY,
        "invalid-capability",
    );
    set_field(
        &mut event,
        constants::field::UNMANAGED_BROWSER_ENFORCEMENT,
        "invalid-enforcement",
    );

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(
        read_model.managed_session_intervention_capability,
        BrowserInterventionCapabilityState::NeedsManagedSession
    );
    assert_eq!(
        read_model.unmanaged_browser_enforcement,
        BrowserUnmanagedEnforcementState::Unavailable
    );
}

#[test]
fn activity_store_downgrades_unmanaged_exact_url_claim_and_redacts_target_value() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let mut event = unmanaged_browser_terminate_event();
    let untrusted_url = "https://private.example/child-profile";
    set_field(
        &mut event,
        constants::browser::INTERVENTION_FIELD_EXACT_URL_CLAIM_STATE,
        BrowserExactUrlClaimState::ExactUrlProven.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::field::INTERVENTION_TARGET_TYPE,
        BrowserInterventionTargetType::Url.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::field::INTERVENTION_TARGET_VALUE,
        untrusted_url,
    );
    set_field(&mut event, constants::field::REQUESTED_URL, untrusted_url);
    set_field(&mut event, constants::field::OBSERVED_URL, untrusted_url);

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(
        row.browser_boundary_state,
        BrowserBoundaryState::UnmanagedBrowserProcess
    );
    assert_eq!(
        row.exact_url_claim_state,
        BrowserExactUrlClaimState::NotClaimed
    );
    assert_eq!(row.requested_url, None);
    assert_eq!(row.observed_url, None);
    assert_eq!(
        row.intervention_target_value,
        constants::browser::INTERVENTION_TARGET_VALUE_REDACTED
    );
    let serialized =
        serde_json::to_string(&read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(serialized.contains(untrusted_url), false);
}

#[test]
fn activity_store_rejects_unmanaged_fallback_without_explicit_process_identity() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let mut event = unmanaged_browser_terminate_event();
    remove_field(&mut event, constants::field::PROCESS_ID);

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(row.process_id, None);
    assert_eq!(
        row.unmanaged_detection_state,
        BrowserUnmanagedDetectionState::Unavailable
    );
    assert_eq!(
        row.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Unavailable
    );
    assert_eq!(
        read_model.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Unavailable
    );
    assert_eq!(
        row.intervention_outcome,
        BrowserInterventionOutcome::ManualRequired
    );
    assert_eq!(
        row.intervention_target_value,
        constants::browser::INTERVENTION_TARGET_VALUE_REDACTED
    );
}

#[test]
fn activity_store_keeps_unsupported_unmanaged_browser_state_unavailable() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let mut event = unmanaged_browser_terminate_event();
    set_field(
        &mut event,
        constants::browser::INTERVENTION_FIELD_BROWSER_BOUNDARY_STATE,
        BrowserBoundaryState::Unsupported.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::browser::INTERVENTION_FIELD_EXACT_URL_CLAIM_STATE,
        BrowserExactUrlClaimState::ExactUrlProven.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::browser::INTERVENTION_FIELD_UNMANAGED_DETECTION_STATE,
        BrowserUnmanagedDetectionState::Unavailable.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::field::INTERVENTION_ACTION,
        BrowserInterventionAction::Unknown.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::field::INTERVENTION_OUTCOME,
        BrowserInterventionOutcome::Unsupported.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::field::UNMANAGED_BROWSER_ENFORCEMENT,
        BrowserUnmanagedEnforcementState::Unsupported.as_protocol_str(),
    );
    remove_field(&mut event, constants::field::UNMANAGED_FALLBACK_ACTION);

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(
        row.browser_boundary_state,
        BrowserBoundaryState::Unsupported
    );
    assert_eq!(
        row.exact_url_claim_state,
        BrowserExactUrlClaimState::NotClaimed
    );
    assert_eq!(
        row.unmanaged_detection_state,
        BrowserUnmanagedDetectionState::Unavailable
    );
    assert_eq!(
        row.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Unavailable
    );
    assert_eq!(
        read_model.unmanaged_browser_enforcement,
        BrowserUnmanagedEnforcementState::Unsupported
    );
    assert_eq!(
        read_model.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Unavailable
    );
}

#[test]
fn activity_store_preserves_degraded_unmanaged_fallback_without_success_claim() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let mut event = unmanaged_browser_terminate_event();
    set_field(
        &mut event,
        constants::browser::INTERVENTION_FIELD_BROWSER_BOUNDARY_STATE,
        BrowserBoundaryState::BrowserLikeProcess.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::browser::INTERVENTION_FIELD_UNMANAGED_DETECTION_STATE,
        BrowserUnmanagedDetectionState::Detected.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::field::INTERVENTION_ACTION,
        BrowserInterventionAction::Unknown.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::field::INTERVENTION_OUTCOME,
        BrowserInterventionOutcome::Failed.as_protocol_str(),
    );
    set_field(
        &mut event,
        constants::field::UNMANAGED_BROWSER_ENFORCEMENT,
        BrowserUnmanagedEnforcementState::Degraded.as_protocol_str(),
    );
    remove_field(&mut event, constants::field::UNMANAGED_FALLBACK_ACTION);

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    let row = &read_model.rows[0];
    assert_eq!(
        row.browser_boundary_state,
        BrowserBoundaryState::BrowserLikeProcess
    );
    assert_eq!(
        row.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Degraded
    );
    assert_eq!(
        read_model.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Degraded
    );
    assert_eq!(row.intervention_outcome, BrowserInterventionOutcome::Failed);
    assert_eq!(row.requested_url, None);
    assert_eq!(row.observed_url, None);
}

#[test]
fn activity_store_replays_browser_interventions_from_encrypted_journal() {
    let journal_path = {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(constants::activity_store::TEST_BROWSER_INTERVENTION_JOURNAL_SUFFIX);
        name.push('.');
        name.push_str(constants::journal::FILE_EXTENSION);
        std::env::temp_dir().join(name)
    };
    let store_path = {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(constants::activity_store::TEST_BROWSER_INTERVENTION_STORE_SUFFIX);
        name.push('.');
        name.push_str(constants::activity_store::FILE_EXTENSION);
        std::env::temp_dir().join(name)
    };
    cleanup_paths(&journal_path, &store_path);
    let key = test_key();
    let mut journal = ActivityJournal::open(journal_path.clone(), key.clone())
        .expect_value(constants::error::JOURNAL_OPENS);
    let event = browser_intervention_event();
    journal
        .append(&event)
        .expect_value(constants::error::JOURNAL_APPENDS);
    let journal_bytes = read(&journal_path).expect_value(constants::error::JOURNAL_READS);
    let reader = ActivityJournal::open(journal_path.clone(), key)
        .expect_value(constants::error::JOURNAL_OPENS);
    let store =
        ActivityStore::open(&store_path).expect_value(constants::error::ACTIVITY_STORE_OPENS);

    let status = store
        .ingest_journal(&reader)
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
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
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);

    let read_model = store
        .browser_intervention_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

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
    assert_eq!(
        read_model.unmanaged_fallback_action,
        BrowserUnmanagedFallbackActionState::Unavailable
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
            intervention_action_id: Some(
                constants::activity_store::TEST_BROWSER_INTERVENTION_ACTION_ID.to_string(),
            ),
            intervention_audit_id: Some(
                constants::activity_store::TEST_BROWSER_INTERVENTION_AUDIT_ID.to_string(),
            ),
            evidence_reference_ids: vec![
                constants::activity_store::TEST_BROWSER_INTERVENTION_EVIDENCE_ID.to_string(),
            ],
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
            unmanaged_fallback_action: BrowserUnmanagedFallbackActionState::Unavailable,
            child_delivery_state: BrowserInterventionDeliveryState::BlockPageRendered,
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

fn unmanaged_browser_terminate_event() -> ActivityEvent {
    browser_intervention_applied_event(
        BrowserInterventionObservation {
            browser_family: Some(BrowserFamily::Chrome),
            browser_channel: Some(BrowserChannel::Stable),
            managed_browser_session_id: None,
            profile_id: None,
            process_id: Some(constants::activity_store::TEST_BROWSER_PROCESS_ID),
            intervention_action_id: Some(
                constants::activity_store::TEST_BROWSER_INTERVENTION_ACTION_ID.to_string(),
            ),
            intervention_audit_id: Some(
                constants::activity_store::TEST_BROWSER_INTERVENTION_AUDIT_ID.to_string(),
            ),
            evidence_reference_ids: vec![
                constants::activity_store::TEST_BROWSER_INTERVENTION_EVIDENCE_ID.to_string(),
            ],
            policy_decision_id: Some(
                constants::activity_store::TEST_POLICY_DECISION_ID.to_string(),
            ),
            decision_source: BrowserInterventionDecisionSource::ParentRule,
            intervention_action: BrowserInterventionAction::TerminateProcess,
            intervention_target_type: BrowserInterventionTargetType::BrowserProcess,
            intervention_target_value: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
            requested_url: None,
            observed_url: None,
            intervention_mechanism: BrowserInterventionMechanism::OsAppControl,
            intervention_outcome: BrowserInterventionOutcome::Terminated,
            browser_boundary_state: BrowserBoundaryState::UnmanagedBrowserProcess,
            exact_url_claim_state: BrowserExactUrlClaimState::NotClaimed,
            unmanaged_detection_state: BrowserUnmanagedDetectionState::Terminated,
            unmanaged_fallback_action: BrowserUnmanagedFallbackActionState::TerminateProcess,
            child_delivery_state: BrowserInterventionDeliveryState::ManualRequired,
            managed_session_intervention_capability:
                BrowserInterventionCapabilityState::NeedsManagedSession,
            unmanaged_browser_enforcement: BrowserUnmanagedEnforcementState::TerminateProcess,
            reason: Some(constants::value::MANAGED_BROWSER_UNMANAGED_PROCESS.to_string()),
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        1,
    )
}

fn remove_browser_claim_fields(event: &mut ActivityEvent) {
    event.fields = event
        .fields
        .clone()
        .into_inner()
        .into_iter()
        .filter(|(key, _)| {
            key != constants::browser::INTERVENTION_FIELD_BROWSER_BOUNDARY_STATE
                && key != constants::browser::INTERVENTION_FIELD_EXACT_URL_CLAIM_STATE
                && key != constants::browser::INTERVENTION_FIELD_UNMANAGED_DETECTION_STATE
        })
        .collect();
}

fn remove_browser_intervention_proof_fields(event: &mut ActivityEvent) {
    event.fields = event
        .fields
        .clone()
        .into_inner()
        .into_iter()
        .filter(|(key, _)| {
            key != constants::field::BROWSER_INTERVENTION_ACTION_ID
                && key != constants::field::BROWSER_INTERVENTION_AUDIT_ID
                && key != constants::field::EVIDENCE_REFERENCE_IDS
                && key != constants::field::CHILD_DELIVERY_STATE
        })
        .collect();
}

fn set_field(event: &mut ActivityEvent, key: &str, value: &str) {
    event
        .fields
        .insert(key.to_string(), LogFieldValue::String(value.to_string()));
}

fn remove_field(event: &mut ActivityEvent, key: &str) {
    event.fields = event
        .fields
        .clone()
        .into_inner()
        .into_iter()
        .filter(|(field, _)| field != key)
        .collect();
}

fn cleanup_paths(journal_path: &Path, store_path: &Path) {
    let _ = remove_file(journal_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.to_path_buf();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.to_path_buf();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([11; JOURNAL_KEY_BYTES])
}
