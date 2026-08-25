use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterKind, EnforcementCapabilityState, EnforcementMode, ParentPlatform,
};
use ocentra_parent_agent_protocol::enforcement_readiness::{
    EnforcementBroadAdapterCapability, EnforcementBroadAdapterReadinessEntry,
    EnforcementBroadOsAdapterReadinessMatrix, EnforcementReadinessProofLevel,
    EnforcementReadinessRuntimeOwner, EnforcementReadinessState,
};
use ocentra_parent_agent_protocol::policy_constants;

use crate::enforcement_adapter::{
    app_block_control_capability, managed_browser_control_capability, network_control_capability,
    process_control_capability,
};
use crate::enforcement_app_time_limit::app_time_limit_capability;

struct ReadinessEntryInput<'a> {
    readiness_id: &'a str,
    capability: EnforcementBroadAdapterCapability,
    platform: ParentPlatform,
    adapter_kind: EnforcementAdapterKind,
    capability_state: EnforcementCapabilityState,
    readiness_state: EnforcementReadinessState,
    proof_level: EnforcementReadinessProofLevel,
    runtime_owner: EnforcementReadinessRuntimeOwner,
    supported_modes: Vec<EnforcementMode>,
    claim_boundary: &'a str,
    fallback_behavior: &'a str,
    required_artifacts: Vec<String>,
    checked_at: &'a str,
}

pub fn broad_os_adapter_readiness(checked_at: &str) -> EnforcementBroadOsAdapterReadinessMatrix {
    EnforcementBroadOsAdapterReadinessMatrix {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        matrix_id: enforcement_constants::READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER.to_string(),
        generated_at: checked_at.to_string(),
        entries: vec![
            owned_process_terminate_readiness(checked_at),
            app_time_limit_readiness(checked_at),
            broad_app_blocking_readiness(checked_at),
            network_domain_blocking_readiness(checked_at),
            managed_browser_service_command_readiness(checked_at),
            managed_browser_exact_url_readiness(checked_at),
            unmanaged_browser_process_only_readiness(checked_at),
            unmanaged_browser_exact_evidence_readiness(checked_at),
            admin_anti_tamper_rollback_readiness(checked_at),
        ],
    }
}

fn owned_process_terminate_readiness(checked_at: &str) -> EnforcementBroadAdapterReadinessEntry {
    let capability = process_control_capability(checked_at);
    readiness_entry(ReadinessEntryInput {
        readiness_id: enforcement_constants::READINESS_ID_OWNED_PROCESS_TERMINATE,
        capability: EnforcementBroadAdapterCapability::OwnedProcessTerminate,
        platform: capability.platform,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: capability.capability_state,
        readiness_state: implemented_or_unavailable(capability.capability_state),
        proof_level: proof_for_supported_capability(capability.capability_state),
        runtime_owner: EnforcementReadinessRuntimeOwner::OsAdapter,
        supported_modes: vec![EnforcementMode::TerminateProcess],
        claim_boundary: enforcement_constants::CLAIM_BOUNDARY_OWNED_PROCESS_TERMINATE,
        fallback_behavior: enforcement_constants::FALLBACK_OWNED_PROCESS_TERMINATE,
        required_artifacts: artifacts_for_supported_capability(capability.capability_state),
        checked_at,
    })
}

fn app_time_limit_readiness(checked_at: &str) -> EnforcementBroadAdapterReadinessEntry {
    let capability = app_time_limit_capability(checked_at);
    readiness_entry(ReadinessEntryInput {
        readiness_id: enforcement_constants::READINESS_ID_APP_TIME_LIMIT,
        capability: EnforcementBroadAdapterCapability::AppTimeLimit,
        platform: capability.platform,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: capability.capability_state,
        readiness_state: manual_or_unavailable(capability.capability_state),
        proof_level: proof_for_manual_capability(capability.capability_state),
        runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
        supported_modes: capability.supported_actions,
        claim_boundary: enforcement_constants::CLAIM_BOUNDARY_APP_TIME_LIMIT,
        fallback_behavior: enforcement_constants::FALLBACK_APP_TIME_LIMIT,
        required_artifacts: vec![
            enforcement_constants::ARTIFACT_APP_TIME_LIMIT_EXECUTOR.to_string()
        ],
        checked_at,
    })
}

