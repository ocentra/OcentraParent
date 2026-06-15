use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogSource {
    #[serde(rename = "agent-service")]
    AgentService,
    #[serde(rename = "dev-server")]
    DevServer,
    #[serde(rename = "local-api")]
    LocalApi,
    #[serde(rename = "portal")]
    Portal,
    #[serde(rename = "codex")]
    Codex,
    #[serde(rename = "validation")]
    Validation,
    #[serde(rename = "rust-test")]
    RustTest,
}

impl LogSource {
    pub fn compat_file_prefix(&self) -> &'static str {
        match self {
            Self::AgentService => "agent-service",
            Self::DevServer => "dev-server",
            Self::LocalApi => "local-api",
            Self::Portal => "portal",
            Self::Codex => "codex",
            Self::Validation => "validation",
            Self::RustTest => "rust-test",
        }
    }
}
