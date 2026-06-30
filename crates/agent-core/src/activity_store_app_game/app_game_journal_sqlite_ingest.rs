use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceRef, ActivityObserver, ActivitySource,
    ActivitySubject, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameLauncherEvidenceRow,
    AppGameRuntimeEvidenceRow,
};
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CLASSIFICATION_KNOWN_GAME,
    APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER, APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE,
    APP_GAME_CLASSIFICATION_PERMISSION_LIMITED, APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED,
    APP_GAME_FOREGROUND_BACKGROUND, APP_GAME_FOREGROUND_FOREGROUND,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL,
    APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE, APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL,
    APP_GAME_JOURNAL_FIELD_REPLAY_STATE, APP_GAME_JOURNAL_FIELD_ROW_JSON,
    APP_GAME_JOURNAL_FIELD_ROW_KIND, APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID,
    APP_GAME_JOURNAL_LAUNCHER_SUBJECT_ID, APP_GAME_JOURNAL_REPLAY_STATE_STORED,
    APP_GAME_JOURNAL_ROW_KIND_FOREGROUND, APP_GAME_JOURNAL_ROW_KIND_INVENTORY,
    APP_GAME_JOURNAL_ROW_KIND_LAUNCHER, APP_GAME_JOURNAL_ROW_KIND_RUNTIME,
    APP_GAME_JOURNAL_SOURCE_ID, APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE,
    APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME,
    APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME, APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY,
    APP_GAME_LAUNCHER_PROOF_MANIFEST_CANDIDATE, APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW,
    APP_GAME_OBSERVATION_MODE_PROCESS_EXIT, APP_GAME_OBSERVATION_MODE_PROCESS_START,
    APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_RUNTIME_NOT_RUNNING, APP_GAME_RUNTIME_RUNNING,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

pub mod protocol_rows;
pub mod read_model;

#[derive(Debug, PartialEq, Eq)]
pub enum AppGameJournalSqliteIngestError {
    InventoryClaimsUse,
    RuntimeClaimsForeground,
    RuntimeExitInvalid,
    RuntimeStartInvalid,
    ForegroundWrongMode,
    ForegroundClaimsContent,
    ForegroundOpenInvalid,
    ForegroundBackgroundInvalid,
    LauncherKnownGameMissingProof,
    LauncherCandidatePromoted,
    LauncherOnlyPromoted,
    LauncherPermissionLimitedClaim,
    EvidenceClaimInventoryClaimsUse,
    AuthorityInactiveGrants,
    ActionResultManualExecution,
    PlatformAuthorityManualExecution,
    ClassifierRequestsAction,
    Json,
}

pub fn app_game_inventory_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameInventoryEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_inventory_row(row)?;
    let fields = fields_for_row(
        APP_GAME_JOURNAL_ROW_KIND_INVENTORY,
        &serde_json::to_string(row).map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
        None,
    );
    Ok(activity_event(ActivityEventInput {
        event_id: row.inventory_entry_id.clone(),
        observed_at: row.observed_at.clone(),
        observer: ActivityObserver::AgentService,
        kind: ActivityEventKind::DeviceIdleStateObserved,
        subject_kind: ActivitySubjectKind::Device,
        subject_id: APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID.to_string(),
        display_name: Some(row.display_label.clone()),
        device_id,
        platform,
        fields,
        evidence: row.evidence.clone(),
    }))
}

pub fn app_game_runtime_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameRuntimeEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_runtime_row(row)?;
    let mut fields = fields_for_row(
        APP_GAME_JOURNAL_ROW_KIND_RUNTIME,
        &serde_json::to_string(row).map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
        Some(&row.classification_state),
    );
    insert_number(&mut fields, constants::field::PID, row.process_id);
    insert_string(
        &mut fields,
        constants::field::PROCESS_NAME,
        &row.process_name,
    );
    insert_string(
        &mut fields,
        constants::field::OBSERVATION_MODE,
        &row.observation_mode,
    );
    insert_string(
        &mut fields,
        constants::field::CAPABILITY_STATUS,
        &row.capability_status,
    );
    insert_boolean(&mut fields, constants::field::FOREGROUND, false);
    Ok(activity_event(ActivityEventInput {
        event_id: row.runtime_evidence_id.clone(),
        observed_at: row.observed_at.clone(),
        observer: ActivityObserver::WindowsProcess,
        kind: ActivityEventKind::ProcessObserved,
        subject_kind: ActivitySubjectKind::Process,
        subject_id: row.process_identity.clone(),
        display_name: Some(row.process_name.clone()),
        device_id,
        platform,
        fields,
        evidence: row.evidence.clone(),
    }))
}

