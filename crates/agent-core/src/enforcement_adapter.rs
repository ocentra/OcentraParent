use std::path::Path;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::Signature;
use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterKind, EnforcementAdapterResultCode, EnforcementCapabilityState,
    EnforcementCapabilityStatus, EnforcementDependencyState, EnforcementMode,
    EnforcementPermissionState, EnforcementResultStatus, EnforcementRollbackState,
    EnforcementUnavailableReason, ParentPlatform,
};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_schema::{
    authenticated_delivery_grant::{
        authenticated_delivery_grant_audit_fingerprint, AuthenticatedDeliveryGrant,
    },
    authenticated_delivery_managed_process::AuthenticatedManagedProcessTargetBinding,
};
use sha2::{Digest, Sha256};

use crate::activity_store::ActivityStore;
use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantTrustedIssuer;

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

impl AuthenticatedOwnedProcessTerminationTarget {
    fn from_local_binding(
        binding: &AuthenticatedManagedProcessTargetBinding,
        process_id: u32,
        process_name: String,
        executable_path_ref: String,
        process_start_time: u64,
    ) -> Self {
        Self {
            pid: process_id,
            expected_process_name: process_name,
            grant_fingerprint: binding.grant_fingerprint.clone(),
            issuer_key_id: binding.issuer_key_id.clone(),
            issuer_actor_id: binding.issuer_actor_id.clone(),
            household_id: binding.household_id.clone(),
            parent_device_id: binding.parent_device_id.clone(),
            child_profile_id: binding.child_profile_id.clone(),
            target_device_id: binding.target_device_id.clone(),
            policy_decision_id: binding.policy_decision_id.clone(),
            policy_version: binding.policy_version.clone(),
            action_id: binding.action_id.clone(),
            capability_id: binding.capability_id.clone(),
            managed_process_identity: binding.managed_process_identity.clone(),
            expected_executable_path_ref: executable_path_ref,
            process_start_time,
        }
    }

    pub(crate) fn raw_target(&self) -> OwnedProcessTerminationTarget {
        OwnedProcessTerminationTarget {
            pid: self.pid,
            expected_process_name: self.expected_process_name.clone(),
        }
    }

    pub(crate) fn grant_fingerprint(&self) -> &str {
        &self.grant_fingerprint
    }

    pub(crate) fn issuer_key_id(&self) -> &str {
        &self.issuer_key_id
    }

    pub(crate) fn issuer_actor_id(&self) -> &str {
        &self.issuer_actor_id
    }

    pub(crate) fn household_id(&self) -> &str {
        &self.household_id
    }

    pub(crate) fn parent_device_id(&self) -> &str {
        &self.parent_device_id
    }

    pub(crate) fn child_profile_id(&self) -> &str {
        &self.child_profile_id
    }

    pub(crate) fn target_device_id(&self) -> &str {
        &self.target_device_id
    }

    pub(crate) fn policy_decision_id(&self) -> &str {
        &self.policy_decision_id
    }

    pub(crate) fn policy_version(&self) -> &str {
        &self.policy_version
    }

    pub(crate) fn action_id(&self) -> &str {
        &self.action_id
    }

    pub(crate) fn capability_id(&self) -> &str {
        &self.capability_id
    }

    pub(crate) fn pid(&self) -> u32 {
        self.pid
    }

    pub(crate) fn expected_process_name(&self) -> &str {
        &self.expected_process_name
    }

    pub(crate) fn expected_executable_path(&self) -> &str {
        &self.expected_executable_path_ref
    }

    pub(crate) fn process_start_time(&self) -> u64 {
        self.process_start_time
    }

    pub(crate) fn managed_process_identity(&self) -> &str {
        &self.managed_process_identity
    }
}

