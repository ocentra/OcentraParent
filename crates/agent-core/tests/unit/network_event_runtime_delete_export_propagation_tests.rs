use std::fmt::Display;

use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_delete_export_propagation::prove_network_runtime_remote_delivery_delete_export_propagation,
    remote_delivery_delete_export_propagation_types::NetworkRuntimeRemoteDeliveryDeleteExportPropagationError,
    remote_delivery_durable_envelope_types::NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    remote_delivery_fixture_transport_types::NetworkRuntimeRemoteDeliveryFixtureTransportError,
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptLedgerError,
};
use ocentra_parent_agent_protocol::constants;

use crate::test_text::TestText;

type TestResult = Result<(), TestText>;

fn expect_runtime_owner_unavailable<T>(
    result: Result<T, NetworkRuntimeRemoteDeliveryDeleteExportPropagationError>,
    context: impl Display,
) -> TestResult {
    match result {
        Err(NetworkRuntimeRemoteDeliveryDeleteExportPropagationError::FixtureTransport(
            NetworkRuntimeRemoteDeliveryFixtureTransportError::OutboxHandoff(
                NetworkRuntimeRemoteDeliveryOutboxHandoffError::DurableEnvelope(
                    NetworkRuntimeRemoteDeliveryDurableEnvelopeError::ReceiptLedger(
                        NetworkRuntimeRemoteDeliveryReceiptLedgerError::RuntimeOwnerUnavailable,
                    ),
                ),
            ),
        )) => Ok(()),
        Err(error) => Err(TestText::from_display(format!(
            "{context}: expected FixtureTransport(...RuntimeOwnerUnavailable), got {error:?}"
        ))),
        Ok(_) => Err(TestText::from_display(format!(
            "{context}: expected FixtureTransport(...RuntimeOwnerUnavailable), got success"
        ))),
    }
}

#[tokio::test]
async fn delete_export_propagation_rejects_without_runtime_owner() -> TestResult {
    expect_runtime_owner_unavailable(
        prove_network_runtime_remote_delivery_delete_export_propagation().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DELETE_EXPORT_PROPAGATION,
    )
}
