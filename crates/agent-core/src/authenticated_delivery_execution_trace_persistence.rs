use sha2::{Digest, Sha256};

use super::{
    AuthenticatedAdapterExecutionTrace, AuthenticatedOwnedProcessTerminationTarget,
    PersistedAuthenticatedAdapterExecutionTrace,
};

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

pub(super) fn trace_for_execution(
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
