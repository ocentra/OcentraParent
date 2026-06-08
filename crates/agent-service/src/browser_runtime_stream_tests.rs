use std::fs::remove_file;

use ocentra_parent_agent_core::{
    browser_tab_observation_event, request_browser_runtime_action_intent_handoff_for_input,
    request_browser_runtime_action_intent_status_for_input, ActivityStore,
    BrowserBridgeTargetObservation, BrowserRuntimeInput, BrowserRuntimePhase,
};
use ocentra_parent_agent_protocol::{
    constants, policy_constants, ActivityEvent, AgentCommandEnvelope, AgentCommandName,
    AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserEvidenceReadModel, BrowserFamily, BrowserQueryVisibilityLabel,
    BrowserTabEvidence, LogFieldValue, LogFields, ParentEvidenceReference,
    ParentEvidenceReferenceKind, PolicyAction, PolicyDecision, PolicyDecisionHandoffState,
    PolicyPreviewReadModel, PolicyPreviewReadModelRow, PolicyTarget, PolicyTargetType,
    AGENT_PROTOCOL_SCHEMA_VERSION, BROWSER_EVIDENCE_SCHEMA_VERSION, POLICY_DRY_RUN_SCHEMA_VERSION,
};
use serde_json::Value;

use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK,
    browser_runtime_stream_payload::{
        browser_runtime_event_chain_stream_payload,
        stream_browser_runtime_event_chain_for_read_model_with_policy_preview,
        BrowserRuntimeServiceStreamReport,
    },
    lan_pairing::LanPairingRuntime,
    websocket::handle_command_text_for_test,
};

mod browser_runtime_social_provider_receipt_service_status_tests;

const BROWSER_ACTION_INTENT_EXECUTION_FIELDS: [&str; 4] = [
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_DISPATCH_ATTEMPTS,
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_ADAPTER_EXECUTIONS,
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_INTERVENTION_EXECUTIONS,
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_ENFORCEMENT_EXECUTIONS,
];

const BROWSER_ACTION_INTENT_CHILD_STATUS_REF_FIELDS: [&str; 3] = [
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_COMMAND_REFS,
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_EVENT_REFS,
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_PARENT_READ_MODEL_REFS,
];

macro_rules! assert_child_status_payload_empty {
    ($payload:expr) => {{
        assert_eq!(
            $payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_ROWS),
            Some(&LogFieldValue::Number(0.0))
        );
        let empty_array =
            LogFieldValue::String(serde_json::to_string(&Vec::<String>::new()).unwrap());
        for field in BROWSER_ACTION_INTENT_CHILD_STATUS_REF_FIELDS {
            assert_eq!($payload.get(field), Some(&empty_array));
        }
    }};
}

#[tokio::test]
async fn service_browser_runtime_streams_protocol_event_chain_entries() {
    let report = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &read_model(vec![managed_row()]),
        None,
    )
    .await;
    let payload = browser_runtime_event_chain_stream_payload(&report);
    let entries = stream_entries(&payload);

    assert_eq!(report.observed_rows, 1);
    assert_eq!(
        report.streamed_events,
        BrowserRuntimePhase::ordered_chain().len() - 6
    );
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.exact_url_rows, 1);
    assert_eq!(report.manual_required_rows, 0);
    assert_eq!(report.intervention_command_events, 0);
    assert_eq!(report.read_model_projection_events, 1);
    assert_eq!(report.action_intent_candidates, 0);
    assert!(report.action_intent_handoff_outbox_refs.is_empty());
    assert!(report.action_intent_handoff_refs.is_empty());
    assert_eq!(report.action_intent_dispatch_attempts, 0);
    assert_eq!(report.action_intent_adapter_executions, 0);
    assert_eq!(report.action_intent_child_intervention_executions, 0);
    assert_eq!(report.action_intent_enforcement_executions, 0);
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CANDIDATES),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_child_status_payload_empty!(payload);
    assert_eq!(
        entries[0][constants::field::EVENT_TYPE],
        constants::browser::EVENT_BROWSER_EVIDENCE_OBSERVED
    );
    assert!(entries[0][constants::field::EVENT_REF]
        .as_str()
        .unwrap_or_default()
        .ends_with(constants::browser::EVENT_BROWSER_EVIDENCE_OBSERVED));
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::EXACT_URL_CLAIMED],
        true
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::CAPABILITY_STATUS],
        constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::CUSTODY_LABEL],
        constants::browser::CUSTODY_CHILD_DEVICE_LOCAL
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::QUERY_VISIBILITY],
        constants::browser::QUERY_VISIBILITY_LIVE_LOCAL
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::DEGRADED_REASON],
        Value::Null
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::POLICY_DRY_RUN],
        false
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::ADAPTER_DISPATCH_CLAIMED],
        false
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::POLICY_PREVIEW_ID],
        Value::Null
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::ACTION_INTENT_ID],
        Value::Null
    );
    assert_eq!(
        entries.last().unwrap()[constants::field::EVENT_TYPE],
        constants::browser::EVENT_BROWSER_READ_MODEL_PROJECTED
    );
}