fn broad_app_blocking_readiness(checked_at: &str) -> EnforcementBroadAdapterReadinessEntry {
    let capability = app_block_control_capability(checked_at);
    readiness_entry(ReadinessEntryInput {
        readiness_id: enforcement_constants::READINESS_ID_BROAD_APP_BLOCKING,
        capability: EnforcementBroadAdapterCapability::BroadAppBlocking,
        platform: capability.platform,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: capability.capability_state,
        readiness_state: manual_or_unavailable(capability.capability_state),
        proof_level: proof_for_manual_capability(capability.capability_state),
        runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
        supported_modes: capability.supported_actions,
        claim_boundary: enforcement_constants::CLAIM_BOUNDARY_BROAD_APP_BLOCKING,
        fallback_behavior: enforcement_constants::FALLBACK_BROAD_APP_BLOCKING,
        required_artifacts: vec![
            enforcement_constants::ARTIFACT_OS_APP_IDENTITY.to_string(),
            enforcement_constants::ARTIFACT_APP_BLOCK_ROLLBACK.to_string(),
        ],
        checked_at,
    })
}

fn network_domain_blocking_readiness(checked_at: &str) -> EnforcementBroadAdapterReadinessEntry {
    let capability = network_control_capability(checked_at);
    readiness_entry(ReadinessEntryInput {
        readiness_id: enforcement_constants::READINESS_ID_NETWORK_DOMAIN_BLOCKING,
        capability: EnforcementBroadAdapterCapability::NetworkDomainBlocking,
        platform: capability.platform,
        adapter_kind: EnforcementAdapterKind::NetworkControl,
        capability_state: capability.capability_state,
        readiness_state: manual_or_unavailable(capability.capability_state),
        proof_level: proof_for_manual_capability(capability.capability_state),
        runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
        supported_modes: capability.supported_actions,
        claim_boundary: enforcement_constants::CLAIM_BOUNDARY_NETWORK_DOMAIN_BLOCKING,
        fallback_behavior: enforcement_constants::FALLBACK_NETWORK_DOMAIN_BLOCKING,
        required_artifacts: vec![
            enforcement_constants::ARTIFACT_NETWORK_FILTER.to_string(),
            enforcement_constants::ARTIFACT_DOMAIN_BLOCK_ROLLBACK.to_string(),
        ],
        checked_at,
    })
}

fn managed_browser_service_command_readiness(
    checked_at: &str,
) -> EnforcementBroadAdapterReadinessEntry {
    let capability = managed_browser_control_capability(checked_at);
    readiness_entry(ReadinessEntryInput {
        readiness_id: enforcement_constants::READINESS_ID_MANAGED_BROWSER_SERVICE_COMMAND,
        capability: EnforcementBroadAdapterCapability::ManagedBrowserServiceCommand,
        platform: capability.platform,
        adapter_kind: EnforcementAdapterKind::ManagedBrowserControl,
        capability_state: capability.capability_state,
        readiness_state: manual_or_unavailable(capability.capability_state),
        proof_level: proof_for_manual_capability(capability.capability_state),
        runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
        supported_modes: capability.supported_actions,
        claim_boundary: enforcement_constants::CLAIM_BOUNDARY_MANAGED_BROWSER_SERVICE_COMMAND,
        fallback_behavior: enforcement_constants::FALLBACK_MANAGED_BROWSER_SERVICE_COMMAND,
        required_artifacts: vec![
            enforcement_constants::ARTIFACT_MANAGED_BROWSER_COMMAND.to_string(),
            enforcement_constants::ARTIFACT_EXACT_URL_APPLY_AUDIT.to_string(),
        ],
        checked_at,
    })
}

