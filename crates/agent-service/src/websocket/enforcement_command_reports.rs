use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};

use crate::{
    enforcement_api::{
        build_enforcement_audit_report, build_enforcement_policy_dispatch_report,
        build_enforcement_product_control_spine_report,
        enforcement_broad_adapter_proof_report::build_enforcement_broad_adapter_proof_report,
        enforcement_supported_adapter_runtime_proof_report::build_enforcement_supported_adapter_runtime_proof_report,
    },
    enforcement_timer_api::build_enforcement_timer_report,
};

use super::basic_reports::build_log_snapshot_report;

pub(super) async fn build_enforcement_command_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentEnforcementExecute => build_enforcement_audit_report(command).await,
        AgentCommandName::AgentEnforcementProductControlSpineGet => {
            build_enforcement_product_control_spine_report(command).await
        }
        AgentCommandName::AgentEnforcementPolicyDispatchGet => {
            build_enforcement_policy_dispatch_report(command).await
        }
        AgentCommandName::AgentEnforcementBroadAdapterProofGet => {
            build_enforcement_broad_adapter_proof_report(command).await
        }
        AgentCommandName::AgentEnforcementSupportedAdapterRuntimeProofGet => {
            build_enforcement_supported_adapter_runtime_proof_report(command).await
        }
        AgentCommandName::AgentEnforcementTimerRecover
        | AgentCommandName::AgentEnforcementTimerExpire
        | AgentCommandName::AgentEnforcementOverrideCancel => {
            build_enforcement_timer_report(command).await
        }
        _ => build_log_snapshot_report(command),
    }
}
