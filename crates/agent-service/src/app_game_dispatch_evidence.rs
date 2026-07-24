use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CLASSIFICATION_KNOWN_APP, APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_RUNTIME_RUNNING,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AppGameDispatchEvidenceRejection {
    Required,
    Mismatch,
}

impl AppGameDispatchEvidenceRejection {
    pub(crate) fn as_protocol_str(self) -> &'static str {
        match self {
            Self::Required => constants::enforcement::REJECTION_APP_GAME_SESSION_EVIDENCE_REQUIRED,
            Self::Mismatch => constants::enforcement::REJECTION_APP_GAME_RUNTIME_EVIDENCE_MISMATCH,
        }
    }
}

pub(crate) async fn validate_app_game_dispatch_evidence(
    payload: &LogFields,
    store_path: std::path::PathBuf,
) -> Result<(), AppGameDispatchEvidenceRejection> {
    let runtime_evidence_id = match payload.get(constants::field::APP_GAME_RUNTIME_EVIDENCE_ID) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => value.trim().to_string(),
        _ => return Err(AppGameDispatchEvidenceRejection::Required),
    };
    let process_id = match payload.get(constants::field::PROCESS_ID) {
        Some(LogFieldValue::Number(value)) if *value >= 0.0 && *value <= f64::from(u32::MAX) => {
            *value as u32
        }
        _ => return Err(AppGameDispatchEvidenceRejection::Required),
    };
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(store_path)
            .map_err(|_| AppGameDispatchEvidenceRejection::Mismatch)?;
        let model = store
            .app_game_service_read_model(constants::activity_store::DEFAULT_RECENT_LIMIT, "")
            .map_err(|_| AppGameDispatchEvidenceRejection::Mismatch)?;
        let matches = model.running_now_rows.iter().any(|row| {
            row.runtime_evidence_id == runtime_evidence_id
                && row.process_id == u64::from(process_id)
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
