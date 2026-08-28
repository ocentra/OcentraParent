use std::fmt::Display;

use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_dispatch_readiness_types::NetworkRuntimeRemoteDeliveryDispatchReadinessError,
    remote_delivery_durable_envelope_types::NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    remote_delivery_no_enforcement_invariant_types::NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptLedgerError,
    remote_delivery_transport_dispatch_state::prove_network_runtime_remote_delivery_transport_dispatch_state,
    remote_delivery_transport_dispatch_state_types::NetworkRuntimeRemoteDeliveryTransportDispatchStateError,
};
use ocentra_parent_agent_protocol::constants;

use crate::test_text::TestText;

type TestResult = Result<(), TestText>;

fn expect_runtime_owner_unavailable<T>(
    result: Result<T, NetworkRuntimeRemoteDeliveryTransportDispatchStateError>,
    context: impl Display,
) -> TestResult {
    match result {
        Err(NetworkRuntimeRemoteDeliveryTransportDispatchStateError::NoEnforcementInvariant(
            NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError::DispatchReadiness(
                NetworkRuntimeRemoteDeliveryDispatchReadinessError::OutboxHandoff(
                    NetworkRuntimeRemoteDeliveryOutboxHandoffError::DurableEnvelope(
                        NetworkRuntimeRemoteDeliveryDurableEnvelopeError::ReceiptLedger(
                            NetworkRuntimeRemoteDeliveryReceiptLedgerError::RuntimeOwnerUnavailable,
                        ),
                    ),
                ),
            ),
        )) => Ok(()),
        Err(error) => Err(TestText::from_display(format!(
            "{context}: expected NoEnforcementInvariant(...RuntimeOwnerUnavailable), got {error:?}"
        ))),
        Ok(_) => Err(TestText::from_display(format!(
            "{context}: expected NoEnforcementInvariant(...RuntimeOwnerUnavailable), got success"
        ))),
    }
}

#[tokio::test]
async fn transport_dispatch_state_rejects_without_runtime_owner() -> TestResult {
    expect_runtime_owner_unavailable(
        prove_network_runtime_remote_delivery_transport_dispatch_state().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_TRANSPORT_DISPATCH_STATE,
    )
}
