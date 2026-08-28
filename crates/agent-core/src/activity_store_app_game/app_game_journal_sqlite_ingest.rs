use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameLauncherEvidenceRow,
    AppGameRuntimeEvidenceRow,
};

#[path = "app_game_journal_sqlite_ingest_event.rs"]
mod app_game_journal_sqlite_ingest_event;
#[path = "app_game_journal_sqlite_ingest_launcher_validation.rs"]
mod app_game_journal_sqlite_ingest_launcher_validation;
#[path = "app_game_journal_sqlite_ingest_validation.rs"]
mod app_game_journal_sqlite_ingest_validation;

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
    SchemaVersionUnsupported,
    EvidenceClaimInventoryClaimsUse,
    AuthorityInactiveGrants,
    ActionResultManualExecution,
    PlatformAuthorityManualExecution,
    ClassifierRequestsAction,
    ClassifierInputInvalid,
    Json,
}

pub fn app_game_inventory_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameInventoryEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    app_game_journal_sqlite_ingest_validation::validate_inventory_row(row)?;
    let fields = app_game_journal_sqlite_ingest_event::fields_for_row(
        ocentra_parent_agent_protocol::app_game::APP_GAME_JOURNAL_ROW_KIND_INVENTORY,
        &serde_json::to_string(row).map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
        None,
    );
    Ok(app_game_journal_sqlite_ingest_event::activity_event(
        app_game_journal_sqlite_ingest_event::ActivityEventInput {
            event_id: row.inventory_entry_id.clone(),
            observed_at: row.observed_at.clone(),
            observer: ocentra_parent_agent_protocol::activity::ActivityObserver::AgentService,
            kind:
                ocentra_parent_agent_protocol::activity::ActivityEventKind::DeviceIdleStateObserved,
            subject_kind: ocentra_parent_agent_protocol::activity::ActivitySubjectKind::Device,
            subject_id:
                ocentra_parent_agent_protocol::app_game::APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID
                    .to_string(),
            display_name: Some(row.display_label.clone()),
            device_id,
            platform,
            fields,
            evidence: row.evidence.clone(),
        },
    ))
}

pub fn app_game_runtime_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameRuntimeEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    app_game_journal_sqlite_ingest_validation::validate_runtime_row(row)?;
    let mut fields = app_game_journal_sqlite_ingest_event::fields_for_row(
        ocentra_parent_agent_protocol::app_game::APP_GAME_JOURNAL_ROW_KIND_RUNTIME,
        &serde_json::to_string(row).map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
        Some(&row.classification_state),
    );
    app_game_journal_sqlite_ingest_event::insert_number(
        &mut fields,
        ocentra_parent_agent_protocol::constants::field::PID,
        row.process_id,
    );
    app_game_journal_sqlite_ingest_event::insert_string(
        &mut fields,
        ocentra_parent_agent_protocol::constants::field::PROCESS_NAME,
        &row.process_name,
    );
    app_game_journal_sqlite_ingest_event::insert_string(
        &mut fields,
        ocentra_parent_agent_protocol::constants::field::OBSERVATION_MODE,
        &row.observation_mode,
    );
    app_game_journal_sqlite_ingest_event::insert_string(
        &mut fields,
        ocentra_parent_agent_protocol::constants::field::CAPABILITY_STATUS,
        &row.capability_status,
    );
    app_game_journal_sqlite_ingest_event::insert_boolean(
        &mut fields,
        ocentra_parent_agent_protocol::constants::field::FOREGROUND,
        false,
    );
    Ok(app_game_journal_sqlite_ingest_event::activity_event(
        app_game_journal_sqlite_ingest_event::ActivityEventInput {
            event_id: row.runtime_evidence_id.clone(),
            observed_at: row.observed_at.clone(),
            observer: ocentra_parent_agent_protocol::activity::ActivityObserver::WindowsProcess,
            kind: ocentra_parent_agent_protocol::activity::ActivityEventKind::ProcessObserved,
            subject_kind: ocentra_parent_agent_protocol::activity::ActivitySubjectKind::Process,
            subject_id: row.process_identity.clone(),
            display_name: Some(row.process_name.clone()),
            device_id,
            platform,
            fields,
            evidence: row.evidence.clone(),
        },
    ))
}

