use std::io::{self, Read};
use std::time::{Duration, Instant};

use serde_json::Value;

use super::provider_transport::{PipeStream, IO_TIMEOUT};

const MAXIMUM_TEST_FRAME_BYTES: usize = 2 * 1024 * 1024;

pub(super) fn read_json(stream: &mut PipeStream) -> io::Result<Option<Value>> {
    read_frame(stream)?
        .map(|body| {
            serde_json::from_slice(&body).map_err(|_error| {
                io::Error::new(io::ErrorKind::InvalidData, "provider frame is not JSON")
            })
        })
        .transpose()
}

pub(super) fn require_json(stream: &mut PipeStream) -> io::Result<Value> {
    read_json(stream)?.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "provider closed before its frame",
        )
    })
}

pub(super) fn read_frame(stream: &mut PipeStream) -> io::Result<Option<Vec<u8>>> {
    let deadline = Instant::now() + IO_TIMEOUT;
    let mut prefix = [0_u8; 4];
    if !read_exact_until(stream, &mut prefix, deadline)? {
        return Ok(None);
    }
    let length = u32::from_be_bytes(prefix) as usize;
    if length == 0 || length > MAXIMUM_TEST_FRAME_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid frame bound",
        ));
    }
    let mut body = vec![0_u8; length];
    if !read_exact_until(stream, &mut body, deadline)? {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "partial provider frame",
        ));
    }
    Ok(Some(body))
}

fn read_exact_until(
    stream: &mut PipeStream,
    buffer: &mut [u8],
    deadline: Instant,
) -> io::Result<bool> {
    let mut offset = 0_usize;
    while offset < buffer.len() {
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "provider read timed out",
            ));
        }
        match stream.read(&mut buffer[offset..]) {
            Ok(0) => return Ok(false),
            Ok(read) => offset += read,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
    Ok(true)
}
