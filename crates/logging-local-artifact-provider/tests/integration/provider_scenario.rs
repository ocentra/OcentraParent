use std::io::{self, Write};
use std::time::{Duration, Instant};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::{json, Value};

use super::provider_assertions::{
    assert_ready, assert_success, cleanup, flush_until, invalid_request, success_result,
    wait_for_success,
};
use super::provider_framing::{read_frame, require_json};
use super::provider_transport::{connect, start_provider, test_paths, PipeStream, IO_TIMEOUT};

#[test]
fn provider_enforces_frames_leases_replay_snapshots_transactions_and_shutdown() -> io::Result<()> {
    let (root, pipe_name) = test_paths()?;
    let mut provider = start_provider(&pipe_name, &root)?;
    reject_unknown_fields(&pipe_name)?;
    let mut stream = connect(&pipe_name)?;
    assert_ready(&require_json(&mut stream)?);
    let lease_id = begin_lease(&mut stream)?;
    ensure_directories(&mut stream, &lease_id)?;
    exercise_replay_and_removals(&mut stream, &lease_id)?;
    assert_first_snapshot(&mut stream, &lease_id)?;
    apply_transaction(&mut stream, &lease_id)?;
    assert_transaction_results(&mut stream, &lease_id)?;
    end_lease_and_shutdown(&mut stream, &lease_id)?;
    drop(stream);
    wait_for_success(&mut provider)?;
    cleanup(&root)
}

fn reject_unknown_fields(pipe_name: &str) -> io::Result<()> {
    let mut stream = connect(pipe_name)?;
    assert_ready(&require_json(&mut stream)?);
    write_json(&mut stream, &invalid_request())?;
    assert_eq!(read_frame(&mut stream)?, None);
    Ok(())
}

fn begin_lease(stream: &mut PipeStream) -> io::Result<String> {
    let response = exchange(stream, &request(2, None, json!({ "kind": "beginLease" })))?;
    let lease_id = success_result(&response)["lease_id"]
        .as_str()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "beginLease result lacks a lease id",
            )
        })?
        .to_owned();
    assert!(lease_id.len() >= 32);
    assert!(lease_id
        .bytes()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
    Ok(lease_id)
}

fn ensure_directories(stream: &mut PipeStream, lease_id: &str) -> io::Result<()> {
    assert_success(
        &exchange(
            stream,
            &request(
                3,
                Some(lease_id),
                json!({ "kind": "ensureDirectory", "relative_path": "app-logs/sessions" }),
            ),
        )?,
        &json!({ "ready": true }),
    );
    assert_success(
        &exchange(
            stream,
            &request(
                4,
                Some(lease_id),
                json!({ "kind": "ensureDirectory", "relative_path": "test-logs/scope" }),
            ),
        )?,
        &json!({ "ready": true }),
    );
    Ok(())
}

fn exercise_replay_and_removals(stream: &mut PipeStream, lease_id: &str) -> io::Result<()> {
    let first_payload = STANDARD.encode(b"{\"sequence\":1}\n");
    let first_replace = request(
        5,
        Some(lease_id),
        json!({
            "kind": "replace",
            "relative_path": "app-logs/sessions/current.ndjson",
            "payload_base64": first_payload,
        }),
    );
    assert_success(
        &exchange(stream, &first_replace)?,
        &json!({ "written": 15, "replayed": false }),
    );
    assert_success(
        &exchange(stream, &first_replace)?,
        &json!({ "written": 15, "replayed": true }),
    );
    assert_success(
        &exchange(
            stream,
            &request(
                6,
                Some(lease_id),
                json!({
                    "kind": "replace",
                    "relative_path": "test-logs/scope/run.ndjson",
                    "payload_base64": STANDARD.encode(b"{\"temporary\":true}\n"),
                }),
            ),
        )?,
        &json!({ "written": 19, "replayed": false }),
    );
    assert_success(
        &exchange(
            stream,
            &request(
                60,
                Some(lease_id),
                json!({
                    "kind": "replace",
                    "relative_path": "app-logs/sessions/remove-me.ndjson",
                    "payload_base64": STANDARD.encode(b"{\"remove\":true}\n"),
                }),
            ),
        )?,
        &json!({ "written": 16, "replayed": false }),
    );
    Ok(())
}

