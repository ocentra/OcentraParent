use std::time::Duration;

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream};
use interprocess::ConnectWaitMode;
use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;

use super::io;
use crate::ClientError;

type PipeStream = DuplexPipeStream<pipe_mode::Bytes>;

const CONNECT_RETRY_MILLIS: u64 = 10;

pub(super) fn connect_pipe(pipe_name: &BrokerPipeName) -> Result<PipeStream, ClientError> {
    let deadline = io::connection_deadline()?;
    loop {
        if std::time::Instant::now() >= deadline {
            return Err(ClientError::BrokerUnavailable);
        }
        let wait = Duration::from_millis(CONNECT_RETRY_MILLIS);
        match PipeStream::connect_by_path_with_wait_mode(
            pipe_name.as_path(),
            ConnectWaitMode::Timeout(wait),
        ) {
            Ok(stream) => return Ok(stream),
            Err(error) if error.kind() == std::io::ErrorKind::TimedOut => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(io::map_transport_error(error)),
        }
    }
}
