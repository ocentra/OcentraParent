use std::time::Instant;

use tokio::{io::AsyncReadExt, process::ChildStdout, time::timeout};

const MAX_DOCKER_PROBE_OUTPUT_BYTES: usize = 4096;

pub(super) struct BoundedOutput {
    pub(super) bytes: Vec<u8>,
    pub(super) overflow: bool,
    pub(super) read_error: bool,
    pub(super) timed_out: bool,
}

pub(super) async fn read_bounded_until(
    mut stdout: ChildStdout,
    deadline: Instant,
) -> BoundedOutput {
    // A cancelled async read drops the pipe here; no blocking reader task
    // survives a timeout or an inherited descendant handle.
    let mut bytes = Vec::with_capacity(MAX_DOCKER_PROBE_OUTPUT_BYTES);
    let mut overflow = false;
    let mut read_error = false;
    let mut timed_out = false;
    let mut chunk = [0_u8; 512];
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            timed_out = true;
            break;
        }
        let read = match timeout(remaining, stdout.read(&mut chunk)).await {
            Ok(Ok(0)) => break,
            Ok(Ok(read)) => read,
            Ok(Err(_)) => {
                read_error = true;
                break;
            }
            Err(_) => {
                timed_out = true;
                break;
            }
        };
        let remaining = MAX_DOCKER_PROBE_OUTPUT_BYTES.saturating_sub(bytes.len());
        bytes.extend_from_slice(&chunk[..read.min(remaining)]);
        overflow = read > remaining;
        if overflow {
            break;
        }
    }
    BoundedOutput {
        bytes,
        overflow,
        read_error,
        timed_out,
    }
}
