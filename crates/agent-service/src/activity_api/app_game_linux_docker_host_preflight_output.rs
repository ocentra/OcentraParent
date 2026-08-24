use std::io::Read;

const MAX_DOCKER_PROBE_OUTPUT_BYTES: usize = 4096;

pub(super) struct BoundedOutput {
    pub(super) bytes: Vec<u8>,
    pub(super) overflow: bool,
    pub(super) read_error: bool,
}

pub(super) fn read_bounded(mut stdout: impl Read) -> BoundedOutput {
    let mut bytes = Vec::with_capacity(MAX_DOCKER_PROBE_OUTPUT_BYTES);
    let mut overflow = false;
    let mut read_error = false;
    let mut chunk = [0_u8; 512];
    loop {
        let read = match stdout.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => read,
            Err(_) => {
                read_error = true;
                break;
            }
        };
        let remaining = MAX_DOCKER_PROBE_OUTPUT_BYTES.saturating_sub(bytes.len());
        bytes.extend_from_slice(&chunk[..read.min(remaining)]);
        overflow |= read > remaining;
    }
    BoundedOutput {
        bytes,
        overflow,
        read_error,
    }
}
