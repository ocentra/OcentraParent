use std::{io::ErrorKind, str};

use ocentra_parent_agent_protocol::constants;

use crate::browser_bridge_poll::BrowserBridgePollError;

pub(crate) fn devtools_request(request_line: &str) -> String {
    [
        request_line,
        constants::browser::HTTP_HEADER_HOST_LOOPBACK,
        constants::browser::HTTP_CONNECTION_CLOSE,
    ]
    .join(constants::browser::HTTP_LINE_SEPARATOR)
        + constants::browser::HTTP_BODY_SEPARATOR
}

pub(crate) fn http_response_body(response: &str) -> Result<String, BrowserBridgePollError> {
    if !response.starts_with(constants::browser::HTTP_OK_PREFIX) {
        return Err(BrowserBridgePollError::InvalidHttpResponse);
    }
    let Some((_, body)) = response.split_once(constants::browser::HTTP_BODY_SEPARATOR) else {
        return Err(BrowserBridgePollError::InvalidHttpResponse);
    };
    if body.len() > constants::browser::DEVTOOLS_MAX_RESPONSE_BYTES {
        return Err(BrowserBridgePollError::ResponseTooLarge);
    }
    Ok(body.to_string())
}

pub(crate) fn complete_http_response_body(
    response: &[u8],
) -> Result<Option<String>, BrowserBridgePollError> {
    let response_text =
        str::from_utf8(response).map_err(|_error| BrowserBridgePollError::InvalidHttpResponse)?;
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
    if content_length > constants::browser::DEVTOOLS_MAX_RESPONSE_BYTES {
        return Err(BrowserBridgePollError::ResponseTooLarge);
    }
    if body.len() < content_length {
        return Ok(None);
    }
    let body = body
        .as_bytes()
        .get(..content_length)
        .ok_or(BrowserBridgePollError::InvalidHttpResponse)?;
    String::from_utf8(body.to_vec())
        .map(Some)
        .map_err(|_error| BrowserBridgePollError::InvalidHttpResponse)
}

fn content_length(headers: &str) -> Option<usize> {
    headers.lines().find_map(|line| {
        let normalized = line.trim().to_ascii_lowercase();
        normalized
            .strip_prefix(constants::browser::HTTP_HEADER_CONTENT_LENGTH)
            .and_then(|value| value.trim().parse::<usize>().ok())
    })
}

pub(crate) fn map_io_error(error: &std::io::Error) -> BrowserBridgePollError {
    if matches!(error.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock) {
        return BrowserBridgePollError::Timeout;
    }
    BrowserBridgePollError::Io
}
