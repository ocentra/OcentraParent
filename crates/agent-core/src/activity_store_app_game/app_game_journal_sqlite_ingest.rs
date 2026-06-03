use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow,
    AppGameLauncherEvidenceRow, AppGameRuntimeEvidenceRow, AppGameSessionDailyRollup,
    LogFieldValue, LogFields, ACTIVITY_SCHEMA_VERSION,
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
use rusqlite::{params, Connection, Row};

use crate::ActivityStoreError;

use super::app_game_session_daily_rollups;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum AppGameJournalSqliteIngestError {
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
    Json,
}

pub(crate) struct AppGameJournalSqliteReadModel {
    pub inventory_rows: Vec<AppGameInventoryEvidenceRow>,
    pub running_now_rows: Vec<AppGameRuntimeEvidenceRow>,
    pub foreground_now_rows: Vec<AppGameForegroundEvidenceRow>,
    pub launcher_rows: Vec<AppGameLauncherEvidenceRow>,
    pub daily_rollups: Vec<AppGameSessionDailyRollup>,
}

struct StoredAppGameJournalRow {
    fields: LogFields,
}

pub(crate) fn app_game_inventory_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameInventoryEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_inventory_row(row)?;
    let fields = fields_for_row(
        APP_GAME_JOURNAL_ROW_KIND_INVENTORY,
        &serde_json::to_string(row).map_err(|_| AppGameJournalSqliteIngestError::Json)?,
        None,
    );
    Ok(activity_event(
        row.inventory_entry_id.clone(),
        row.observed_at.clone(),
        ActivityObserver::AgentService,
        ActivityEventKind::DeviceIdleStateObserved,
        ActivitySubjectKind::Device,
        APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID.to_string(),
        Some(row.display_label.clone()),
        device_id,
        platform,
        fields,
        row.evidence.clone(),
    ))
}

pub(crate) fn app_game_runtime_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameRuntimeEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_runtime_row(row)?;
    let mut fields = fields_for_row(
        APP_GAME_JOURNAL_ROW_KIND_RUNTIME,
        &serde_json::to_string(row).map_err(|_| AppGameJournalSqliteIngestError::Json)?,
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
    Ok(activity_event(
        row.runtime_evidence_id.clone(),
        row.observed_at.clone(),
        ActivityObserver::WindowsProcess,
        ActivityEventKind::ProcessObserved,
        ActivitySubjectKind::Process,
        row.process_identity.clone(),
        Some(row.process_name.clone()),
        device_id,
        platform,
        fields,
        row.evidence.clone(),
    ))
}

pub(crate) fn app_game_foreground_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameForegroundEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_foreground_row(row)?;
    let mut fields = fields_for_row(
        APP_GAME_JOURNAL_ROW_KIND_FOREGROUND,
        &serde_json::to_string(row).map_err(|_| AppGameJournalSqliteIngestError::Json)?,
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
    Ok(activity_event(
        row.foreground_evidence_id.clone(),
        row.observed_at.clone(),
        ActivityObserver::WindowsWindow,
        ActivityEventKind::WindowFocused,
        ActivitySubjectKind::Window,
        row.process_identity.clone(),
        Some(row.process_name.clone()),
        device_id,
        platform,
        fields,
        row.evidence.clone(),
    ))
}

pub(crate) fn app_game_launcher_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameLauncherEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    validate_launcher_row(row)?;
    let fields = fields_for_row(
        APP_GAME_JOURNAL_ROW_KIND_LAUNCHER,
        &serde_json::to_string(row).map_err(|_| AppGameJournalSqliteIngestError::Json)?,
        Some(&row.classification_state),
    );
    Ok(activity_event(
        row.launcher_evidence_id.clone(),
        row.observed_at.clone(),
        ActivityObserver::AgentService,
        ActivityEventKind::DeviceIdleStateObserved,
        ActivitySubjectKind::Device,
        APP_GAME_JOURNAL_LAUNCHER_SUBJECT_ID.to_string(),
        row.launcher_process_name.clone(),
        device_id,
        platform,
        fields,
        row.evidence.clone(),
    ))
}

