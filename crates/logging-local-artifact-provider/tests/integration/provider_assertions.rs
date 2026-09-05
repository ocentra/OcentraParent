use std::fs;
use std::io;
use std::path::Path;
use std::process::Child;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{json, Value};

use super::provider_transport::{PipeStream, IO_TIMEOUT};

pub(super) fn assert_ready(value: &Value) {
    assert_eq!(value["protocol_version"], 1);
    assert_eq!(value["binary_sha256"].as_str().map(str::len), Some(64));
    assert!(value["provider_instance_id"].is_string());
    assert!(value["root_identity"]["device"].is_string());
    assert!(value["root_identity"]["inode"].is_string());
}

pub(super) fn assert_success(response: &Value, expected: &Value) {
    assert_eq!(success_result(response), expected);
}

pub(super) fn success_result(response: &Value) -> &Value {
    assert_eq!(response["protocol_version"], 1);
    assert_eq!(response["ok"], true);
    assert!(response.get("error").is_none());
    &response["result"]
}

pub(super) fn flush_until(stream: &PipeStream, deadline: Instant) -> io::Result<()> {
    loop {
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "provider flush timed out",
            ));
        }
        match stream.flush() {
            Ok(()) => return Ok(()),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(5));
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
}

pub(super) fn wait_for_success(child: &mut Child) -> io::Result<()> {
    let deadline = Instant::now() + IO_TIMEOUT;
    loop {
        if let Some(status) = child.try_wait()? {
            if status.success() {
                return Ok(());
            }
            return Err(io::Error::other(
                "provider exited unsuccessfully after shutdown",
            ));
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "provider did not exit after shutdown",
            ));
        }
        thread::sleep(Duration::from_millis(10));
    }
}

pub(super) fn cleanup(root: &Path) -> io::Result<()> {
    fs::remove_dir_all(root)
}

pub(super) fn invalid_request() -> Value {
    json!({
        "protocol_version": 1,
        "request_id": format!("{:048x}", 1),
        "nonce": format!("{:048x}", 10_001),
        "lease_id": Value::Null,
        "operation": { "kind": "beginLease" },
        "unexpected": true,
    })
}
