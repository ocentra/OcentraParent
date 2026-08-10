use super::{RemoteActorRole, RemoteCapabilityAuthorizationError, RemoteDiagnosticRedactionState};

pub(super) fn require_safe_support_redaction(
    actor_role: &RemoteActorRole,
    redaction_state: RemoteDiagnosticRedactionState,
) -> Result<(), RemoteCapabilityAuthorizationError> {
    if *actor_role == RemoteActorRole::SupportAdmin
        && redaction_state != RemoteDiagnosticRedactionState::Redacted
    {
        return Err(RemoteCapabilityAuthorizationError::DiagnosticRedactionRequired);
    }
    Ok(())
}
