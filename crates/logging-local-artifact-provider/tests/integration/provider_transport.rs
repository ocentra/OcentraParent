use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream};
use interprocess::ConnectWaitMode;

pub(super) const IO_TIMEOUT: Duration = Duration::from_secs(10);

pub(super) type PipeStream = DuplexPipeStream<pipe_mode::Bytes>;

pub(super) fn test_paths() -> io::Result<(PathBuf, String)> {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "system clock precedes Unix epoch",
            )
        })?
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "ocentra-local-artifact-provider-{}-{unique:x}",
        std::process::id()
    ));
    let pipe_name = format!(
        r"\\.\pipe\ocentra-local-artifact-provider-test-{}-{unique:x}",
        std::process::id()
    );
    Ok((root, pipe_name))
}

pub(super) fn start_provider(pipe_name: &str, root: &Path) -> io::Result<Child> {
    let parent_pid = std::process::id().to_string();
    let root = root.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "temporary root is not Unicode")
    })?;
    Command::new(env!(
        "CARGO_BIN_EXE_ocentra-logging-local-artifact-provider"
    ))
    .args([
        "--pipe-name",
        pipe_name,
        "--root",
        root,
        "--parent-pid",
        &parent_pid,
    ])
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
}

pub(super) fn connect(pipe_name: &str) -> io::Result<PipeStream> {
    let deadline = Instant::now() + IO_TIMEOUT;
    loop {
        match PipeStream::connect_by_path_with_wait_mode(
            pipe_name,
            ConnectWaitMode::Timeout(Duration::from_millis(100)),
        ) {
            Ok(stream) => return Ok(stream),
            Err(error) if error.kind() == io::ErrorKind::NotFound && Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(error) => return Err(error),
        }
    }
}
