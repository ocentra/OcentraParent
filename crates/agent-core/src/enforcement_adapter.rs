use std::path::Path;

use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantTrustedIssuer;
use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterKind, EnforcementAdapterResultCode, EnforcementCapabilityState,
    EnforcementCapabilityStatus, EnforcementDependencyState, EnforcementMode,
    EnforcementPermissionState, EnforcementResultStatus, EnforcementRollbackState,
    EnforcementUnavailableReason, ParentPlatform,
};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_schema::{
    authenticated_delivery_grant::AuthenticatedDeliveryGrant,
    authenticated_delivery_managed_process::AuthenticatedManagedProcessTargetBinding,
};

#[path = "enforcement_adapter_authenticated_target.rs"]
mod enforcement_adapter_authenticated_target;
#[path = "enforcement_adapter_process_control.rs"]
mod enforcement_adapter_process_control;
#[path = "enforcement_adapter_target_resolution.rs"]
mod enforcement_adapter_target_resolution;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementAdapterOutcome {
    pub status: EnforcementResultStatus,
    pub adapter_result_code: EnforcementAdapterResultCode,
    pub completed_at: Option<String>,
    pub unavailable_reason: Option<String>,
    pub failed_reason: Option<String>,
    pub rollback_token: Option<String>,
    pub rollback_state: EnforcementRollbackState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedProcessTerminationTarget {
    pub pid: u32,
    pub expected_process_name: String,
}

/// A process target is only constructible inside the authenticated delivery
/// verifier.  Public callers can still submit `OwnedProcessTerminationTarget`
/// as raw adapter evidence, but that type cannot authorize a policy receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticatedOwnedProcessTerminationTarget {
    pid: u32,
    expected_process_name: String,
    grant_fingerprint: String,
    issuer_key_id: String,
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
    expected_executable_path_ref: String,
    process_start_time: u64,
}

pub(crate) fn resolve_authenticated_managed_process_target(
    grant: &AuthenticatedDeliveryGrant,
    binding: &AuthenticatedManagedProcessTargetBinding,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
    activity_store_path: impl AsRef<Path>,
) -> Result<AuthenticatedOwnedProcessTerminationTarget, ()> {
    enforcement_adapter_target_resolution::resolve(
        grant,
        binding,
        trusted_issuer,
        activity_store_path,
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdapterObservedProcessIdentity {
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub executable_path: Option<String>,
    pub process_start_time: Option<u64>,
    pub owner_sid: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AuthenticatedAdapterExecution {
    pub outcome: EnforcementAdapterOutcome,
    pub observed_process: AdapterObservedProcessIdentity,
    pub observed_at: String,
}

pub fn process_control_capability(checked_at: &str) -> EnforcementCapabilityStatus {
    #[cfg(windows)]
    {
        EnforcementCapabilityStatus {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            platform: ParentPlatform::Windows,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            capability_state: EnforcementCapabilityState::Supported,
            permission_state: EnforcementPermissionState::NotRequired,
            dependency_state: EnforcementDependencyState::Installed,
            supported_actions: vec![
                EnforcementMode::TerminateProcess,
                EnforcementMode::TimeLimit,
            ],
            degraded_reason: None,
            last_checked_at: checked_at.to_string(),
        }
    }

    #[cfg(not(windows))]
    {
        EnforcementCapabilityStatus {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            platform: current_platform(),
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            capability_state: EnforcementCapabilityState::Unavailable,
            permission_state: EnforcementPermissionState::NotRequired,
            dependency_state: EnforcementDependencyState::NotRequired,
            supported_actions: Vec::new(),
            degraded_reason: Some(
                enforcement_constants::UNAVAILABLE_UNSUPPORTED_PLATFORM.to_string(),
            ),
            last_checked_at: checked_at.to_string(),
        }
    }
}

pub fn timer_control_capability(checked_at: &str) -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: timer_control_platform(),
        adapter_kind: EnforcementAdapterKind::TimerControl,
        capability_state: EnforcementCapabilityState::Supported,
        permission_state: EnforcementPermissionState::NotRequired,
        dependency_state: EnforcementDependencyState::Installed,
        supported_actions: vec![EnforcementMode::AskParent],
        degraded_reason: None,
        last_checked_at: checked_at.to_string(),
    }
}

pub fn app_block_control_capability(checked_at: &str) -> EnforcementCapabilityStatus {
    manual_required_capability(
        ParentPlatform::Windows,
        EnforcementAdapterKind::ProcessControl,
        vec![EnforcementMode::BlockProcess],
        checked_at,
    )
}

pub fn network_control_capability(checked_at: &str) -> EnforcementCapabilityStatus {
    manual_required_capability(
        ParentPlatform::Windows,
        EnforcementAdapterKind::NetworkControl,
        vec![EnforcementMode::TemporaryBlock],
        checked_at,
    )
}

pub fn managed_browser_control_capability(checked_at: &str) -> EnforcementCapabilityStatus {
    manual_required_capability(
        ParentPlatform::Windows,
        EnforcementAdapterKind::ManagedBrowserControl,
        vec![EnforcementMode::TemporaryBlock],
        checked_at,
    )
}

pub fn terminate_owned_process(
    target: OwnedProcessTerminationTarget,
    completed_at: &str,
) -> EnforcementAdapterOutcome {
    terminate_owned_process_impl(target, completed_at, None).0
}

pub(crate) fn terminate_authenticated_owned_process(
    target: &AuthenticatedOwnedProcessTerminationTarget,
    completed_at: &str,
) -> AuthenticatedAdapterExecution {
    let (outcome, observed_process) = terminate_owned_process_impl(
        target.raw_target(),
        completed_at,
        Some((
            target.expected_executable_path(),
            target.process_start_time(),
        )),
    );
    AuthenticatedAdapterExecution {
        outcome,
        observed_process,
        observed_at: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
    }
}

#[cfg(windows)]
fn manual_required_capability(
    platform: ParentPlatform,
    adapter_kind: EnforcementAdapterKind,
    supported_actions: Vec<EnforcementMode>,
    checked_at: &str,
) -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform,
        adapter_kind,
        capability_state: EnforcementCapabilityState::ManualRequired,
        permission_state: EnforcementPermissionState::Unknown,
        dependency_state: EnforcementDependencyState::Unknown,
        supported_actions,
        degraded_reason: Some(enforcement_constants::UNAVAILABLE_MANUAL_REQUIRED.to_string()),
        last_checked_at: checked_at.to_string(),
    }
}

