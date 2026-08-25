use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use super::super::SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES;

mod bounded_read;
mod bounded_write;
mod connect;
mod deadline;

pub(super) fn write_probe_request<W: Write>(
    stream: &mut W,
    endpoint: &SocketAddr,
    path: &str,
) -> std::io::Result<()> {
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nAccept: text/html, application/json;q=0.9, */*;q=0.1\r\nConnection: close\r\n\r\n",
        path, endpoint
    );
    stream.write_all(request.as_bytes())
}

pub(super) fn read_probe_response<R: Read>(stream: &mut R) -> Option<Vec<u8>> {
    let mut response = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        let read = read_probe_chunk(stream, &mut chunk)?;
        if read == 0 {
            break;
        }
        if response.len().saturating_add(read) > SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES {
            return None;
        }
        response.extend_from_slice(&chunk[..read]);
    }
    (!response.is_empty()).then_some(response)
}

pub(in crate::network_inventory::service_identity) fn write_probe_request_until<W: Write>(
    stream: &mut W,
    endpoint: &SocketAddr,
    path: &str,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<()> {
    bounded_write::write_probe_request_until(stream, endpoint, path, deadline, cancellation)
}

pub(in crate::network_inventory::service_identity) fn write_all_until<W: Write>(
    stream: &mut W,
    bytes: &[u8],
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<()> {
    bounded_write::write_all_until(stream, bytes, deadline, cancellation)
}

pub(in crate::network_inventory::service_identity) fn read_probe_response_until<R: Read>(
    stream: &mut R,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<Vec<u8>> {
    bounded_read::read_probe_response_until(stream, deadline, cancellation)
}

pub(in crate::network_inventory::service_identity) const SERVICE_IDENTITY_IO_POLL_SLICE: Duration =
    deadline::IO_POLL_SLICE;

pub(in crate::network_inventory::service_identity) fn poll_timeout(
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<Duration> {
    deadline::poll_timeout(deadline, cancellation)
}

pub(in crate::network_inventory::service_identity) fn connect_until(
    endpoint: SocketAddr,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<TcpStream> {
    connect::connect_until(endpoint, deadline, cancellation)
}

fn read_probe_chunk<R: Read>(stream: &mut R, chunk: &mut [u8]) -> Option<usize> {
    match stream.read(chunk) {
        Ok(read) => Some(read),
        Err(error) => is_probe_timeout(&error).then_some(0),
    }
}

fn is_probe_timeout(error: &std::io::Error) -> bool {
    matches!(
        error.kind(),
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
    )
}
