use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};

use crate::{
    local_ai_chat_generation::build_local_ai_chat_generation_report,
    local_ai_runtime_status::build_local_ai_runtime_status_report,
    parent_assistant_runtime::build_parent_assistant_answer_report,
    policy_preview_api::build_policy_preview_read_model_report,
};

use super::{
    basic_reports::build_log_snapshot_report,
    policy_request_confirm::build_policy_request_assistant_preview_confirm_report,
    policy_request_resolution::build_policy_request_parent_resolution_report,
};

pub(super) async fn build_ai_command_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentLocalAiRuntimeStatusGet => {
            build_local_ai_runtime_status_report(command).await
        }
        AgentCommandName::AgentLocalAiChatGenerate => {
            build_local_ai_chat_generation_report(command).await
        }
        AgentCommandName::AgentParentAssistantAnswerGenerate
        | AgentCommandName::AgentParentAssistantMessageSend
        | AgentCommandName::AgentParentAssistantQuickActionStart => {
            build_parent_assistant_answer_report(command).await
        }
        AgentCommandName::AgentPolicyPreviewReadModelGet => {
            build_policy_preview_read_model_report(command).await
        }
        AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm => {
            build_policy_request_assistant_preview_confirm_report(command).await
        }
        AgentCommandName::AgentPolicyRequestParentResolutionResolve => {
            build_policy_request_parent_resolution_report(command).await
        }
        _ => build_log_snapshot_report(command),
    }
}
