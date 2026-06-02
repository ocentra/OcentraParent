pub(crate) static REPORT_ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
