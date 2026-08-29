use std::collections::BTreeSet;

use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::app_game::{
    AppGameEvidenceClaim, AppGameIdentity, APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_JOURNAL_AUTHORITY_SUBJECT_ID,
    APP_GAME_JOURNAL_CLASSIFIER_SUBJECT_ID, APP_GAME_JOURNAL_EVIDENCE_CLAIM_SUBJECT_ID,
    APP_GAME_JOURNAL_IDENTITY_SUBJECT_ID, APP_GAME_JOURNAL_ROW_KIND_AI_CLASSIFIER_RESULT,
    APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT, APP_GAME_JOURNAL_ROW_KIND_APPROVAL_AUTHORITY,
    APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM, APP_GAME_JOURNAL_ROW_KIND_IDENTITY,
    APP_GAME_JOURNAL_ROW_KIND_PLATFORM_AUTHORITY_MATRIX, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameAiClassifierResult, AppGameControlActionResult, AppGameControlApprovalAuthority,
    AppGamePlatformAuthorityMatrix, APP_GAME_AI_CLASSIFIER_CANDIDATE_GAME_CONTEXT,
    APP_GAME_AI_CLASSIFIER_CANDIDATE_UNKNOWN_IDENTITY, APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY,
    APP_GAME_AI_CLASSIFIER_DIGEST_SESSION_SUMMARY,
    APP_GAME_AI_CLASSIFIER_FALLBACK_LOCAL_MODEL_UNAVAILABLE,
    APP_GAME_AI_CLASSIFIER_FALLBACK_NOT_NEEDED, APP_GAME_AI_CLASSIFIER_HANDOFF_MANUAL_REVIEW,
    APP_GAME_AI_CLASSIFIER_HANDOFF_PARENT_REVIEW, APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_APP,
    APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_GAME, APP_GAME_AI_CLASSIFIER_STATE_CANDIDATE,
    APP_GAME_AI_CLASSIFIER_STATE_PROVIDER_UNAVAILABLE,
    APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED, APP_GAME_CONTROL_AUTHORITY_ACTIVE,
    APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION, APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED,
};

use super::app_game_journal_sqlite_ingest_event::{
    activity_event, fields_for_row, ActivityEventInput,
};
use super::AppGameJournalSqliteIngestError;

pub fn app_game_evidence_claim_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameEvidenceClaim,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_evidence_claim(row)?;
    stored_protocol_event(
        device_id,
        platform,
        StoredProtocolEventInput {
            event_id: row.claim_id.clone(),
            observed_at: row.observed_at.clone(),
            observer: ActivityObserver::AgentService,
            event_kind: ActivityEventKind::DeviceIdleStateObserved,
            row_kind: APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM,
            row_json: &serde_json::to_string(row)
                .map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
            classification_state: Some(&row.classification_state),
            subject_id: APP_GAME_JOURNAL_EVIDENCE_CLAIM_SUBJECT_ID.to_string(),
            display_name: Some(row.display_name.clone()),
        },
    )
}

pub fn app_game_identity_journal_event(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    row: &AppGameIdentity,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    let row_json = serde_json::to_string(row)
        .map_err(|_error| AppGameJournalSqliteIngestError::Json)?;
    serde_json::from_str::<AppGameIdentity>(&row_json)
        .map_err(|_error| AppGameJournalSqliteIngestError::IdentityInvalid)?;
    stored_protocol_event(
        device_id,
        platform,
        StoredProtocolEventInput {
            event_id: row.identity_id.clone(),
            observed_at: observed_at.to_string(),
            observer: ActivityObserver::AgentService,
            event_kind: ActivityEventKind::DeviceIdleStateObserved,
            row_kind: APP_GAME_JOURNAL_ROW_KIND_IDENTITY,
            row_json: &row_json,
            classification_state: Some(&row.classification_state),
            subject_id: APP_GAME_JOURNAL_IDENTITY_SUBJECT_ID.to_string(),
            display_name: Some(row.display_label.clone()),
        },
    )
}

