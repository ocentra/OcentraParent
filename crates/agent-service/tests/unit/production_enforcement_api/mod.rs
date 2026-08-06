use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

pub(crate) type EnforcementJournalPaths = crate::enforcement_api::EnforcementJournalPaths;

pub(crate) async fn build_enforcement_audit_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    crate::enforcement_api::enforcement_command_execution::build_enforcement_audit_report_with_paths(
        command, paths,
    )
    .await
}
