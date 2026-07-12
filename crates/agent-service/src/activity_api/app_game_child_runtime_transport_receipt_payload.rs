#[path = "app_game_child_runtime_transport_receipt_payload_support.rs"]
mod app_game_child_runtime_transport_receipt_payload_support;

use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::AppGameChildRuntimeTransportReceiptReadModel;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use self::app_game_child_runtime_transport_receipt_payload_support::{
    app_game_child_runtime_transport_receipt_payload as app_game_child_runtime_transport_receipt_payload_support,
    app_game_child_runtime_transport_receipt_read_model as app_game_child_runtime_transport_receipt_read_model_support,
    app_game_child_runtime_transport_receipt_read_model_from_service_model as app_game_child_runtime_transport_receipt_read_model_from_service_model_support,
    build_activity_app_game_child_runtime_transport_receipt_report as build_activity_app_game_child_runtime_transport_receipt_report_support,
};

#[derive(Clone, Debug)]
pub(crate) struct AppGameReceiptGeneratedAt(pub(crate) String);

pub async fn build_activity_app_game_child_runtime_transport_receipt_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_activity_app_game_child_runtime_transport_receipt_report_support(command).await
}

pub fn app_game_child_runtime_transport_receipt_read_model(
    generated_at: AppGameReceiptGeneratedAt,
) -> AppGameChildRuntimeTransportReceiptReadModel {
    app_game_child_runtime_transport_receipt_read_model_support(generated_at)
}

pub fn app_game_child_runtime_transport_receipt_read_model_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGameChildRuntimeTransportReceiptReadModel {
    app_game_child_runtime_transport_receipt_read_model_from_service_model_support(model)
}

pub fn app_game_child_runtime_transport_receipt_payload(
    read_model: &AppGameChildRuntimeTransportReceiptReadModel,
) -> LogFields {
    app_game_child_runtime_transport_receipt_payload_support(read_model)
}
