use std::{
    io::{ErrorKind, Read, Write},
    net::{SocketAddr, TcpStream},
    str,
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

    let mut response = Vec::new();
    let mut buffer = [0; 4096];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(read) => {
                response.extend_from_slice(&buffer[..read]);
                if let Some(body) = complete_http_response_body(&response)? {
                    return Ok(body);
                }
            }
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::TimedOut | ErrorKind::WouldBlock | ErrorKind::Interrupted
                ) =>
            {
                if error.kind() == ErrorKind::Interrupted {
                    continue;
                }
                break;
            }
            Err(_) => return Err(BrowserBridgePollError::Io),
        }
    }
    let response =
        str::from_utf8(&response).map_err(|_| BrowserBridgePollError::InvalidHttpResponse)?;
    http_response_body(response)
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

fn complete_http_response_body(response: &[u8]) -> Result<Option<String>, BrowserBridgePollError> {
    let response_text =
        str::from_utf8(response).map_err(|_| BrowserBridgePollError::InvalidHttpResponse)?;
    if !response_text.starts_with(constants::browser::HTTP_OK_PREFIX) {
        return Err(BrowserBridgePollError::InvalidHttpResponse);
    }
    let Some((headers, body)) = response_text.split_once(constants::browser::HTTP_BODY_SEPARATOR)
    else {
        return Ok(None);
    };
    let Some(content_length) = content_length(headers) else {
        return Ok(None);
    };
    if body.len() < content_length {
        return Ok(None);
    }
    let body = body
        .as_bytes()
        .get(..content_length)
        .ok_or(BrowserBridgePollError::InvalidHttpResponse)?;
    String::from_utf8(body.to_vec())
        .map(Some)
        .map_err(|_| BrowserBridgePollError::InvalidHttpResponse)
}

fn content_length(headers: &str) -> Option<usize> {
    headers.lines().find_map(|line| {
        let normalized = line.trim().to_ascii_lowercase();
        normalized
            .strip_prefix(constants::browser::HTTP_HEADER_CONTENT_LENGTH)
            .and_then(|value| value.trim().parse::<usize>().ok())
    })
}