pub fn app_game_foreground_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameForegroundEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_foreground_row(row)?;
    let mut fields = fields_for_row(
        APP_GAME_JOURNAL_ROW_KIND_FOREGROUND,
        &serde_json::to_string(row).map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
        Some(&row.classification_state),
    );
    insert_number(&mut fields, constants::field::PID, row.process_id);
    insert_string(
        &mut fields,
        constants::field::PROCESS_NAME,
        &row.process_name,
    );
    insert_string(
        &mut fields,
        constants::field::OBSERVATION_MODE,
        &row.observation_mode,
    );
    insert_string(
        &mut fields,
        constants::field::CAPABILITY_STATUS,
        &row.capability_status,
    );
    insert_boolean(
        &mut fields,
        constants::field::FOREGROUND,
        row.foreground_state == APP_GAME_FOREGROUND_FOREGROUND,
    );
    Ok(activity_event(ActivityEventInput {
        event_id: row.foreground_evidence_id.clone(),
        observed_at: row.observed_at.clone(),
        observer: ActivityObserver::WindowsWindow,
        kind: ActivityEventKind::WindowFocused,
        subject_kind: ActivitySubjectKind::Window,
        subject_id: row.process_identity.clone(),
        display_name: Some(row.process_name.clone()),
        device_id,
        platform,
        fields,
        evidence: row.evidence.clone(),
    }))
}

pub fn app_game_launcher_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameLauncherEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_launcher_row(row)?;
    let fields = fields_for_row(
        APP_GAME_JOURNAL_ROW_KIND_LAUNCHER,
        &serde_json::to_string(row).map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
        Some(&row.classification_state),
    );
    Ok(activity_event(ActivityEventInput {
        event_id: row.launcher_evidence_id.clone(),
        observed_at: row.observed_at.clone(),
        observer: ActivityObserver::AgentService,
        kind: ActivityEventKind::DeviceIdleStateObserved,
        subject_kind: ActivitySubjectKind::Device,
        subject_id: APP_GAME_JOURNAL_LAUNCHER_SUBJECT_ID.to_string(),
        display_name: row.launcher_process_name.clone(),
        device_id,
        platform,
        fields,
        evidence: row.evidence.clone(),
    }))
}

fn validate_inventory_row(
    row: &AppGameInventoryEvidenceRow,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.runtime_state != APP_GAME_RUNTIME_NOT_CLAIMED
        || row.foreground_state != APP_GAME_FOREGROUND_NOT_CLAIMED
        || row.running_duration_ms != 0
        || row.foreground_duration_ms != 0
    {
        return Err(AppGameJournalSqliteIngestError::InventoryClaimsUse);
    }
    Ok(())
}

fn validate_runtime_row(
    row: &AppGameRuntimeEvidenceRow,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.foreground_state != APP_GAME_FOREGROUND_NOT_CLAIMED {
        return Err(AppGameJournalSqliteIngestError::RuntimeClaimsForeground);
    }
    if row.observation_mode == APP_GAME_OBSERVATION_MODE_PROCESS_EXIT
        && (row.runtime_state != APP_GAME_RUNTIME_NOT_RUNNING || row.exited_at.is_none())
    {
        return Err(AppGameJournalSqliteIngestError::RuntimeExitInvalid);
    }
    if row.observation_mode == APP_GAME_OBSERVATION_MODE_PROCESS_START
        && (row.runtime_state != APP_GAME_RUNTIME_RUNNING
            || row.started_at.is_none()
            || row.exited_at.is_some())
    {
        return Err(AppGameJournalSqliteIngestError::RuntimeStartInvalid);
    }
    Ok(())
}

