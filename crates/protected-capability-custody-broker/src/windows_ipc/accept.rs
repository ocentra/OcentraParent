use std::io;
use std::thread;
use std::time::{Duration, Instant};

use ocentra_protected_capability_custody_protocol::constants::BROKER_ACCEPT_DEADLINE_MILLIS;
use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;

use crate::BrokerError;

pub(super) fn run(pipe_name: &BrokerPipeName) -> Result<(), BrokerError> {
    let bootstrap = super::read_bootstrap()?;
    if &BrokerPipeName::from_nonce(bootstrap.identity().pipe_nonce()) != pipe_name {
        return Err(BrokerError::InvalidLaunch);
    }
    let listener = super::create_listener(pipe_name)?;
    let deadline = Instant::now()
        .checked_add(Duration::from_millis(BROKER_ACCEPT_DEADLINE_MILLIS))
        .ok_or(BrokerError::Transport)?;
    loop {
        let mut stream = match listener.accept() {
            Ok(stream) => stream,
            Err(error)
                if error.kind() == io::ErrorKind::WouldBlock && Instant::now() < deadline =>
            {
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(error) => return Err(super::map_transport_error(error)),
        };
        if stream.set_nonblocking(true).is_err() {
            // A peer-side transport failure must not tear down the one-shot
            // listener or leave the handshake on an unbounded blocking read.
            continue;
        }
        match super::serve_authenticated_peer(&mut stream, &bootstrap, deadline) {
            Ok(()) => return Ok(()),
            Err(error) if Instant::now() < deadline => {
                drop(error);
                continue;
            }
            Err(error) => return Err(error),
        }
    }
}
