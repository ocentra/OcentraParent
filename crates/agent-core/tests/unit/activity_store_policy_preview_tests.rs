use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecisionHandoffState;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyAssistantConfirmationState;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModelRow;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewTargetState;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestOrigin;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicySourceStatus;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicySourceSurface;
use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants as policy;
use std::fmt::Debug;

use crate::test_text::{TestResult, TestText};
use crate::{
    activity_store_policy_preview_support::{
        browser_event, network_flow_event, network_flow_event_at, network_retention_deleted_event,
        network_retention_deleted_event_at, parent_rule_context,
    },
    ActivityStore,
};

#[test]
fn policy_preview_read_model_evaluates_stored_browser_evidence_without_enforcement() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let event = browser_event();
    ok(
        store.ingest_events(std::slice::from_ref(&event)),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;

    let read_model = ok(
        store.policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

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
    assert_eq!(row.policy_preview_target_state, None);
    assert_eq!(row.policy_preview_target_explanation_code, None);
    assert_eq!(row.policy_preview_finding_kinds, None);
    assert_eq!(row.policy_source_status, None);
    assert_eq!(row.policy_request_status, None);
    assert_eq!(row.policy_approval_id, None);
    assert_eq!(row.policy_override_id, None);
    assert_eq!(row.policy_audit_reference_id, None);
    Ok(())
}

#[test]
fn policy_preview_read_model_prefers_explicit_target_fields_and_projects_policy_lifecycle(
) -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    ok(
        store.ingest_events(&[policy_preview_confirmed_target_event()]),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;

    let read_model = ok(
        store.policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    let row = &read_model.rows[0];
    assert_policy_preview_confirmed_target_row(row);
    Ok(())
}

#[test]
fn policy_preview_read_model_evaluates_stored_network_flow_evidence_without_enforcement(
) -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let event = network_flow_event();
    let network_evidence_id = event.evidence[0].evidence_id.clone();
    ok(
        store.ingest_events(std::slice::from_ref(&event)),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    ok(
        store.replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: event.subject.subject_id.clone(),
                target_type: PolicyTargetType::Domain,
                target_value: constants::activity_store::TEST_NETWORK_DOMAIN.to_string(),
            },
            policy::TEST_BLOCK_RULE_ID,
            PolicyAction::Block,
            policy::TEST_REASON_PARENT_BLOCK,
            vec![
                crate::test_text::TestText::from_display(event.event_id.clone()),
                crate::test_text::TestText::from_display(network_evidence_id.clone()),
            ],
        )]),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;

    let read_model = ok(
        store.policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

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
    assert_network_evidence_refs(row, &network_evidence_id);
    assert_network_parent_rule_refs(row, &network_evidence_id);
    assert_grade_b_network_mapping(row)?;
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
    Ok(())
}

fn assert_network_evidence_refs(
    row: &PolicyPreviewReadModelRow,
    network_evidence_id: impl std::fmt::Display,
) {
    let network_evidence_id = network_evidence_id.to_string();
    assert_eq!(row.evidence_references.len(), 2);
    assert_eq!(
        row.evidence_references[0].kind,
        ParentEvidenceReferenceKind::ActivityEvent
    );
    assert_eq!(
        row.evidence_references[0].evidence_reference_id,
        row.source_event_id
    );
    assert_eq!(
        row.evidence_references[1].kind,
        ParentEvidenceReferenceKind::QueryStoreSummary
    );
    assert_eq!(
        row.evidence_references[1].evidence_reference_id,
        network_evidence_id
    );
}

fn assert_network_parent_rule_refs(
    row: &PolicyPreviewReadModelRow,
    network_evidence_id: impl std::fmt::Display,
) {
    let network_evidence_id = network_evidence_id.to_string();
    assert_eq!(row.parent_rule_context_references.len(), 1);
    assert_eq!(
        row.parent_rule_context_references[0].target_evidence_refs,
        vec![row.source_event_id.clone(), network_evidence_id]
    );
}