pub fn app_game_approval_authority_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameControlApprovalAuthority,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_authority(row)?;
    stored_protocol_event(
        device_id,
        platform,
        StoredProtocolEventInput {
            event_id: row.authority_id.clone(),
            observed_at: row.checked_at.clone(),
            observer: ActivityObserver::AgentService,
            event_kind: ActivityEventKind::EnforcementAuditRecorded,
            row_kind: APP_GAME_JOURNAL_ROW_KIND_APPROVAL_AUTHORITY,
            row_json: &serde_json::to_string(row)
                .map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
            classification_state: None,
            subject_id: APP_GAME_JOURNAL_AUTHORITY_SUBJECT_ID.to_string(),
            display_name: Some(row.authority_state.clone()),
        },
    )
}

pub fn app_game_approval_action_result_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameControlActionResult,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_action_result(row)?;
    stored_protocol_event(
        device_id,
        platform,
        StoredProtocolEventInput {
            event_id: row.result_id.clone(),
            observed_at: row.recorded_at.clone(),
            observer: ActivityObserver::AgentService,
            event_kind: ActivityEventKind::EnforcementAuditRecorded,
            row_kind: APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT,
            row_json: &serde_json::to_string(row)
                .map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
            classification_state: None,
            subject_id: APP_GAME_JOURNAL_AUTHORITY_SUBJECT_ID.to_string(),
            display_name: Some(row.result_status.clone()),
        },
    )
}

pub fn app_game_platform_authority_matrix_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGamePlatformAuthorityMatrix,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_platform_authority_matrix(row)?;
    stored_protocol_event(
        device_id,
        platform,
        StoredProtocolEventInput {
            event_id: row.matrix_id.clone(),
            observed_at: row.generated_at.clone(),
            observer: ActivityObserver::AgentService,
            event_kind: ActivityEventKind::EnforcementAuditRecorded,
            row_kind: APP_GAME_JOURNAL_ROW_KIND_PLATFORM_AUTHORITY_MATRIX,
            row_json: &serde_json::to_string(row)
                .map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
            classification_state: None,
            subject_id: APP_GAME_JOURNAL_AUTHORITY_SUBJECT_ID.to_string(),
            display_name: Some(row.matrix_id.clone()),
        },
    )
}

pub fn app_game_ai_classifier_result_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameAiClassifierResult,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_classifier_result(row)?;
    stored_protocol_event(
        device_id,
        platform,
        StoredProtocolEventInput {
            event_id: row.classifier_run_id.clone(),
            observed_at: row.generated_at.clone(),
            observer: ActivityObserver::LocalAi,
            event_kind: ActivityEventKind::DeviceIdleStateObserved,
            row_kind: APP_GAME_JOURNAL_ROW_KIND_AI_CLASSIFIER_RESULT,
            row_json: &serde_json::to_string(row)
                .map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
            classification_state: Some(&row.classifier_state),
            subject_id: APP_GAME_JOURNAL_CLASSIFIER_SUBJECT_ID.to_string(),
            display_name: Some(row.candidate_label.clone()),
        },
    )
}

pub(super) fn validate_evidence_claim(
    row: &AppGameEvidenceClaim,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    if row.claim_kind == APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY
        && (row.runtime_state != APP_GAME_RUNTIME_NOT_CLAIMED
            || row.foreground_state != APP_GAME_FOREGROUND_NOT_CLAIMED)
    {
        return Err(AppGameJournalSqliteIngestError::EvidenceClaimInventoryClaimsUse);
    }
    Ok(())
}

pub(super) fn validate_authority(
    row: &AppGameControlApprovalAuthority,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    if row.authority_state != APP_GAME_CONTROL_AUTHORITY_ACTIVE
        && (row.can_approve || row.can_deny || row.can_extend || row.can_override)
    {
        return Err(AppGameJournalSqliteIngestError::AuthorityInactiveGrants);
    }
    Ok(())
}

pub(super) fn validate_action_result(
    row: &AppGameControlActionResult,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    if row.result_status == APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED
        && row.enforcement_result.is_some()
    {
        return Err(AppGameJournalSqliteIngestError::ActionResultManualExecution);
    }
    Ok(())
}

