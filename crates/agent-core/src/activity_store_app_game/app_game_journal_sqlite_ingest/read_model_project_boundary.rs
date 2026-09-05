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
        APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM => project_evidence_claim(row_json, model)?,
        APP_GAME_JOURNAL_ROW_KIND_IDENTITY => project_identity(row_json, model)?,
        APP_GAME_JOURNAL_ROW_KIND_APPROVAL_AUTHORITY => {
            project_approval_authority(row_json, model)?
        }
        APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT => {
            project_approval_action_result(row_json, model)?
        }
        APP_GAME_JOURNAL_ROW_KIND_PLATFORM_AUTHORITY_MATRIX => {
            project_platform_authority_matrix(row_json, model)?
        }
        APP_GAME_JOURNAL_ROW_KIND_AI_CLASSIFIER_RESULT => {
            project_ai_classifier_result(row_json, model)?
        }
        _ => return Ok(false),
    }
    Ok(true)
}

fn project_evidence_claim(
    row_json: &str,
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
) -> Result<(), ActivityStoreError> {
    let claim = serde_json::from_str::<AppGameEvidenceClaim>(row_json)
        .map_err(|_error| invalid_row("invalid-evidence-claim"))?;
    super::super::super::protocol_rows::validate_evidence_claim(&claim)
        .map_err(|_error| invalid_row("invalid-evidence-claim"))?;
    model.evidence_claim_rows.push(claim);
    Ok(())
}

fn project_identity(
    row_json: &str,
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
) -> Result<(), ActivityStoreError> {
    let identity = serde_json::from_str::<AppGameIdentity>(row_json)
        .map_err(|_error| invalid_row("invalid-identity-row"))?;
    model.identity_rows.push(identity);
    Ok(())
}

fn project_approval_authority(
    row_json: &str,
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
) -> Result<(), ActivityStoreError> {
    let authority = serde_json::from_str::<AppGameControlApprovalAuthority>(row_json)
        .map_err(|_error| invalid_row("invalid-approval-authority"))?;
    super::super::super::protocol_rows::validate_authority(&authority)
        .map_err(|_error| invalid_row("invalid-approval-authority"))?;
    model.approval_authority_rows.push(authority);
    Ok(())
}

fn project_approval_action_result(
    row_json: &str,
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
) -> Result<(), ActivityStoreError> {
    let action_result = serde_json::from_str::<AppGameControlActionResult>(row_json)
        .map_err(|_error| invalid_row("invalid-approval-action-result"))?;
    super::super::super::protocol_rows::validate_action_result(&action_result)
        .map_err(|_error| invalid_row("invalid-approval-action-result"))?;
    model.approval_action_result_rows.push(action_result);
    Ok(())
}

fn project_platform_authority_matrix(
    row_json: &str,
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
) -> Result<(), ActivityStoreError> {
    let matrix = serde_json::from_str::<AppGamePlatformAuthorityMatrix>(row_json)
        .map_err(|_error| invalid_row("invalid-platform-authority-matrix"))?;
    super::super::super::protocol_rows::validate_platform_authority_matrix(&matrix)
        .map_err(|_error| invalid_row("invalid-platform-authority-matrix"))?;
    model.platform_authority_matrices.push(matrix);
    Ok(())
}

fn project_ai_classifier_result(
    row_json: &str,
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
) -> Result<(), ActivityStoreError> {
    let classifier = serde_json::from_str::<AppGameAiClassifierResult>(row_json)
        .map_err(|_error| invalid_row("invalid-ai-classifier-result"))?;
    super::super::super::protocol_rows::validate_classifier_result(&classifier)
        .map_err(|_error| invalid_row("invalid-ai-classifier-result"))?;
    model.ai_classifier_result_rows.push(classifier);
    Ok(())
}

fn invalid_row(reason: &'static str) -> ActivityStoreError {
    ActivityStoreError::InvalidAppGameJournalRow { reason }
}
