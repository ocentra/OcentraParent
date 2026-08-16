#![forbid(unsafe_code)]

//! Durable execution intent/receipt state for authenticated delivery.
//!
//! This deliberately records an intent before any platform effect.  An OS
//! adapter is not transactionally composable with SQLite, so recovery must
//! resume a persisted intent rather than pretending the effect was atomic.

use std::{path::Path, time::Duration};

use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    authenticated_delivery_grant::AuthenticatedDeliveryGrantTrustedIssuer,
    enforcement_adapter::{
        resolve_authenticated_managed_process_target, terminate_authenticated_owned_process,
        terminate_owned_process, AuthenticatedOwnedProcessTerminationTarget,
        OwnedProcessTerminationTarget,
    },
};
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

impl AuthenticatedAdapterExecutionTrace {
    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    pub fn grant_fingerprint(&self) -> &str {
        &self.grant_fingerprint
    }

    pub fn issuer_key_id(&self) -> &str {
        &self.issuer_key_id
    }

    pub fn nonce_digest(&self) -> &str {
        &self.nonce_digest
    }

    pub fn correlation_id(&self) -> &str {
        &self.correlation_id
    }

    pub fn issuer_actor_id(&self) -> &str {
        &self.issuer_actor_id
    }

    pub fn household_id(&self) -> &str {
        &self.household_id
    }

    pub fn parent_device_id(&self) -> &str {
        &self.parent_device_id
    }

    pub fn child_profile_id(&self) -> &str {
        &self.child_profile_id
    }

    pub fn target_device_id(&self) -> &str {
        &self.target_device_id
    }

    pub fn policy_decision_id(&self) -> &str {
        &self.policy_decision_id
    }

    pub fn policy_version(&self) -> &str {
        &self.policy_version
    }

    pub fn action_id(&self) -> &str {
        &self.action_id
    }

    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }

    pub fn process_id(&self) -> u32 {
        self.process_id
    }

    pub fn expected_process_name(&self) -> &str {
        &self.expected_process_name
    }

    pub fn expected_executable_path(&self) -> &str {
        &self.expected_executable_path_ref
    }

    pub fn process_start_time(&self) -> u64 {
        self.process_start_time
    }

    pub fn managed_process_identity(&self) -> &str {
        &self.managed_process_identity
    }

    pub fn observed_process_id(&self) -> Option<u32> {
        self.observed_process_id
    }

    pub fn observed_process_name(&self) -> Option<&str> {
        self.observed_process_name.as_deref()
    }

    pub fn observed_executable_path(&self) -> Option<&str> {
        self.observed_executable_path_ref.as_deref()
    }

    pub fn observed_process_start_time(&self) -> Option<u64> {
        self.observed_process_start_time
    }

    pub fn adapter_result(&self) -> &str {
        &self.adapter_result
    }

    pub fn adapter_status(&self) -> &str {
        &self.adapter_status
    }

    pub fn completed_at(&self) -> Option<&str> {
        self.completed_at.as_deref()
    }

    pub fn rollback_required(&self) -> bool {
        self.rollback_required
    }

    pub fn rollback_state(&self) -> &str {
        &self.rollback_state
    }
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

impl From<PersistedAuthenticatedAdapterExecutionTrace> for AuthenticatedAdapterExecutionTrace {
    fn from(value: PersistedAuthenticatedAdapterExecutionTrace) -> Self {
        Self {
            trace_id: value.trace_id,
            grant_fingerprint: value.grant_fingerprint,
            issuer_key_id: value.issuer_key_id,
            nonce_digest: value.nonce_digest,
            correlation_id: value.correlation_id,
            issuer_actor_id: value.issuer_actor_id,
            household_id: value.household_id,
            parent_device_id: value.parent_device_id,
            child_profile_id: value.child_profile_id,
            target_device_id: value.target_device_id,
            policy_decision_id: value.policy_decision_id,
            policy_version: value.policy_version,
            action_id: value.action_id,
            capability_id: value.capability_id,
            managed_process_identity: value.managed_process_identity,
            process_id: value.process_id,
            expected_process_name: value.expected_process_name,
            expected_executable_path_ref: value.expected_executable_path_ref,
            process_start_time: value.process_start_time,
            observed_process_id: value.observed_process_id,
            observed_process_name: value.observed_process_name,
            observed_executable_path_ref: value.observed_executable_path_ref,
            observed_process_start_time: value.observed_process_start_time,
            adapter_result: value.adapter_result,
            adapter_status: value.adapter_status,
            completed_at: value.completed_at,
            rollback_required: value.rollback_required,
            rollback_state: value.rollback_state,
        }
    }
}

