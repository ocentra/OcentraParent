use ocentra_schema::parent_ui_bridge::ParentRouteContext;

use super::loaders;
use super::types::{
    AgentServiceResult, AppGameChildRuntimeTransportReceiptAgentServiceSnapshot,
    AppGameNotificationReadinessAgentServiceSnapshot,
    AppGamePlatformProofStatusAgentServiceSnapshot, AppGamePolicyReadinessAgentServiceSnapshot,
};

pub(crate) fn load_app_game_notification_readiness_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<AppGameNotificationReadinessAgentServiceSnapshot> {
    loaders::load_app_game_notification_readiness_read_model_snapshot(context)
}

pub(crate) fn load_app_game_policy_readiness_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<AppGamePolicyReadinessAgentServiceSnapshot> {
    loaders::load_app_game_policy_readiness_read_model_snapshot(context)
}

pub(crate) fn load_app_game_platform_proof_status_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<AppGamePlatformProofStatusAgentServiceSnapshot> {
    loaders::load_app_game_platform_proof_status_read_model_snapshot(context)
}

pub(crate) fn load_app_game_child_runtime_transport_receipt_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<AppGameChildRuntimeTransportReceiptAgentServiceSnapshot> {
    loaders::load_app_game_child_runtime_transport_receipt_read_model_snapshot(context)
}
