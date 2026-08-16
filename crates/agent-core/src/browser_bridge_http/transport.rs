use std::{
    io::{ErrorKind, Read, Write},
    net::{SocketAddr, TcpStream},
    str,
    time::Duration,
};

use ocentra_parent_agent_protocol::constants;

use crate::browser_bridge_poll::BrowserBridgePollError;

use super::helpers;

pub(crate) fn read_devtools_body(
    endpoint: &SocketAddr,
    request_line: &str,
) -> Result<String, BrowserBridgePollError> {
    let timeout = Duration::from_millis(constants::browser::DEVTOOLS_TIMEOUT_MS);
    let mut stream = TcpStream::connect_timeout(endpoint, timeout)
        .map_err(|error| helpers::map_io_error(&error))?;
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|error| helpers::map_io_error(&error))?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|error| helpers::map_io_error(&error))?;
    stream
        .write_all(helpers::devtools_request(request_line).as_bytes())
        .map_err(|error| helpers::map_io_error(&error))?;

    read_devtools_body_from_stream(&mut stream)
}

fn read_devtools_body_from_stream(
    stream: &mut TcpStream,
) -> Result<String, BrowserBridgePollError> {
    let mut response = Vec::new();
    let mut buffer = [0; 4096];
    loop {
        let Some(read) = read_devtools_chunk(stream, &mut buffer)? else {
            break;
        };
        if let Some(body) = append_response_chunk(&mut response, &buffer[..read])? {
            return Ok(body);
        }
    }
    let response =
        str::from_utf8(&response).map_err(|_error| BrowserBridgePollError::InvalidHttpResponse)?;
    helpers::http_response_body(response)
}

fn read_devtools_chunk(
    stream: &mut TcpStream,
    buffer: &mut [u8; 4096],
) -> Result<Option<usize>, BrowserBridgePollError> {
    match stream.read(buffer) {
        Ok(0) => Ok(None),
        Ok(read) => Ok(Some(read)),
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::TimedOut | ErrorKind::WouldBlock | ErrorKind::Interrupted
            ) =>
        {
            if error.kind() == ErrorKind::Interrupted {
                return read_devtools_chunk(stream, buffer);
            }
            Err(BrowserBridgePollError::Timeout)
        }
        Err(_) => Err(BrowserBridgePollError::Io),
    }
}

fn append_response_chunk(
    response: &mut Vec<u8>,
    chunk: &[u8],
) -> Result<Option<String>, BrowserBridgePollError> {
    response.extend_from_slice(chunk);
    if response.len() > constants::browser::DEVTOOLS_MAX_RESPONSE_BYTES {
        return Err(BrowserBridgePollError::ResponseTooLarge);
    }
    helpers::complete_http_response_body(response)
}
