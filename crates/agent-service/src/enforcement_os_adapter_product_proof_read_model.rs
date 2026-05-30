use ocentra_parent_agent_core::broad_os_adapter_readiness;
use ocentra_parent_agent_protocol::{
    constants::{
        enforcement, v08_os_adapter_product_proof as proof, windows_adapter_artifact_gate,
        windows_adapter_capability,
    },
    policy_constants, EnforcementAdapterKind, EnforcementBroadAdapterCapability,
    EnforcementBroadAdapterReadinessEntry, EnforcementBroadOsAdapterReadinessMatrix,
    EnforcementCapabilityState, EnforcementReadinessProofLevel, EnforcementReadinessRuntimeOwner,
    EnforcementReadinessState, EnforcementResultStatus, EnforcementRollbackState,
    V08OsAdapterProductProofAuditState, V08OsAdapterProductProofEntry,
    V08OsAdapterProductProofParentOverrideState, V08OsAdapterProductProofReadModel,
    V08OsAdapterProductProofSurface, V08OsAdapterProductProofTimerRecoveryState,
    WindowsAdapterArtifactGateProof, WindowsAdapterCapabilityProof,
};

use crate::{
    windows_adapter_artifact_gate_read_model::windows_adapter_artifact_gate_proof,
    windows_adapter_capability_read_model::windows_adapter_capability_proof,
};

pub(crate) fn v08_os_adapter_product_proof_read_model(
    generated_at: &str,
) -> V08OsAdapterProductProofReadModel {
    let readiness = broad_os_adapter_readiness(generated_at);
    let capability = windows_adapter_capability_proof(generated_at);
    let artifact_gate = windows_adapter_artifact_gate_proof(generated_at);

    V08OsAdapterProductProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![
            enforcement::READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER.to_string(),
            windows_adapter_capability::READ_MODEL_ID_V0_8.to_string(),
            windows_adapter_artifact_gate::READ_MODEL_ID_V0_8.to_string(),
        ],
        entries: entry_specs()
            .iter()
            .map(|spec| {
                entry_from_spec(spec, &readiness, &capability, &artifact_gate, generated_at)
            })
            .collect(),
    }
}

struct EntrySpec<'a> {
    proof_entry_id: &'a str,
    surface: V08OsAdapterProductProofSurface,
    capability: EnforcementBroadAdapterCapability,
    adapter_kind: EnforcementAdapterKind,
    runtime_owner: EnforcementReadinessRuntimeOwner,
    linked_capability_entry_ids: Vec<&'a str>,
    linked_artifact_gate_entry_ids: Vec<&'a str>,
    implemented_result: EnforcementResultStatus,
    implemented_rollback: EnforcementRollbackState,
    implemented_timer: V08OsAdapterProductProofTimerRecoveryState,
    implemented_parent_override: V08OsAdapterProductProofParentOverrideState,
    capability_requirement: &'a str,
    proof_requirement: &'a str,
    claim_boundary: &'a str,
    fallback_behavior: &'a str,
}

struct EntryProofLinks<'a> {
    capability_entry_id: &'a str,
    artifact_gate_entry_id: &'a str,
}

struct EntryProofText<'a> {
    capability_requirement: &'a str,
    proof_requirement: &'a str,
    claim_boundary: &'a str,
    fallback_behavior: &'a str,
}

fn entry_specs<'a>() -> Vec<EntrySpec<'a>> {
    let mut specs = Vec::new();
    specs.extend(core_implemented_entry_specs());
    specs.extend(browser_boundary_entry_specs());
    specs.extend(manual_boundary_entry_specs());
    specs.extend(lifecycle_entry_specs());
    specs.extend(audit_and_rollback_entry_specs());
    specs
}