pub(super) fn validate_platform_authority_matrix(
    row: &AppGamePlatformAuthorityMatrix,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    if row.rows.iter().any(|authority| {
        authority.authority_tier == APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED
            && authority.can_execute_adapter
    }) {
        return Err(AppGameJournalSqliteIngestError::PlatformAuthorityManualExecution);
    }
    Ok(())
}

pub(super) fn validate_classifier_result(
    row: &AppGameAiClassifierResult,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    if row.direct_action_requested || row.raw_scan_included || row.content_claim_included {
        return Err(AppGameJournalSqliteIngestError::ClassifierRequestsAction);
    }
    if !row.confidence.is_finite() || !(0.0..=1.0).contains(&row.confidence) {
        return Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid);
    }
    if [
        row.classifier_run_id.as_str(),
        row.digest_ref.as_str(),
        row.model_runtime_ref.as_str(),
        row.prompt_template_ref.as_str(),
        row.prompt_version.as_str(),
        row.generated_at.as_str(),
    ]
    .iter()
    .any(|value| !valid_classifier_identifier(value))
        || !valid_classifier_text(&row.candidate_label)
        || !valid_classifier_references(&row.source_evidence_refs, true)
        || !valid_classifier_references(&row.source_session_refs, false)
        || row
            .uncertainty_reason_codes
            .iter()
            .any(|reason| !valid_classifier_text(reason))
    {
        return Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid);
    }

    let candidate_kind_matches_product = match row.product_kind.as_str() {
        APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_APP => {
            row.candidate_kind == APP_GAME_AI_CLASSIFIER_CANDIDATE_UNKNOWN_IDENTITY
        }
        APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_GAME => {
            row.candidate_kind == APP_GAME_AI_CLASSIFIER_CANDIDATE_GAME_CONTEXT
        }
        _ => false,
    };
    if !candidate_kind_matches_product
        || !matches!(
            row.source_digest_kind.as_str(),
            APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY | APP_GAME_AI_CLASSIFIER_DIGEST_SESSION_SUMMARY
        )
        || !matches!(
            (
                row.classifier_state.as_str(),
                row.fallback_state.as_str(),
                row.policy_handoff.as_str(),
            ),
            (
                APP_GAME_AI_CLASSIFIER_STATE_CANDIDATE,
                APP_GAME_AI_CLASSIFIER_FALLBACK_NOT_NEEDED,
                APP_GAME_AI_CLASSIFIER_HANDOFF_PARENT_REVIEW,
            ) | (
                APP_GAME_AI_CLASSIFIER_STATE_PROVIDER_UNAVAILABLE,
                APP_GAME_AI_CLASSIFIER_FALLBACK_LOCAL_MODEL_UNAVAILABLE,
                APP_GAME_AI_CLASSIFIER_HANDOFF_MANUAL_REVIEW,
            )
        )
    {
        return Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid);
    }
    Ok(())
}

fn valid_classifier_identifier(value: &str) -> bool {
    !value.trim().is_empty() && value == value.trim()
}

fn valid_classifier_text(value: &str) -> bool {
    !value.trim().is_empty()
}

fn valid_classifier_references(values: &[String], require_one: bool) -> bool {
    (!require_one || !values.is_empty())
        && values
            .iter()
            .all(|value| valid_classifier_identifier(value))
        && {
            let mut seen = BTreeSet::new();
            values.iter().all(|value| seen.insert(value.as_str()))
        }
}

struct StoredProtocolEventInput<'a> {
    event_id: String,
    observed_at: String,
    observer: ActivityObserver,
    event_kind: ActivityEventKind,
    row_kind: &'a str,
    row_json: &'a str,
    classification_state: Option<&'a str>,
    subject_id: String,
    display_name: Option<String>,
}

fn stored_protocol_event(
    device_id: &str,
    platform: &str,
    input: StoredProtocolEventInput<'_>,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    Ok(activity_event(ActivityEventInput {
        event_id: input.event_id,
        observed_at: input.observed_at,
        observer: input.observer,
        kind: input.event_kind,
        subject_kind: ActivitySubjectKind::Device,
        subject_id: input.subject_id,
        display_name: input.display_name,
        device_id,
        platform,
        fields: fields_for_row(input.row_kind, input.row_json, input.classification_state),
        evidence: Vec::new(),
    }))
}
