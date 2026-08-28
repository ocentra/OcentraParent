use ocentra_parent_agent_core::browser_event_runtime::{
    request_browser_runtime_action_intent_handoff_for_input,
    request_browser_runtime_action_intent_status_for_input, BrowserRuntimeInput,
};
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecision;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecisionHandoffState;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::activity::policy::POLICY_DRY_RUN_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModel;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModelRow;
use ocentra_parent_agent_protocol::browser::BrowserActiveProofSource;
use ocentra_parent_agent_protocol::browser::BrowserActiveTabState;
use ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus;
use ocentra_parent_agent_protocol::browser::BrowserChannel;
use ocentra_parent_agent_protocol::browser::BrowserCustodyLabel;
use ocentra_parent_agent_protocol::browser::BrowserFamily;
use ocentra_parent_agent_protocol::browser::BrowserRuntimePhase;
use ocentra_parent_agent_protocol::browser::BROWSER_EVIDENCE_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::browser_read_model::BrowserTabEvidence;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use serde_json::Value;
use std::primitive::str as TestStr;
use std::string::String as TestString;
use std::{error::Error, io::Error as IoError};

use crate::{
    browser_runtime_stream_api::action_intent_child_status_from_handoff,
    browser_runtime_stream_payload::{
        browser_runtime_event_chain_stream_payload,
        stream_browser_runtime_event_chain_for_read_model_with_policy_preview,
        BrowserRuntimeServiceStreamReport,
    },
    test_invariants::require_some,
};

#[path = "browser_runtime_stream_tests/browser_runtime_service_stream_eventing_tests.rs"]
mod browser_runtime_service_stream_eventing_tests;
#[path = "browser_runtime_stream_tests/browser_runtime_social_provider_receipt_service_status_tests.rs"]
mod browser_runtime_social_provider_receipt_service_status_tests;
#[path = "browser_runtime_stream_tests/browser_runtime_stream_test_assertions.rs"]
mod browser_runtime_stream_test_assertions;

use browser_runtime_stream_test_assertions::{
    assert_action_intent_execution_payload_zero, assert_action_intent_handoff_payload_refs,
    assert_action_intent_handoff_report_ready,
};

const BROWSER_ACTION_INTENT_CHILD_STATUS_REF_FIELDS: [&TestStr; 3] = [
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_COMMAND_REFS,
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_EVENT_REFS,
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_PARENT_READ_MODEL_REFS,
];

pub(super) type TestResult = Result<(), Box<dyn Error>>;

macro_rules! assert_child_status_payload_empty {
    ($payload:expr) => {{
        assert_eq!(
            $payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_ROWS),
            Some(&LogFieldValue::Number(0.0))
        );
        let empty_array = LogFieldValue::String(TestString::from("[]"));
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
    let last_entry = require_some(entries.last(), constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(
        last_entry[constants::field::EVENT_TYPE],
        constants::browser::EVENT_BROWSER_READ_MODEL_PROJECTED
    );
}

#[tokio::test]
async fn service_browser_runtime_action_intent_status_projects_pending_candidate() -> TestResult {
    let status = request_browser_runtime_action_intent_status_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await?
    .request_report
    .response;
    let mut report = BrowserRuntimeServiceStreamReport::default();
    report.record_action_intent_status(&status);
    let payload = browser_runtime_event_chain_stream_payload(&report);

    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CANDIDATES),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_action_intent_execution_payload_zero(&payload);

    let handoff = request_browser_runtime_action_intent_handoff_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await?
    .request_report
    .response;
    let mut report = BrowserRuntimeServiceStreamReport::default();
    report.record_action_intent_handoff(&handoff);
    let payload = browser_runtime_event_chain_stream_payload(&report);

    assert_eq!(report.action_intent_dispatch_attempts, 0);
    assert_eq!(report.action_intent_adapter_executions, 0);
    assert_eq!(report.action_intent_child_intervention_executions, 0);
    assert_eq!(report.action_intent_enforcement_executions, 0);
    assert_action_intent_handoff_report_ready(
        &report,
        &payload,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID,
    );
    assert_action_intent_handoff_payload_refs(
        &payload,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID,
    );
    assert_action_intent_execution_payload_zero(&payload);
    assert_child_status_payload_empty!(payload);

    let handoff = request_browser_runtime_action_intent_handoff_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await?;
    let child_status_response =
        action_intent_child_status_from_handoff(&handoff.request_report.response).await;
    assert!(child_status_response.is_none());
    let report = BrowserRuntimeServiceStreamReport::default();
    let payload = browser_runtime_event_chain_stream_payload(&report);

    assert_child_status_payload_empty!(payload);

    Ok(())
}

#[tokio::test]
async fn service_browser_runtime_stream_projects_store_backed_policy_preview_candidate(
) -> TestResult {
    let read_model = read_model(vec![managed_row()]);
    let policy_preview = policy_preview_read_model_for_browser(&read_model)?;
    let report = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &read_model,
        Some(&policy_preview),
    )
    .await;
    let payload = browser_runtime_event_chain_stream_payload(&report);
    let entries = stream_entries(&payload);
    let policy_entry = require_some(
        entries.iter().find(|entry| {
            entry[constants::field::EVENT_TYPE]
                == constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED
        }),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(report.action_intent_candidates, 1);
    assert_eq!(report.action_intent_dispatch_attempts, 0);
    assert_eq!(report.action_intent_adapter_executions, 0);
    assert_eq!(report.action_intent_child_intervention_executions, 0);
    assert_eq!(report.intervention_command_events, 0);
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CANDIDATES),
        Some(&LogFieldValue::Number(1.0))
    );
    let expected_action_intent_id = {
        let mut value = TestString::from(constants::browser::ACTION_INTENT_ID_PREFIX);
        value.push_str(policy_constants::TEST_DECISION_ID);
        value
    };
    assert_action_intent_handoff_report_ready(&report, &payload, &expected_action_intent_id);
    assert_action_intent_handoff_payload_refs(&payload, &expected_action_intent_id);
    assert_child_status_payload_empty!(payload);
    assert_eq!(
        policy_entry[constants::field::PAYLOAD][constants::field::POLICY_PREVIEW_ID],
        policy_constants::TEST_PREVIEW_ID
    );
    assert_eq!(
        policy_entry[constants::field::PAYLOAD][constants::field::ACTION_INTENT_ID],
        expected_action_intent_id
    );
    assert_eq!(
        policy_entry[constants::field::PAYLOAD][constants::field::POLICY_DRY_RUN],
        true
    );
    assert_eq!(
        policy_entry[constants::field::PAYLOAD][constants::field::ADAPTER_DISPATCH_CLAIMED],
        false
    );

    Ok(())
}