fn core_implemented_entry_specs<'a>() -> Vec<EntrySpec<'a>> {
    vec![
        EntrySpec {
            proof_entry_id: proof::ENTRY_ID_OWNED_PROCESS_TERMINATE,
            surface: V08OsAdapterProductProofSurface::OwnedProcessTerminate,
            capability: EnforcementBroadAdapterCapability::OwnedProcessTerminate,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            runtime_owner: EnforcementReadinessRuntimeOwner::OsAdapter,
            linked_capability_entry_ids: vec![
                windows_adapter_capability::ENTRY_ID_UNMANAGED_BROWSER_TARGET,
            ],
            linked_artifact_gate_entry_ids: Vec::new(),
            implemented_result: EnforcementResultStatus::ActuallyEnforced,
            implemented_rollback: EnforcementRollbackState::NotRequired,
            implemented_timer: V08OsAdapterProductProofTimerRecoveryState::NotRequired,
            implemented_parent_override: V08OsAdapterProductProofParentOverrideState::NotRequired,
            capability_requirement: proof::CAPABILITY_OWNED_PROCESS,
            proof_requirement: proof::PROOF_OWNED_PROCESS,
            claim_boundary: proof::CLAIM_OWNED_PROCESS,
            fallback_behavior: proof::FALLBACK_OWNED_PROCESS,
        },
        EntrySpec {
            proof_entry_id: proof::ENTRY_ID_APP_TIME_LIMIT_LIFECYCLE,
            surface: V08OsAdapterProductProofSurface::AppTimeLimitLifecycle,
            capability: EnforcementBroadAdapterCapability::AppTimeLimit,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            runtime_owner: EnforcementReadinessRuntimeOwner::RustService,
            linked_capability_entry_ids: vec![windows_adapter_capability::ENTRY_ID_APP_TARGET],
            linked_artifact_gate_entry_ids: Vec::new(),
            implemented_result: EnforcementResultStatus::Expired,
            implemented_rollback: EnforcementRollbackState::Completed,
            implemented_timer: V08OsAdapterProductProofTimerRecoveryState::Expired,
            implemented_parent_override:
                V08OsAdapterProductProofParentOverrideState::CancelSupported,
            capability_requirement: proof::CAPABILITY_APP_TIME_LIMIT,
            proof_requirement: proof::PROOF_APP_TIME_LIMIT,
            claim_boundary: proof::CLAIM_APP_TIME_LIMIT,
            fallback_behavior: proof::FALLBACK_APP_TIME_LIMIT,
        },
    ]
}

fn browser_boundary_entry_specs<'a>() -> Vec<EntrySpec<'a>> {
    vec![
        EntrySpec {
            proof_entry_id: proof::ENTRY_ID_UNMANAGED_BROWSER_PROCESS_ONLY,
            surface: V08OsAdapterProductProofSurface::UnmanagedBrowserProcessOnly,
            capability: EnforcementBroadAdapterCapability::UnmanagedBrowserProcessOnly,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            runtime_owner: EnforcementReadinessRuntimeOwner::OsAdapter,
            linked_capability_entry_ids: vec![
                windows_adapter_capability::ENTRY_ID_UNMANAGED_BROWSER_TARGET,
            ],
            linked_artifact_gate_entry_ids: vec![
                windows_adapter_artifact_gate::ENTRY_ID_UNMANAGED_BROWSER_TARGET,
            ],
            implemented_result: EnforcementResultStatus::ActuallyEnforced,
            implemented_rollback: EnforcementRollbackState::NotRequired,
            implemented_timer: V08OsAdapterProductProofTimerRecoveryState::NotRequired,
            implemented_parent_override: V08OsAdapterProductProofParentOverrideState::NotRequired,
            capability_requirement: proof::CAPABILITY_UNMANAGED_PROCESS,
            proof_requirement: proof::PROOF_UNMANAGED_PROCESS,
            claim_boundary: proof::CLAIM_UNMANAGED_PROCESS,
            fallback_behavior: proof::FALLBACK_UNMANAGED_PROCESS,
        },
        EntrySpec {
            proof_entry_id: proof::ENTRY_ID_UNMANAGED_BROWSER_EXACT_EVIDENCE,
            surface: V08OsAdapterProductProofSurface::UnmanagedBrowserExactEvidence,
            capability: EnforcementBroadAdapterCapability::UnmanagedBrowserExactEvidence,
            adapter_kind: EnforcementAdapterKind::ManagedBrowserControl,
            runtime_owner: EnforcementReadinessRuntimeOwner::NotImplemented,
            linked_capability_entry_ids: vec![
                windows_adapter_capability::ENTRY_ID_UNMANAGED_BROWSER_TARGET,
            ],
            linked_artifact_gate_entry_ids: vec![
                windows_adapter_artifact_gate::ENTRY_ID_UNMANAGED_BROWSER_TARGET,
            ],
            implemented_result: EnforcementResultStatus::NoOp,
            implemented_rollback: EnforcementRollbackState::NotRequired,
            implemented_timer: V08OsAdapterProductProofTimerRecoveryState::NotRequired,
            implemented_parent_override: V08OsAdapterProductProofParentOverrideState::Unavailable,
            capability_requirement: proof::CAPABILITY_BROWSER_INTEGRATION,
            proof_requirement: proof::PROOF_UNMANAGED_EXACT,
            claim_boundary: proof::CLAIM_UNMANAGED_EXACT,
            fallback_behavior: proof::FALLBACK_UNMANAGED_EXACT,
        },
    ]
}

