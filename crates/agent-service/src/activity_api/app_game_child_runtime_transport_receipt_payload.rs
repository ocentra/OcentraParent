#[path = "app_game_child_runtime_transport_receipt_payload_support.rs"]
mod app_game_child_runtime_transport_receipt_payload_support;

pub(super) fn app_game_child_runtime_transport_receipt_read_model_from_service_model(
    model: ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
) -> ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::AppGameChildRuntimeTransportReceiptReadModel{
    app_game_child_runtime_transport_receipt_payload_support::app_game_child_runtime_transport_receipt_read_model_from_service_model(model)
}

use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use self::app_game_child_runtime_transport_receipt_payload_support::build_activity_app_game_child_runtime_transport_receipt_report as build_activity_app_game_child_runtime_transport_receipt_report_support;

#[derive(Clone, Debug)]
pub(crate) struct AppGameReceiptGeneratedAt(pub(crate) String);

pub async fn build_activity_app_game_child_runtime_transport_receipt_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_activity_app_game_child_runtime_transport_receipt_report_support(command).await
}
