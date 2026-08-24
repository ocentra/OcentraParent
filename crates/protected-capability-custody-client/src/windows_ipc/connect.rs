use std::thread;
use std::time::Duration;

use super::CONNECT_RETRY_MILLIS;
use super::{AuthenticatedBrokerSession, ClientError};

pub(super) fn retry(
    mut connect_once: impl FnMut() -> Result<AuthenticatedBrokerSession, ClientError>,
) -> Result<AuthenticatedBrokerSession, ClientError> {
    let mut last_error = ClientError::BrokerUnavailable;
    for attempt in
        0..ocentra_protected_capability_custody_protocol::constants::BROKER_RESTART_ATTEMPTS
    {
        match connect_once() {
            Ok(session) => return Ok(session),
            Err(error) if retryable(&error) && attempt + 1 < retry_limit() => {
                last_error = error;
                thread::sleep(Duration::from_millis(CONNECT_RETRY_MILLIS));
            }
            Err(error) => return Err(error),
        }
    }
    Err(last_error)
}

fn retry_limit() -> u8 {
    ocentra_protected_capability_custody_protocol::constants::BROKER_RESTART_ATTEMPTS
}

fn retryable(error: &ClientError) -> bool {
    matches!(
        error,
        ClientError::BrokerUnavailable | ClientError::Transport | ClientError::PeerAuthentication
    )
}
