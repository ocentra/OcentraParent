use std::net::{SocketAddr, TcpStream};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use super::super::http::{io_error, parse_allowed_http_location};
use super::super::{SsdpDeviceDescription, SsdpDiscoveryError};
use super::parse_device_description_response;

mod deadline;
mod read;
mod write;

use self::deadline::{ensure_active, is_retryable_timeout, poll_timeout};
use self::write::write_all_until;

pub(super) fn fetch_ssdp_description(
    location: &str,
    timeout: Duration,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    let now = Instant::now();
    let deadline = now.checked_add(timeout).unwrap_or(now);
    fetch_ssdp_description_until(location, deadline, None)
}

pub(super) fn fetch_ssdp_description_until(
    location: &str,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    ensure_active(deadline, cancellation)?;
    let allowed_location = parse_allowed_http_location(location)?;
    let mut stream = connect_until(allowed_location.addr, deadline, cancellation)?;
    ensure_active(deadline, cancellation)?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAccept: application/xml,text/xml\r\n\r\n",
        allowed_location.path, allowed_location.addr
    );
    write_all_until(&mut stream, request.as_bytes(), deadline, cancellation)?;
    let response = read::read_http_response_until(&mut stream, deadline, cancellation)?;
    parse_device_description_response(&response, location)
}

fn connect_until(
    addr: SocketAddr,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Result<TcpStream, SsdpDiscoveryError> {
    loop {
        match TcpStream::connect_timeout(&addr, poll_timeout(deadline, cancellation)?) {
            Ok(stream) => return Ok(stream),
            Err(error) if is_retryable_timeout(&error) => continue,
            Err(error) => return Err(io_error(&error)),
        }
    }
}

pub(super) fn read_http_response(stream: &mut TcpStream) -> Result<Vec<u8>, SsdpDiscoveryError> {
    read::read_http_response(stream)
}