#[tokio::test]
async fn service_browser_runtime_action_intent_status_projects_pending_candidate() {
    let status = request_browser_runtime_action_intent_status_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await
    .unwrap()
    .request_report
    .response;
    let mut report = BrowserRuntimeServiceStreamReport::default();
    report.record_action_intent_status(&status);
    let payload = browser_runtime_event_chain_stream_payload(&report);

    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CANDIDATES),
        Some(&LogFieldValue::Number(1.0))
    );
    for field in BROWSER_ACTION_INTENT_EXECUTION_FIELDS {
        assert_eq!(payload.get(field), Some(&LogFieldValue::Number(0.0)));
    }

    let handoff = request_browser_runtime_action_intent_handoff_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await
    .unwrap()
    .request_report
    .response;
    let mut report = BrowserRuntimeServiceStreamReport::default();
    report.record_action_intent_handoff(&handoff);
    let payload = browser_runtime_event_chain_stream_payload(&report);

    assert_eq!(report.action_intent_handoff_candidates, 1);
    assert_eq!(
        report.action_intent_handoff_outbox_refs,
        vec![constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF.to_string()]
    );
    assert_eq!(
        report.action_intent_handoff_refs,
        vec![constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF.to_string()]
    );
    assert_eq!(report.action_intent_dispatch_attempts, 0);
    assert_eq!(report.action_intent_adapter_executions, 0);
    assert_eq!(report.action_intent_child_intervention_executions, 0);
    assert_eq!(report.action_intent_enforcement_executions, 0);
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_CANDIDATES),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_OUTBOX_REFS),
        Some(&LogFieldValue::String(
            serde_json::to_string(&vec![
                constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF
            ])
            .unwrap()
        ))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REFS),
        Some(&LogFieldValue::String(
            serde_json::to_string(&vec![
                constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF
            ])
            .unwrap()
        ))
    );
    for field in BROWSER_ACTION_INTENT_EXECUTION_FIELDS {
        assert_eq!(payload.get(field), Some(&LogFieldValue::Number(0.0)));
    }
    assert_child_status_payload_empty!(payload);
}

#[tokio::test]
async fn service_browser_runtime_stream_projects_store_backed_policy_preview_candidate() {
    let read_model = read_model(vec![managed_row()]);
    let policy_preview = policy_preview_read_model_for_browser(&read_model);
    let report = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &read_model,
        Some(&policy_preview),
    )
    .await;
    let payload = browser_runtime_event_chain_stream_payload(&report);
    let entries = stream_entries(&payload);
    let policy_entry = entries
        .iter()
        .find(|entry| {
            entry[constants::field::EVENT_TYPE]
                == constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED
        })
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(report.action_intent_candidates, 1);
    assert_eq!(report.action_intent_handoff_candidates, 1);
    assert_eq!(
        report.action_intent_handoff_outbox_refs,
        vec![constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF.to_string()]
    );
    assert_eq!(
        report.action_intent_handoff_refs,
        vec![constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF.to_string()]
    );
    assert_eq!(report.action_intent_dispatch_attempts, 0);
    assert_eq!(report.action_intent_adapter_executions, 0);
    assert_eq!(report.action_intent_child_intervention_executions, 0);
    assert_eq!(report.intervention_command_events, 0);
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CANDIDATES),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_CANDIDATES),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_OUTBOX_REFS),
        Some(&LogFieldValue::String(
            serde_json::to_string(&vec![
                constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF
            ])
            .unwrap()
        ))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REFS),
        Some(&LogFieldValue::String(
            serde_json::to_string(&vec![
                constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF
            ])
            .unwrap()
        ))
    );
    assert_child_status_payload_empty!(payload);
    assert_eq!(
        policy_entry[constants::field::PAYLOAD][constants::field::POLICY_PREVIEW_ID],
        policy_constants::TEST_PREVIEW_ID
    );
    assert_eq!(
        policy_entry[constants::field::PAYLOAD][constants::field::ACTION_INTENT_ID],
        {
            let mut value = String::from(constants::browser::ACTION_INTENT_ID_PREFIX);
            value.push_str(policy_constants::TEST_DECISION_ID);
            value
        }
    );
    assert_eq!(
        policy_entry[constants::field::PAYLOAD][constants::field::POLICY_DRY_RUN],
        true
    );
    assert_eq!(
        policy_entry[constants::field::PAYLOAD][constants::field::ADAPTER_DISPATCH_CLAIMED],
        false
    );
}

