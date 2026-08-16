use ocentra_parent_agent_core::enforcement_readiness::broad_os_adapter_readiness;
use ocentra_parent_agent_protocol::constants::enforcement;
use ocentra_parent_agent_protocol::constants::v08_os_adapter_product_proof as proof;
use ocentra_parent_agent_protocol::constants::windows_adapter_artifact_gate;
use ocentra_parent_agent_protocol::constants::windows_adapter_capability;
use ocentra_parent_agent_protocol::enforcement::EnforcementAdapterKind;
use ocentra_parent_agent_protocol::enforcement::EnforcementResultStatus;
use ocentra_parent_agent_protocol::enforcement::EnforcementRollbackState;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofParentOverrideState;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofReadModel;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofSurface;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofTimerRecoveryState;
use ocentra_parent_agent_protocol::enforcement_readiness::EnforcementBroadAdapterCapability;
use ocentra_parent_agent_protocol::enforcement_readiness::EnforcementReadinessRuntimeOwner;
use ocentra_parent_agent_protocol::policy_constants;

use crate::{
    host_identity_read_model::GeneratedAtText,
    windows_adapter_artifact_gate_read_model::{
        windows_adapter_artifact_gate_proof, ArtifactGateGeneratedAtTextRef,
    },
    windows_adapter_capability_read_model::windows_adapter_capability_proof,
};

#[path = "enforcement_os_adapter_product_proof_read_model/entry_factory.rs"]
mod entry_factory;
#[path = "enforcement_os_adapter_product_proof_read_model/product_control_spine.rs"]
pub(crate) mod product_control_spine;

use self::entry_factory::{entry_from_spec, lifecycle_entry_spec, manual_entry_spec};

#[derive(Clone, Copy)]
pub(crate) struct GeneratedAtTextRef<'a>(pub(crate) &'a str);

#[derive(Clone, Copy)]
struct ProofEntryIdRef<'a>(pub(crate) &'a str);

pub(crate) fn v08_os_adapter_product_proof_read_model<'a>(
    generated_at: impl Into<GeneratedAtTextRef<'a>>,
) -> V08OsAdapterProductProofReadModel {
    let generated_at = generated_at.into();
    let readiness = broad_os_adapter_readiness(generated_at.0);
    let capability = windows_adapter_capability_proof(GeneratedAtText(generated_at.0.to_string()));
    let artifact_gate =
        windows_adapter_artifact_gate_proof(ArtifactGateGeneratedAtTextRef(generated_at.0));

    V08OsAdapterProductProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.0.to_string(),
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

#[derive(Clone, Copy)]
struct EntryProofLinks<'a> {
    capability_entry_id: &'a str,
    artifact_gate_entry_id: &'a str,
}

#[derive(Clone, Copy)]
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
            ProofEntryIdRef(proof::ENTRY_ID_BROAD_APP_BLOCKING),
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
            ProofEntryIdRef(proof::ENTRY_ID_NETWORK_DOMAIN_BLOCKING),
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
            ProofEntryIdRef(proof::ENTRY_ID_MANAGED_BROWSER_SERVICE_COMMAND),
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
            ProofEntryIdRef(proof::ENTRY_ID_MANAGED_BROWSER_EXACT_URL),
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
            ProofEntryIdRef(proof::ENTRY_ID_RESTART_RECOVERY),
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
            ProofEntryIdRef(proof::ENTRY_ID_PARENT_CANCEL_OVERRIDE),
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
            ProofEntryIdRef(proof::ENTRY_ID_ROLLBACK_ARTIFACT_GATE),
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
