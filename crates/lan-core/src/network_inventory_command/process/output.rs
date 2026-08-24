use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use tokio::{io::AsyncRead, io::AsyncReadExt, time::timeout};

const COMMAND_OUTPUT_CAPTURE_LIMIT: usize = 1024 * 1024;
const PIPE_READ_POLL_SLICE: Duration = Duration::from_millis(25);

pub(super) struct BoundedOutput {
    pub(super) bytes: Vec<u8>,
    overflow: bool,
    read_error: bool,
    cancelled: bool,
}

impl BoundedOutput {
    pub(super) fn complete(&self) -> bool {
        !self.overflow && !self.read_error && !self.cancelled
    }
}

pub(super) async fn read_bounded_until(
    mut reader: impl AsyncRead + Unpin,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
    terminate: &AtomicBool,
) -> BoundedOutput {
    let mut result = BoundedOutput {
        bytes: Vec::new(),
        overflow: false,
        read_error: false,
        cancelled: false,
    };
    let mut chunk = [0_u8; 8192];
    loop {
        if externally_cancelled(cancellation) || Instant::now() >= deadline {
            result.cancelled = true;
            terminate.store(true, Ordering::Release);
            return result;
        }
        let poll = PIPE_READ_POLL_SLICE.min(deadline.saturating_duration_since(Instant::now()));
        let read = match timeout(poll, reader.read(&mut chunk)).await {
            Ok(Ok(0)) => return result,
            Ok(Ok(read)) => read,
            Ok(Err(_error)) => {
                result.read_error = true;
                terminate.store(true, Ordering::Release);
                return result;
            }
            Err(_) => continue,
        };
        let remaining = COMMAND_OUTPUT_CAPTURE_LIMIT.saturating_sub(result.bytes.len());
        if read > remaining {
            result.bytes.extend_from_slice(&chunk[..remaining]);
            result.overflow = true;
            terminate.store(true, Ordering::Release);
            return result;
        }
        result.bytes.extend_from_slice(&chunk[..read]);
    }
}

fn externally_cancelled(cancellation: Option<&AtomicBool>) -> bool {
    cancellation.is_some_and(|value| value.load(Ordering::Acquire))
}
