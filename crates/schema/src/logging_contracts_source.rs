use super::LogSource;

impl LogSource {
    pub fn compat_file_prefix(&self) -> &str {
        match self {
            Self::AgentService => Self::AGENT_SERVICE_FILE_PREFIX,
            Self::DevServer => Self::DEV_SERVER_FILE_PREFIX,
            Self::LocalApi => Self::LOCAL_API_FILE_PREFIX,
            Self::Portal => Self::PORTAL_FILE_PREFIX,
            Self::Codex => Self::CODEX_FILE_PREFIX,
            Self::Validation => Self::VALIDATION_FILE_PREFIX,
            Self::RustTest => Self::RUST_TEST_FILE_PREFIX,
        }
    }
}
