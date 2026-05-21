use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use ocentra_parent_agent_protocol::constants;

use crate::browser_bridge_poll::BrowserBridgePollError;

pub fn read_devtools_body(
    endpoint: &SocketAddr,
    request_line: &str,
) -> Result<String, BrowserBridgePollError> {
    let timeout = Duration::from_millis(constants::browser::DEVTOOLS_TIMEOUT_MS);
    let mut stream =
        TcpStream::connect_timeout(endpoint, timeout).map_err(|_| BrowserBridgePollError::Io)?;
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|_| BrowserBridgePollError::Io)?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|_| BrowserBridgePollError::Io)?;
    stream
        .write_all(devtools_request(request_line).as_bytes())
        .map_err(|_| BrowserBridgePollError::Io)?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|_| BrowserBridgePollError::Io)?;
    http_response_body(&response)
}

fn devtools_request(request_line: &str) -> String {
    [
        request_line,
        constants::browser::HTTP_HEADER_HOST_LOOPBACK,
        constants::browser::HTTP_CONNECTION_CLOSE,
    ]
    .join(constants::browser::HTTP_LINE_SEPARATOR)
        + constants::browser::HTTP_BODY_SEPARATOR
}

fn http_response_body(response: &str) -> Result<String, BrowserBridgePollError> {
    if !response.starts_with(constants::browser::HTTP_OK_PREFIX) {
        return Err(BrowserBridgePollError::InvalidHttpResponse);
    }
    response
        .split_once(constants::browser::HTTP_BODY_SEPARATOR)
        .map(|(_, body)| body.to_string())
        .ok_or(BrowserBridgePollError::InvalidHttpResponse)
}