fn assert_first_snapshot(stream: &mut PipeStream, lease_id: &str) -> io::Result<()> {
    let snapshot = exchange(
        stream,
        &request(
            7,
            Some(lease_id),
            json!({
                "kind": "readSnapshot",
                "relative_path": "app-logs/sessions/current.ndjson",
                "maximum_bytes": 1024,
            }),
        ),
    )?;
    let snapshot = success_result(&snapshot);
    assert_eq!(
        snapshot["content_base64"],
        STANDARD.encode(b"{\"sequence\":1}\n")
    );
    assert_eq!(snapshot["stat"]["size"], 15);
    assert!(snapshot["stat"]["identity"]["device"].is_string());
    assert!(snapshot["stat"]["identity"]["inode"].is_string());
    Ok(())
}

fn apply_transaction(stream: &mut PipeStream, lease_id: &str) -> io::Result<()> {
    let second_payload = STANDARD.encode(b"{\"sequence\":2}\n");
    assert_success(
        &exchange(
            stream,
            &request(
                8,
                Some(lease_id),
                json!({
                    "kind": "applyTransaction",
                    "mutations": [
                        { "kind": "removeTree", "relative_path": "test-logs/scope" },
                        {
                            "kind": "replace",
                            "relative_path": "app-logs/sessions/current.ndjson",
                            "payload_base64": second_payload,
                        },
                        {
                            "kind": "remove",
                            "relative_path": "app-logs/sessions/remove-me.ndjson",
                        },
                    ],
                }),
            ),
        )?,
        &json!({ "applied": 3, "replayed": false }),
    );
    Ok(())
}

fn assert_transaction_results(stream: &mut PipeStream, lease_id: &str) -> io::Result<()> {
    let removed = exchange(
        stream,
        &request(
            9,
            Some(lease_id),
            json!({ "kind": "stat", "relative_path": "test-logs/scope" }),
        ),
    )?;
    assert_eq!(success_result(&removed), &Value::Null);
    let removed_file = exchange(
        stream,
        &request(
            61,
            Some(lease_id),
            json!({
                "kind": "stat",
                "relative_path": "app-logs/sessions/remove-me.ndjson",
            }),
        ),
    )?;
    assert_eq!(success_result(&removed_file), &Value::Null);
    let replaced = exchange(
        stream,
        &request(
            10,
            Some(lease_id),
            json!({
                "kind": "readSnapshot",
                "relative_path": "app-logs/sessions/current.ndjson",
                "maximum_bytes": 1024,
            }),
        ),
    )?;
    assert_eq!(
        success_result(&replaced)["content_base64"],
        STANDARD.encode(b"{\"sequence\":2}\n")
    );
    Ok(())
}

fn end_lease_and_shutdown(stream: &mut PipeStream, lease_id: &str) -> io::Result<()> {
    assert_success(
        &exchange(
            stream,
            &request(
                11,
                Some(lease_id),
                json!({ "kind": "endLease", "lease_id": lease_id }),
            ),
        )?,
        &json!({ "released": true }),
    );
    assert_success(
        &exchange(stream, &request(12, None, json!({ "kind": "shutdown" })))?,
        &json!({ "shutdown": true }),
    );
    Ok(())
}

fn request(sequence: u128, lease_id: Option<&str>, operation: Value) -> Value {
    let mut request = json!({
        "protocol_version": 1,
        "request_id": format!("{sequence:048x}"),
        "nonce": format!("{:048x}", sequence + 10_000),
        "lease_id": lease_id,
        "operation": Value::Null,
    });
    request["operation"] = operation;
    request
}

fn exchange(stream: &mut PipeStream, request: &Value) -> io::Result<Value> {
    write_json(stream, request)?;
    let response = require_json(stream)?;
    assert_eq!(response["request_id"], request["request_id"]);
    assert_eq!(response["nonce"], request["nonce"]);
    assert_eq!(response["operation"], request["operation"]["kind"]);
    Ok(response)
}

fn write_json(stream: &mut PipeStream, value: &Value) -> io::Result<()> {
    let body = serde_json::to_vec(value)
        .map_err(|_error| io::Error::new(io::ErrorKind::InvalidInput, "frame is not JSON"))?;
    let length = u32::try_from(body.len())
        .map_err(|_error| io::Error::new(io::ErrorKind::InvalidInput, "frame is too large"))?;
    let mut frame = Vec::with_capacity(body.len() + 4);
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(&body);
    write_all_until(stream, &frame, Instant::now() + IO_TIMEOUT)
}

fn write_all_until(stream: &mut PipeStream, buffer: &[u8], deadline: Instant) -> io::Result<()> {
    let mut offset = 0_usize;
    while offset < buffer.len() {
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "provider write timed out",
            ));
        }
        match stream.write(&buffer[offset..]) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "provider pipe closed",
                ))
            }
            Ok(written) => offset += written,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
    flush_until(stream, deadline)
}