fn manual_boundary_entry_specs<'a>() -> Vec<EntrySpec<'a>> {
    vec![
        manual_entry_spec(
            proof::ENTRY_ID_BROAD_APP_BLOCKING,
            V08OsAdapterProductProofSurface::BroadAppBlocking,
            EnforcementBroadAdapterCapability::BroadAppBlocking,
            EnforcementAdapterKind::ProcessControl,
            EntryProofLinks {
                capability_entry_id: windows_adapter_capability::ENTRY_ID_APP_TARGET,
                artifact_gate_entry_id: windows_adapter_artifact_gate::ENTRY_ID_APP_TARGET,
            },
            EntryProofText {
                capability_requirement: proof::CAPABILITY_BROAD_APP,
                proof_requirement: proof::PROOF_BROAD_APP,
                claim_boundary: proof::CLAIM_BROAD_APP,
                fallback_behavior: proof::FALLBACK_BROAD_APP,
            },
        ),
        manual_entry_spec(
            proof::ENTRY_ID_NETWORK_DOMAIN_BLOCKING,
            V08OsAdapterProductProofSurface::NetworkDomainBlocking,
            EnforcementBroadAdapterCapability::NetworkDomainBlocking,
            EnforcementAdapterKind::NetworkControl,
            EntryProofLinks {
                capability_entry_id: windows_adapter_capability::ENTRY_ID_DOMAIN_NETWORK_TARGET,
                artifact_gate_entry_id:
                    windows_adapter_artifact_gate::ENTRY_ID_DOMAIN_NETWORK_TARGET,
            },
            EntryProofText {
                capability_requirement: proof::CAPABILITY_NETWORK_DOMAIN,
                proof_requirement: proof::PROOF_NETWORK_DOMAIN,
                claim_boundary: proof::CLAIM_NETWORK_DOMAIN,
                fallback_behavior: proof::FALLBACK_NETWORK_DOMAIN,
            },
        ),
        manual_entry_spec(
            proof::ENTRY_ID_MANAGED_BROWSER_SERVICE_COMMAND,
            V08OsAdapterProductProofSurface::ManagedBrowserServiceCommand,
            EnforcementBroadAdapterCapability::ManagedBrowserServiceCommand,
            EnforcementAdapterKind::ManagedBrowserControl,
            EntryProofLinks {
                capability_entry_id: windows_adapter_capability::ENTRY_ID_MANAGED_BROWSER_TARGET,
                artifact_gate_entry_id:
                    windows_adapter_artifact_gate::ENTRY_ID_MANAGED_BROWSER_TARGET,
            },
            EntryProofText {
                capability_requirement: proof::CAPABILITY_MANAGED_BROWSER_COMMAND,
                proof_requirement: proof::PROOF_MANAGED_BROWSER_COMMAND,
                claim_boundary: proof::CLAIM_MANAGED_BROWSER_COMMAND,
                fallback_behavior: proof::FALLBACK_MANAGED_BROWSER_COMMAND,
            },
        ),
        manual_entry_spec(
            proof::ENTRY_ID_MANAGED_BROWSER_EXACT_URL,
            V08OsAdapterProductProofSurface::ManagedBrowserExactUrl,
            EnforcementBroadAdapterCapability::ManagedBrowserExactUrlControl,
            EnforcementAdapterKind::ManagedBrowserControl,
            EntryProofLinks {
                capability_entry_id: windows_adapter_capability::ENTRY_ID_MANAGED_BROWSER_TARGET,
                artifact_gate_entry_id:
                    windows_adapter_artifact_gate::ENTRY_ID_MANAGED_BROWSER_TARGET,
            },
            EntryProofText {
                capability_requirement: proof::CAPABILITY_MANAGED_EXACT_URL,
                proof_requirement: proof::PROOF_MANAGED_EXACT_URL,
                claim_boundary: proof::CLAIM_MANAGED_EXACT_URL,
                fallback_behavior: proof::FALLBACK_MANAGED_EXACT_URL,
            },
        ),
    ]
}

