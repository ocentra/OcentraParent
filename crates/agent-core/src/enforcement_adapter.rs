use ocentra_parent_agent_protocol::{
    constants::enforcement as enforcement_constants, policy_constants, EnforcementAdapterKind,
    EnforcementAdapterResultCode, EnforcementCapabilityState, EnforcementCapabilityStatus,
    EnforcementDependencyState, EnforcementMode, EnforcementPermissionState,
    EnforcementResultStatus, EnforcementRollbackState, EnforcementUnavailableReason,
    ParentPlatform,
};

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

pub fn terminate_owned_process(
    target: OwnedProcessTerminationTarget,
    completed_at: &str,
) -> EnforcementAdapterOutcome {
    terminate_owned_process_impl(target, completed_at)
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
) -> EnforcementAdapterOutcome {
    use sysinfo::{Pid, ProcessesToUpdate, System};

    let mut system = System::new();
    let pid = Pid::from_u32(target.pid);
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
    let Some(process) = system.process(pid) else {
        return adapter_outcome(
            EnforcementResultStatus::ActuallyEnforced,
            EnforcementAdapterResultCode::ProcessAlreadyExited,
            Some(completed_at.to_string()),
            None,
            None,
            None,
            EnforcementRollbackState::NotRequired,
        );
    };

    if process.name().to_string_lossy() != target.expected_process_name {
        return adapter_outcome(
            EnforcementResultStatus::Failed,
            EnforcementAdapterResultCode::AdapterFailed,
            Some(completed_at.to_string()),
            None,
            Some(enforcement_constants::REJECTION_TARGET_MISMATCH.to_string()),
            None,
            EnforcementRollbackState::Failed,
        );
    }

    if process.kill() {
        adapter_outcome(
            EnforcementResultStatus::ActuallyEnforced,
            EnforcementAdapterResultCode::ProcessTerminated,
            Some(completed_at.to_string()),
            None,
            None,
            None,
            EnforcementRollbackState::NotRequired,
        )
    } else {
        adapter_outcome(
            EnforcementResultStatus::Failed,
            EnforcementAdapterResultCode::AdapterFailed,
            Some(completed_at.to_string()),
            None,
            Some(enforcement_constants::ADAPTER_FAILED.to_string()),
            None,
            EnforcementRollbackState::Failed,
        )
    }
}

#[cfg(not(windows))]
fn terminate_owned_process_impl(
    _target: OwnedProcessTerminationTarget,
    completed_at: &str,
) -> EnforcementAdapterOutcome {
    unavailable_adapter_outcome(
        EnforcementUnavailableReason::UnsupportedPlatform,
        completed_at,
    )
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
        | EnforcementUnavailableReason::AdapterUnavailable => {
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
