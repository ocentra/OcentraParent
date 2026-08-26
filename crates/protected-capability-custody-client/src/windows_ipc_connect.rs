//! Fixed named-pipe connection entry point.

use std::time::Duration;

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream};
use interprocess::ConnectWaitMode;
use ocentra_protected_capability_custody_protocol::constants::BROKER_ACCEPT_DEADLINE_MILLIS;
use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;

use crate::admission::AuthenticatedBrokerSession;
use crate::ClientError;

pub(crate) fn connect() -> Result<AuthenticatedBrokerSession, ClientError> {
    let pipe = BrokerPipeName::fixed();
    let stream = DuplexPipeStream::<pipe_mode::Bytes>::connect_by_path_with_wait_mode(
        pipe.as_path(),
        ConnectWaitMode::Timeout(Duration::from_millis(BROKER_ACCEPT_DEADLINE_MILLIS)),
    )
    .map_err(|_error| ClientError::BrokerUnavailable)?;
    stream
        .set_nonblocking(true)
        .map_err(|_error| ClientError::Transport)?;
    super::session::establish(stream)
}