fn lifecycle_entry_specs<'a>() -> Vec<EntrySpec<'a>> {
    vec![
        lifecycle_entry_spec(
            proof::ENTRY_ID_RESTART_RECOVERY,
            V08OsAdapterProductProofSurface::RestartRecovery,
            EnforcementResultStatus::Expired,
            V08OsAdapterProductProofTimerRecoveryState::RestartRecovered,
            EntryProofText {
                capability_requirement: proof::CAPABILITY_RESTART_RECOVERY,
                proof_requirement: proof::PROOF_RESTART_RECOVERY,
                claim_boundary: proof::CLAIM_RESTART_RECOVERY,
                fallback_behavior: proof::FALLBACK_RESTART_RECOVERY,
            },
        ),
        lifecycle_entry_spec(
            proof::ENTRY_ID_PARENT_CANCEL_OVERRIDE,
            V08OsAdapterProductProofSurface::ParentCancelOverride,
            EnforcementResultStatus::RolledBack,
            V08OsAdapterProductProofTimerRecoveryState::Cancelled,
            EntryProofText {
                capability_requirement: proof::CAPABILITY_PARENT_CANCEL,
                proof_requirement: proof::PROOF_PARENT_CANCEL,
                claim_boundary: proof::CLAIM_PARENT_CANCEL,
                fallback_behavior: proof::FALLBACK_PARENT_CANCEL,
            },
        ),
    ]
}

fn audit_and_rollback_entry_specs<'a>() -> Vec<EntrySpec<'a>> {
    vec![
        EntrySpec {
            proof_entry_id: proof::ENTRY_ID_AUDIT_CUSTODY,
            surface: V08OsAdapterProductProofSurface::AuditCustody,
            capability: EnforcementBroadAdapterCapability::OwnedProcessTerminate,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            runtime_owner: EnforcementReadinessRuntimeOwner::RustService,
            linked_capability_entry_ids: vec![
                windows_adapter_capability::ENTRY_ID_ROLLBACK_AUDIT_TARGET,
            ],
            linked_artifact_gate_entry_ids: vec![
                windows_adapter_artifact_gate::ENTRY_ID_ROLLBACK_AUDIT_TARGET,
            ],
            implemented_result: EnforcementResultStatus::ActuallyEnforced,
            implemented_rollback: EnforcementRollbackState::Available,
            implemented_timer: V08OsAdapterProductProofTimerRecoveryState::NotRequired,
            implemented_parent_override:
                V08OsAdapterProductProofParentOverrideState::CancelSupported,
            capability_requirement: proof::CAPABILITY_AUDIT_CUSTODY,
            proof_requirement: proof::PROOF_AUDIT_CUSTODY,
            claim_boundary: proof::CLAIM_AUDIT_CUSTODY,
            fallback_behavior: proof::FALLBACK_AUDIT_CUSTODY,
        },
        manual_entry_spec(
            proof::ENTRY_ID_ROLLBACK_ARTIFACT_GATE,
            V08OsAdapterProductProofSurface::RollbackArtifactGate,
            EnforcementBroadAdapterCapability::AdminAntiTamperRollback,
            EnforcementAdapterKind::ProcessControl,
            EntryProofLinks {
                capability_entry_id: windows_adapter_capability::ENTRY_ID_ROLLBACK_AUDIT_TARGET,
                artifact_gate_entry_id:
                    windows_adapter_artifact_gate::ENTRY_ID_ROLLBACK_AUDIT_TARGET,
            },
            EntryProofText {
                capability_requirement: proof::CAPABILITY_ROLLBACK_ARTIFACT_GATE,
                proof_requirement: proof::PROOF_ROLLBACK_ARTIFACT_GATE,
                claim_boundary: proof::CLAIM_ROLLBACK_ARTIFACT_GATE,
                fallback_behavior: proof::FALLBACK_ROLLBACK_ARTIFACT_GATE,
            },
        ),
    ]
}

