use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CLASSIFICATION_KNOWN_APP, APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_RUNTIME_RUNNING,
};
use ocentra_parent_agent_protocol::constants;
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

pub(crate) async fn validate_app_game_dispatch_evidence(
    payload: &LogFields,
    store_path: AppGameDispatchStorePath,
) -> Result<(), AppGameDispatchEvidenceRejection> {
    let payload = AppGameDispatchEvidencePayload::parse(payload)?;
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(store_path.0)
            .map_err(|_| AppGameDispatchEvidenceRejection::Mismatch)?;
        let model = store
            .app_game_service_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                constants::enforcement::APP_GAME_RUNTIME_EVIDENCE_GENERATED_AT,
            )
            .map_err(|_| AppGameDispatchEvidenceRejection::Mismatch)?;
        let matches = model.running_now_rows.iter().any(|row| {
            row.runtime_evidence_id == payload.runtime_evidence_id.0
                && row.process_id == u64::from(payload.process_id)
                && row.process_name == payload.target_value.0
                && row.runtime_state == APP_GAME_RUNTIME_RUNNING
                && matches!(
                    row.classification_state.as_str(),
                    APP_GAME_CLASSIFICATION_KNOWN_APP | APP_GAME_CLASSIFICATION_KNOWN_GAME
                )
        });
        matches
            .then_some(())
            .ok_or(AppGameDispatchEvidenceRejection::Mismatch)
    })
    .await
    .map_err(|_| AppGameDispatchEvidenceRejection::Mismatch)?
}
