use super::{
    adapter_outcome, unavailable_adapter_outcome, AdapterObservedProcessIdentity,
    EnforcementAdapterOutcome, OwnedProcessTerminationTarget,
};
use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterResultCode, EnforcementResultStatus, EnforcementRollbackState,
    EnforcementUnavailableReason,
};

#[cfg(windows)]
pub(super) fn terminate(
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
        return already_exited(completed_at);
    };
    let (executable_path, observed_process) = observe_process(process, pid);
    if let Some(outcome) = identity_outcome(
        expected_identity,
        executable_path.as_deref(),
        process.start_time(),
        completed_at,
        &observed_process,
    ) {
        return outcome;
    }
    if process.name().to_string_lossy() != expected_process_name {
        return target_mismatch(completed_at, observed_process);
    }
    (termination_outcome(process, completed_at), observed_process)
}

#[cfg(windows)]
fn already_exited(
    completed_at: &str,
) -> (EnforcementAdapterOutcome, AdapterObservedProcessIdentity) {
    (
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
    )
}

#[cfg(windows)]
fn observe_process(
    process: &sysinfo::Process,
    pid: sysinfo::Pid,
) -> (Option<String>, AdapterObservedProcessIdentity) {
    let executable_path = process
        .exe()
        .and_then(|path| std::fs::canonicalize(path).ok())
        .map(|path| {
            use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
            use sha2::{Digest, Sha256};

            let digest = Sha256::digest(path.to_string_lossy().as_bytes());
            let mut value = String::from(
                ocentra_parent_agent_protocol::app_game::APP_GAME_EXECUTABLE_PATH_REF_PREFIX,
            );
            value.push_str(&URL_SAFE_NO_PAD.encode(digest));
            value
        });
    let observed_process = AdapterObservedProcessIdentity {
        pid: Some(pid.as_u32()),
        process_name: Some(process.name().to_string_lossy().into_owned()),
        executable_path: executable_path.clone(),
        process_start_time: Some(process.start_time()),
        owner_sid: None,
    };
    (executable_path, observed_process)
}

#[cfg(windows)]
fn identity_outcome(
    expected_identity: Option<(&str, u64)>,
    executable_path: Option<&str>,
    process_start_time: u64,
    completed_at: &str,
    observed_process: &AdapterObservedProcessIdentity,
) -> Option<(EnforcementAdapterOutcome, AdapterObservedProcessIdentity)> {
    let Some((expected_executable_path, expected_start_time)) = expected_identity else {
        return None;
    };
    if executable_path.is_none() || process_start_time == 0 {
        return Some((
            unavailable_adapter_outcome(EnforcementUnavailableReason::ManualRequired, completed_at),
            observed_process.clone(),
        ));
    }
    if executable_path != Some(expected_executable_path)
        || process_start_time != expected_start_time
    {
        return Some(target_mismatch(completed_at, observed_process.clone()));
    }
    None
}

#[cfg(windows)]
fn target_mismatch(
    completed_at: &str,
    observed_process: AdapterObservedProcessIdentity,
) -> (EnforcementAdapterOutcome, AdapterObservedProcessIdentity) {
    (
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
    )
}

#[cfg(windows)]
fn termination_outcome(
    process: &sysinfo::Process,
    completed_at: &str,
) -> EnforcementAdapterOutcome {
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
pub(super) fn terminate(
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
