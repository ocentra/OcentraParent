use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::app_game::{
    AppGameEvidenceClaim, AppGameIdentity, APP_GAME_JOURNAL_AUTHORITY_SUBJECT_ID,
    APP_GAME_JOURNAL_CLASSIFIER_SUBJECT_ID, APP_GAME_JOURNAL_EVIDENCE_CLAIM_SUBJECT_ID,
    APP_GAME_JOURNAL_IDENTITY_SUBJECT_ID, APP_GAME_JOURNAL_ROW_KIND_AI_CLASSIFIER_RESULT,
    APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT, APP_GAME_JOURNAL_ROW_KIND_APPROVAL_AUTHORITY,
    APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM, APP_GAME_JOURNAL_ROW_KIND_IDENTITY,
    APP_GAME_JOURNAL_ROW_KIND_PLATFORM_AUTHORITY_MATRIX, APP_GAME_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameAiClassifierResult, AppGameControlActionResult, AppGameControlApprovalAuthority,
    AppGamePlatformAuthorityMatrix,
};

use super::app_game_journal_sqlite_ingest_event::{
    activity_event, fields_for_row, ActivityEventInput,
};
use super::AppGameJournalSqliteIngestError;

#[path = "protocol_rows_classifier_validation.rs"]
mod protocol_rows_classifier_validation;
#[path = "protocol_rows_contract_validation.rs"]
mod protocol_rows_contract_validation;

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
    let row_json =
        serde_json::to_string(row).map_err(|_error| AppGameJournalSqliteIngestError::Json)?;
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
    protocol_rows_contract_validation::validate_evidence_claim(row)
}

pub(super) fn validate_authority(
    row: &AppGameControlApprovalAuthority,
) -> Result<(), AppGameJournalSqliteIngestError> {
    protocol_rows_contract_validation::validate_authority(row)
}

pub(super) fn validate_action_result(
    row: &AppGameControlActionResult,
) -> Result<(), AppGameJournalSqliteIngestError> {
    protocol_rows_contract_validation::validate_action_result(row)
}

pub(super) fn validate_platform_authority_matrix(
    row: &AppGamePlatformAuthorityMatrix,
) -> Result<(), AppGameJournalSqliteIngestError> {
    protocol_rows_contract_validation::validate_platform_authority_matrix(row)
}

pub(super) fn validate_classifier_result(
    row: &AppGameAiClassifierResult,
) -> Result<(), AppGameJournalSqliteIngestError> {
    protocol_rows_classifier_validation::validate_classifier_result(row)
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
