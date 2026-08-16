use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CLASSIFICATION_KNOWN_APP, APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_RUNTIME_RUNNING,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::AppGameTimerSessionBinding;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

#[path = "app_game_dispatch_evidence/payload.rs"]
mod payload;

use payload::AppGameDispatchEvidencePayload;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AppGameDispatchStorePath(pub(crate) std::path::PathBuf);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AppGameDispatchEvidenceRejection {
    Required,
    Mismatch,
}

impl AppGameDispatchEvidenceRejection {
    pub(crate) fn log_value(self) -> LogFieldValue {
        let value = match self {
            Self::Required => constants::enforcement::REJECTION_APP_GAME_SESSION_EVIDENCE_REQUIRED,
            Self::Mismatch => constants::enforcement::REJECTION_APP_GAME_RUNTIME_EVIDENCE_MISMATCH,
        };
        LogFieldValue::String(value.to_string())
    }
}

fn mismatch_from_error<E>(error: &E) -> AppGameDispatchEvidenceRejection {
    let _ = error;
    AppGameDispatchEvidenceRejection::Mismatch
}

pub(crate) async fn validate_app_game_dispatch_evidence(
    payload: &LogFields,
    store_path: AppGameDispatchStorePath,
) -> Result<AppGameTimerSessionBinding, AppGameDispatchEvidenceRejection> {
    let payload = AppGameDispatchEvidencePayload::parse(payload)?;
    tokio::task::spawn_blocking(move || {
        let store =
            ActivityStore::open(store_path.0).map_err(|error| mismatch_from_error(&error))?;
        let model = store
            .app_game_service_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                constants::enforcement::APP_GAME_RUNTIME_EVIDENCE_GENERATED_AT,
            )
            .map_err(|error| mismatch_from_error(&error))?;
        let runtime = model
            .running_now_rows
            .iter()
            .find(|row| {
                row.runtime_evidence_id == payload.runtime_evidence_id.0
                    && row.process_id == u64::from(payload.process_id)
                    && row.process_name == payload.target_value.0
                    && row.runtime_state == APP_GAME_RUNTIME_RUNNING
                    && matches!(
                        row.classification_state.as_str(),
                        APP_GAME_CLASSIFICATION_KNOWN_APP | APP_GAME_CLASSIFICATION_KNOWN_GAME
                    )
            })
            .ok_or(AppGameDispatchEvidenceRejection::Mismatch)?;
        let summary = store
            .app_game_session_summaries(constants::activity_store::DEFAULT_RECENT_LIMIT)
            .map_err(|error| mismatch_from_error(&error))?
            .into_iter()
            .find(|summary| {
                summary.primary_process_identity == runtime.process_identity
                    && summary.last_observed_at >= runtime.observed_at
            })
            .ok_or(AppGameDispatchEvidenceRejection::Mismatch)?;
        Ok(AppGameTimerSessionBinding {
            session_id: summary.session_id,
            runtime_evidence_id: runtime.runtime_evidence_id.clone(),
            process_identity: runtime.process_identity.clone(),
            process_id: runtime.process_id,
            process_name: runtime.process_name.clone(),
            classification_state: runtime.classification_state.clone(),
            last_observed_at: summary.last_observed_at,
            running_duration_ms: summary.running_duration_ms,
            foreground_duration_ms: summary.foreground_duration_ms,
        })
    })
    .await
    .map_err(|error| mismatch_from_error(&error))?
}

pub(crate) async fn validate_app_game_timer_session(
    binding: &AppGameTimerSessionBinding,
    store_path: AppGameDispatchStorePath,
) -> Result<(), AppGameDispatchEvidenceRejection> {
    let binding = binding.clone();
    tokio::task::spawn_blocking(move || {
        let store =
            ActivityStore::open(store_path.0).map_err(|error| mismatch_from_error(&error))?;
        let model = store
            .app_game_service_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                constants::enforcement::APP_GAME_RUNTIME_EVIDENCE_GENERATED_AT,
            )
            .map_err(|error| mismatch_from_error(&error))?;
        let runtime = model
            .running_now_rows
            .iter()
            .find(|row| {
                row.runtime_evidence_id == binding.runtime_evidence_id
                    && row.process_identity == binding.process_identity
                    && row.process_id == binding.process_id
                    && row.process_name == binding.process_name
                    && row.classification_state == binding.classification_state
                    && row.runtime_state == APP_GAME_RUNTIME_RUNNING
            })
            .ok_or(AppGameDispatchEvidenceRejection::Mismatch)?;
        store
            .app_game_session_summaries(constants::activity_store::DEFAULT_RECENT_LIMIT)
            .map_err(|error| mismatch_from_error(&error))?
            .into_iter()
            .find(|summary| {
                summary.session_id == binding.session_id
                    && summary.primary_process_identity == runtime.process_identity
                    && summary.last_observed_at >= binding.last_observed_at
                    && summary.running_duration_ms >= binding.running_duration_ms
                    && summary.foreground_duration_ms >= binding.foreground_duration_ms
            })
            .map(|_| ())
            .ok_or(AppGameDispatchEvidenceRejection::Mismatch)
    })
    .await
    .map_err(|error| mismatch_from_error(&error))?
}
