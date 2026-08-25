use crate::admission::AuthenticatedBrokerSession;
use crate::ClientError;

pub(crate) fn connect() -> Result<AuthenticatedBrokerSession, ClientError> {
    // The previous path trusted a sysinfo PID/session/path snapshot and an
    // administrator-writable sibling executable. Neither is an immutable
    // client-side broker identity anchor. Fail before opening the pipe or
    // emitting bootstrap material until a safe signed-image or pinned SCM /
    // installer custody adapter is linked.
    Err(ClientError::DeploymentRequired)
}