#[tokio::test]
async fn service_browser_runtime_stream_reports_unavailable_rows_fail_closed() {
    let report = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &read_model(vec![unavailable_row()]),
        None,
    )
    .await;
    let payload = browser_runtime_event_chain_stream_payload(&report);
    let entries = stream_entries(&payload);

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.streamed_events, 0);
    assert_eq!(report.failed_rows, 1);
    assert_eq!(report.exact_url_rows, 0);
    assert_eq!(report.manual_required_rows, 1);
    assert_eq!(report.intervention_command_events, 0);
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_EXACT_URL_ROWS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_STREAMED_EVENTS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_FAILED_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_MANUAL_REQUIRED_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert!(entries.is_empty());
}

#[tokio::test]
async fn service_browser_runtime_stream_reports_stale_and_unsupported_rows_fail_closed() {
    let report = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &read_model(vec![stale_row(), unsupported_row()]),
        None,
    )
    .await;
    let payload = browser_runtime_event_chain_stream_payload(&report);
    let entries = stream_entries(&payload);

    assert_eq!(report.observed_rows, 2);
    assert_eq!(report.streamed_events, 0);
    assert_eq!(report.failed_rows, 2);
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
        payload.get(constants::field::BROWSER_RUNTIME_STREAMED_EVENTS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_FAILED_ROWS),
        Some(&LogFieldValue::Number(2.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_MANUAL_REQUIRED_ROWS),
        Some(&LogFieldValue::Number(2.0))
    );
    assert!(entries.is_empty());
}

pub(super) fn read_model(rows: Vec<BrowserTabEvidence>) -> BrowserEvidenceReadModel {
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

pub(super) fn policy_preview_read_model_for_browser(
    read_model: &BrowserEvidenceReadModel,
) -> Result<PolicyPreviewReadModel, IoError> {
    let evidence_reference_id = read_model
        .latest_event_id
        .clone()
        .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?;
    Ok(PolicyPreviewReadModel {
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
            policy_preview_save_state: None,
            policy_preview_manual_review_state: None,
            policy_preview_target_state: None,
            policy_preview_target_explanation_code: None,
            policy_preview_finding_kinds: None,
            policy_source_status: None,
            policy_source_surface: None,
            policy_request_origin: None,
            policy_assistant_confirmation_state: None,
            policy_request_status: None,
            policy_approval_id: None,
            policy_override_id: None,
            policy_replay_of_approval_id: None,
            policy_reviewed_by_actor_id: None,
            policy_reviewed_by_actor_role: None,
            policy_reviewed_at: None,
            policy_audit_reference_id: None,
            network_evidence_mapping: None,
            confirmation_context: None,
        }],
    })
}

pub(super) fn managed_row() -> BrowserTabEvidence {
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

pub(super) fn unavailable_row() -> BrowserTabEvidence {
    BrowserTabEvidence {
        managed_browser_session_id: constants::value::EMPTY.to_string(),
        capability_status: BrowserCapabilityStatus::BridgeMissing,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
        degraded_reason: Some(constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string()),
        ..managed_row()
    }
}

pub(super) fn stale_row() -> BrowserTabEvidence {
    BrowserTabEvidence {
        capability_status: BrowserCapabilityStatus::Stale,
        degraded_reason: Some(constants::value::BROWSER_BRIDGE_STALE_SESSION.to_string()),
        ..managed_row()
    }
}

pub(super) fn unsupported_row() -> BrowserTabEvidence {
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

pub(super) fn stream_entries(payload: &LogFields) -> Vec<Value> {
    match payload.get(constants::field::BROWSER_RUNTIME_EVENT_CHAIN_STREAM) {
        Some(LogFieldValue::String(text)) => serde_json::from_str(text).unwrap_or_default(),
        _ => Vec::new(),
    }
}
