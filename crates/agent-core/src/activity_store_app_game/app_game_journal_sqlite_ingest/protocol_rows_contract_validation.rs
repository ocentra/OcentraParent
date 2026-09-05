use ocentra_parent_agent_protocol::app_game::{
    AppGameEvidenceClaim, APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameControlActionResult, AppGameControlApprovalAuthority, AppGamePlatformAuthorityMatrix,
    APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED, APP_GAME_CONTROL_AUTHORITY_ACTIVE,
    APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION, APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED,
};

use super::AppGameJournalSqliteIngestError;

pub(super) fn validate_evidence_claim(
    row: &AppGameEvidenceClaim,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    if row.claim_kind == APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY
        && (row.runtime_state != APP_GAME_RUNTIME_NOT_CLAIMED
            || row.foreground_state != APP_GAME_FOREGROUND_NOT_CLAIMED)
    {
        return Err(AppGameJournalSqliteIngestError::EvidenceClaimInventoryClaimsUse);
    }
    Ok(())
}

pub(super) fn validate_authority(
    row: &AppGameControlApprovalAuthority,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    if row.authority_state != APP_GAME_CONTROL_AUTHORITY_ACTIVE
        && (row.can_approve || row.can_deny || row.can_extend || row.can_override)
    {
        return Err(AppGameJournalSqliteIngestError::AuthorityInactiveGrants);
    }
    Ok(())
}

pub(super) fn validate_action_result(
    row: &AppGameControlActionResult,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    if row.result_status == APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED
        && row.enforcement_result.is_some()
    {
        return Err(AppGameJournalSqliteIngestError::ActionResultManualExecution);
    }
    Ok(())
}

pub(super) fn validate_platform_authority_matrix(
    row: &AppGamePlatformAuthorityMatrix,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    if row.rows.iter().any(|authority| {
        authority.authority_tier == APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED
            && authority.can_execute_adapter
    }) {
        return Err(AppGameJournalSqliteIngestError::PlatformAuthorityManualExecution);
    }
    Ok(())
}
