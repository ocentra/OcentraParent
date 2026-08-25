use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream, PipeListener};
use ocentra_protected_capability_custody_protocol::constants::BROKER_ACCEPT_DEADLINE_MILLIS;

use crate::BrokerError;

type PipeListenerType = PipeListener<pipe_mode::Bytes, pipe_mode::Bytes>;
type PipeStream = DuplexPipeStream<pipe_mode::Bytes>;

const POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(super) fn accept_until(
    listener: &PipeListenerType,
    stopping: &AtomicBool,
) -> Result<Option<PipeStream>, BrokerError> {
    let deadline = connection_deadline()?;
    loop {
        if stopping.load(Ordering::Acquire) {
            return Ok(None);
        }
        match poll_accept(listener, deadline)? {
            AcceptPoll::Connected(stream) => return Ok(Some(stream)),
            AcceptPoll::Retry => continue,
            AcceptPoll::Deadline => return Ok(None),
        }
    }
}

enum AcceptPoll {
    Connected(PipeStream),
    Retry,
    Deadline,
}

fn poll_accept(listener: &PipeListenerType, deadline: Instant) -> Result<AcceptPoll, BrokerError> {
    match listener.accept() {
        Ok(stream) => Ok(AcceptPoll::Connected(stream)),
        Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
            if Instant::now() >= deadline {
                return Ok(AcceptPoll::Deadline);
            }
            thread::sleep(POLL_INTERVAL);
            Ok(AcceptPoll::Retry)
        }
        Err(error) => Err(map_transport_error(error)),
    }
}

pub(super) fn connection_deadline() -> Result<Instant, BrokerError> {
    Instant::now()
        .checked_add(Duration::from_millis(BROKER_ACCEPT_DEADLINE_MILLIS))
        .ok_or(BrokerError::Transport)
}

fn map_transport_error(_error: io::Error) -> BrokerError {
    BrokerError::Transport
}
