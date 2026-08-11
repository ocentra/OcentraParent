use super::{
    RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantError, RemoteAccessGrantState,
    RemoteAccessGrantTransition,
};

mod confirm;
mod lifecycle;
mod reconnect;
mod terminal;

pub(super) fn apply(
    grant: &mut RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    let next = match transition {
        RemoteAccessGrantTransition::ConfirmParent => confirm::parent(grant, context)?,
        RemoteAccessGrantTransition::Pair => confirm::pair(grant, context)?,
        RemoteAccessGrantTransition::Activate => lifecycle::activate(grant, context)?,
        RemoteAccessGrantTransition::Pause => lifecycle::pause(grant)?,
        RemoteAccessGrantTransition::Stop => lifecycle::stop(grant, context)?,
        RemoteAccessGrantTransition::RequestReconnect => reconnect::request(grant)?,
        RemoteAccessGrantTransition::Reconnect => reconnect::complete(grant, context)?,
        RemoteAccessGrantTransition::Revoke
        | RemoteAccessGrantTransition::RemoveDevice
        | RemoteAccessGrantTransition::Deny
        | RemoteAccessGrantTransition::Fail
        | RemoteAccessGrantTransition::Supersede => terminal::apply(grant, transition, context)?,
    };
    grant.state = next;
    Ok(next)
}
