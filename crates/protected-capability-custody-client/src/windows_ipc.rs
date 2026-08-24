use crate::admission::AuthenticatedBrokerSession;
use crate::ClientError;

pub(crate) struct WindowsBrokerSession;

impl WindowsBrokerSession {
    pub(crate) fn execute(
        self,
        _request: crate::admission::ClientRequest,
    ) -> Result<crate::admission::AuthenticatedResponse, ClientError> {
        Err(ClientError::DeploymentRequired)
    }
}

/// The client cannot safely create the privileged custody broker. The old
/// same-token child process path was removed: it could not establish the
/// SYSTEM/LocalService service identity, SCM launch ownership, or service
/// endpoint DACL required by the protocol. A signed installer/service
/// workpack must provide that boundary before this function can return a
/// session. Keeping the failure typed prevents callers from treating an
/// ordinary child process as an isolated broker.
pub(crate) fn connect() -> Result<AuthenticatedBrokerSession, ClientError> {
    Err(ClientError::DeploymentRequired)
}
