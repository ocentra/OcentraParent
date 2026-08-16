use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use super::super::http::{io_error, parse_allowed_http_location};
use super::super::{SsdpDeviceDescription, SsdpDiscoveryError, SSDP_MAX_DESCRIPTION_BYTES};
use super::parse_device_description_response;

pub(super) fn fetch_ssdp_description(
    location: &str,
    timeout: Duration,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    let allowed_location = parse_allowed_http_location(location)?;
    let mut stream = TcpStream::connect_timeout(&allowed_location.addr, timeout)
        .map_err(|error| io_error(&error))?;
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|error| io_error(&error))?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|error| io_error(&error))?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAccept: application/xml,text/xml\r\n\r\n",
        allowed_location.path, allowed_location.addr
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| io_error(&error))?;
    let response = read_http_response(&mut stream)?;
    parse_device_description_response(&response, location)
}

pub(super) fn read_http_response(stream: &mut TcpStream) -> Result<Vec<u8>, SsdpDiscoveryError> {
    let mut response = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        let read = read_http_chunk(stream, &mut chunk)?;
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

fn read_http_chunk(stream: &mut TcpStream, chunk: &mut [u8]) -> Result<usize, SsdpDiscoveryError> {
    stream.read(chunk).map_err(|error| io_error(&error))
}