#[test]
fn policy_preview_read_model_marks_unsupported_browser_targets_as_not_ready() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let mut event = browser_event();
    event.fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(
            constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER.to_string(),
        ),
    );
    event.fields.insert(
        constants::field::DEGRADED_REASON.to_string(),
        LogFieldValue::String(
            constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER.to_string(),
        ),
    );
    ok(
        store.ingest_events(std::slice::from_ref(&event)),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;

    let read_model = ok(
        store.policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    let row = &read_model.rows[0];
    assert_eq!(
        row.policy_preview_target_state,
        Some(PolicyPreviewTargetState::Unsupported)
    );
    assert_eq!(
        row.policy_preview_target_explanation_code.as_deref(),
        Some(constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER)
    );
    assert_eq!(
        row.policy_preview_finding_kinds.as_deref(),
        Some("unsupported-target")
    );
    Ok(())
}

#[test]
fn policy_preview_read_model_marks_manual_required_browser_targets_as_not_ready() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let mut event = browser_event();
    event.fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING.to_string()),
    );
    ok(
        store.ingest_events(std::slice::from_ref(&event)),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;

    let read_model = ok(
        store.policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    let row = &read_model.rows[0];
    assert_eq!(
        row.policy_preview_target_state,
        Some(PolicyPreviewTargetState::ManualRequired)
    );
    assert_eq!(
        row.policy_preview_target_explanation_code.as_deref(),
        Some(constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING)
    );
    assert_eq!(
        row.policy_preview_finding_kinds.as_deref(),
        Some("manual-required-target")
    );
    Ok(())
}

#[test]
fn policy_preview_read_model_fails_closed_when_network_mapping_refs_are_malformed() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let event = network_flow_event();
    ok(
        store.ingest_events(std::slice::from_ref(&event)),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    ok(
        store.replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: event.subject.subject_id.clone(),
                target_type: PolicyTargetType::Domain,
                target_value: constants::activity_store::TEST_NETWORK_DOMAIN.to_string(),
            },
            constants::value::EMPTY,
            PolicyAction::Block,
            policy::TEST_REASON_PARENT_BLOCK,
            vec![crate::test_text::TestText::from_display(event.event_id)],
        )]),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;

    let read_model = ok(
        store.policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    let row = &read_model.rows[0];
    assert_grade_b_network_mapping(row)?;
    assert_eq!(row.decision.action, PolicyAction::AskParent);
    assert_eq!(
        row.decision.reason_codes,
        vec![
            policy::TEST_REASON_PARENT_BLOCK.to_string(),
            policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW.to_string()
        ]
    );
    assert!(row.decision.dry_run);
    assert_eq!(
        row.decision.enforcement_handoff_state,
        PolicyDecisionHandoffState::Disabled
    );
    Ok(())
}

fn assert_grade_b_network_mapping(row: &PolicyPreviewReadModelRow) -> TestResult {
    let network_mapping = some(
        row.network_evidence_mapping.as_ref(),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;
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
    Ok(())
}

#[test]
fn policy_preview_read_model_excludes_network_flow_deleted_by_retention_tombstone() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let event = network_flow_event();
    let deleted_event_id = event.event_id.clone();
    ok(
        store.ingest_events(&[
            event,
            network_retention_deleted_event(crate::test_text::TestText::from_display(
                deleted_event_id.clone(),
            )),
        ]),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    ok(
        store.replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: constants::activity_store::TEST_NETWORK_EVENT_ID.to_string(),
                target_type: PolicyTargetType::Domain,
                target_value: constants::activity_store::TEST_NETWORK_DOMAIN.to_string(),
            },
            policy::TEST_BLOCK_RULE_ID,
            PolicyAction::Block,
            policy::TEST_REASON_PARENT_BLOCK,
            vec![crate::test_text::TestText::from_display(deleted_event_id)],
        )]),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;

    let read_model = ok(
        store.policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.capability_status,
        policy::PREVIEW_CAPABILITY_NO_EVIDENCE
    );
    Ok(())
}

#[test]
fn policy_preview_read_model_applies_retention_tombstones_before_limit() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let event = network_flow_event_at(constants::activity_store::TEST_THIRD_OBSERVED_AT, 1);
    let deleted_event_id = event.event_id.clone();
    let tombstone = network_retention_deleted_event_at(
        crate::test_text::TestText::from_display(deleted_event_id.clone()),
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    ok(
        store.ingest_events(&[tombstone, event]),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    ok(
        store.replace_parent_rule_contexts(&[parent_rule_context(
            PolicyTarget {
                target_id: constants::activity_store::TEST_NETWORK_EVENT_ID.to_string(),
                target_type: PolicyTargetType::Domain,
                target_value: constants::activity_store::TEST_NETWORK_DOMAIN.to_string(),
            },
            policy::TEST_BLOCK_RULE_ID,
            PolicyAction::Block,
            policy::TEST_REASON_PARENT_BLOCK,
            vec![crate::test_text::TestText::from_display(deleted_event_id)],
        )]),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;

    let read_model = ok(
        store.policy_preview_read_model(1, constants::activity_store::TEST_THIRD_OBSERVED_AT),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.capability_status,
        policy::PREVIEW_CAPABILITY_NO_EVIDENCE
    );
    Ok(())
}

#[test]
fn policy_preview_read_model_reports_empty_store_without_inventing_rows() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;

    let read_model = ok(
        store.policy_preview_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.capability_status,
        policy::PREVIEW_CAPABILITY_NO_EVIDENCE
    );
    Ok(())
}

fn ok<T, E: Debug>(result: Result<T, E>, context: impl std::fmt::Display) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{}: {error:?}", context)))
}

