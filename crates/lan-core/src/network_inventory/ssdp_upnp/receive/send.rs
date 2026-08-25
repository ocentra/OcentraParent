use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use super::super::http::io_error;
use super::super::SsdpDiscoveryError;
use super::{is_cancelled, is_ssdp_timeout, SSDP_IO_POLL_SLICE};

pub(super) fn send_request_until(
    socket: &UdpSocket,
    request: &[u8],
    target: SocketAddr,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Result<(), SsdpDiscoveryError> {
    loop {
        if is_cancelled(cancellation) || Instant::now() >= deadline {
            return Err(SsdpDiscoveryError::Timeout);
        }
        socket
            .set_write_timeout(Some(
                deadline
                    .saturating_duration_since(Instant::now())
                    .min(SSDP_IO_POLL_SLICE),
            ))
            .map_err(|error| io_error(&error))?;
        match socket.send_to(request, target) {
            Ok(_) => return Ok(()),
            Err(error) if is_ssdp_timeout(&error) => continue,
            Err(error) => return Err(io_error(&error)),
        }
    }
}
