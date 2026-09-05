use std::fmt::Display;

use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_cross_process_custody_readiness_types::NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError,
    remote_delivery_cross_process_replay::prove_network_runtime_remote_delivery_cross_process_replay,
    remote_delivery_cross_process_replay_types::NetworkRuntimeRemoteDeliveryCrossProcessReplayError,
    remote_delivery_durable_envelope_types::NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    remote_delivery_fixture_transport_types::NetworkRuntimeRemoteDeliveryFixtureTransportError,
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    remote_delivery_provider_child_readiness_types::NetworkRuntimeRemoteDeliveryProviderChildReadinessError,
    remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptLedgerError,
};
use ocentra_parent_agent_protocol::constants;

use crate::test_text::TestText;

type TestResult = Result<(), TestText>;

fn expect_runtime_owner_unavailable<T>(
    result: Result<T, NetworkRuntimeRemoteDeliveryCrossProcessReplayError>,
    context: impl Display,
) -> TestResult {
    match result {
        Err(
            NetworkRuntimeRemoteDeliveryCrossProcessReplayError::CrossProcessCustodyReadiness(
                NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError::ProviderChildReadiness(
                    NetworkRuntimeRemoteDeliveryProviderChildReadinessError::FixtureTransport(
                        NetworkRuntimeRemoteDeliveryFixtureTransportError::OutboxHandoff(
                            NetworkRuntimeRemoteDeliveryOutboxHandoffError::DurableEnvelope(
                                NetworkRuntimeRemoteDeliveryDurableEnvelopeError::ReceiptLedger(
                                    NetworkRuntimeRemoteDeliveryReceiptLedgerError::RuntimeOwnerUnavailable,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ) => Ok(()),
        Err(error) => Err(TestText::from_display(format!(
            "{context}: expected CrossProcessCustodyReadiness(...RuntimeOwnerUnavailable), got {error:?}"
        ))),
        Ok(_) => Err(TestText::from_display(format!(
            "{context}: expected CrossProcessCustodyReadiness(...RuntimeOwnerUnavailable), got success"
        ))),
    }
}

#[tokio::test]
async fn cross_process_replay_rejects_without_runtime_owner() -> TestResult {
    expect_runtime_owner_unavailable(
        prove_network_runtime_remote_delivery_cross_process_replay().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_REPLAY,
    )
}
