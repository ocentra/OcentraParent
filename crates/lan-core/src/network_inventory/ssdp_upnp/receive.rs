use std::collections::HashSet;
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use super::http::io_error;
use super::{SsdpDiscoveryError, SsdpDiscoveryRecord, SSDP_MAX_RESPONSE_BYTES};

mod record;

pub(super) fn collect_ssdp_records_with_cancellation(
    socket: &UdpSocket,
    request: &[u8],
    target: SocketAddr,
    response_timeout: Duration,
    attempts: usize,
    description_timeout: Duration,
    cancellation: Option<&AtomicBool>,
) -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    let mut results = Vec::new();
    let mut seen = HashSet::new();
    for _ in 0..attempts {
        if is_cancelled(cancellation) || results.len() >= super::SSDP_MAX_RECORDS {
            break;
        }
        socket
            .send_to(request, target)
            .map_err(|error| io_error(&error))?;
        receive_ssdp_attempt(
            socket,
            response_timeout,
            description_timeout,
            &mut seen,
            &mut results,
            cancellation,
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
    cancellation: Option<&AtomicBool>,
) -> Result<(), SsdpDiscoveryError> {
    let deadline = Instant::now() + response_timeout;
    let mut received_responses = 0;
    loop {
        if is_cancelled(cancellation)
            || results.len() >= super::SSDP_MAX_RECORDS
            || received_responses >= super::SSDP_MAX_RESPONSES_PER_ATTEMPT
        {
            return Ok(());
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Ok(());
        }
        socket
            .set_read_timeout(Some(remaining))
            .map_err(|error| io_error(&error))?;
        let mut buffer = vec![0_u8; SSDP_MAX_RESPONSE_BYTES];
        match socket.recv_from(&mut buffer) {
            Ok((size, _)) => {
                received_responses = received_responses.saturating_add(1);
                record::add_ssdp_record(
                    &buffer[..size],
                    description_timeout,
                    seen,
                    results,
                    cancellation,
                )?
            }
            Err(error) if is_ssdp_timeout(&error) => return Ok(()),
            Err(error) => return Err(io_error(&error)),
        }
    }
}

fn is_cancelled(cancellation: Option<&AtomicBool>) -> bool {
    cancellation.is_some_and(|value| value.load(std::sync::atomic::Ordering::Acquire))
}

fn is_ssdp_timeout(error: &std::io::Error) -> bool {
    matches!(
        error.kind(),
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
    )
}
