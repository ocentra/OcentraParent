use std::io::{Read, Write};
use std::net::SocketAddr;

use super::super::SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES;

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
