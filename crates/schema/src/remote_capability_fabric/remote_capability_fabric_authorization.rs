use super::{
    RemoteActorRole, RemoteCapabilityAuthorizationError, RemoteDiagnosticRedactionState,
    RemoteParentGrantState, RemoteRoute,
};

pub(super) fn require_actor(
    role: &RemoteActorRole,
    parent_grant: RemoteParentGrantState,
    support_actor_ref: Option<&str>,
    parent_actor_ref: &str,
    requesting_actor_ref: &str,
) -> Result<(), RemoteCapabilityAuthorizationError> {
    match role {
        RemoteActorRole::ParentOwner | RemoteActorRole::CoParent => {
            if support_actor_ref.is_some() {
                return Err(RemoteCapabilityAuthorizationError::WrongActorRole);
            }
            if parent_actor_ref != requesting_actor_ref {
                return Err(RemoteCapabilityAuthorizationError::WrongParentActor);
            }
        }
        RemoteActorRole::SupportAdmin => {
            if parent_grant != RemoteParentGrantState::Granted {
                return Err(RemoteCapabilityAuthorizationError::WrongActorRole);
            }
            let support_actor_ref = support_actor_ref
                .filter(|actor| !actor.trim().is_empty())
                .ok_or(RemoteCapabilityAuthorizationError::MissingSupportActor)?;
            if support_actor_ref != requesting_actor_ref {
                return Err(RemoteCapabilityAuthorizationError::WrongSupportActor);
            }
        }
        RemoteActorRole::ChildAgent => {
            return Err(RemoteCapabilityAuthorizationError::WrongActorRole)
        }
    }
    Ok(())
}

pub(super) fn require_target(
    child_device_ref: &str,
    requested_child_device_ref: &str,
    route: RemoteRoute,
    expected_route: RemoteRoute,
) -> Result<(), RemoteCapabilityAuthorizationError> {
    if child_device_ref != requested_child_device_ref {
        return Err(RemoteCapabilityAuthorizationError::WrongChildDevice);
    }
    if route != expected_route {
        return Err(RemoteCapabilityAuthorizationError::WrongRoute);
    }
    Ok(())
}

pub(super) fn require_safe_support_redaction(
    _actor_role: &RemoteActorRole,
    redaction_state: RemoteDiagnosticRedactionState,
) -> Result<(), RemoteCapabilityAuthorizationError> {
    if redaction_state != RemoteDiagnosticRedactionState::Redacted {
        return Err(RemoteCapabilityAuthorizationError::DiagnosticRedactionRequired);
    }
    Ok(())
}