#[tokio::test]
async fn service_browser_runtime_stream_keeps_unavailable_rows_manual_required() {
    let report = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &read_model(vec![unavailable_row()]),
        None,
    )
    .await;
    let entries = stream_entries(&browser_runtime_event_chain_stream_payload(&report));
    let event_types = entries
        .iter()
        .map(|entry| {
            entry[constants::field::EVENT_TYPE]
                .as_str()
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    assert_eq!(report.observed_rows, 1);
    assert_eq!(
        report.streamed_events,
        BrowserRuntimePhase::ordered_chain().len() - 6
    );
    assert_eq!(report.exact_url_rows, 0);
    assert_eq!(report.manual_required_rows, 1);
    assert_eq!(report.intervention_command_events, 0);
    assert!(!event_types.contains(&constants::browser::EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED));
    assert!(!event_types.contains(&constants::browser::EVENT_BROWSER_INTERVENTION_RESULT_OBSERVED));
    assert_eq!(
        entries.last().unwrap()[constants::field::PAYLOAD][constants::field::EXACT_URL_CLAIMED],
        false
    );
    assert_eq!(
        entries.last().unwrap()[constants::field::PAYLOAD][constants::field::CAPABILITY_STATUS],
        constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING
    );
    assert_eq!(
        entries.last().unwrap()[constants::field::PAYLOAD][constants::field::QUERY_VISIBILITY],
        constants::browser::QUERY_VISIBILITY_UNAVAILABLE
    );
    assert_eq!(
        entries.last().unwrap()[constants::field::PAYLOAD][constants::field::DEGRADED_REASON],
        constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS
    );
}

#[tokio::test]
async fn service_browser_runtime_stream_keeps_stale_and_unsupported_rows_parent_visible() {
    let report = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &read_model(vec![stale_row(), unsupported_row()]),
        None,
    )
    .await;
    let payload = browser_runtime_event_chain_stream_payload(&report);
    let entries = stream_entries(&payload);

    assert_eq!(report.observed_rows, 2);
    assert_eq!(report.exact_url_rows, 0);
    assert_eq!(report.manual_required_rows, 2);
    assert_eq!(report.intervention_command_events, 0);
    assert_eq!(report.action_intent_candidates, 0);
    assert_eq!(report.action_intent_handoff_candidates, 0);
    assert!(report.action_intent_handoff_outbox_refs.is_empty());
    assert!(report.action_intent_handoff_refs.is_empty());
    assert_eq!(report.action_intent_dispatch_attempts, 0);
    assert_eq!(report.action_intent_adapter_executions, 0);
    assert_eq!(report.action_intent_child_intervention_executions, 0);
    assert_eq!(report.action_intent_enforcement_executions, 0);
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_EXACT_URL_ROWS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_MANUAL_REQUIRED_ROWS),
        Some(&LogFieldValue::Number(2.0))
    );

    let stale_entry =
        first_entry_with_capability(&entries, constants::browser::CAPABILITY_STATUS_STALE);
    assert_eq!(
        stale_entry[constants::field::PAYLOAD][constants::field::EXACT_URL_CLAIMED],
        false
    );
    assert_eq!(
        stale_entry[constants::field::PAYLOAD][constants::field::DEGRADED_REASON],
        constants::value::BROWSER_BRIDGE_STALE_SESSION
    );

    let unsupported_entry = first_entry_with_capability(
        &entries,
        constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER,
    );
    assert_eq!(
        unsupported_entry[constants::field::PAYLOAD][constants::field::EXACT_URL_CLAIMED],
        false
    );
    assert_eq!(
        unsupported_entry[constants::field::PAYLOAD][constants::field::QUERY_VISIBILITY],
        constants::browser::QUERY_VISIBILITY_UNAVAILABLE
    );
    assert_eq!(
        unsupported_entry[constants::field::PAYLOAD][constants::field::DEGRADED_REASON],
        constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER
    );
}

