use std::{io::Read, net::TcpStream, sync::atomic::AtomicBool, time::Instant};

use super::super::super::http::io_error;
use super::super::super::{SsdpDiscoveryError, SSDP_MAX_DESCRIPTION_BYTES};
use super::deadline::{is_retryable_timeout, poll_timeout};

pub(super) fn read_http_response(stream: &mut TcpStream) -> Result<Vec<u8>, SsdpDiscoveryError> {
    read_http_response_inner(stream, None, None)
}

pub(super) fn read_http_response_until(
    stream: &mut TcpStream,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Result<Vec<u8>, SsdpDiscoveryError> {
    read_http_response_inner(stream, Some(deadline), cancellation)
}

fn read_http_response_inner(
    stream: &mut TcpStream,
    deadline: Option<Instant>,
    cancellation: Option<&AtomicBool>,
) -> Result<Vec<u8>, SsdpDiscoveryError> {
    let mut response = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        let read = read_chunk(stream, &mut chunk, deadline, cancellation)?;
        if read == 0 {
            break;
        }
        if response.len().saturating_add(read) > SSDP_MAX_DESCRIPTION_BYTES {
            return Err(SsdpDiscoveryError::ResponseTooLarge);
        }
        response.extend_from_slice(&chunk[..read]);
    }
    (!response.is_empty())
        .then_some(response)
        .ok_or(SsdpDiscoveryError::Timeout)
}

fn read_chunk(
    stream: &mut TcpStream,
    chunk: &mut [u8],
    deadline: Option<Instant>,
    cancellation: Option<&AtomicBool>,
) -> Result<usize, SsdpDiscoveryError> {
    let Some(deadline) = deadline else {
        return stream.read(chunk).map_err(|error| io_error(&error));
    };
    stream
        .set_read_timeout(Some(poll_timeout(deadline, cancellation)?))
        .map_err(|error| io_error(&error))?;
    loop {
        match stream.read(chunk) {
            Ok(read) => return Ok(read),
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) if is_retryable_timeout(&error) => {
                stream
                    .set_read_timeout(Some(poll_timeout(deadline, cancellation)?))
                    .map_err(|error| io_error(&error))?;
            }
            Err(error) => return Err(io_error(&error)),
        }
    }
}
