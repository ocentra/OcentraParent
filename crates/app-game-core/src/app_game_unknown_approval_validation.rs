use crate::app_game_unknown_approval_types::{
    AppGameUnknownApprovalError, AppGameUnknownCandidate, AppGameUnknownCandidateKind,
    AppGameUnknownClassification,
};

pub(crate) fn require_text(
    value: &str,
    field: &'static str,
) -> Result<(), AppGameUnknownApprovalError> {
    if value.trim().is_empty() {
        return Err(AppGameUnknownApprovalError::InvalidField { field });
    }
    Ok(())
}

pub(crate) fn require_refs(
    values: &[String],
    field: &'static str,
) -> Result<(), AppGameUnknownApprovalError> {
    if values.is_empty() || values.iter().any(|value| value.trim().is_empty()) {
        return Err(AppGameUnknownApprovalError::InvalidField { field });
    }
    Ok(())
}

pub(crate) fn validate_optional_refs(
    values: &[String],
    field: &'static str,
) -> Result<(), AppGameUnknownApprovalError> {
    if values.iter().any(|value| value.trim().is_empty()) {
        return Err(AppGameUnknownApprovalError::InvalidField { field });
    }
    Ok(())
}

pub(crate) fn require_optional_text(
    value: &Option<String>,
    field: &'static str,
) -> Result<(), AppGameUnknownApprovalError> {
    match value {
        Some(value) => require_text(value, field),
        None => Err(AppGameUnknownApprovalError::InvalidField { field }),
    }
}

pub(crate) fn validate_unknown_candidate(
    candidate: &AppGameUnknownCandidate,
) -> Result<(), AppGameUnknownApprovalError> {
    require_text(
        &candidate.candidate_id,
        "app_game.unknown_candidate.candidate_id",
    )?;
    require_text(
        &candidate.subject_ref,
        "app_game.unknown_candidate.subject_ref",
    )?;
    require_text(
        &candidate.device_ref,
        "app_game.unknown_candidate.device_ref",
    )?;
    require_text(
        &candidate.local_user_ref,
        "app_game.unknown_candidate.local_user_ref",
    )?;
    require_refs(
        &candidate.evidence_refs,
        "app_game.unknown_candidate.evidence_refs",
    )?;
    validate_optional_ref(
        candidate.category_candidate_ref.as_ref(),
        "app_game.unknown_candidate.category_candidate_ref",
    )?;
    require_refs(
        &candidate.child_status_refs,
        "app_game.unknown_candidate.child_status_refs",
    )?;
    validate_candidate_classification(candidate)
}

fn validate_optional_ref(
    value: Option<&String>,
    field: &'static str,
) -> Result<(), AppGameUnknownApprovalError> {
    if value.is_some_and(|value| value.trim().is_empty()) {
        return Err(AppGameUnknownApprovalError::InvalidField { field });
    }
    Ok(())
}

fn validate_candidate_classification(
    candidate: &AppGameUnknownCandidate,
) -> Result<(), AppGameUnknownApprovalError> {
    if candidate.classification == AppGameUnknownClassification::PossibleGame
        && !matches!(
            candidate.kind,
            AppGameUnknownCandidateKind::UnknownProcess
                | AppGameUnknownCandidateKind::LauncherGameCandidate
                | AppGameUnknownCandidateKind::GameLikeExecutable
        )
    {
        return invalid_transition(
            "possible-game classification requires game-like evidence source",
        );
    }
    Ok(())
}

pub(crate) fn invalid_transition<T>(
    reason: &'static str,
) -> Result<T, AppGameUnknownApprovalError> {
    Err(AppGameUnknownApprovalError::InvalidTransition { reason })
}
