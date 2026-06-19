use std::{
    env, fs,
    path::PathBuf,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{constants, LogFields};
use ocentra_parent_logging_core::{
    compat_dev_log::resolve_compat_dev_log_path,
    path::{LOG_ROOT_ENV, LOG_SCOPE_ENV},
    source::LogSource,
};

use super::{write_agent_debug, write_agent_error, write_agent_info, write_agent_warn};

fn dev_log_test_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn temp_dev_log_dir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time available")
        .as_nanos();
    let mut path = env::temp_dir();
    path.push(format!("ocentra-parent-dev-log-test-{nanos}"));
    path
}

#[test]
fn dev_log_path_uses_configured_directory_and_agent_file_prefix() {
    let _guard = dev_log_test_lock().lock().expect("dev log test mutex");
    let temp_dir = temp_dev_log_dir();
    env::set_var(constants::env_var::DEV_LOG_DIR, &temp_dir);

    let path = resolve_compat_dev_log_path(&LogSource::AgentService, "2026-06-15T01:00:00Z")
        .expect("path resolves");

    env::remove_var(constants::env_var::DEV_LOG_DIR);

    expect_eq_path(path, temp_dir.join("agent-service-2026-06-15.ndjson"));
}

#[test]
fn write_agent_info_writes_dev_log_ndjson_line() {
    let _guard = dev_log_test_lock().lock().expect("dev log test mutex");
    let temp_dir = temp_dev_log_dir();
    env::set_var(constants::env_var::DEV_LOG_DIR, &temp_dir);
    let mut fields = LogFields::new();
    fields.insert(
        "context".to_owned(),
        ocentra_parent_agent_protocol::LogFieldValue::String("startup".to_owned()),
    );

    write_agent_info(constants::dev_log_message::AGENT_SERVICE_STARTED, fields)
        .expect("agent info writes");

    env::remove_var(constants::env_var::DEV_LOG_DIR);

    let entries = fs::read_dir(&temp_dir)
        .expect("dev log dir exists")
        .collect::<Result<Vec<_>, _>>()
        .expect("dev log files readable");
    assert_eq!(entries.len(), 1);

    let payload = fs::read_to_string(entries[0].path()).expect("ndjson readable");
    let line = payload.lines().next().expect("one ndjson line");
    let value: serde_json::Value = serde_json::from_str(line).expect("ndjson parses");

    assert_eq!(
        value["message"].as_str(),
        Some(constants::dev_log_message::AGENT_SERVICE_STARTED)
    );
    assert_eq!(value["source"].as_str(), Some("agent-service"));
    assert_eq!(value["fields"]["context"].as_str(), Some("startup"));
}

#[test]
fn write_agent_all_levels_emit_ndjson_lines() {
    let _guard = dev_log_test_lock().lock().expect("dev log test mutex");
    let existing_log_root = env::var_os(LOG_ROOT_ENV);
    let existing_log_scope = env::var_os(LOG_SCOPE_ENV);
    let temp_dir = existing_log_root
        .clone()
        .map(PathBuf::from)
        .unwrap_or_else(temp_dev_log_dir);
    env::set_var(LOG_ROOT_ENV, &temp_dir);
    env::set_var(LOG_SCOPE_ENV, "parent-agent");

    let mut info_fields = LogFields::new();
    info_fields.insert(
        "context".to_owned(),
        ocentra_parent_agent_protocol::LogFieldValue::String("hello-world".to_owned()),
    );
    write_agent_info("agent info hello", info_fields).expect("agent info writes");

    let mut warn_fields = LogFields::new();
    warn_fields.insert(
        "context".to_owned(),
        ocentra_parent_agent_protocol::LogFieldValue::String("hello-world".to_owned()),
    );
    write_agent_warn("agent warn hello", warn_fields).expect("agent warn writes");

    let mut error_fields = LogFields::new();
    error_fields.insert(
        "context".to_owned(),
        ocentra_parent_agent_protocol::LogFieldValue::String("hello-world".to_owned()),
    );
    write_agent_error("agent error hello", error_fields).expect("agent error writes");

    let mut debug_fields = LogFields::new();
    debug_fields.insert(
        "context".to_owned(),
        ocentra_parent_agent_protocol::LogFieldValue::String("hello-world".to_owned()),
    );
    write_agent_debug("agent debug hello", debug_fields).expect("agent debug writes");

    match existing_log_scope {
        Some(value) => env::set_var(LOG_SCOPE_ENV, value),
        None => env::remove_var(LOG_SCOPE_ENV),
    }
    match existing_log_root {
        Some(value) => env::set_var(LOG_ROOT_ENV, value),
        None => env::remove_var(LOG_ROOT_ENV),
    }

    let entries = fs::read_dir(temp_dir.join("parent-agent").join("ndjson").join("dev-log"))
        .expect("dev log dir exists")
        .collect::<Result<Vec<_>, _>>()
        .expect("dev log files readable");
    assert_eq!(entries.len(), 1);

    let payload = fs::read_to_string(entries[0].path()).expect("ndjson readable");
    let rows = payload
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("ndjson parses"))
        .collect::<Vec<_>>();

    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0]["level"].as_str(), Some("info"));
    assert_eq!(rows[1]["level"].as_str(), Some("warn"));
    assert_eq!(rows[2]["level"].as_str(), Some("error"));
    assert_eq!(rows[3]["level"].as_str(), Some("debug"));
    for row in &rows {
        assert_eq!(row["source"].as_str(), Some("agent-service"));
        assert_eq!(row["fields"]["context"].as_str(), Some("hello-world"));
    }
}

fn expect_eq_path(actual: PathBuf, expected: PathBuf) {
    assert_eq!(actual, expected);
}