#[tokio::test]
async fn websocket_browser_runtime_stream_command_reports_store_backed_chain() {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path(constants::activity_store::TEST_CAPTURE_BROWSER_STORE_SUFFIX);
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[browser_activity_event()])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let entries = stream_entries(&event.payload);

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);

    assert_eq!(
        event.event,
        AgentEventName::AgentBrowserRuntimeEventChainStreamReported
    );
    assert_eq!(
        entries.len(),
        BrowserRuntimePhase::ordered_chain().len() - 4
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::BROWSER_RUNTIME_STREAMED_EVENTS),
        Some(&LogFieldValue::Number(
            (BrowserRuntimePhase::ordered_chain().len() - 4) as f64
        ))
    );
    assert_eq!(
        entries.last().unwrap()[constants::field::EVENT_TYPE],
        constants::browser::EVENT_BROWSER_READ_MODEL_PROJECTED
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::CAPABILITY_STATUS],
        constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::QUERY_VISIBILITY],
        constants::browser::QUERY_VISIBILITY_LIVE_LOCAL
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::DEGRADED_REASON],
        constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CANDIDATES),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_child_status_payload_empty!(event.payload);
    assert_eq!(
        event
            .payload
            .get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_DISPATCH_ATTEMPTS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_INTERVENTION_EXECUTIONS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_ENFORCEMENT_EXECUTIONS),
        Some(&LogFieldValue::Number(0.0))
    );
}

fn read_model(rows: Vec<BrowserTabEvidence>) -> BrowserEvidenceReadModel {
    BrowserEvidenceReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: rows.len() as u64,
        latest_event_id: Some(constants::browser::TEST_BROWSER_RUNTIME_JOURNAL_REF.to_string()),
        latest_observed_at: rows.first().map(|row| row.observed_at.clone()),
        capability_status: Some(BrowserCapabilityStatus::TabListOnly),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        rows,
    }
}

fn policy_preview_read_model_for_browser(
    read_model: &BrowserEvidenceReadModel,
) -> PolicyPreviewReadModel {
    let evidence_reference_id = read_model
        .latest_event_id
        .clone()
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    PolicyPreviewReadModel {
        schema_version: POLICY_DRY_RUN_SCHEMA_VERSION.to_string(),
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: policy_constants::PREVIEW_CUSTODY_ACTIVITY_STORE.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        capability_status: policy_constants::PREVIEW_CAPABILITY_READY.to_string(),
        rows: vec![PolicyPreviewReadModelRow {
            preview_id: policy_constants::TEST_PREVIEW_ID.to_string(),
            source_event_id: evidence_reference_id.clone(),
            observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
            target: PolicyTarget {
                target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
                target_type: PolicyTargetType::Domain,
                target_value: constants::activity_store::TEST_BROWSER_DOMAIN.to_string(),
            },
            evidence_references: vec![ParentEvidenceReference {
                evidence_reference_id,
                kind: ParentEvidenceReferenceKind::ActivityEvent,
                observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            }],
            parent_rule_context_references: Vec::new(),
            decision: PolicyDecision {
                schema_version: POLICY_DRY_RUN_SCHEMA_VERSION.to_string(),
                decision_id: policy_constants::TEST_DECISION_ID.to_string(),
                action: PolicyAction::Block,
                reason_codes: vec![policy_constants::TEST_REASON_PARENT_BLOCK.to_string()],
                evidence_references: Vec::new(),
                rule_ids: vec![policy_constants::TEST_BLOCK_RULE_ID.to_string()],
                local_ai_result_id: None,
                dry_run: true,
                enforcement_handoff_state: PolicyDecisionHandoffState::Disabled,
                expires_at: None,
            },
        }],
    }
}

