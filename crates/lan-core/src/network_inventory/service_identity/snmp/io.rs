use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

const SNMP_IO_POLL_SLICE: Duration = Duration::from_millis(50);

pub(super) fn send_until(
    socket: &UdpSocket,
    request: &[u8],
    endpoint: SocketAddr,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<()> {
    loop {
        socket
            .set_write_timeout(Some(poll_timeout(deadline, cancellation)?))
            .ok()?;
        match socket.send_to(request, endpoint) {
            Ok(_) => return Some(()),
            Err(error) if retryable(&error) => continue,
            Err(_) => return None,
        }
    }
}

pub(super) fn receive_until(
    socket: &UdpSocket,
    response: &mut [u8],
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<usize> {
    loop {
        socket
            .set_read_timeout(Some(poll_timeout(deadline, cancellation)?))
            .ok()?;
        match socket.recv_from(response) {
            Ok((read, _)) => return Some(read),
            Err(error) if retryable(&error) => continue,
            Err(_) => return None,
        }
    }
}

fn poll_timeout(deadline: Instant, cancellation: Option<&AtomicBool>) -> Option<Duration> {
    if cancellation.is_some_and(|value| value.load(Ordering::Acquire)) || Instant::now() >= deadline
    {
        return None;
    }
    Some(
        deadline
            .saturating_duration_since(Instant::now())
            .min(SNMP_IO_POLL_SLICE),
    )
}

fn retryable(error: &std::io::Error) -> bool {
    matches!(
        error.kind(),
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
    )
}