fn managed_browser_exact_url_readiness(checked_at: &str) -> EnforcementBroadAdapterReadinessEntry {
    let capability = managed_browser_control_capability(checked_at);
    readiness_entry(ReadinessEntryInput {
        readiness_id: enforcement_constants::READINESS_ID_MANAGED_BROWSER_EXACT_URL,
        capability: EnforcementBroadAdapterCapability::ManagedBrowserExactUrlControl,
        platform: capability.platform,
        adapter_kind: EnforcementAdapterKind::ManagedBrowserControl,
        capability_state: capability.capability_state,
        readiness_state: manual_or_unavailable(capability.capability_state),
        proof_level: proof_for_manual_capability(capability.capability_state),
        runtime_owner: EnforcementReadinessRuntimeOwner::ManagedBrowserBoundary,
        supported_modes: capability.supported_actions,
        claim_boundary: enforcement_constants::CLAIM_BOUNDARY_MANAGED_BROWSER_EXACT_URL,
        fallback_behavior: enforcement_constants::FALLBACK_MANAGED_BROWSER_EXACT_URL,
        required_artifacts: vec![
            enforcement_constants::ARTIFACT_MANAGED_BROWSER_ACTIVE_TAB.to_string(),
            enforcement_constants::ARTIFACT_MANAGED_EXACT_URL.to_string(),
        ],
        checked_at,
    })
}

fn unmanaged_browser_process_only_readiness(
    checked_at: &str,
) -> EnforcementBroadAdapterReadinessEntry {
    let capability = process_control_capability(checked_at);
    readiness_entry(ReadinessEntryInput {
        readiness_id: enforcement_constants::READINESS_ID_UNMANAGED_BROWSER_PROCESS_ONLY,
        capability: EnforcementBroadAdapterCapability::UnmanagedBrowserProcessOnly,
        platform: capability.platform,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: capability.capability_state,
        readiness_state: implemented_or_unavailable(capability.capability_state),
        proof_level: proof_for_supported_capability(capability.capability_state),
        runtime_owner: EnforcementReadinessRuntimeOwner::OsAdapter,
        supported_modes: vec![
            EnforcementMode::TerminateProcess,
            EnforcementMode::ObserveOnly,
        ],
        claim_boundary: enforcement_constants::CLAIM_BOUNDARY_UNMANAGED_BROWSER_PROCESS_ONLY,
        fallback_behavior: enforcement_constants::FALLBACK_UNMANAGED_BROWSER_PROCESS_ONLY,
        required_artifacts: artifacts_for_supported_capability(capability.capability_state),
        checked_at,
    })
}

fn unmanaged_browser_exact_evidence_readiness(
    checked_at: &str,
) -> EnforcementBroadAdapterReadinessEntry {
    readiness_entry(ReadinessEntryInput {
        readiness_id: enforcement_constants::READINESS_ID_UNMANAGED_BROWSER_EXACT_EVIDENCE,
        capability: EnforcementBroadAdapterCapability::UnmanagedBrowserExactEvidence,
        platform: readiness_platform(),
        adapter_kind: EnforcementAdapterKind::ManagedBrowserControl,
        capability_state: EnforcementCapabilityState::ManualRequired,
        readiness_state: EnforcementReadinessState::NotClaimed,
        proof_level: EnforcementReadinessProofLevel::NotProved,
        runtime_owner: EnforcementReadinessRuntimeOwner::NotImplemented,
        supported_modes: Vec::new(),
        claim_boundary: enforcement_constants::CLAIM_BOUNDARY_UNMANAGED_BROWSER_EXACT_EVIDENCE,
        fallback_behavior: enforcement_constants::FALLBACK_UNMANAGED_BROWSER_EXACT_EVIDENCE,
        required_artifacts: vec![enforcement_constants::ARTIFACT_BROWSER_INTEGRATION.to_string()],
        checked_at,
    })
}