fn managed_row() -> BrowserTabEvidence {
    BrowserTabEvidence {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        browser_evidence_id: constants::browser::TEST_BROWSER_RUNTIME_EVIDENCE_REF.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        fresh_until: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source_id: constants::browser::SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
        adapter_id: constants::browser::ADAPTER_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
        device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        browser_family: BrowserFamily::Edge,
        browser_channel: BrowserChannel::Stable,
        managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        process_id: constants::browser::PROCESS_ID_UNKNOWN,
        window_id: None,
        tab_id: Some(constants::activity_store::TEST_BROWSER_TAB_ID_FROM_TARGET.to_string()),
        target_id: Some(constants::activity_store::TEST_BROWSER_TARGET_ID.to_string()),
        active_state: BrowserActiveTabState::Unknown,
        active_proof_source: BrowserActiveProofSource::TargetListOnly,
        url: constants::activity_store::TEST_BROWSER_URL.to_string(),
        origin: constants::activity_store::TEST_BROWSER_ORIGIN.to_string(),
        domain: constants::activity_store::TEST_BROWSER_DOMAIN.to_string(),
        title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
        capability_status: BrowserCapabilityStatus::TabListOnly,
        degraded_reason: None,
        stale_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}

fn unavailable_row() -> BrowserTabEvidence {
    BrowserTabEvidence {
        managed_browser_session_id: constants::value::EMPTY.to_string(),
        capability_status: BrowserCapabilityStatus::BridgeMissing,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
        degraded_reason: Some(constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string()),
        ..managed_row()
    }
}

fn stale_row() -> BrowserTabEvidence {
    BrowserTabEvidence {
        capability_status: BrowserCapabilityStatus::Stale,
        degraded_reason: Some(constants::value::BROWSER_BRIDGE_STALE_SESSION.to_string()),
        ..managed_row()
    }
}

fn unsupported_row() -> BrowserTabEvidence {
    BrowserTabEvidence {
        browser_family: BrowserFamily::Firefox,
        browser_channel: BrowserChannel::Stable,
        capability_status: BrowserCapabilityStatus::UnsupportedBrowser,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
        degraded_reason: Some(
            constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER.to_string(),
        ),
        ..managed_row()
    }
}

fn browser_activity_event() -> ActivityEvent {
    browser_tab_observation_event(
        BrowserBridgeTargetObservation {
            browser_family: BrowserFamily::Edge,
            browser_channel: BrowserChannel::Stable,
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: constants::activity_store::TEST_BROWSER_PROCESS_ID,
            target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
            tab_id: Some(constants::activity_store::TEST_BROWSER_TAB_ID.to_string()),
            window_id: Some(constants::activity_store::TEST_BROWSER_WINDOW_ID.to_string()),
            active_state: BrowserActiveTabState::Unknown,
            active_proof_source: BrowserActiveProofSource::TargetListOnly,
            url: constants::activity_store::TEST_BROWSER_URL.to_string(),
            title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
            capability_status: BrowserCapabilityStatus::TabListOnly,
            degraded_reason: Some(constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string()),
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET)
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::BROWSER_RUNTIME_EVENT_CHAIN_STREAM_REPORTED.to_string(),
        sent_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentBrowserRuntimeEventChainStreamGet,
        payload: LogFields::new(),
    }
}

fn stream_entries(payload: &LogFields) -> Vec<Value> {
    match payload.get(constants::field::BROWSER_RUNTIME_EVENT_CHAIN_STREAM) {
        Some(LogFieldValue::String(text)) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn first_entry_with_capability<'a>(entries: &'a [Value], capability: &str) -> &'a Value {
    entries
        .iter()
        .find(|entry| {
            entry[constants::field::PAYLOAD][constants::field::CAPABILITY_STATUS] == capability
        })
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn temp_path(suffix: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_path(path: &std::path::PathBuf) {
    let _ = remove_file(path);
    let mut wal_path = path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