fn validate_foreground_row(
    row: &AppGameForegroundEvidenceRow,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.observation_mode != APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW {
        return Err(AppGameJournalSqliteIngestError::ForegroundWrongMode);
    }
    if row.content_knowledge_state != APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED {
        return Err(AppGameJournalSqliteIngestError::ForegroundClaimsContent);
    }
    if row.foreground_state == APP_GAME_FOREGROUND_FOREGROUND
        && (row.foreground_started_at.is_none() || row.foreground_ended_at.is_some())
    {
        return Err(AppGameJournalSqliteIngestError::ForegroundOpenInvalid);
    }
    if row.foreground_state == APP_GAME_FOREGROUND_BACKGROUND && row.foreground_ended_at.is_none() {
        return Err(AppGameJournalSqliteIngestError::ForegroundBackgroundInvalid);
    }
    Ok(())
}

fn validate_launcher_row(
    row: &AppGameLauncherEvidenceRow,
) -> Result<(), AppGameJournalSqliteIngestError> {
    let has_child_game_proof = matches!(
        row.game_proof_state.as_str(),
        APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME
            | APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME
    ) && row.child_game_evidence_claim_id.is_some();
    if row.classification_state == APP_GAME_CLASSIFICATION_KNOWN_GAME && !has_child_game_proof {
        return Err(AppGameJournalSqliteIngestError::LauncherKnownGameMissingProof);
    }
    if matches!(
        row.game_proof_state.as_str(),
        APP_GAME_LAUNCHER_PROOF_MANIFEST_CANDIDATE
            | APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE
    ) && row.classification_state != APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE
    {
        return Err(AppGameJournalSqliteIngestError::LauncherCandidatePromoted);
    }
    if row.game_proof_state == APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY
        && row.classification_state != APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER
    {
        return Err(AppGameJournalSqliteIngestError::LauncherOnlyPromoted);
    }
    if row.classification_state == APP_GAME_CLASSIFICATION_PERMISSION_LIMITED
        && row.capability_status != APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED
    {
        return Err(AppGameJournalSqliteIngestError::LauncherPermissionLimitedClaim);
    }
    Ok(())
}

pub(super) fn fields_for_row(
    row_kind: &str,
    row_json: &str,
    classification_state: Option<&str>,
) -> LogFields {
    let mut fields = LogFields::new();
    insert_string(&mut fields, APP_GAME_JOURNAL_FIELD_ROW_KIND, row_kind);
    insert_string(
        &mut fields,
        APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL,
        APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL,
    );
    insert_string(
        &mut fields,
        APP_GAME_JOURNAL_FIELD_REPLAY_STATE,
        APP_GAME_JOURNAL_REPLAY_STATE_STORED,
    );
    if let Some(classification_state) = classification_state {
        insert_string(
            &mut fields,
            APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE,
            classification_state,
        );
    }
    insert_string(&mut fields, APP_GAME_JOURNAL_FIELD_ROW_JSON, row_json);
    fields
}

pub(super) struct ActivityEventInput<'a> {
    pub event_id: String,
    pub observed_at: String,
    pub observer: ActivityObserver,
    pub kind: ActivityEventKind,
    pub subject_kind: ActivitySubjectKind,
    pub subject_id: String,
    pub display_name: Option<String>,
    pub device_id: &'a str,
    pub platform: &'a str,
    pub fields: LogFields,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub(super) fn activity_event(input: ActivityEventInput<'_>) -> ActivityEvent {
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: input.event_id,
        observed_at: input.observed_at,
        source: ActivitySource {
            device_id: input.device_id.to_string(),
            platform: input.platform.to_string(),
            observer: input.observer,
            source_id: APP_GAME_JOURNAL_SOURCE_ID.to_string(),
        },
        kind: input.kind,
        subject: ActivitySubject {
            kind: input.subject_kind,
            subject_id: input.subject_id,
            display_name: input.display_name,
        },
        fields: input.fields,
        evidence: input.evidence,
    }
}

fn insert_string(fields: &mut LogFields, key: &str, value: &str) {
    fields.insert(key.to_string(), LogFieldValue::String(value.to_string()));
}

fn insert_number(fields: &mut LogFields, key: &str, value: u64) {
    fields.insert(key.to_string(), LogFieldValue::Number(value as f64));
}

fn insert_boolean(fields: &mut LogFields, key: &str, value: bool) {
    fields.insert(key.to_string(), LogFieldValue::Boolean(value));
}