#[cfg(not(windows))]
fn manual_required_capability(
    _platform: ParentPlatform,
    adapter_kind: EnforcementAdapterKind,
    _supported_actions: Vec<EnforcementMode>,
    checked_at: &str,
) -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: current_platform(),
        adapter_kind,
        capability_state: EnforcementCapabilityState::Unavailable,
        permission_state: EnforcementPermissionState::NotRequired,
        dependency_state: EnforcementDependencyState::NotRequired,
        supported_actions: Vec::new(),
        degraded_reason: Some(enforcement_constants::UNAVAILABLE_UNSUPPORTED_PLATFORM.to_string()),
        last_checked_at: checked_at.to_string(),
    }
}

pub fn unavailable_adapter_outcome(
    unavailable_reason: EnforcementUnavailableReason,
    completed_at: &str,
) -> EnforcementAdapterOutcome {
    adapter_outcome(
        EnforcementResultStatus::Unavailable,
        adapter_result_code_for_unavailable_reason(unavailable_reason),
        Some(completed_at.to_string()),
        Some(unavailable_reason.as_protocol_str().to_string()),
        None,
        None,
        EnforcementRollbackState::Unavailable,
    )
}

#[cfg(windows)]
fn terminate_owned_process_impl(
    target: OwnedProcessTerminationTarget,
    completed_at: &str,
    expected_identity: Option<(&str, u64)>,
) -> (EnforcementAdapterOutcome, AdapterObservedProcessIdentity) {
    enforcement_adapter_process_control::terminate(target, completed_at, expected_identity)
}

#[cfg(not(windows))]
fn terminate_owned_process_impl(
    _target: OwnedProcessTerminationTarget,
    completed_at: &str,
    _expected_identity: Option<(&str, u64)>,
) -> (EnforcementAdapterOutcome, AdapterObservedProcessIdentity) {
    enforcement_adapter_process_control::terminate(_target, completed_at, _expected_identity)
}

fn adapter_outcome(
    status: EnforcementResultStatus,
    adapter_result_code: EnforcementAdapterResultCode,
    completed_at: Option<String>,
    unavailable_reason: Option<String>,
    failed_reason: Option<String>,
    rollback_token: Option<String>,
    rollback_state: EnforcementRollbackState,
) -> EnforcementAdapterOutcome {
    EnforcementAdapterOutcome {
        status,
        adapter_result_code,
        completed_at,
        unavailable_reason,
        failed_reason,
        rollback_token,
        rollback_state,
    }
}

fn adapter_result_code_for_unavailable_reason(
    unavailable_reason: EnforcementUnavailableReason,
) -> EnforcementAdapterResultCode {
    match unavailable_reason {
        EnforcementUnavailableReason::UnsupportedPlatform => {
            EnforcementAdapterResultCode::UnsupportedPlatform
        }
        EnforcementUnavailableReason::AdapterError => EnforcementAdapterResultCode::AdapterFailed,
        EnforcementUnavailableReason::UnsupportedAction
        | EnforcementUnavailableReason::MissingPermission
        | EnforcementUnavailableReason::MissingDependency
        | EnforcementUnavailableReason::AdapterUnavailable
        | EnforcementUnavailableReason::ManualRequired => {
            EnforcementAdapterResultCode::AdapterUnavailable
        }
    }
}

#[cfg(windows)]
fn timer_control_platform() -> ParentPlatform {
    ParentPlatform::Windows
}

#[cfg(not(windows))]
fn timer_control_platform() -> ParentPlatform {
    current_platform()
}

#[cfg(not(windows))]
fn current_platform() -> ParentPlatform {
    match std::env::consts::OS {
        enforcement_constants::PLATFORM_LINUX => ParentPlatform::Linux,
        enforcement_constants::PLATFORM_MACOS => ParentPlatform::Macos,
        enforcement_constants::PLATFORM_ANDROID => ParentPlatform::Android,
        enforcement_constants::PLATFORM_IOS => ParentPlatform::Ios,
        _ => ParentPlatform::Linux,
    }
}
