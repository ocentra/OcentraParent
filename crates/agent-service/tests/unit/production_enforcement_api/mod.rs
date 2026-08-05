use std::path::PathBuf;

use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use crate::enforcement_pre_action_journal;

#[path = "../../../src/enforcement_api/enforcement_command_execution.rs"]
mod enforcement_command_execution;
#[path = "../../../src/enforcement_api/enforcement_report_payload.rs"]
pub(crate) mod enforcement_report_payload;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementJournalPaths {
    pub(crate) journal_path: PathBuf,
    pub(crate) key_path: PathBuf,
    pub(crate) store_path: PathBuf,
    pub(crate) timer_state_path: crate::enforcement_timer_state_path::EnforcementTimerStatePath,
}

pub(crate) async fn build_enforcement_audit_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    enforcement_command_execution::build_enforcement_audit_report_with_paths(command, paths).await
}