fn manual_entry_spec<'a>(
    proof_entry_id: &'a str,
    surface: V08OsAdapterProductProofSurface,
    capability: EnforcementBroadAdapterCapability,
    adapter_kind: EnforcementAdapterKind,
    links: EntryProofLinks<'a>,
    text: EntryProofText<'a>,
) -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id,
        surface,
        capability,
        adapter_kind,
        runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
        linked_capability_entry_ids: vec![links.capability_entry_id],
        linked_artifact_gate_entry_ids: vec![links.artifact_gate_entry_id],
        implemented_result: EnforcementResultStatus::Unavailable,
        implemented_rollback: EnforcementRollbackState::Unavailable,
        implemented_timer: V08OsAdapterProductProofTimerRecoveryState::ManualRequired,
        implemented_parent_override: V08OsAdapterProductProofParentOverrideState::ManualRequired,
        capability_requirement: text.capability_requirement,
        proof_requirement: text.proof_requirement,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn lifecycle_entry_spec<'a>(
    proof_entry_id: &'a str,
    surface: V08OsAdapterProductProofSurface,
    result_status: EnforcementResultStatus,
    timer_state: V08OsAdapterProductProofTimerRecoveryState,
    text: EntryProofText<'a>,
) -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id,
        surface,
        capability: EnforcementBroadAdapterCapability::AppTimeLimit,
        adapter_kind: EnforcementAdapterKind::TimerControl,
        runtime_owner: EnforcementReadinessRuntimeOwner::RustService,
        linked_capability_entry_ids: vec![windows_adapter_capability::ENTRY_ID_APP_TARGET],
        linked_artifact_gate_entry_ids: Vec::new(),
        implemented_result: result_status,
        implemented_rollback: EnforcementRollbackState::Completed,
        implemented_timer: timer_state,
        implemented_parent_override: V08OsAdapterProductProofParentOverrideState::CancelSupported,
        capability_requirement: text.capability_requirement,
        proof_requirement: text.proof_requirement,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn entry_from_spec(
    spec: &EntrySpec<'_>,
    readiness: &EnforcementBroadOsAdapterReadinessMatrix,
    capability: &WindowsAdapterCapabilityProof,
    artifact_gate: &WindowsAdapterArtifactGateProof,
    generated_at: &str,
) -> V08OsAdapterProductProofEntry {
    let primary = readiness_entry(readiness, spec.capability);
    assert_links(spec, capability, artifact_gate);
    let proof_level = if primary.readiness_state == EnforcementReadinessState::Implemented {
        primary.proof_level
    } else if primary.readiness_state == EnforcementReadinessState::NotClaimed {
        EnforcementReadinessProofLevel::NotProved
    } else {
        EnforcementReadinessProofLevel::ManualProofRequired
    };
    let runtime_owner = if primary.capability_state == EnforcementCapabilityState::Supported
        || primary.readiness_state == EnforcementReadinessState::NotClaimed
    {
        spec.runtime_owner
    } else {
        EnforcementReadinessRuntimeOwner::ManualProof
    };

    V08OsAdapterProductProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: spec.proof_entry_id.to_string(),
        surface: spec.surface,
        platform: primary.platform,
        adapter_kind: spec.adapter_kind,
        capability_state: primary.capability_state,
        readiness_state: primary.readiness_state,
        proof_level,
        runtime_owner,
        supported_modes: primary.supported_modes.clone(),
        result_status: result_for_product_entry(primary, spec),
        rollback_state: rollback_for_product_entry(primary, spec),
        timer_recovery_state: timer_for_product_entry(primary, spec),
        audit_state: audit_for_product_entry(primary),
        parent_override_state: parent_override_for_product_entry(primary, spec),
        linked_readiness_ids: vec![primary.readiness_id.clone()],
        linked_capability_entry_ids: spec
            .linked_capability_entry_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        linked_artifact_gate_entry_ids: spec
            .linked_artifact_gate_entry_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        capability_requirement: spec.capability_requirement.to_string(),
        proof_requirement: spec.proof_requirement.to_string(),
        claim_boundary: spec.claim_boundary.to_string(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        claim_upgrade_allowed: false,
        broad_blocking_claimed: false,
        exact_url_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

fn readiness_entry(
    readiness: &EnforcementBroadOsAdapterReadinessMatrix,
    capability: EnforcementBroadAdapterCapability,
) -> &EnforcementBroadAdapterReadinessEntry {
    readiness
        .entries
        .iter()
        .find(|entry| entry.capability == capability)
        .expect(enforcement::READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER)
}

fn assert_links(
    spec: &EntrySpec<'_>,
    capability: &WindowsAdapterCapabilityProof,
    artifact_gate: &WindowsAdapterArtifactGateProof,
) {
    for entry_id in &spec.linked_capability_entry_ids {
        capability
            .entries
            .iter()
            .find(|entry| entry.proof_entry_id == **entry_id)
            .expect(windows_adapter_capability::READ_MODEL_ID_V0_8);
    }
    for entry_id in &spec.linked_artifact_gate_entry_ids {
        artifact_gate
            .entries
            .iter()
            .find(|entry| entry.gate_entry_id == **entry_id)
            .expect(windows_adapter_artifact_gate::READ_MODEL_ID_V0_8);
    }
}

fn result_for_product_entry(
    primary: &EnforcementBroadAdapterReadinessEntry,
    spec: &EntrySpec<'_>,
) -> EnforcementResultStatus {
    match primary.readiness_state {
        EnforcementReadinessState::Implemented => spec.implemented_result,
        EnforcementReadinessState::NotClaimed => EnforcementResultStatus::NoOp,
        _ => EnforcementResultStatus::Unavailable,
    }
}

fn rollback_for_product_entry(
    primary: &EnforcementBroadAdapterReadinessEntry,
    spec: &EntrySpec<'_>,
) -> EnforcementRollbackState {
    if primary.readiness_state == EnforcementReadinessState::Implemented {
        spec.implemented_rollback
    } else if primary.readiness_state == EnforcementReadinessState::NotClaimed {
        EnforcementRollbackState::NotRequired
    } else {
        EnforcementRollbackState::Unavailable
    }
}

fn timer_for_product_entry(
    primary: &EnforcementBroadAdapterReadinessEntry,
    spec: &EntrySpec<'_>,
) -> V08OsAdapterProductProofTimerRecoveryState {
    if primary.readiness_state == EnforcementReadinessState::Implemented {
        spec.implemented_timer
    } else if primary.readiness_state == EnforcementReadinessState::ManualRequired {
        V08OsAdapterProductProofTimerRecoveryState::ManualRequired
    } else if primary.readiness_state == EnforcementReadinessState::NotClaimed {
        V08OsAdapterProductProofTimerRecoveryState::NotRequired
    } else {
        V08OsAdapterProductProofTimerRecoveryState::Unavailable
    }
}

fn audit_for_product_entry(
    primary: &EnforcementBroadAdapterReadinessEntry,
) -> V08OsAdapterProductProofAuditState {
    if primary.readiness_state == EnforcementReadinessState::Implemented {
        V08OsAdapterProductProofAuditState::Journaled
    } else if primary.readiness_state == EnforcementReadinessState::ManualRequired {
        V08OsAdapterProductProofAuditState::ManualRequired
    } else {
        V08OsAdapterProductProofAuditState::Unavailable
    }
}

fn parent_override_for_product_entry(
    primary: &EnforcementBroadAdapterReadinessEntry,
    spec: &EntrySpec<'_>,
) -> V08OsAdapterProductProofParentOverrideState {
    if primary.readiness_state == EnforcementReadinessState::Implemented {
        spec.implemented_parent_override
    } else if primary.readiness_state == EnforcementReadinessState::ManualRequired {
        V08OsAdapterProductProofParentOverrideState::ManualRequired
    } else {
        V08OsAdapterProductProofParentOverrideState::Unavailable
    }
}