pub(crate) fn resolve_authenticated_managed_process_target(
    grant: &AuthenticatedDeliveryGrant,
    binding: &AuthenticatedManagedProcessTargetBinding,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
    activity_store_path: impl AsRef<Path>,
) -> Result<AuthenticatedOwnedProcessTerminationTarget, ()> {
    if grant.validate_shape().is_err()
        || binding.validate_shape().is_err()
        || binding.issuer_key_id != trusted_issuer.key_id
        || binding.issuer_key_id != grant.issuer_key_id
        || binding.grant_fingerprint != authenticated_delivery_grant_audit_fingerprint(grant)
        || binding.nonce != grant.nonce
        || binding.issuer_actor_id != grant.issuer_actor_id
        || binding.household_id != grant.household_id
        || binding.parent_device_id != grant.parent_device_id
        || binding.child_profile_id != grant.child_profile_id
        || binding.target_device_id != grant.target_device_id
        || binding.policy_decision_id != grant.policy_decision_id
        || binding.policy_version != grant.policy_version
        || binding.action_id != grant.action_id
        || binding.capability_id != grant.capability_id
    {
        return Err(());
    }
    let signature = Signature::from_slice(&binding.signature).map_err(|_error| ())?;
    trusted_issuer
        .verifying_key
        .verify_strict(&binding.signing_bytes(), &signature)
        .map_err(|_error| ())?;
    let store = ActivityStore::open(activity_store_path).map_err(|_error| ())?;
    let model = store
        .app_game_service_read_model(
            ocentra_parent_agent_protocol::constants::activity_store::DEFAULT_RECENT_LIMIT,
            ocentra_parent_agent_protocol::constants::enforcement::APP_GAME_RUNTIME_EVIDENCE_GENERATED_AT,
        )
        .map_err(|_error| ())?;
    let runtime = model
        .running_now_rows
        .iter()
        .find(|row| {
            row.process_identity == binding.managed_process_identity
                && row.launcher_ref.is_some()
                && matches!(
                row.classification_state.as_str(),
                ocentra_parent_agent_protocol::app_game::APP_GAME_CLASSIFICATION_KNOWN_APP
                    | ocentra_parent_agent_protocol::app_game::APP_GAME_CLASSIFICATION_KNOWN_GAME
            ) && row.executable_path_ref.is_some()
                && row.started_at.is_some()
        })
        .ok_or(())?;
    let process_start_time =
        chrono::DateTime::parse_from_rfc3339(runtime.started_at.as_deref().ok_or(())?)
            .map_err(|_error| ())?
            .timestamp()
            .try_into()
            .map_err(|_error| ())?;
    let summary = store
        .app_game_session_summaries(
            ocentra_parent_agent_protocol::constants::activity_store::DEFAULT_RECENT_LIMIT,
        )
        .map_err(|_error| ())?
        .into_iter()
        .find(|summary| {
            summary.primary_process_identity == runtime.process_identity
                && summary.launcher_ref.is_some()
                && summary.last_observed_at >= runtime.observed_at
        })
        .ok_or(())?;
    if summary.primary_process_identity != binding.managed_process_identity {
        return Err(());
    }
    Ok(
        AuthenticatedOwnedProcessTerminationTarget::from_local_binding(
            binding,
            u32::try_from(runtime.process_id).map_err(|_error| ())?,
            runtime.process_name.clone(),
            runtime.executable_path_ref.clone().ok_or(())?,
            process_start_time,
        ),
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
    use sysinfo::{Pid, ProcessesToUpdate, System};

    let OwnedProcessTerminationTarget {
        pid,
        expected_process_name,
    } = target;

    let mut system = System::new();
    let pid = Pid::from_u32(pid);
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
    let Some(process) = system.process(pid) else {
        return (
            adapter_outcome(
                EnforcementResultStatus::NoOp,
                EnforcementAdapterResultCode::ProcessAlreadyExited,
                Some(completed_at.to_string()),
                None,
                None,
                None,
                EnforcementRollbackState::NotRequired,
            ),
            AdapterObservedProcessIdentity {
                pid: None,
                process_name: None,
                executable_path: None,
                process_start_time: None,
                owner_sid: None,
            },
        );
    };

    let executable_path = process
        .exe()
        .and_then(|path| std::fs::canonicalize(path).ok())
        .map(|path| {
            let digest = Sha256::digest(path.to_string_lossy().as_bytes());
            let mut value = String::from(
                ocentra_parent_agent_protocol::app_game::APP_GAME_EXECUTABLE_PATH_REF_PREFIX,
            );
            value.push_str(&URL_SAFE_NO_PAD.encode(digest));
            value
        });
    let process_start_time = Some(process.start_time());
    let observed_process = AdapterObservedProcessIdentity {
        pid: Some(pid.as_u32()),
        process_name: Some(process.name().to_string_lossy().into_owned()),
        executable_path: executable_path.clone(),
        process_start_time,
        owner_sid: None,
    };

    if let Some((expected_executable_path, expected_start_time)) = expected_identity {
        if executable_path.is_none() || process.start_time() == 0 {
            return (
                unavailable_adapter_outcome(
                    EnforcementUnavailableReason::ManualRequired,
                    completed_at,
                ),
                observed_process,
            );
        }
        if executable_path.as_deref() != Some(expected_executable_path)
            || process.start_time() != expected_start_time
        {
            return (
                adapter_outcome(
                    EnforcementResultStatus::Failed,
                    EnforcementAdapterResultCode::AdapterFailed,
                    Some(completed_at.to_string()),
                    None,
                    Some(enforcement_constants::REJECTION_TARGET_MISMATCH.to_string()),
                    None,
                    EnforcementRollbackState::Failed,
                ),
                observed_process,
            );
        }
    }

    if process.name().to_string_lossy() != expected_process_name {
        return (
            adapter_outcome(
                EnforcementResultStatus::Failed,
                EnforcementAdapterResultCode::AdapterFailed,
                Some(completed_at.to_string()),
                None,
                Some(enforcement_constants::REJECTION_TARGET_MISMATCH.to_string()),
                None,
                EnforcementRollbackState::Failed,
            ),
            observed_process,
        );
    }

    if process.kill() {
        (
            adapter_outcome(
                EnforcementResultStatus::ActuallyEnforced,
                EnforcementAdapterResultCode::ProcessTerminated,
                Some(completed_at.to_string()),
                None,
                None,
                None,
                EnforcementRollbackState::NotRequired,
            ),
            observed_process,
        )
    } else {
        (
            adapter_outcome(
                EnforcementResultStatus::Failed,
                EnforcementAdapterResultCode::AdapterFailed,
                Some(completed_at.to_string()),
                None,
                Some(enforcement_constants::ADAPTER_FAILED.to_string()),
                None,
                EnforcementRollbackState::Failed,
            ),
            observed_process,
        )
    }
}

#[cfg(not(windows))]
fn terminate_owned_process_impl(
    _target: OwnedProcessTerminationTarget,
    completed_at: &str,
    _expected_identity: Option<(&str, u64)>,
) -> (EnforcementAdapterOutcome, AdapterObservedProcessIdentity) {
    (
        unavailable_adapter_outcome(
            EnforcementUnavailableReason::UnsupportedPlatform,
            completed_at,
        ),
        AdapterObservedProcessIdentity {
            pid: None,
            process_name: None,
            executable_path: None,
            process_start_time: None,
            owner_sid: None,
        },
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
