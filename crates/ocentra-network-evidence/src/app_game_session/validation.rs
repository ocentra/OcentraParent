use super::{
    NetworkAppGameSessionCorrelationError, NetworkAppGameSessionCorrelationInput,
    NetworkAppGameStoredSessionEvidence,
};

pub(super) fn validate_app_game_session_input(
    input: &NetworkAppGameSessionCorrelationInput,
) -> Result<(), NetworkAppGameSessionCorrelationError> {
    if input.network_flow_ref.trim().is_empty() {
        return Err(NetworkAppGameSessionCorrelationError::EmptyNetworkFlowRef);
    }
    if let Some(session) = &input.stored_session {
        validate_session(session)?;
    }
    Ok(())
}

fn validate_session(
    session: &NetworkAppGameStoredSessionEvidence,
) -> Result<(), NetworkAppGameSessionCorrelationError> {
    if session.evidence_ref.trim().is_empty() {
        return Err(NetworkAppGameSessionCorrelationError::EmptyEvidenceRef);
    }

    for (value, error) in [
        (
            session.session_summary_ref.as_ref(),
            NetworkAppGameSessionCorrelationError::EmptySessionSummaryRef,
        ),
        (
            session.foreground_evidence_ref.as_ref(),
            NetworkAppGameSessionCorrelationError::EmptyForegroundEvidenceRef,
        ),
        (
            session.process_correlation_ref.as_ref(),
            NetworkAppGameSessionCorrelationError::EmptyProcessCorrelationRef,
        ),
        (
            session.app_id.as_ref(),
            NetworkAppGameSessionCorrelationError::EmptyAppId,
        ),
        (
            session.display_name.as_ref(),
            NetworkAppGameSessionCorrelationError::EmptyDisplayName,
        ),
        (
            session.process_name.as_ref(),
            NetworkAppGameSessionCorrelationError::EmptyProcessName,
        ),
        (
            session.launcher_ref.as_ref(),
            NetworkAppGameSessionCorrelationError::EmptyLauncherRef,
        ),
    ] {
        validate_optional_ref(value, error)?;
    }

    if let Some(confidence) = session.confidence {
        if confidence > 100 {
            return Err(NetworkAppGameSessionCorrelationError::InvalidConfidence(
                confidence,
            ));
        }
    }
    if session.foreground_time_seconds > session.running_time_seconds {
        return Err(NetworkAppGameSessionCorrelationError::ForegroundTimeExceedsRunningTime);
    }
    validate_session_non_claims(session)?;
    Ok(())
}

fn validate_session_non_claims(
    session: &NetworkAppGameStoredSessionEvidence,
) -> Result<(), NetworkAppGameSessionCorrelationError> {
    for (claimed, error) in [
        (
            session.exact_url_claimed,
            NetworkAppGameSessionCorrelationError::UnsupportedExactUrlClaim,
        ),
        (
            session.screen_content_claimed,
            NetworkAppGameSessionCorrelationError::UnsupportedScreenContentClaim,
        ),
        (
            session.ai_scanned_device,
            NetworkAppGameSessionCorrelationError::UnsupportedAiScannerClaim,
        ),
        (
            session.policy_action_authority,
            NetworkAppGameSessionCorrelationError::UnsupportedPolicyAuthorityClaim,
        ),
        (
            session.adapter_action_authorized,
            NetworkAppGameSessionCorrelationError::UnsupportedAdapterAuthorityClaim,
        ),
        (
            session.enforcement_command_published,
            NetworkAppGameSessionCorrelationError::UnsupportedEnforcementCommandClaim,
        ),
    ] {
        if claimed {
            return Err(error);
        }
    }
    Ok(())
}

fn validate_optional_ref(
    value: Option<&String>,
    error: NetworkAppGameSessionCorrelationError,
) -> Result<(), NetworkAppGameSessionCorrelationError> {
    if value.is_some_and(|value| value.trim().is_empty()) {
        return Err(error);
    }
    Ok(())
}
