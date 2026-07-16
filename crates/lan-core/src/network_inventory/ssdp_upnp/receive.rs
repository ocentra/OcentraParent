use std::collections::HashSet;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

use super::http::io_error;
use super::{
    fetch_ssdp_description, parse_ssdp_response, SsdpDiscoveryError, SsdpDiscoveryRecord,
    SSDP_MAX_RESPONSE_BYTES,
};

pub(super) fn collect_ssdp_records(
    socket: &UdpSocket,
    request: &[u8],
    target: SocketAddr,
    response_timeout: Duration,
    attempts: usize,
    description_timeout: Duration,
) -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    let mut results = Vec::new();
    let mut seen = HashSet::new();
    for _ in 0..attempts {
        socket
            .send_to(request, target)
            .map_err(|error| io_error(&error))?;
        receive_ssdp_attempt(
            socket,
            response_timeout,
            description_timeout,
            &mut seen,
            &mut results,
        )?;
    }
    Ok(results)
}

fn receive_ssdp_attempt(
    socket: &UdpSocket,
    response_timeout: Duration,
    description_timeout: Duration,
    seen: &mut HashSet<String>,
    results: &mut Vec<SsdpDiscoveryRecord>,
) -> Result<(), SsdpDiscoveryError> {
    let deadline = Instant::now() + response_timeout;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Ok(());
        }
        socket
            .set_read_timeout(Some(remaining))
            .map_err(|error| io_error(&error))?;
        let mut buffer = vec![0_u8; SSDP_MAX_RESPONSE_BYTES];
        match socket.recv_from(&mut buffer) {
            Ok((size, _)) => add_ssdp_record(&buffer[..size], description_timeout, seen, results)?,
            Err(error) if is_ssdp_timeout(&error) => return Ok(()),
            Err(error) => return Err(io_error(&error)),
        }
    }
}

fn add_ssdp_record(
    response_bytes: &[u8],
    description_timeout: Duration,
    seen: &mut HashSet<String>,
    results: &mut Vec<SsdpDiscoveryRecord>,
) -> Result<(), SsdpDiscoveryError> {
    let response = match parse_ssdp_response(response_bytes) {
        Ok(response) => response,
        Err(SsdpDiscoveryError::MalformedResponse)
        | Err(SsdpDiscoveryError::MissingLocation)
        | Err(SsdpDiscoveryError::MissingSearchTarget)
        | Err(SsdpDiscoveryError::MissingUsn) => return Ok(()),
        Err(error) => return Err(error),
    };
    if !seen.insert(response.dedup_key()) {
        return Ok(());
    }
    let description = response
        .description_fetch_allowed()
        .then(|| fetch_ssdp_description(&response.location, description_timeout).ok())
        .flatten();
    results.push(SsdpDiscoveryRecord {
        response,
        description,
    });
    Ok(())
}

fn is_ssdp_timeout(error: &std::io::Error) -> bool {
    matches!(
        error.kind(),
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
    )
}
