use std::io::Read;
use std::process::{ChildStderr, ChildStdout};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread::{self, JoinHandle};
use std::time::Instant;

use crate::{
    error::AppGameWindowsLocalPolicyError, Result, APP_GAME_WINDOWS_LOCAL_POLICY_OUTPUT_MAX_BYTES,
};

const STDOUT_THREAD_NAME: &str = "app-game-local-policy-stdout";
const STDERR_THREAD_NAME: &str = "app-game-local-policy-stderr";

pub(super) struct OutputReaders {
    stdout: OutputReader,
    stderr: OutputReader,
}

struct OutputReader {
    receiver: Receiver<Result<Vec<u8>>>,
    thread: JoinHandle<()>,
}

pub(super) struct ProcessOutput {
    pub(super) stdout: Vec<u8>,
    pub(super) stderr: Vec<u8>,
}

impl OutputReaders {
    pub(super) fn start(stdout: ChildStdout, stderr: ChildStderr) -> Result<Self> {
        let (stdout_sender, stdout_receiver) = mpsc::sync_channel(1);
        let stdout_thread = thread::Builder::new()
            .name(STDOUT_THREAD_NAME.to_string())
            .spawn(move || drop(stdout_sender.send(read_bounded(stdout))))
            .map_err(|error| io_error(&error))?;
        let (stderr_sender, stderr_receiver) = mpsc::sync_channel(1);
        let stderr_thread = thread::Builder::new()
            .name(STDERR_THREAD_NAME.to_string())
            .spawn(move || drop(stderr_sender.send(read_bounded(stderr))))
            .map_err(|error| io_error(&error))?;
        Ok(Self {
            stdout: OutputReader {
                receiver: stdout_receiver,
                thread: stdout_thread,
            },
            stderr: OutputReader {
                receiver: stderr_receiver,
                thread: stderr_thread,
            },
        })
    }

    pub(super) fn finish(self, deadline: Instant) -> Result<ProcessOutput> {
        let stdout = self.stdout.finish(deadline)?;
        let stderr = self.stderr.finish(deadline)?;
        if stdout.len().saturating_add(stderr.len())
            > APP_GAME_WINDOWS_LOCAL_POLICY_OUTPUT_MAX_BYTES
        {
            return Err(AppGameWindowsLocalPolicyError::OutputTooLarge);
        }
        Ok(ProcessOutput { stdout, stderr })
    }
}

impl OutputReader {
    fn finish(self, deadline: Instant) -> Result<Vec<u8>> {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let result = self
            .receiver
            .recv_timeout(remaining)
            .map_err(map_receive_error)?;
        drop(self.thread);
        result
    }
}

fn map_receive_error(error: RecvTimeoutError) -> AppGameWindowsLocalPolicyError {
    match error {
        RecvTimeoutError::Timeout => AppGameWindowsLocalPolicyError::ProcessTimeout,
        RecvTimeoutError::Disconnected => AppGameWindowsLocalPolicyError::OutputInvalidInvariant,
    }
}

fn read_bounded(mut reader: impl Read) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let mut buffer = [0u8; 512];
    while output.len() <= APP_GAME_WINDOWS_LOCAL_POLICY_OUTPUT_MAX_BYTES {
        let read = reader.read(&mut buffer).map_err(|error| io_error(&error))?;
        if read == 0 {
            return Ok(output);
        }
        output.extend_from_slice(&buffer[..read]);
    }
    Err(AppGameWindowsLocalPolicyError::OutputTooLarge)
}

fn io_error(error: &std::io::Error) -> AppGameWindowsLocalPolicyError {
    error.raw_os_error().map_or(
        AppGameWindowsLocalPolicyError::OutputInvalidInvariant,
        |value| {
            u32::try_from(value).map_or(
                AppGameWindowsLocalPolicyError::OutputInvalidInvariant,
                AppGameWindowsLocalPolicyError::WindowsApi,
            )
        },
    )
}