fn admin_anti_tamper_rollback_readiness(checked_at: &str) -> EnforcementBroadAdapterReadinessEntry {
    readiness_entry(ReadinessEntryInput {
        readiness_id: enforcement_constants::READINESS_ID_ADMIN_ANTI_TAMPER_ROLLBACK,
        capability: EnforcementBroadAdapterCapability::AdminAntiTamperRollback,
        platform: readiness_platform(),
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::ManualRequired,
        readiness_state: EnforcementReadinessState::ManualRequired,
        proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
        runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
        supported_modes: Vec::new(),
        claim_boundary: enforcement_constants::CLAIM_BOUNDARY_ADMIN_ANTI_TAMPER_ROLLBACK,
        fallback_behavior: enforcement_constants::FALLBACK_ADMIN_ANTI_TAMPER_ROLLBACK,
        required_artifacts: vec![
            enforcement_constants::ARTIFACT_ADMIN_HARDENING.to_string(),
            enforcement_constants::ARTIFACT_ANTI_TAMPER.to_string(),
            enforcement_constants::ARTIFACT_ROLLBACK_BYPASS.to_string(),
        ],
        checked_at,
    })
}

fn readiness_entry(input: ReadinessEntryInput<'_>) -> EnforcementBroadAdapterReadinessEntry {
    EnforcementBroadAdapterReadinessEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        readiness_id: input.readiness_id.to_string(),
        capability: input.capability,
        platform: input.platform,
        adapter_kind: input.adapter_kind,
        capability_state: input.capability_state,
        readiness_state: input.readiness_state,
        proof_level: input.proof_level,
        runtime_owner: input.runtime_owner,
        supported_modes: input.supported_modes,
        claim_boundary: input.claim_boundary.to_string(),
        fallback_behavior: input.fallback_behavior.to_string(),
        required_artifacts: input.required_artifacts,
        last_checked_at: input.checked_at.to_string(),
    }
}

fn implemented_or_unavailable(
    capability_state: EnforcementCapabilityState,
) -> EnforcementReadinessState {
    if capability_state == EnforcementCapabilityState::Supported {
        EnforcementReadinessState::Implemented
    } else {
        EnforcementReadinessState::Unavailable
    }
}

fn manual_or_unavailable(
    capability_state: EnforcementCapabilityState,
) -> EnforcementReadinessState {
    if capability_state == EnforcementCapabilityState::ManualRequired {
        EnforcementReadinessState::ManualRequired
    } else {
        EnforcementReadinessState::Unavailable
    }
}

fn proof_for_supported_capability(
    capability_state: EnforcementCapabilityState,
) -> EnforcementReadinessProofLevel {
    if capability_state == EnforcementCapabilityState::Supported {
        EnforcementReadinessProofLevel::RealServiceProof
    } else {
        EnforcementReadinessProofLevel::ManualProofRequired
    }
}

fn proof_for_manual_capability(
    _capability_state: EnforcementCapabilityState,
) -> EnforcementReadinessProofLevel {
    EnforcementReadinessProofLevel::ManualProofRequired
}

fn artifacts_for_supported_capability(capability_state: EnforcementCapabilityState) -> Vec<String> {
    if capability_state == EnforcementCapabilityState::Supported {
        Vec::new()
    } else {
        vec![enforcement_constants::UNAVAILABLE_UNSUPPORTED_PLATFORM.to_string()]
    }
}

#[cfg(windows)]
fn readiness_platform() -> ParentPlatform {
    ParentPlatform::Windows
}

#[cfg(not(windows))]
fn readiness_platform() -> ParentPlatform {
    match std::env::consts::OS {
        enforcement_constants::PLATFORM_LINUX => ParentPlatform::Linux,
        enforcement_constants::PLATFORM_MACOS => ParentPlatform::Macos,
        enforcement_constants::PLATFORM_ANDROID => ParentPlatform::Android,
        enforcement_constants::PLATFORM_IOS => ParentPlatform::Ios,
        _ => ParentPlatform::Linux,
    }
}
