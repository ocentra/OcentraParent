use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementAdapterKind, EnforcementAdapterResultCode,
    EnforcementCapabilityState, EnforcementCapabilityStatus, EnforcementDependencyState,
    EnforcementMode, EnforcementPermissionState, EnforcementResultStatus, EnforcementRollbackState,
    EnforcementUnavailableReason, ParentPlatform,
};
use ocentra_parent_agent_protocol::policy_constants;

use crate::enforcement_adapter::{
    terminate_owned_process, unavailable_adapter_outcome, EnforcementAdapterOutcome,
    OwnedProcessTerminationTarget,
};

#[path = "enforcement_app_time_limit_platform.rs"]
mod enforcement_app_time_limit_platform;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppTimeLimitAdapterTarget {
    pub pid: u32,
    pub expected_process_name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppTimeLimitTargetRejection {
    UnsupportedCapability,
    ProcessIdRequired,
    TargetMismatch,
}

impl AppTimeLimitTargetRejection {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::UnsupportedCapability => enforcement_constants::REJECTION_UNSUPPORTED_CAPABILITY,
            Self::ProcessIdRequired => enforcement_constants::REJECTION_PROCESS_ID_REQUIRED,
            Self::TargetMismatch => enforcement_constants::REJECTION_TARGET_MISMATCH,
        }
    }
}

pub fn app_time_limit_capability(checked_at: &str) -> EnforcementCapabilityStatus {
    #[cfg(windows)]
    {
        EnforcementCapabilityStatus {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            platform: ParentPlatform::Windows,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            capability_state: EnforcementCapabilityState::Supported,
            permission_state: EnforcementPermissionState::NotRequired,
            dependency_state: EnforcementDependencyState::Installed,
            supported_actions: vec![EnforcementMode::TimeLimit],
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

pub fn app_time_limit_target_from_action(
    action: &EnforcementAction,
    process_id: Option<u32>,
) -> Result<AppTimeLimitAdapterTarget, AppTimeLimitTargetRejection> {
    if action.adapter_kind != EnforcementAdapterKind::ProcessControl
        || action.mode != EnforcementMode::TimeLimit
        || action.expires_at.is_none()
    {
        return Err(AppTimeLimitTargetRejection::UnsupportedCapability);
    }
    if !matches!(
        action.target.target_type,
        PolicyTargetType::App | PolicyTargetType::Process
    ) || action.target.target_value.trim().is_empty()
    {
        return Err(AppTimeLimitTargetRejection::TargetMismatch);
    }
    let pid = process_id.ok_or(AppTimeLimitTargetRejection::ProcessIdRequired)?;

    Ok(AppTimeLimitAdapterTarget {
        pid,
        expected_process_name: action.target.target_value.clone(),
    })
}

pub fn expire_app_time_limit_for_owned_process(
    target: AppTimeLimitAdapterTarget,
    completed_at: &str,
) -> EnforcementAdapterOutcome {
    let outcome = terminate_owned_process(
        OwnedProcessTerminationTarget {
            pid: target.pid,
            expected_process_name: target.expected_process_name,
        },
        completed_at,
    );

    time_limit_outcome_from_process_outcome(outcome, completed_at)
}

pub fn unavailable_app_time_limit_outcome(
    unavailable_reason: EnforcementUnavailableReason,
    completed_at: &str,
) -> EnforcementAdapterOutcome {
    unavailable_adapter_outcome(unavailable_reason, completed_at)
}

fn time_limit_outcome_from_process_outcome(
    outcome: EnforcementAdapterOutcome,
    completed_at: &str,
) -> EnforcementAdapterOutcome {
    if outcome.status == EnforcementResultStatus::ActuallyEnforced
        || (outcome.status == EnforcementResultStatus::NoOp
            && outcome.adapter_result_code == EnforcementAdapterResultCode::ProcessAlreadyExited)
    {
        return EnforcementAdapterOutcome {
            status: EnforcementResultStatus::Expired,
            adapter_result_code: outcome.adapter_result_code,
            completed_at: Some(completed_at.to_string()),
            unavailable_reason: None,
            failed_reason: None,
            rollback_token: outcome.rollback_token,
            rollback_state: EnforcementRollbackState::NotRequired,
        };
    }

    outcome
}

#[cfg(not(windows))]
fn current_platform() -> ParentPlatform {
    enforcement_app_time_limit_platform::current_platform()
}
