use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    error::Error,
    fs,
    process::Command,
    sync::{Mutex, OnceLock},
    thread,
};

use ocentra_parent_logging_core::{
    dev_log::write_agent_info,
    field::{LogFieldValue, LogFields},
    path::{DEV_LOG_DIR_ENV, LANE_ID_ENV, LEDGER_LANE_ENV, LOG_ROOT_ENV, LOG_RUN_ID_ENV},
    redaction::REDACTED_VALUE,
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

#[test]
fn dev_logger_redacts_secret_fields_before_persisting() {
    let result = dev_logger_redacts_secret_fields_before_persisting_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn dev_logger_compat_file_keeps_concurrent_records_parseable() {
    let result = dev_logger_compat_file_keeps_concurrent_records_parseable_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn dev_logger_subprocess_worker() {
    let result = dev_logger_subprocess_worker_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn dev_logger_ids_do_not_collide_across_subprocess_restarts() {
    let result = dev_logger_ids_do_not_collide_across_subprocess_restarts_impl();
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

fn dev_logger_redacts_secret_fields_before_persisting_impl() -> Result<(), Box<dyn Error>> {
    let _guard = env_lock()
        .lock()
        .map_err(|error| std::io::Error::other(format!("failed to lock env mutex: {error:?}")))?;

    let temp = temp_dir!();
    env::remove_var(LOG_ROOT_ENV);
    env::set_var(DEV_LOG_DIR_ENV, &temp);
    let mut fields = LogFields::new();
    fields.insert(
        "apiToken".to_owned(),
        LogFieldValue::String("persisted-secret-value".to_owned()),
    );

    let path = write_agent_info(LogSource::AgentService, "redaction check", fields)?;

    env::remove_var(DEV_LOG_DIR_ENV);
    let payload = fs::read_to_string(path)?;
    let entry: serde_json::Value = serde_json::from_str(payload.trim())?;
    assert_eq!(entry["fields"]["apiToken"], REDACTED_VALUE);
    assert_ne!(entry["fields"]["apiToken"], "persisted-secret-value");
    Ok(())
}

fn dev_logger_compat_file_keeps_concurrent_records_parseable_impl() -> Result<(), Box<dyn Error>> {
    let _guard = env_lock()
        .lock()
        .map_err(|error| std::io::Error::other(format!("failed to lock env mutex: {error:?}")))?;

    let temp = temp_dir!();
    env::remove_var(LOG_ROOT_ENV);
    env::set_var(DEV_LOG_DIR_ENV, &temp);
    let workers = (0..16)
        .map(|worker| {
            thread::spawn(move || {
                let mut path = None;
                for record in 0..16 {
                    path = Some(write_agent_info(
                        LogSource::AgentService,
                        "concurrent compatibility event",
                        LogFields::from(BTreeMap::from([(
                            "record".to_owned(),
                            LogFieldValue::String(format!("{worker}-{record}")),
                        )])),
                    )?);
                }
                path.ok_or_else(|| std::io::Error::other("worker wrote no records"))
            })
        })
        .collect::<Vec<_>>();

    let mut paths = Vec::new();
    for worker in workers {
        let result = worker
            .join()
            .map_err(|_error| std::io::Error::other("concurrent logger worker panicked"))?;
        paths.push(result?);
    }
    env::remove_var(DEV_LOG_DIR_ENV);

    let path = paths
        .into_iter()
        .next()
        .ok_or_else(|| std::io::Error::other("no compatibility log path returned"))?;
    let payload = fs::read_to_string(path)?;
    let rows = payload
        .lines()
        .map(serde_json::from_str::<serde_json::Value>)
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(rows.len(), 256);
    Ok(())
}

fn dev_logger_subprocess_worker_impl() -> Result<(), Box<dyn Error>> {
    if env::var_os("OCENTRA_DEV_LOG_SUBPROCESS").is_none() {
        return Ok(());
    }
    write_agent_info(
        LogSource::AgentService,
        "restart collision check",
        Default::default(),
    )?;
    Ok(())
}

fn dev_logger_ids_do_not_collide_across_subprocess_restarts_impl() -> Result<(), Box<dyn Error>> {
    let _guard = env_lock()
        .lock()
        .map_err(|error| std::io::Error::other(format!("failed to lock env mutex: {error:?}")))?;
    let directory = temp_dir!();
    let executable = env::current_exe()?;
    let mut children = Vec::new();
    for _ in 0..8 {
        children.push(
            Command::new(&executable)
                .args(["--exact", "dev_logger_subprocess_worker", "--nocapture"])
                .env("OCENTRA_DEV_LOG_SUBPROCESS", "1")
                .env(DEV_LOG_DIR_ENV, &directory)
                .env_remove(LOG_ROOT_ENV)
                .spawn()?,
        );
    }
    for mut child in children {
        if !child.wait()?.success() {
            return Err(std::io::Error::other("dev log subprocess worker failed").into());
        }
    }
    let mut records = BTreeSet::new();
    for file in fs::read_dir(&directory)?.filter_map(Result::ok) {
        if file.file_name().to_string_lossy().ends_with(".ndjson") {
            for entry in fs::read_to_string(file.path())?.lines() {
                let _: serde_json::Value = serde_json::from_str(entry)?;
                records.insert(entry.to_owned());
            }
        }
    }
    assert_eq!(records.len(), 8);
    Ok(())
}
