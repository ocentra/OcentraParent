#![forbid(unsafe_code)]

//! Durable execution intent/receipt state for authenticated delivery.
//!
//! This deliberately records an intent before any platform effect.  An OS
//! adapter is not transactionally composable with SQLite, so recovery must
//! resume a persisted intent rather than pretending the effect was atomic.

use std::{path::Path, time::Duration};

use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};

use crate::{
    authenticated_delivery_grant::AuthenticatedDeliveryGrantTrustedIssuer,
    enforcement_adapter::{
        resolve_authenticated_managed_process_target, terminate_authenticated_owned_process,
        terminate_owned_process, AuthenticatedOwnedProcessTerminationTarget,
        OwnedProcessTerminationTarget,
    },
};
#[path = "authenticated_delivery_execution_trace_identity.rs"]
mod authenticated_delivery_execution_trace_identity;
#[path = "authenticated_delivery_execution_trace_outcome.rs"]
mod authenticated_delivery_execution_trace_outcome;
#[path = "authenticated_delivery_execution_trace_persistence.rs"]
mod authenticated_delivery_execution_trace_persistence;
use ocentra_schema::{
    authenticated_delivery_grant::AuthenticatedDeliveryGrant,
    authenticated_delivery_managed_process::AuthenticatedManagedProcessTargetBinding,
};

const CREATE_EXECUTIONS: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_executions_v1 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, correlation_id TEXT NOT NULL, state TEXT NOT NULL, receipt_json TEXT, lease_owner TEXT, lease_expires_at TEXT, PRIMARY KEY (issuer_key_id, nonce))";
const CREATE_TRACES: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_adapter_traces_v1 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, trace_json TEXT NOT NULL, PRIMARY KEY (issuer_key_id, nonce))";
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
    TargetBindingRejected,
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

pub fn authenticated_managed_process_target(
    grant: &AuthenticatedDeliveryGrant,
    binding: &AuthenticatedManagedProcessTargetBinding,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
    activity_store_path: impl AsRef<std::path::Path>,
) -> Result<AuthenticatedOwnedProcessTerminationTarget, AuthenticatedDeliveryExecutionError> {
    resolve_authenticated_managed_process_target(
        grant,
        binding,
        trusted_issuer,
        activity_store_path,
    )
    .map_err(|_error| AuthenticatedDeliveryExecutionError::TargetBindingRejected)
}

/// Adapter-owned execution identity.  The fields are intentionally private:
/// callers can inspect a persisted trace, but cannot construct one to mint a
/// policy receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedAdapterExecutionTrace {
    trace_id: String,
    grant_fingerprint: String,
    issuer_key_id: String,
    nonce_digest: String,
    correlation_id: String,
    issuer_actor_id: String,
    household_id: String,
    parent_device_id: String,
    child_profile_id: String,
    target_device_id: String,
    policy_decision_id: String,
    policy_version: String,
    action_id: String,
    capability_id: String,
    managed_process_identity: String,
    process_id: u32,
    expected_process_name: String,
    expected_executable_path_ref: String,
    process_start_time: u64,
    observed_process_id: Option<u32>,
    observed_process_name: Option<String>,
    observed_executable_path_ref: Option<String>,
    observed_process_start_time: Option<u64>,
    adapter_result: String,
    adapter_status: String,
    completed_at: Option<String>,
    rollback_required: bool,
    rollback_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PersistedAuthenticatedAdapterExecutionTrace {
    trace_id: String,
    grant_fingerprint: String,
    issuer_key_id: String,
    nonce_digest: String,
    correlation_id: String,
    issuer_actor_id: String,
    household_id: String,
    parent_device_id: String,
    child_profile_id: String,
    target_device_id: String,
    policy_decision_id: String,
    policy_version: String,
    action_id: String,
    capability_id: String,
    managed_process_identity: String,
    process_id: u32,
    expected_process_name: String,
    expected_executable_path_ref: String,
    process_start_time: u64,
    observed_process_id: Option<u32>,
    observed_process_name: Option<String>,
    observed_executable_path_ref: Option<String>,
    observed_process_start_time: Option<u64>,
    adapter_result: String,
    adapter_status: String,
    completed_at: Option<String>,
    rollback_required: bool,
    rollback_state: String,
}

impl AuthenticatedDeliveryExecutionStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AuthenticatedDeliveryExecutionError> {
        let connection = Connection::open(path)
            .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        connection
            .busy_timeout(BUSY_TIMEOUT)
            .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        connection
            .execute(CREATE_EXECUTIONS, [])
            .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        connection
            .execute(CREATE_TRACES, [])
            .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
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
            .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        let existing: Option<String> = tx.query_row("SELECT receipt_json FROM authenticated_delivery_executions_v1 WHERE issuer_key_id=?1 AND nonce=?2", params![issuer_key_id, nonce], |r| r.get(0)).optional().map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        if let Some(existing) = existing {
            let _: AuthenticatedDeliveryExecutionReceipt = serde_json::from_str(&existing)
                .map_err(|_error| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
            tx.commit()
                .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
            return Ok(false);
        }
        let json = serde_json::to_string(receipt)
            .map_err(|_error| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        tx.execute("INSERT INTO authenticated_delivery_executions_v1 (issuer_key_id,nonce,correlation_id,state,receipt_json) VALUES (?1,?2,?3,?4,?5)", params![issuer_key_id,nonce,receipt.correlation_id,"pending",json]).map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        tx.commit()
            .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
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

    pub fn execute_authenticated_owned_process(
        &mut self,
        issuer_key_id: &str,
        nonce: &str,
        target: &AuthenticatedOwnedProcessTerminationTarget,
        correlation_id: &str,
        completed_at: &str,
    ) -> Result<AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionError> {
        let mut receipt = self.claim(issuer_key_id, nonce)?;
        let execution = terminate_authenticated_owned_process(target, completed_at);
        let trace = authenticated_delivery_execution_trace_persistence::trace_for_execution(
            target,
            correlation_id,
            nonce,
            &execution,
        );
        self.store_trace(issuer_key_id, nonce, &trace)?;
        receipt.adapter_result = Some(format!("{:?}", execution.outcome.adapter_result_code));
        receipt.rollback_required = !matches!(
            execution.outcome.rollback_state,
            ocentra_parent_agent_protocol::enforcement::EnforcementRollbackState::NotRequired
        );
        receipt.state = if matches!(
            execution.outcome.status,
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
        let json: Option<String> = self.connection.query_row("SELECT receipt_json FROM authenticated_delivery_executions_v1 WHERE issuer_key_id=?1 AND nonce=?2", params![issuer_key_id, nonce], |r| r.get(0)).optional().map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        json.map(|value| {
            serde_json::from_str(&value)
                .map_err(|_error| AuthenticatedDeliveryExecutionError::IntegrityRejected)
        })
        .transpose()
    }

    pub fn read_trace(
        &self,
        issuer_key_id: &str,
        nonce: &str,
    ) -> Result<Option<AuthenticatedAdapterExecutionTrace>, AuthenticatedDeliveryExecutionError>
    {
        let json: Option<String> = self
            .connection
            .query_row(
                "SELECT trace_json FROM authenticated_delivery_adapter_traces_v1 WHERE issuer_key_id=?1 AND nonce=?2",
                params![issuer_key_id, nonce],
                |row| row.get(0),
            )
            .optional()
            .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        json.map(|value| {
            serde_json::from_str::<PersistedAuthenticatedAdapterExecutionTrace>(&value)
                .map(AuthenticatedAdapterExecutionTrace::from)
                .map_err(|_error| AuthenticatedDeliveryExecutionError::IntegrityRejected)
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
            .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        let json: String = tx.query_row("SELECT receipt_json FROM authenticated_delivery_executions_v1 WHERE issuer_key_id=?1 AND nonce=?2", params![issuer_key_id, nonce], |r| r.get(0)).map_err(|_error| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        let mut receipt: AuthenticatedDeliveryExecutionReceipt = serde_json::from_str(&json)
            .map_err(|_error| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        if matches!(
            receipt.state,
            AuthenticatedDeliveryExecutionState::Succeeded
                | AuthenticatedDeliveryExecutionState::Claimed
                | AuthenticatedDeliveryExecutionState::RolledBack
        ) {
            return Err(AuthenticatedDeliveryExecutionError::IntegrityRejected);
        }
        receipt.state = AuthenticatedDeliveryExecutionState::Claimed;
        let receipt_json = serde_json::to_string(&receipt)
            .map_err(|_error| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        tx.execute("UPDATE authenticated_delivery_executions_v1 SET state=?3, receipt_json=?4 WHERE issuer_key_id=?1 AND nonce=?2", params![issuer_key_id, nonce, "claimed", receipt_json]).map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        tx.commit()
            .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        Ok(receipt)
    }

    fn store_receipt(
        &self,
        issuer_key_id: &str,
        nonce: &str,
        receipt: &AuthenticatedDeliveryExecutionReceipt,
    ) -> Result<(), AuthenticatedDeliveryExecutionError> {
        let json = serde_json::to_string(receipt)
            .map_err(|_error| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        self.connection.execute("UPDATE authenticated_delivery_executions_v1 SET state=?3, receipt_json=?4 WHERE issuer_key_id=?1 AND nonce=?2", params![issuer_key_id, nonce, format!("{:?}", receipt.state).to_lowercase(), json]).map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        Ok(())
    }

    fn store_trace(
        &self,
        issuer_key_id: &str,
        nonce: &str,
        trace: &AuthenticatedAdapterExecutionTrace,
    ) -> Result<(), AuthenticatedDeliveryExecutionError> {
        let json = serde_json::to_string(&PersistedAuthenticatedAdapterExecutionTrace::from(trace))
            .map_err(|_error| AuthenticatedDeliveryExecutionError::IntegrityRejected)?;
        self.connection
            .execute(
                "INSERT INTO authenticated_delivery_adapter_traces_v1 (issuer_key_id,nonce,trace_json) VALUES (?1,?2,?3)",
                params![issuer_key_id, nonce, json],
            )
            .map_err(|_error| AuthenticatedDeliveryExecutionError::StorageUnavailable)?;
        Ok(())
    }
}