pub fn app_game_foreground_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameForegroundEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    app_game_journal_sqlite_ingest_validation::validate_foreground_row(row)?;
    let mut fields = app_game_journal_sqlite_ingest_event::fields_for_row(
        ocentra_parent_agent_protocol::app_game::APP_GAME_JOURNAL_ROW_KIND_FOREGROUND,
        &serde_json::to_string(row).map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
        Some(&row.classification_state),
    );
    app_game_journal_sqlite_ingest_event::insert_number(
        &mut fields,
        ocentra_parent_agent_protocol::constants::field::PID,
        row.process_id,
    );
    app_game_journal_sqlite_ingest_event::insert_string(
        &mut fields,
        ocentra_parent_agent_protocol::constants::field::PROCESS_NAME,
        &row.process_name,
    );
    app_game_journal_sqlite_ingest_event::insert_string(
        &mut fields,
        ocentra_parent_agent_protocol::constants::field::OBSERVATION_MODE,
        &row.observation_mode,
    );
    app_game_journal_sqlite_ingest_event::insert_string(
        &mut fields,
        ocentra_parent_agent_protocol::constants::field::CAPABILITY_STATUS,
        &row.capability_status,
    );
    app_game_journal_sqlite_ingest_event::insert_boolean(
        &mut fields,
        ocentra_parent_agent_protocol::constants::field::FOREGROUND,
        row.foreground_state
            == ocentra_parent_agent_protocol::app_game::APP_GAME_FOREGROUND_FOREGROUND,
    );
    Ok(app_game_journal_sqlite_ingest_event::activity_event(
        app_game_journal_sqlite_ingest_event::ActivityEventInput {
            event_id: row.foreground_evidence_id.clone(),
            observed_at: row.observed_at.clone(),
            observer: ocentra_parent_agent_protocol::activity::ActivityObserver::WindowsWindow,
            kind: ocentra_parent_agent_protocol::activity::ActivityEventKind::WindowFocused,
            subject_kind: ocentra_parent_agent_protocol::activity::ActivitySubjectKind::Window,
            subject_id: row.process_identity.clone(),
            display_name: Some(row.process_name.clone()),
            device_id,
            platform,
            fields,
            evidence: row.evidence.clone(),
        },
    ))
}

pub fn app_game_launcher_journal_event(
    device_id: &str,
    platform: &str,
    row: &AppGameLauncherEvidenceRow,
) -> Result<ActivityEvent, AppGameJournalSqliteIngestError> {
    app_game_journal_sqlite_ingest_launcher_validation::validate_launcher_row(row)?;
    let fields = app_game_journal_sqlite_ingest_event::fields_for_row(
        ocentra_parent_agent_protocol::app_game::APP_GAME_JOURNAL_ROW_KIND_LAUNCHER,
        &serde_json::to_string(row).map_err(|_error| AppGameJournalSqliteIngestError::Json)?,
        Some(&row.classification_state),
    );
    Ok(app_game_journal_sqlite_ingest_event::activity_event(
        app_game_journal_sqlite_ingest_event::ActivityEventInput {
            event_id: row.launcher_evidence_id.clone(),
            observed_at: row.observed_at.clone(),
            observer: ocentra_parent_agent_protocol::activity::ActivityObserver::AgentService,
            kind:
                ocentra_parent_agent_protocol::activity::ActivityEventKind::DeviceIdleStateObserved,
            subject_kind: ocentra_parent_agent_protocol::activity::ActivitySubjectKind::Device,
            subject_id:
                ocentra_parent_agent_protocol::app_game::APP_GAME_JOURNAL_LAUNCHER_SUBJECT_ID
                    .to_string(),
            display_name: row.launcher_process_name.clone(),
            device_id,
            platform,
            fields,
            evidence: row.evidence.clone(),
        },
    ))
}