pub(crate) fn app_game_journal_sqlite_read_model(
    connection: &Connection,
    limit: u64,
) -> Result<AppGameJournalSqliteReadModel, ActivityStoreError> {
    let mut seen_runtime_processes = Vec::new();
    let mut seen_foreground_processes = Vec::new();
    let mut model = AppGameJournalSqliteReadModel {
        inventory_rows: Vec::new(),
        running_now_rows: Vec::new(),
        foreground_now_rows: Vec::new(),
        launcher_rows: Vec::new(),
        daily_rollups: app_game_session_daily_rollups(connection, limit)?,
    };
    let mut statement = connection.prepare(constants::sqlite::SELECT_POLICY_PREVIEW_ACTIVITY)?;
    let rows = statement.query_map(params![limit as i64], stored_row_from_sqlite)?;
    for row in rows {
        project_stored_row(
            row?,
            &mut model,
            &mut seen_runtime_processes,
            &mut seen_foreground_processes,
        )?;
    }
    Ok(model)
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

fn fields_for_row(row_kind: &str, row_json: &str, classification_state: Option<&str>) -> LogFields {
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

fn activity_event(
    event_id: String,
    observed_at: String,
    observer: ActivityObserver,
    kind: ActivityEventKind,
    subject_kind: ActivitySubjectKind,
    subject_id: String,
    display_name: Option<String>,
    device_id: &str,
    platform: &str,
    fields: LogFields,
    evidence: Vec<ocentra_parent_agent_protocol::ActivityEvidenceRef>,
) -> ActivityEvent {
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id,
        observed_at,
        source: ActivitySource {
            device_id: device_id.to_string(),
            platform: platform.to_string(),
            observer,
            source_id: APP_GAME_JOURNAL_SOURCE_ID.to_string(),
        },
        kind,
        subject: ActivitySubject {
            kind: subject_kind,
            subject_id,
            display_name,
        },
        fields,
        evidence,
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

fn stored_row_from_sqlite(row: &Row<'_>) -> rusqlite::Result<StoredAppGameJournalRow> {
    let fields_json: String = row.get(5)?;
    let fields = serde_json::from_str::<LogFields>(&fields_json).map_err(json_to_sqlite_error)?;
    Ok(StoredAppGameJournalRow { fields })
}

fn project_stored_row(
    row: StoredAppGameJournalRow,
    model: &mut AppGameJournalSqliteReadModel,
    seen_runtime_processes: &mut Vec<String>,
    seen_foreground_processes: &mut Vec<String>,
) -> Result<(), ActivityStoreError> {
    let Some(row_kind) = string_field(&row.fields, APP_GAME_JOURNAL_FIELD_ROW_KIND) else {
        return Ok(());
    };
    let Some(row_json) = string_field(&row.fields, APP_GAME_JOURNAL_FIELD_ROW_JSON) else {
        return Ok(());
    };
    match row_kind.as_str() {
        APP_GAME_JOURNAL_ROW_KIND_INVENTORY => model.inventory_rows.push(serde_json::from_str::<
            AppGameInventoryEvidenceRow,
        >(&row_json)?),
        APP_GAME_JOURNAL_ROW_KIND_RUNTIME => {
            let runtime = serde_json::from_str::<AppGameRuntimeEvidenceRow>(&row_json)?;
            if !seen_runtime_processes
                .iter()
                .any(|candidate| candidate == &runtime.process_identity)
            {
                seen_runtime_processes.push(runtime.process_identity.clone());
            } else {
                return Ok(());
            }
            if runtime.runtime_state == APP_GAME_RUNTIME_RUNNING {
                model.running_now_rows.push(runtime);
            }
        }
        APP_GAME_JOURNAL_ROW_KIND_FOREGROUND => {
            let foreground = serde_json::from_str::<AppGameForegroundEvidenceRow>(&row_json)?;
            if !seen_foreground_processes
                .iter()
                .any(|candidate| candidate == &foreground.process_identity)
            {
                seen_foreground_processes.push(foreground.process_identity.clone());
            } else {
                return Ok(());
            }
            if foreground.foreground_state == APP_GAME_FOREGROUND_FOREGROUND {
                model.foreground_now_rows.push(foreground);
            }
        }
        APP_GAME_JOURNAL_ROW_KIND_LAUNCHER => model.launcher_rows.push(serde_json::from_str::<
            AppGameLauncherEvidenceRow,
        >(&row_json)?),
        _ => {}
    }
    Ok(())
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn json_to_sqlite_error(error: serde_json::Error) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(error))
}
