use std::{error::Error, io::Error as IoError};

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

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

#[tokio::test]
async fn network_remote_delivery_status_payload_rejects_without_runtime_owner() -> TestResult {
    expect_status_payload_rejection(
        &crate::network_remote_delivery_status_payload::network_remote_delivery_status_payload()
            .await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_REPLAY_STATUS_BRIDGE,
    )
}

#[tokio::test]
async fn network_remote_delivery_status_payload_repeatedly_fails_closed_without_runtime_owner(
) -> TestResult {
    expect_status_payload_rejection(
        &crate::network_remote_delivery_status_payload::network_remote_delivery_status_payload()
            .await,
        "first remote-delivery status payload request",
    )?;
    expect_status_payload_rejection(
        &crate::network_remote_delivery_status_payload::network_remote_delivery_status_payload()
            .await,
        "second remote-delivery status payload request",
    )
}

#[tokio::test]
async fn network_remote_delivery_transport_dispatch_state_rejects_without_runtime_owner(
) -> TestResult {
    expect_transport_dispatch_runtime_owner_unavailable(
        prove_network_runtime_remote_delivery_transport_dispatch_state().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_TRANSPORT_DISPATCH_STATE,
    )
}

fn expect_status_payload_rejection<T>(result: &Result<T, ()>, context: &str) -> TestResult {
    match result {
        Err(()) => Ok(()),
        Ok(_) => Err(IoError::other(format!(
            "{context}: expected fail-closed status payload rejection, got success"
        ))
        .into()),
    }
}

fn expect_transport_dispatch_runtime_owner_unavailable<T>(
    result: Result<T, NetworkRuntimeRemoteDeliveryTransportDispatchStateError>,
    context: &str,
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
        Err(error) => Err(IoError::other(format!(
            "{context}: expected NoEnforcementInvariant(...RuntimeOwnerUnavailable), got {error:?}"
        ))
        .into()),
        Ok(_) => Err(IoError::other(format!(
            "{context}: expected NoEnforcementInvariant(...RuntimeOwnerUnavailable), got success"
        ))
        .into()),
    }
}
