use serde::{Deserialize, Serialize};

pub const AGENT_RUN_SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    Passed,
    Failed,
    Timeout,
    Cancelled,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRunEvent {
    pub schema_version: u16,
    #[serde(rename = "eventType")]
    pub record_type: String,
    pub run_id: String,
    pub command_id: String,
    pub lane_id: Option<String>,
    pub machine: Option<String>,
    pub workspace: String,
    pub cwd: String,
    pub command: Vec<String>,
    pub started_at: String,
    pub ended_at: String,
    pub duration_ms: u64,
    pub status: RunStatus,
    pub exit_code: Option<i32>,
    pub stdout_artifact: Option<String>,
    pub stderr_artifact: Option<String>,
    pub summary: Option<String>,
}