fn some<T>(value: Option<T>, context: impl std::fmt::Display) -> Result<T, TestText> {
    value.ok_or_else(|| TestText::from_display(context))
}

fn policy_preview_confirmed_target_event() -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(policy::TARGET_TYPE_APP.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String("discord.exe".to_string()),
    );
    fields.insert(
        constants::field::PROCESS_NAME.to_string(),
        LogFieldValue::String("discord.exe".to_string()),
    );
    fields.insert(
        constants::field::POLICY_SOURCE_STATUS.to_string(),
        LogFieldValue::String(constants::policy_control::source::STATUS_CONFIRMED.to_string()),
    );
    fields.insert(
        constants::field::POLICY_SOURCE_SURFACE.to_string(),
        LogFieldValue::String(constants::policy_control::source::SURFACE_AI_PREVIEW.to_string()),
    );
    fields.insert(
        constants::field::POLICY_REQUEST_ORIGIN.to_string(),
        LogFieldValue::String("assistant-draft".to_string()),
    );
    fields.insert(
        constants::field::POLICY_ASSISTANT_CONFIRMATION_STATE.to_string(),
        LogFieldValue::String("parent-confirmed".to_string()),
    );
    fields.insert(
        constants::field::POLICY_REQUEST_STATUS.to_string(),
        LogFieldValue::String(
            constants::policy_control::request::STATUS_PENDING_PARENT_REVIEW.to_string(),
        ),
    );
    fields.insert(
        constants::field::POLICY_REVIEWED_BY_ACTOR_ID.to_string(),
        LogFieldValue::String("parent-1".to_string()),
    );
    fields.insert(
        constants::field::POLICY_REVIEWED_BY_ACTOR_ROLE.to_string(),
        LogFieldValue::String("parent".to_string()),
    );
    fields.insert(
        constants::field::POLICY_REVIEWED_AT.to_string(),
        LogFieldValue::String("2026-06-18T10:05:00Z".to_string()),
    );
    fields.insert(
        constants::field::POLICY_AUDIT_REFERENCE_ID.to_string(),
        LogFieldValue::String("audit.policy-request.confirmed".to_string()),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: "audit.policy-request.confirmed".to_string(),
        observed_at: "2026-06-18T10:05:00Z".to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            observer: ActivityObserver::AgentService,
            source_id: "policy-request-assistant-preview-confirm".to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: "policy-target-ref-1".to_string(),
            display_name: Some("Discord".to_string()),
        },
        fields,
        evidence: Vec::new(),
    }
}

fn assert_policy_preview_confirmed_target_row(row: &PolicyPreviewReadModelRow) {
    assert_eq!(row.target.target_type, PolicyTargetType::App);
    assert_eq!(row.target.target_value, "discord.exe");
    assert_eq!(
        row.policy_source_status,
        Some(PolicySourceStatus::Confirmed)
    );
    assert_eq!(
        row.policy_source_surface,
        Some(PolicySourceSurface::AiPreview)
    );
    assert_eq!(
        row.policy_request_origin,
        Some(PolicyRequestOrigin::AssistantDraft)
    );
    assert_eq!(
        row.policy_assistant_confirmation_state,
        Some(PolicyAssistantConfirmationState::ParentConfirmed)
    );
    assert_eq!(
        row.policy_request_status,
        Some(PolicyRequestStatus::PendingParentReview)
    );
    assert_eq!(row.policy_reviewed_by_actor_id.as_deref(), Some("parent-1"));
    assert_eq!(row.policy_reviewed_by_actor_role.as_deref(), Some("parent"));
    assert_eq!(
        row.policy_reviewed_at.as_deref(),
        Some("2026-06-18T10:05:00Z")
    );
    assert_eq!(
        row.policy_audit_reference_id.as_deref(),
        Some("audit.policy-request.confirmed")
    );
}