impl From<&AuthenticatedAdapterExecutionTrace> for PersistedAuthenticatedAdapterExecutionTrace {
    fn from(value: &AuthenticatedAdapterExecutionTrace) -> Self {
        Self {
            trace_id: value.trace_id.clone(),
            grant_fingerprint: value.grant_fingerprint.clone(),
            issuer_key_id: value.issuer_key_id.clone(),
            nonce_digest: value.nonce_digest.clone(),
            correlation_id: value.correlation_id.clone(),
            issuer_actor_id: value.issuer_actor_id.clone(),
            household_id: value.household_id.clone(),
            parent_device_id: value.parent_device_id.clone(),
            child_profile_id: value.child_profile_id.clone(),
            target_device_id: value.target_device_id.clone(),
            policy_decision_id: value.policy_decision_id.clone(),
            policy_version: value.policy_version.clone(),
            action_id: value.action_id.clone(),
            capability_id: value.capability_id.clone(),
            managed_process_identity: value.managed_process_identity.clone(),
            process_id: value.process_id,
            expected_process_name: value.expected_process_name.clone(),
            expected_executable_path_ref: value.expected_executable_path_ref.clone(),
            process_start_time: value.process_start_time,
            observed_process_id: value.observed_process_id,
            observed_process_name: value.observed_process_name.clone(),
            observed_executable_path_ref: value.observed_executable_path_ref.clone(),
            observed_process_start_time: value.observed_process_start_time,
            adapter_result: value.adapter_result.clone(),
            adapter_status: value.adapter_status.clone(),
            completed_at: value.completed_at.clone(),
            rollback_required: value.rollback_required,
            rollback_state: value.rollback_state.clone(),
        }
    }
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
        let trace = trace_for_execution(target, correlation_id, nonce, &execution);
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

fn trace_for_execution(
    target: &AuthenticatedOwnedProcessTerminationTarget,
    correlation_id: &str,
    nonce: &str,
    execution: &crate::enforcement_adapter::AuthenticatedAdapterExecution,
) -> AuthenticatedAdapterExecutionTrace {
    let nonce_digest = digest(nonce);
    let mut trace_digest = Sha256::new();
    trace_digest.update(b"ocentra.authenticated-adapter-execution.v1\0");
    trace_digest.update(target.grant_fingerprint().as_bytes());
    trace_digest.update(nonce_digest.as_bytes());
    trace_digest.update(b"windows-process-control");
    let trace_id = format!("{:x}", trace_digest.finalize());
    AuthenticatedAdapterExecutionTrace {
        trace_id,
        grant_fingerprint: target.grant_fingerprint().to_owned(),
        issuer_key_id: target.issuer_key_id().to_owned(),
        nonce_digest,
        correlation_id: correlation_id.to_owned(),
        issuer_actor_id: target.issuer_actor_id().to_owned(),
        household_id: target.household_id().to_owned(),
        parent_device_id: target.parent_device_id().to_owned(),
        child_profile_id: target.child_profile_id().to_owned(),
        target_device_id: target.target_device_id().to_owned(),
        policy_decision_id: target.policy_decision_id().to_owned(),
        policy_version: target.policy_version().to_owned(),
        action_id: target.action_id().to_owned(),
        capability_id: target.capability_id().to_owned(),
        managed_process_identity: target.managed_process_identity().to_owned(),
        process_id: target.pid(),
        expected_process_name: target.expected_process_name().to_owned(),
        expected_executable_path_ref: target.expected_executable_path().to_owned(),
        process_start_time: target.process_start_time(),
        observed_process_id: execution.observed_process.pid,
        observed_process_name: execution.observed_process.process_name.clone(),
        observed_executable_path_ref: execution.observed_process.executable_path.clone(),
        observed_process_start_time: execution.observed_process.process_start_time,
        adapter_result: format!("{:?}", execution.outcome.adapter_result_code),
        adapter_status: format!("{:?}", execution.outcome.status),
        completed_at: Some(execution.observed_at.clone()),
        rollback_required: !matches!(
            execution.outcome.rollback_state,
            ocentra_parent_agent_protocol::enforcement::EnforcementRollbackState::NotRequired
        ),
        rollback_state: format!("{:?}", execution.outcome.rollback_state),
    }
}

fn digest(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}
