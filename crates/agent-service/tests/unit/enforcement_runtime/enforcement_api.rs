use std::path::PathBuf;

use ocentra_parent_agent_protocol::enforcement::AppGameTimerSessionBinding;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

#[path = "../../../src/enforcement_api/enforcement_broad_adapter_proof_read_model.rs"]
pub(crate) mod enforcement_broad_adapter_proof_read_model;
#[path = "../../../src/enforcement_api/enforcement_command_execution.rs"]
pub(crate) mod enforcement_command_execution;
#[path = "../../../src/enforcement_api/enforcement_integrity_runtime_audit_read_model.rs"]
pub(crate) mod enforcement_integrity_runtime_audit_read_model;
#[path = "../../../src/enforcement_api/enforcement_pre_action_journal.rs"]
pub(crate) mod enforcement_pre_action_journal;
#[path = "../../../src/enforcement_api/enforcement_report_payload.rs"]
mod enforcement_report_payload;
#[path = "../../../src/enforcement_api/enforcement_supported_adapter_runtime_proof_read_model.rs"]
pub(crate) mod enforcement_supported_adapter_runtime_proof_read_model;
#[path = "../../../src/enforcement_api/integrity_alert_status_bridge_read_model.rs"]
pub(crate) mod integrity_alert_status_bridge_read_model;
#[path = "../../../src/enforcement_api/notification_provider_status_boundary_read_model.rs"]
pub(crate) mod notification_provider_status_boundary_read_model;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementJournalPaths {
    pub journal_path: PathBuf,
    pub key_path: PathBuf,
    pub store_path: PathBuf,
    pub timer_state_path: crate::enforcement_timer_state_path::EnforcementTimerStatePath,
}

impl EnforcementJournalPaths {
    pub(crate) fn from_environment() -> Self {
        Self {
            journal_path: crate::activity_store_path::activity_journal_path().into(),
            key_path: crate::activity_store_path::activity_journal_key_path().into(),
            store_path: crate::activity_store_path::activity_db_path().into(),
            timer_state_path: crate::enforcement_timer_state_path::enforcement_timer_state_path(),
        }
    }
}

pub(crate) async fn build_enforcement_audit_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    Box::pin(
        enforcement_command_execution::build_enforcement_audit_report_with_paths(command, paths),
    )
    .await
}

pub(crate) async fn build_enforcement_audit_report_with_app_game_session(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
    app_game_session: AppGameTimerSessionBinding,
) -> AgentEventEnvelope {
    Box::pin(
        enforcement_command_execution::build_enforcement_audit_report_with_app_game_session(
            command,
            paths,
            app_game_session,
        ),
    )
    .await
}
