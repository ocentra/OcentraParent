use std::io::Write;
use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use super::deadline::{retryable, unavailable};

pub(super) fn write_probe_request_until<W: Write>(
    stream: &mut W,
    endpoint: &SocketAddr,
    path: &str,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<()> {
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nAccept: text/html, application/json;q=0.9, */*;q=0.1\r\nConnection: close\r\n\r\n",
        path, endpoint
    );
    write_all_until(stream, request.as_bytes(), deadline, cancellation)
}

pub(super) fn write_all_until<W: Write>(
    stream: &mut W,
    bytes: &[u8],
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<()> {
    let mut written = 0;
    while written < bytes.len() {
        if unavailable(deadline, cancellation) {
            return None;
        }
        match stream.write(&bytes[written..]) {
            Ok(0) => return None,
            Ok(count) => written = written.saturating_add(count),
            Err(error) if retryable(&error) => continue,
            Err(_) => return None,
        }
    }
    Some(())
}
