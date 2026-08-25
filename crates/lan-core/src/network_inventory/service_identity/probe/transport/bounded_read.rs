use std::io::Read;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use super::super::super::SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES;
use super::deadline::{retryable, unavailable};

enum ReadStep {
    End,
    Retry,
    Bytes(usize),
}

pub(super) fn read_probe_response_until<R: Read>(
    stream: &mut R,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<Vec<u8>> {
    let mut response = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        if unavailable(deadline, cancellation) {
            return None;
        }
        match read_step(stream, &mut chunk)? {
            ReadStep::End => break,
            ReadStep::Retry => continue,
            ReadStep::Bytes(read) => append_chunk(&mut response, &chunk, read)?,
        }
    }
    (!response.is_empty()).then_some(response)
}

fn read_step<R: Read>(stream: &mut R, chunk: &mut [u8]) -> Option<ReadStep> {
    match stream.read(chunk) {
        Ok(0) => Some(ReadStep::End),
        Ok(read) => Some(ReadStep::Bytes(read)),
        Err(error) if retryable(&error) => Some(ReadStep::Retry),
        Err(_) => None,
    }
}

fn append_chunk(response: &mut Vec<u8>, chunk: &[u8], read: usize) -> Option<()> {
    if response.len().saturating_add(read) > SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES {
        return None;
    }
    response.extend_from_slice(&chunk[..read]);
    Some(())
}
