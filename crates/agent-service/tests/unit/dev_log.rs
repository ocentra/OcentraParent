use std::{
    env, fs,
    path::PathBuf,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_logging_core::{
    dev_log::DevLogger,
    field::LogFields as CoreLogFields,
    path::{LOG_ROOT_ENV, LOG_SCOPE_ENV},
    source::LogSource,
};

use ocentra_parent_agent_service::dev_log::{
    write_agent_debug, write_agent_error, write_agent_info, write_agent_warn,
};

fn dev_log_test_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn require_ok<T, E: std::fmt::Debug>(result: Result<T, E>, message: &str) -> T {
    result.expect(message)
}

fn require_some<T>(value: Option<T>, message: &str) -> T {
    value.expect(message)
}

fn temp_dev_log_dir() -> PathBuf {
    let nanos = require_ok(
        SystemTime::now().duration_since(UNIX_EPOCH),
        "system time available",
    )
    .as_nanos();
    let mut path = env::temp_dir();
    path.push(format!("ocentra-parent-dev-log-test-{nanos}"));
    path
}

#[test]
fn dev_log_path_uses_configured_directory_and_agent_file_prefix() {
    let _guard = require_ok(dev_log_test_lock().lock(), "dev log test mutex");
    let temp_dir = temp_dev_log_dir();
    env::set_var(constants::env_var::DEV_LOG_DIR, &temp_dir);

    let path = require_ok(
        require_ok(
            DevLogger::from_env(LogSource::AgentService),
            "logger resolves",
        )
        .info("compat path", CoreLogFields::new()),
        "compat path writes",
    );

    env::remove_var(constants::env_var::DEV_LOG_DIR);

    assert_eq!(path.parent(), Some(temp_dir.as_path()));
    let file_name = require_some(path.file_name().and_then(|name| name.to_str()), "file name");
    assert!(file_name.starts_with("agent-service-"));
    assert!(file_name.ends_with(".ndjson"));
}

#[test]
fn write_agent_info_writes_dev_log_ndjson_line() {
    let _guard = require_ok(dev_log_test_lock().lock(), "dev log test mutex");
    let temp_dir = temp_dev_log_dir();
    env::set_var(constants::env_var::DEV_LOG_DIR, &temp_dir);
    let mut fields = LogFields::new();
    fields.insert(
        "context".to_owned(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String("startup".to_owned()),
    );

    require_ok(
        write_agent_info(constants::dev_log_message::AGENT_SERVICE_STARTED, fields),
        "agent info writes",
    );

    env::remove_var(constants::env_var::DEV_LOG_DIR);

    let entries =
        require_ok(fs::read_dir(&temp_dir), "dev log dir exists").collect::<Result<Vec<_>, _>>();
    let entries = require_ok(entries, "dev log files readable")
        .into_iter()
        .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) == Some("ndjson"))
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), 1);

    let payload = require_ok(fs::read_to_string(entries[0].path()), "ndjson readable");
    let line = require_some(payload.lines().next(), "one ndjson line");
    let value: serde_json::Value = require_ok(serde_json::from_str(line), "ndjson parses");

    assert_eq!(
        value["message"].as_str(),
        Some(constants::dev_log_message::AGENT_SERVICE_STARTED)
    );
    assert_eq!(value["source"].as_str(), Some("agent-service"));
    assert_eq!(value["fields"]["context"].as_str(), Some("startup"));
}

#[test]
fn write_agent_all_levels_emit_ndjson_lines() {
    let _guard = require_ok(dev_log_test_lock().lock(), "dev log test mutex");
    let existing_log_root = env::var_os(LOG_ROOT_ENV);
    let existing_log_scope = env::var_os(LOG_SCOPE_ENV);
    let temp_dir = temp_dev_log_dir();
    env::set_var(LOG_ROOT_ENV, &temp_dir);
    env::set_var(LOG_SCOPE_ENV, "parent-agent");

    let mut info_fields = LogFields::new();
    info_fields.insert(
        "context".to_owned(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String("hello-world".to_owned()),
    );
    require_ok(
        write_agent_info("agent info hello", info_fields),
        "agent info writes",
    );

    let mut warn_fields = LogFields::new();
    warn_fields.insert(
        "context".to_owned(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String("hello-world".to_owned()),
    );
    require_ok(
        write_agent_warn("agent warn hello", warn_fields),
        "agent warn writes",
    );

    let mut error_fields = LogFields::new();
    error_fields.insert(
        "context".to_owned(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String("hello-world".to_owned()),
    );
    require_ok(
        write_agent_error("agent error hello", error_fields),
        "agent error writes",
    );

    let mut debug_fields = LogFields::new();
    debug_fields.insert(
        "context".to_owned(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String("hello-world".to_owned()),
    );
    require_ok(
        write_agent_debug("agent debug hello", debug_fields),
        "agent debug writes",
    );

    match existing_log_scope {
        Some(value) => env::set_var(LOG_SCOPE_ENV, value),
        None => env::remove_var(LOG_SCOPE_ENV),
    }
    match existing_log_root {
        Some(value) => env::set_var(LOG_ROOT_ENV, value),
        None => env::remove_var(LOG_ROOT_ENV),
    }

    let entries = require_ok(
        fs::read_dir(temp_dir.join("parent-agent").join("ndjson").join("dev-log")),
        "dev log dir exists",
    )
    .collect::<Result<Vec<_>, _>>();
    let entries = require_ok(entries, "dev log files readable")
        .into_iter()
        .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) == Some("ndjson"))
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), 1);

    let payload = require_ok(fs::read_to_string(entries[0].path()), "ndjson readable");
    let rows = payload
        .lines()
        .map(|line| {
            require_ok(
                serde_json::from_str::<serde_json::Value>(line),
                "ndjson parses",
            )
        })
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
