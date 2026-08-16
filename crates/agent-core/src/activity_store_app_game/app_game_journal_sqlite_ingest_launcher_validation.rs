use ocentra_parent_agent_protocol::app_game::{
    AppGameLauncherEvidenceRow, APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER,
    APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE,
    APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME,
    APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME, APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY,
    APP_GAME_LAUNCHER_PROOF_MANIFEST_CANDIDATE,
};

use super::AppGameJournalSqliteIngestError;

pub(super) fn validate_launcher_row(
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
