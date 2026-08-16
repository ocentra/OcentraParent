use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};

use crate::activity_api::{
    app_game_adapter_dispatch_preflight_payload::build_activity_app_game_adapter_dispatch_preflight_report,
    app_game_adapter_dispatch_result_payload::build_activity_app_game_adapter_dispatch_result_report,
    app_game_adapter_execution_readiness_payload::build_activity_app_game_adapter_execution_readiness_report,
    app_game_child_runtime_transport_receipt_payload::build_activity_app_game_child_runtime_transport_receipt_report,
    app_game_platform_proof_status_payload::build_activity_app_game_platform_proof_status_report,
    build_activity_app_game_boundary_read_model_report,
    build_activity_app_game_notification_readiness_report,
    build_activity_app_game_policy_readiness_report,
};

use super::basic_reports::build_log_snapshot_report;

pub(super) async fn build_activity_app_game_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentActivityAppGameBoundaryReadModelGet => {
            build_activity_app_game_boundary_read_model_report(command).await
        }
        AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet => {
            build_activity_app_game_policy_readiness_report(command).await
        }
        AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet => {
            build_activity_app_game_notification_readiness_report(command).await
        }
        AgentCommandName::AgentActivityAppGameAdapterExecutionReadinessReadModelGet => {
            build_activity_app_game_adapter_execution_readiness_report(command).await
        }
        AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet => {
            build_activity_app_game_platform_proof_status_report(command).await
        }
        AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet => {
            build_activity_app_game_child_runtime_transport_receipt_report(command).await
        }
        AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet => {
            build_activity_app_game_adapter_dispatch_preflight_report(command).await
        }
        AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet => {
            build_activity_app_game_adapter_dispatch_result_report(command).await
        }
        _ => build_log_snapshot_report(command),
    }
}
