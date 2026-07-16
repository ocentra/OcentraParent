use serde::{Deserialize, Serialize};

pub const DIAGNOSTIC_SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticKind {
    Rustc,
    Clippy,
    CargoTest,
    TypeScript,
    Eslint,
    NpmScript,
    ArchitecturePolicy,
    NoReexportPolicy,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDiagnostic {
    pub schema_version: u16,
    #[serde(rename = "eventType")]
    pub record_type: String,
    pub diagnostic_id: String,
    pub run_id: String,
    pub command_id: String,
    pub kind: DiagnosticKind,
    pub severity: DiagnosticSeverity,
    pub signature: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,
    pub raw_artifact: Option<String>,
    pub raw_start_line: Option<u32>,
    pub raw_end_line: Option<u32>,
}
