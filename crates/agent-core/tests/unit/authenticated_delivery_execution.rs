use std::path::PathBuf;

use ocentra_parent_agent_core::authenticated_delivery_execution::{
    AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionState,
    AuthenticatedDeliveryExecutionStore,
};

fn store_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-authenticated-delivery-execution-{}.sqlite",
        std::process::id()
    ))
}

#[test]
fn intent_is_durable_and_recovery_claims_it_before_an_external_effect(
) -> Result<(), Box<dyn std::error::Error>> {
    let path = store_path();
    let mut store = AuthenticatedDeliveryExecutionStore::open(&path)?;
    let receipt = AuthenticatedDeliveryExecutionReceipt {
        correlation_id: "correlation-1".to_owned(),
        nonce_digest: "redacted-nonce-digest".to_owned(),
        state: AuthenticatedDeliveryExecutionState::Pending,
        adapter_result: None,
        rollback_required: false,
    };
    assert!(store.persist_intent("issuer-1", "nonce-1", &receipt)?);
    assert!(!store.persist_intent("issuer-1", "nonce-1", &receipt)?);
    drop(store);
    let mut reopened = AuthenticatedDeliveryExecutionStore::open(path)?;
    let recovered = reopened.recover_pending("issuer-1", "nonce-1")?;
    assert_eq!(
        recovered.state,
        AuthenticatedDeliveryExecutionState::Claimed
    );
    assert_eq!(recovered.nonce_digest, "redacted-nonce-digest");
    Ok(())
}
