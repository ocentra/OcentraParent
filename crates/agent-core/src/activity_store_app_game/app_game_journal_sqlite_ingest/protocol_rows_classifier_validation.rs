use std::collections::BTreeSet;

use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameAiClassifierResult, APP_GAME_AI_CLASSIFIER_CANDIDATE_GAME_CONTEXT,
    APP_GAME_AI_CLASSIFIER_CANDIDATE_UNKNOWN_IDENTITY, APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY,
    APP_GAME_AI_CLASSIFIER_DIGEST_SESSION_SUMMARY,
    APP_GAME_AI_CLASSIFIER_FALLBACK_LOCAL_MODEL_UNAVAILABLE,
    APP_GAME_AI_CLASSIFIER_FALLBACK_NOT_NEEDED, APP_GAME_AI_CLASSIFIER_HANDOFF_MANUAL_REVIEW,
    APP_GAME_AI_CLASSIFIER_HANDOFF_PARENT_REVIEW, APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_APP,
    APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_GAME, APP_GAME_AI_CLASSIFIER_STATE_CANDIDATE,
    APP_GAME_AI_CLASSIFIER_STATE_PROVIDER_UNAVAILABLE,
};

use super::AppGameJournalSqliteIngestError;

pub(super) fn validate_classifier_result(
    row: &AppGameAiClassifierResult,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.schema_version != APP_GAME_SCHEMA_VERSION {
        return Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported);
    }
    if row.direct_action_requested || row.raw_scan_included || row.content_claim_included {
        return Err(AppGameJournalSqliteIngestError::ClassifierRequestsAction);
    }
    if !row.confidence.is_finite() || !(0.0..=1.0).contains(&row.confidence) {
        return Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid);
    }
    if !valid_classifier_inputs(row) || !valid_classifier_contract(row) {
        return Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid);
    }
    Ok(())
}

fn valid_classifier_inputs(row: &AppGameAiClassifierResult) -> bool {
    [
        row.classifier_run_id.as_str(),
        row.digest_ref.as_str(),
        row.model_runtime_ref.as_str(),
        row.prompt_template_ref.as_str(),
        row.prompt_version.as_str(),
        row.generated_at.as_str(),
    ]
    .iter()
    .all(|value| valid_classifier_identifier(value))
        && valid_classifier_text(&row.candidate_label)
        && valid_classifier_references(&row.source_evidence_refs, true)
        && valid_classifier_references(&row.source_session_refs, false)
        && row
            .uncertainty_reason_codes
            .iter()
            .all(|reason| valid_classifier_text(reason))
}

fn valid_classifier_contract(row: &AppGameAiClassifierResult) -> bool {
    candidate_kind_matches_product(row)
        && matches!(
            row.source_digest_kind.as_str(),
            APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY | APP_GAME_AI_CLASSIFIER_DIGEST_SESSION_SUMMARY
        )
        && matches!(
            (
                row.classifier_state.as_str(),
                row.fallback_state.as_str(),
                row.policy_handoff.as_str(),
            ),
            (
                APP_GAME_AI_CLASSIFIER_STATE_CANDIDATE,
                APP_GAME_AI_CLASSIFIER_FALLBACK_NOT_NEEDED,
                APP_GAME_AI_CLASSIFIER_HANDOFF_PARENT_REVIEW,
            ) | (
                APP_GAME_AI_CLASSIFIER_STATE_PROVIDER_UNAVAILABLE,
                APP_GAME_AI_CLASSIFIER_FALLBACK_LOCAL_MODEL_UNAVAILABLE,
                APP_GAME_AI_CLASSIFIER_HANDOFF_MANUAL_REVIEW,
            )
        )
}

fn candidate_kind_matches_product(row: &AppGameAiClassifierResult) -> bool {
    match row.product_kind.as_str() {
        APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_APP => {
            row.candidate_kind == APP_GAME_AI_CLASSIFIER_CANDIDATE_UNKNOWN_IDENTITY
        }
        APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_GAME => {
            row.candidate_kind == APP_GAME_AI_CLASSIFIER_CANDIDATE_GAME_CONTEXT
        }
        _ => false,
    }
}

fn valid_classifier_identifier(value: &str) -> bool {
    !value.trim().is_empty() && value == value.trim()
}

fn valid_classifier_text(value: &str) -> bool {
    !value.trim().is_empty()
}

fn valid_classifier_references(values: &[String], require_one: bool) -> bool {
    (!require_one || !values.is_empty())
        && values
            .iter()
            .all(|value| valid_classifier_identifier(value))
        && {
            let mut seen = BTreeSet::new();
            values.iter().all(|value| seen.insert(value.as_str()))
        }
}
