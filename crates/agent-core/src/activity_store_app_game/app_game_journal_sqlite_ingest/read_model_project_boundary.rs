use ocentra_parent_agent_protocol::app_game::{
    AppGameEvidenceClaim, AppGameIdentity, APP_GAME_JOURNAL_ROW_KIND_AI_CLASSIFIER_RESULT,
    APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT, APP_GAME_JOURNAL_ROW_KIND_APPROVAL_AUTHORITY,
    APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM, APP_GAME_JOURNAL_ROW_KIND_IDENTITY,
    APP_GAME_JOURNAL_ROW_KIND_PLATFORM_AUTHORITY_MATRIX,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameControlActionResult, AppGameControlApprovalAuthority,
};
use ocentra_parent_agent_protocol::{AppGameAiClassifierResult, AppGamePlatformAuthorityMatrix};

use crate::ActivityStoreError;

pub(super) fn project_protocol_boundary_row(
    row_kind: &str,
    row_json: &str,
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
) -> Result<bool, ActivityStoreError> {
    match row_kind {
        APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM => model
            .evidence_claim_rows
            .push(serde_json::from_str::<AppGameEvidenceClaim>(row_json)?),
        APP_GAME_JOURNAL_ROW_KIND_IDENTITY => model
            .identity_rows
            .push(serde_json::from_str::<AppGameIdentity>(row_json)?),
        APP_GAME_JOURNAL_ROW_KIND_APPROVAL_AUTHORITY => {
            model.approval_authority_rows.push(serde_json::from_str::<
                AppGameControlApprovalAuthority,
            >(row_json)?)
        }
        APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT => {
            model
                .approval_action_result_rows
                .push(serde_json::from_str::<AppGameControlActionResult>(
                    row_json,
                )?)
        }
        APP_GAME_JOURNAL_ROW_KIND_PLATFORM_AUTHORITY_MATRIX => model
            .platform_authority_matrices
            .push(serde_json::from_str::<AppGamePlatformAuthorityMatrix>(
                row_json,
            )?),
        APP_GAME_JOURNAL_ROW_KIND_AI_CLASSIFIER_RESULT => model
            .ai_classifier_result_rows
            .push(serde_json::from_str::<AppGameAiClassifierResult>(row_json)?),
        _ => return Ok(false),
    }
    Ok(true)
}
