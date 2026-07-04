use std::{
    env,
    error::Error,
    fs,
    sync::{Mutex, OnceLock},
};

use ocentra_parent_logging_core::{
    dev_log::write_agent_info,
    path::{DEV_LOG_DIR_ENV, LANE_ID_ENV, LEDGER_LANE_ENV, LOG_ROOT_ENV, LOG_RUN_ID_ENV},
    source::LogSource,
};

#[macro_use]
#[path = "../support/mod.rs"]
mod support;

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn dev_logger_writes_compat_file_when_dev_log_dir_is_set() {
    let result = dev_logger_writes_compat_file_when_dev_log_dir_is_set_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn dev_logger_prefers_shared_runtime_env_names() {
    let result = dev_logger_prefers_shared_runtime_env_names_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

fn dev_logger_writes_compat_file_when_dev_log_dir_is_set_impl() -> Result<(), Box<dyn Error>> {
    let _guard = env_lock()
        .lock()
        .map_err(|error| std::io::Error::other(format!("failed to lock env mutex: {error:?}")))?;

    let temp = temp_dir!();
    env::remove_var(LOG_ROOT_ENV);
    env::set_var(DEV_LOG_DIR_ENV, &temp);

    let path = write_agent_info(
        LogSource::AgentService,
        "Agent service dev runtime started.",
        Default::default(),
    )?;

    env::remove_var(DEV_LOG_DIR_ENV);

    let payload = fs::read_to_string(&path)?;
    let line = payload.lines().next().unwrap_or_default();
    let value: serde_json::Value = serde_json::from_str(line)?;
    let timestamp = value
        .get("timestamp")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "missing timestamp field")
        })?;
    let expected_name = format!("agent-service-{}.ndjson", &timestamp[..10]);
    assert_eq!(path.parent(), Some(temp.as_path()));
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some(expected_name.as_str())
    );
    assert_eq!(value["message"], "Agent service dev runtime started.");
    Ok(())
}

fn dev_logger_prefers_shared_runtime_env_names_impl() -> Result<(), Box<dyn Error>> {
    let _guard = env_lock()
        .lock()
        .map_err(|error| std::io::Error::other(format!("failed to lock env mutex: {error:?}")))?;

    let temp = temp_dir!();
    env::set_var(DEV_LOG_DIR_ENV, &temp);
    env::set_var(LOG_RUN_ID_ENV, "shared-run-id");
    env::set_var(LEDGER_LANE_ENV, "ledger-lane");
    env::set_var(LANE_ID_ENV, "shared-lane");

    let path = write_agent_info(
        LogSource::AgentService,
        "Agent service dev runtime started.",
        Default::default(),
    )?;

    env::remove_var(DEV_LOG_DIR_ENV);
    env::remove_var(LOG_RUN_ID_ENV);
    env::remove_var(LEDGER_LANE_ENV);
    env::remove_var(LANE_ID_ENV);

    let payload = fs::read_to_string(path)?;
    let line = payload.lines().next().unwrap_or_default();
    let value: serde_json::Value = serde_json::from_str(line)?;
    assert_eq!(value["runId"].as_str(), Some("shared-run-id"));
    assert_eq!(value["laneId"].as_str(), Some("ledger-lane"));
    Ok(())
}
