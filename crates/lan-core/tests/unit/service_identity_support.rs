pub(super) fn agent_addr_env_lock() -> std::sync::MutexGuard<'static, ()> {
    static AGENT_ADDR_ENV_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
        std::sync::OnceLock::new();
    match AGENT_ADDR_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
    {
        Ok(guard) => guard,
        Err(error) => error.into_inner(),
    }
}

pub(super) fn service_identity_env_lock() -> std::sync::MutexGuard<'static, ()> {
    static SERVICE_IDENTITY_ENV_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
        std::sync::OnceLock::new();
    match SERVICE_IDENTITY_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
    {
        Ok(guard) => guard,
        Err(error) => error.into_inner(),
    }
}
