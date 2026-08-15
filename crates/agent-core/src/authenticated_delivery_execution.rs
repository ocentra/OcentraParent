#![forbid(unsafe_code)]

//! Durable execution intent/receipt state for authenticated delivery.
//!
//! This deliberately records an intent before any platform effect.  An OS
//! adapter is not transactionally composable with SQLite, so recovery must
//! resume a persisted intent rather than pretending the effect was atomic.

use std::{path::Path, time::Duration};

use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};

use crate::enforcement_adapter::{terminate_owned_process, OwnedProcessTerminationTarget};

const CREATE_EXECUTIONS: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_executions_v1 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, correlation_id TEXT NOT NULL, state TEXT NOT NULL, receipt_json TEXT, lease_owner TEXT, lease_expires_at TEXT, PRIMARY KEY (issuer_key_id, nonce))";
const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedDeliveryExecutionReceipt {
    pub correlation_id: String,
    pub nonce_digest: String,
    pub state: AuthenticatedDeliveryExecutionState,
    pub adapter_result: Option<String>,
    pub rollback_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthenticatedDeliveryExecutionState {
    Pending,
    Claimed,
    Succeeded,
    Failed,
    RolledBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticatedDeliveryExecutionError {
    StorageUnavailable,
    IntegrityRejected,
    InvalidInput,
}

impl std::fmt::Display for AuthenticatedDeliveryExecutionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "authenticated delivery execution error: {self:?}"
        )
    }
}

impl std::error::Error for AuthenticatedDeliveryExecutionError {}

pub struct AuthenticatedDeliveryExecutionStore {
    connection: Connection,
}

impl AuthenticatedDeliveryExecutionStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AuthenticatedDeliveryExecutionError> {
        let connection = Connection::open(path)
            .map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        connection
            .busy_timeout(BUSY_TIMEOUT)
            .map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        connection
            .execute(CREATE_EXECUTIONS, [])
            .map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        Ok(Self { connection })
    }

    pub fn persist_intent(
        &mut self,
        issuer_key_id: &str,
        nonce: &str,
        receipt: &AuthenticatedDeliveryExecutionReceipt,
    ) -> Result<bool, AuthenticatedDeliveryExecutionError> {
        if issuer_key_id.trim().is_empty()
            || nonce.trim().is_empty()
            || receipt.correlation_id.trim().is_empty()
        {
            return Err(AuthenticatedDeliveryExecutionError::InvalidInput);
        }
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        let existing: Option<String> = tx.query_row("SELECT receipt_json FROM authenticated_delivery_executions_v1 WHERE issuer_key_id=?1 AND nonce=?2", params![issuer_key_id, nonce], |r| r.get(0)).optional().map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        if let Some(existing) = existing {
            let _: AuthenticatedDeliveryExecutionReceipt = serde_json::from_str(&existing)
                .map_err(|_| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
            tx.commit()
                .map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
            return Ok(false);
        }
        let json = serde_json::to_string(receipt)
            .map_err(|_| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        tx.execute("INSERT INTO authenticated_delivery_executions_v1 (issuer_key_id,nonce,correlation_id,state,receipt_json) VALUES (?1,?2,?3,?4,?5)", params![issuer_key_id,nonce,receipt.correlation_id,"pending",json]).map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        tx.commit()
            .map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        Ok(true)
    }

    pub fn execute_owned_process(
        &mut self,
        issuer_key_id: &str,
        nonce: &str,
        target: OwnedProcessTerminationTarget,
        completed_at: &str,
    ) -> Result<AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionError> {
        let mut receipt = self.claim(issuer_key_id, nonce)?;
        let outcome = terminate_owned_process(target, completed_at);
        receipt.adapter_result = Some(format!("{:?}", outcome.adapter_result_code));
        receipt.rollback_required = !matches!(
            outcome.rollback_state,
            ocentra_parent_agent_protocol::enforcement::EnforcementRollbackState::NotRequired
        );
        receipt.state = if matches!(
            outcome.status,
            ocentra_parent_agent_protocol::enforcement::EnforcementResultStatus::ActuallyEnforced
        ) {
            AuthenticatedDeliveryExecutionState::Succeeded
        } else {
            AuthenticatedDeliveryExecutionState::Failed
        };
        self.store_receipt(issuer_key_id, nonce, &receipt)?;
        Ok(receipt)
    }

    pub fn recover_pending(
        &mut self,
        issuer_key_id: &str,
        nonce: &str,
    ) -> Result<AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionError> {
        self.claim(issuer_key_id, nonce)
    }

    pub fn read_receipt(
        &self,
        issuer_key_id: &str,
        nonce: &str,
    ) -> Result<Option<AuthenticatedDeliveryExecutionReceipt>, AuthenticatedDeliveryExecutionError>
    {
        let json: Option<String> = self.connection.query_row("SELECT receipt_json FROM authenticated_delivery_executions_v1 WHERE issuer_key_id=?1 AND nonce=?2", params![issuer_key_id, nonce], |r| r.get(0)).optional().map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        json.map(|value| {
            serde_json::from_str(&value)
                .map_err(|_| AuthenticatedDeliveryExecutionError::IntegrityRejected)
        })
        .transpose()
    }

    fn claim(
        &mut self,
        issuer_key_id: &str,
        nonce: &str,
    ) -> Result<AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionError> {
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        let json: String = tx.query_row("SELECT receipt_json FROM authenticated_delivery_executions_v1 WHERE issuer_key_id=?1 AND nonce=?2", params![issuer_key_id, nonce], |r| r.get(0)).map_err(|_| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        let mut receipt: AuthenticatedDeliveryExecutionReceipt = serde_json::from_str(&json)
            .map_err(|_| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        if matches!(
            receipt.state,
            AuthenticatedDeliveryExecutionState::Succeeded
                | AuthenticatedDeliveryExecutionState::RolledBack
        ) {
            return Err(AuthenticatedDeliveryExecutionError::IntegrityRejected);
        }
        receipt.state = AuthenticatedDeliveryExecutionState::Claimed;
        let receipt_json = serde_json::to_string(&receipt)
            .map_err(|_| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        tx.execute("UPDATE authenticated_delivery_executions_v1 SET state=?3, receipt_json=?4 WHERE issuer_key_id=?1 AND nonce=?2", params![issuer_key_id, nonce, "claimed", receipt_json]).map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        tx.commit()
            .map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        Ok(receipt)
    }

    fn store_receipt(
        &mut self,
        issuer_key_id: &str,
        nonce: &str,
        receipt: &AuthenticatedDeliveryExecutionReceipt,
    ) -> Result<(), AuthenticatedDeliveryExecutionError> {
        let json = serde_json::to_string(receipt)
            .map_err(|_| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        self.connection.execute("UPDATE authenticated_delivery_executions_v1 SET state=?3, receipt_json=?4 WHERE issuer_key_id=?1 AND nonce=?2", params![issuer_key_id, nonce, format!("{:?}", receipt.state).to_lowercase(), json]).map_err(|_| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        Ok(())
    }
}
