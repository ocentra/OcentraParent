use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;

use crate::BrokerError;

/// A privileged custody broker must be launched and owned by the installed
/// SYSTEM/LocalService service through SCM. The previous inherited-stdin,
/// same-token child protocol did not establish that boundary and has been
/// removed. Keep the executable fail-closed until the installer/service
/// workpack supplies the fixed service endpoint, service DACL, and launch
/// identity validation.
pub(crate) fn run(_pipe_name: &BrokerPipeName) -> Result<(), BrokerError> {
    Err(BrokerError::DeploymentRequired)
}
