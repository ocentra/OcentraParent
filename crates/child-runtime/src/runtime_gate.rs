use ocentra_child_enforcement_core::{
    evaluate_enforcement_action, EnforcementActionDecision, EnforcementActionInput,
    EnforcementAdapterExecutionState,
};
use ocentra_entitlement_core::{
    evaluate_entitlement_capability, EntitlementCapabilityAccessState, EntitlementCapabilityInput,
    EntitlementDecision,
};
use ocentra_family_identity_core::{
    authorize_child_device_scope, DeviceScopeAuthorizationState, DeviceScopeDecision,
    DeviceScopeInput,
};
use ocentra_provisioning_core::{
    evaluate_provisioning_readiness, ChildRuntimeReadinessState, ProvisioningReadinessDecision,
    ProvisioningReadinessInput,
};
use ocentra_remote_access_core::{
    evaluate_remote_access_session, RemoteAccessSessionAuthorizationState,
    RemoteAccessSessionDecision, RemoteAccessSessionRequest,
};
use ocentra_storage_custody_core::{
    evaluate_storage_custody, RemoteUploadState, StorageCustodyDecision, StorageCustodyInput,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildRuntimeStartState {
    Allowed,
    Blocked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildRuntimeManualReviewState {
    Required,
    NotRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildRuntimePreflightInput {
    pub device_scope_input: DeviceScopeInput,
    pub provisioning_input: ProvisioningReadinessInput,
    pub entitlement_input: EntitlementCapabilityInput,
    pub storage_custody_input: StorageCustodyInput,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildRuntimePreflightDecision {
    pub device_scope_decision: DeviceScopeDecision,
    pub provisioning_decision: ProvisioningReadinessDecision,
    pub entitlement_decision: EntitlementDecision,
    pub storage_custody_decision: StorageCustodyDecision,
    pub runtime_start_state: ChildRuntimeStartState,
    pub manual_review_state: ChildRuntimeManualReviewState,
}

pub fn evaluate_child_runtime_preflight(
    input: ChildRuntimePreflightInput,
) -> ChildRuntimePreflightDecision {
    let device_scope_decision = authorize_child_device_scope(input.device_scope_input);
    let provisioning_decision = evaluate_provisioning_readiness(input.provisioning_input);
    let entitlement_decision = evaluate_entitlement_capability(input.entitlement_input);
    let storage_custody_decision = evaluate_storage_custody(input.storage_custody_input);

    let runtime_allowed =
        device_scope_decision.authorization_state == DeviceScopeAuthorizationState::Authorized
            && provisioning_decision.child_runtime_readiness_state
                == ChildRuntimeReadinessState::Ready
            && entitlement_decision.access_state == EntitlementCapabilityAccessState::Allowed;

    ChildRuntimePreflightDecision {
        device_scope_decision,
        provisioning_decision,
        entitlement_decision,
        storage_custody_decision,
        runtime_start_state: if runtime_allowed {
            ChildRuntimeStartState::Allowed
        } else {
            ChildRuntimeStartState::Blocked
        },
        manual_review_state: if runtime_allowed {
            ChildRuntimeManualReviewState::NotRequired
        } else {
            ChildRuntimeManualReviewState::Required
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildRuntimeRemoteAccessDecision {
    pub session_decision: RemoteAccessSessionDecision,
    pub runtime_start_state: ChildRuntimeStartState,
}

pub fn evaluate_child_runtime_remote_access(
    request: RemoteAccessSessionRequest,
) -> ChildRuntimeRemoteAccessDecision {
    let session_decision = evaluate_remote_access_session(request);

    ChildRuntimeRemoteAccessDecision {
        runtime_start_state: if session_decision.authorization_state
            == RemoteAccessSessionAuthorizationState::Allowed
        {
            ChildRuntimeStartState::Allowed
        } else {
            ChildRuntimeStartState::Blocked
        },
        session_decision,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildRuntimeEnforcementDecision {
    pub action_decision: EnforcementActionDecision,
    pub runtime_start_state: ChildRuntimeStartState,
}

pub fn evaluate_child_runtime_enforcement(
    input: EnforcementActionInput,
) -> ChildRuntimeEnforcementDecision {
    let action_decision = evaluate_enforcement_action(input);

    ChildRuntimeEnforcementDecision {
        runtime_start_state: if action_decision.adapter_execution_state
            == EnforcementAdapterExecutionState::Execute
        {
            ChildRuntimeStartState::Allowed
        } else {
            ChildRuntimeStartState::Blocked
        },
        action_decision,
    }
}

pub fn child_runtime_remote_upload_allowed(
    decision: &ChildRuntimePreflightDecision,
) -> bool {
    decision.storage_custody_decision.remote_upload_state == RemoteUploadState::Allowed
}
