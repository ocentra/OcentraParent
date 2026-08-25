use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterAuditReferenceState;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterCapability;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterPlatformSupportState;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRefusalReason;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterResult;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRollbackReferenceState;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeBoundary;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeProofEntry;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeProofReadModel;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeState;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterTargetIdentityState;
use ocentra_parent_agent_protocol::policy_constants;

#[derive(Clone, Copy)]
pub(crate) struct GeneratedAtTextRef<'a>(pub(crate) &'a str);

#[derive(Clone, Copy)]
struct ProofEntryId(pub(crate) &'static str);

#[derive(Clone, Copy)]
struct StaticTextRefs(pub(crate) &'static [&'static str]);

pub(crate) fn v08_supported_adapter_runtime_proof_read_model<'a>(
    generated_at: impl Into<GeneratedAtTextRef<'a>>,
) -> V08SupportedAdapterRuntimeProofReadModel {
    let generated_at = generated_at.into();
    V08SupportedAdapterRuntimeProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.0.to_string(),
        source_read_model_ids: vec![
            proof::SOURCE_BROAD_ADAPTER_PROOF.to_string(),
            proof::SOURCE_POLICY_DISPATCH_PROOF.to_string(),
            proof::SOURCE_PRODUCT_CONTROL_PROOF.to_string(),
            proof::SOURCE_NETWORK_FLOW_EVIDENCE.to_string(),
            proof::SOURCE_WINDOWS_ADAPTER_CAPABILITY_PROOF.to_string(),
            proof::SOURCE_WINDOWS_ADAPTER_ARTIFACT_GATE.to_string(),
            proof::SOURCE_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF.to_string(),
        ],
        entries: entry_specs()
            .iter()
            .map(|spec| entry_from_spec(spec, generated_at))
            .collect(),
    }
}

struct EntrySpec {
    proof_entry_id: &'static str,
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    platform: ParentPlatform,
    adapter_capability: V08SupportedAdapterCapability,
    runtime_state: V08SupportedAdapterRuntimeState,
    adapter_result: V08SupportedAdapterResult,
    platform_support_state: V08SupportedAdapterPlatformSupportState,
    target_identity_state: V08SupportedAdapterTargetIdentityState,
    rollback_reference_state: V08SupportedAdapterRollbackReferenceState,
    audit_reference_state: V08SupportedAdapterAuditReferenceState,
    refusal_reason: V08SupportedAdapterRefusalReason,
    evidence_refs: &'static [&'static str],
    linked_proof_commands: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

#[derive(Clone, Copy)]
struct ImplementedSpecInput {
    proof_entry_id: &'static str,
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    adapter_capability: V08SupportedAdapterCapability,
    target_identity_state: V08SupportedAdapterTargetIdentityState,
    rollback_reference_state: V08SupportedAdapterRollbackReferenceState,
    evidence_refs: &'static [&'static str],
    linked_proof_commands: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

#[derive(Clone, Copy)]
struct ManualSpecInput {
    proof_entry_id: &'static str,
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    platform: ParentPlatform,
    adapter_capability: V08SupportedAdapterCapability,
    target_identity_state: V08SupportedAdapterTargetIdentityState,
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

fn entry_specs() -> Vec<EntrySpec> {
    let mut specs = vec![app_game_timer_spec(), network_observe_spec()];
    specs.extend(manual_gate_specs());
    specs.extend(artifact_status_specs());
    specs.extend([
        exact_active_tab_not_claimed_spec(),
        permission_dependency_degraded_spec(),
        unavailable_spec(),
        unsupported_spec(),
        mobile_manual_spec(
            ProofEntryId(proof::ENTRY_ID_ANDROID_MANUAL),
            V08SupportedAdapterRuntimeBoundary::AndroidMobileControlManualGate,
            ParentPlatform::Android,
            StaticTextRefs(&[
                proof::REQUIREMENT_ANDROID_DEVICE_OWNER,
                proof::REQUIREMENT_ANDROID_USAGE_STATS,
                proof::REQUIREMENT_ANDROID_ACCESSIBILITY_VPN_DNS,
            ]),
        ),
        mobile_manual_spec(
            ProofEntryId(proof::ENTRY_ID_IOS_MANUAL),
            V08SupportedAdapterRuntimeBoundary::IosMobileControlManualGate,
            ParentPlatform::Ios,
            StaticTextRefs(&[
                proof::REQUIREMENT_IOS_FAMILY_CONTROLS,
                proof::REQUIREMENT_IOS_DEVICE_ACTIVITY,
                proof::REQUIREMENT_IOS_NETWORK_EXTENSION,
            ]),
        ),
    ]);
    specs
}

fn manual_gate_specs() -> [EntrySpec; 2] {
    [
        manual_spec(ManualSpecInput {
            proof_entry_id: proof::ENTRY_ID_BROAD_APP_MANUAL,
            runtime_boundary:
                V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppBlockingManualGate,
            platform: ParentPlatform::Windows,
            adapter_capability: V08SupportedAdapterCapability::BroadInstalledAppBlocking,
            target_identity_state:
                V08SupportedAdapterTargetIdentityState::InsufficientForBroadTarget,
            manual_proof_requirements: &[
                proof::REQUIREMENT_SAME_APP_IDENTITY,
                proof::REQUIREMENT_HOST_BLOCK_APPLY,
                proof::REQUIREMENT_ROLLBACK,
                proof::REQUIREMENT_AUDIT_CUSTODY,
            ],
            claim_boundary: proof::CLAIM_BROAD_APP_MANUAL,
            fallback_behavior: proof::FALLBACK_BROAD_APP_MANUAL,
        }),
        manual_spec(ManualSpecInput {
            proof_entry_id: proof::ENTRY_ID_HOST_NETWORK_MANUAL,
            runtime_boundary:
                V08SupportedAdapterRuntimeBoundary::WindowsHostNetworkDomainBlockingManualGate,
            platform: ParentPlatform::Windows,
            adapter_capability: V08SupportedAdapterCapability::HostNetworkDomainBlocking,
            target_identity_state:
                V08SupportedAdapterTargetIdentityState::InsufficientForBroadTarget,
            manual_proof_requirements: &[
                proof::REQUIREMENT_HOST_DNS_OR_FILTER_APPLY,
                proof::REQUIREMENT_ROLLBACK,
                proof::REQUIREMENT_AUDIT_CUSTODY,
            ],
            claim_boundary: proof::CLAIM_HOST_NETWORK_MANUAL,
            fallback_behavior: proof::FALLBACK_HOST_NETWORK_MANUAL,
        }),
    ]
}

fn artifact_status_specs() -> [EntrySpec; 3] {
    [
        artifact_status_spec(ArtifactStatusSpecInput {
            proof_entry_id: proof::ENTRY_ID_BROAD_APP_ARTIFACT_STATUS,
            runtime_boundary:
                V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppArtifactStatus,
            adapter_capability: V08SupportedAdapterCapability::BroadInstalledAppArtifactStatus,
            manual_proof_requirements: &[
                proof::REQUIREMENT_SAME_IDENTITY_APP_PACKAGE_EVIDENCE,
                proof::REQUIREMENT_ADAPTER_APPLY_RESULT,
                proof::REQUIREMENT_ADAPTER_ROLLBACK_RESULT,
                proof::REQUIREMENT_AUDIT_CUSTODY_EVENT,
                proof::REQUIREMENT_MANUAL_REVIEW_AFTER_ARTIFACT_GATE,
            ],
            claim_boundary: proof::CLAIM_BROAD_APP_ARTIFACT_STATUS,
            fallback_behavior: proof::FALLBACK_BROAD_APP_ARTIFACT_STATUS,
        }),
        artifact_status_spec(ArtifactStatusSpecInput {
            proof_entry_id: proof::ENTRY_ID_HOST_NETWORK_ARTIFACT_STATUS,
            runtime_boundary:
                V08SupportedAdapterRuntimeBoundary::WindowsHostNetworkDomainArtifactStatus,
            adapter_capability: V08SupportedAdapterCapability::HostNetworkDomainArtifactStatus,
            manual_proof_requirements: &[
                proof::REQUIREMENT_HOST_DNS_OR_FILTER_APPLY,
                proof::REQUIREMENT_NETWORK_FILTER_ROLLBACK,
                proof::REQUIREMENT_AUDIT_CUSTODY_EVENT,
                proof::REQUIREMENT_MANUAL_REVIEW_AFTER_ARTIFACT_GATE,
            ],
            claim_boundary: proof::CLAIM_HOST_NETWORK_ARTIFACT_STATUS,
            fallback_behavior: proof::FALLBACK_HOST_NETWORK_ARTIFACT_STATUS,
        }),
        artifact_status_spec(ArtifactStatusSpecInput {
            proof_entry_id: proof::ENTRY_ID_MANAGED_BROWSER_ARTIFACT_STATUS,
            runtime_boundary:
                V08SupportedAdapterRuntimeBoundary::WindowsManagedBrowserArtifactStatus,
            adapter_capability: V08SupportedAdapterCapability::ManagedBrowserArtifactStatus,
            manual_proof_requirements: &[
                proof::REQUIREMENT_MANAGED_BROWSER_EXACT_URL_EVIDENCE,
                proof::REQUIREMENT_AUDIT_CUSTODY_EVENT,
                proof::REQUIREMENT_MANUAL_REVIEW_AFTER_ARTIFACT_GATE,
            ],
            claim_boundary: proof::CLAIM_MANAGED_BROWSER_ARTIFACT_STATUS,
            fallback_behavior: proof::FALLBACK_MANAGED_BROWSER_ARTIFACT_STATUS,
        }),
    ]
}

fn app_game_timer_spec() -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_APP_GAME_TIMER,
        runtime_boundary: V08SupportedAdapterRuntimeBoundary::WindowsAppGameOwnedProcessTimeLimit,
        platform: ParentPlatform::Windows,
        adapter_capability: V08SupportedAdapterCapability::AppGameOwnedProcessTimeLimit,
        runtime_state: V08SupportedAdapterRuntimeState::ManualRequired,
        adapter_result: V08SupportedAdapterResult::ManualProofRequired,
        platform_support_state: V08SupportedAdapterPlatformSupportState::ManualRequired,
        target_identity_state: V08SupportedAdapterTargetIdentityState::ProcessSessionEvidenceBacked,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::TimerRecoveryBacked,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::AuditReferenceBacked,
        refusal_reason: V08SupportedAdapterRefusalReason::ManualArtifactRequired,
        evidence_refs: &[
            proof::REF_APP_SESSION_EVIDENCE,
            proof::REF_OWNED_PROCESS_IDENTITY,
            proof::REF_TIMER_STATE,
        ],
        linked_proof_commands: &[proof::COMMAND_ENFORCEMENT_TIMER_CARGO],
        linked_proof_artifacts: &[proof::ARTIFACT_ENFORCEMENT_TIMER_STATE],
        manual_proof_requirements: &[
            enforcement_constants::ARTIFACT_APP_TIME_LIMIT_EXECUTOR,
            proof::REQUIREMENT_ROLLBACK,
            proof::REQUIREMENT_AUDIT_CUSTODY,
        ],
        claim_boundary: proof::CLAIM_APP_GAME_TIMER,
        fallback_behavior: proof::FALLBACK_APP_GAME_TIMER,
    }
}

fn network_observe_spec() -> EntrySpec {
    implemented_spec(ImplementedSpecInput {
        proof_entry_id: proof::ENTRY_ID_NETWORK_OBSERVE,
        runtime_boundary:
            V08SupportedAdapterRuntimeBoundary::WindowsNetworkFlowObservePolicyHandoff,
        adapter_capability: V08SupportedAdapterCapability::NetworkFlowObservePolicyHandoff,
        target_identity_state: V08SupportedAdapterTargetIdentityState::NetworkFlowEvidenceBacked,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::ObserveOnlyNotNeeded,
        evidence_refs: &[
            proof::REF_NETWORK_FLOW_SUMMARY,
            proof::REF_DOMAIN_ATTRIBUTION_STATE,
            proof::REF_POLICY_PREVIEW,
        ],
        linked_proof_commands: &[
            proof::COMMAND_NETWORK_FLOW_CARGO,
            proof::COMMAND_POLICY_DISPATCH_PROOF,
        ],
        linked_proof_artifacts: &[
            proof::ARTIFACT_NETWORK_FLOW_DIGEST,
            proof::ARTIFACT_POLICY_DISPATCH_PROOF,
        ],
        claim_boundary: proof::CLAIM_NETWORK_OBSERVE,
        fallback_behavior: proof::FALLBACK_NETWORK_OBSERVE,
    })
}

fn exact_active_tab_not_claimed_spec() -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_EXACT_ACTIVE_TAB_NOT_CLAIMED,
        runtime_boundary:
            V08SupportedAdapterRuntimeBoundary::WindowsManagedExactActiveTabNotClaimed,
        platform: ParentPlatform::Windows,
        adapter_capability: V08SupportedAdapterCapability::ManagedExactActiveTabEnforcement,
        runtime_state: V08SupportedAdapterRuntimeState::NotClaimed,
        adapter_result: V08SupportedAdapterResult::NotClaimed,
        platform_support_state: V08SupportedAdapterPlatformSupportState::ManualRequired,
        target_identity_state: V08SupportedAdapterTargetIdentityState::InsufficientForBroadTarget,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::NotClaimed,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::NotClaimed,
        refusal_reason: V08SupportedAdapterRefusalReason::NotClaimedBoundary,
        evidence_refs: &[],
        linked_proof_commands: &[proof::COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF],
        linked_proof_artifacts: &[proof::ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF],
        manual_proof_requirements: &[
            proof::REQUIREMENT_MANAGED_ACTIVE_TAB,
            proof::REQUIREMENT_EXACT_URL_APPLY,
            proof::REQUIREMENT_ROLLBACK,
        ],
        claim_boundary: proof::CLAIM_EXACT_ACTIVE_TAB_NOT_CLAIMED,
        fallback_behavior: proof::FALLBACK_EXACT_ACTIVE_TAB_NOT_CLAIMED,
    }
}

fn permission_dependency_degraded_spec() -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_PERMISSION_DEGRADED,
        runtime_boundary:
            V08SupportedAdapterRuntimeBoundary::WindowsAdapterPermissionDependencyDegraded,
        platform: ParentPlatform::Windows,
        adapter_capability: V08SupportedAdapterCapability::AdapterPermissionDependency,
        runtime_state: V08SupportedAdapterRuntimeState::Degraded,
        adapter_result: V08SupportedAdapterResult::DegradedPermissionOrDependency,
        platform_support_state: V08SupportedAdapterPlatformSupportState::Degraded,
        target_identity_state: V08SupportedAdapterTargetIdentityState::NotApplicable,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::ManualRequired,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::AuditReferenceBacked,
        refusal_reason: V08SupportedAdapterRefusalReason::PermissionOrDependencyDegraded,
        evidence_refs: &[proof::REF_ADAPTER_CAPABILITY_STATE],
        linked_proof_commands: &[proof::COMMAND_WINDOWS_ADAPTER_CAPABILITY_PROOF],
        linked_proof_artifacts: &[proof::ARTIFACT_WINDOWS_ADAPTER_CAPABILITY_PROOF],
        manual_proof_requirements: &[
            proof::REQUIREMENT_PERMISSION_RESTORE,
            proof::REQUIREMENT_DEPENDENCY_REINSTALL,
            proof::REQUIREMENT_OPERATOR_DEGRADED_STATE,
        ],
        claim_boundary: proof::CLAIM_PERMISSION_DEGRADED,
        fallback_behavior: proof::FALLBACK_PERMISSION_DEGRADED,
    }
}

fn unavailable_spec() -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_LINUX_UNAVAILABLE,
        runtime_boundary: V08SupportedAdapterRuntimeBoundary::LinuxHostAdapterUnavailable,
        platform: ParentPlatform::Linux,
        adapter_capability: V08SupportedAdapterCapability::DesktopHostPlatformAdapter,
        runtime_state: V08SupportedAdapterRuntimeState::Unavailable,
        adapter_result: V08SupportedAdapterResult::TargetUnavailable,
        platform_support_state: V08SupportedAdapterPlatformSupportState::UnavailableOnTarget,
        target_identity_state: V08SupportedAdapterTargetIdentityState::UnsupportedPlatformTarget,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::Unavailable,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::Unavailable,
        refusal_reason: V08SupportedAdapterRefusalReason::TargetUnavailable,
        evidence_refs: &[],
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements: &[
            proof::REQUIREMENT_LINUX_SERVICE,
            proof::REQUIREMENT_LINUX_PERMISSION,
            proof::REQUIREMENT_LINUX_ROLLBACK,
        ],
        claim_boundary: proof::CLAIM_LINUX_UNAVAILABLE,
        fallback_behavior: proof::FALLBACK_LINUX_UNAVAILABLE,
    }
}

fn unsupported_spec() -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_MACOS_UNSUPPORTED,
        runtime_boundary: V08SupportedAdapterRuntimeBoundary::MacosHostAdapterUnsupported,
        platform: ParentPlatform::Macos,
        adapter_capability: V08SupportedAdapterCapability::DesktopHostPlatformAdapter,
        runtime_state: V08SupportedAdapterRuntimeState::Unsupported,
        adapter_result: V08SupportedAdapterResult::UnsupportedPlatform,
        platform_support_state: V08SupportedAdapterPlatformSupportState::UnsupportedPlatform,
        target_identity_state: V08SupportedAdapterTargetIdentityState::UnsupportedPlatformTarget,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::Unavailable,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::Unavailable,
        refusal_reason: V08SupportedAdapterRefusalReason::UnsupportedPlatform,
        evidence_refs: &[],
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements: &[
            proof::REQUIREMENT_MACOS_PERMISSION,
            proof::REQUIREMENT_MACOS_PACKAGE_IDENTITY,
            proof::REQUIREMENT_MACOS_ROLLBACK,
        ],
        claim_boundary: proof::CLAIM_MACOS_UNSUPPORTED,
        fallback_behavior: proof::FALLBACK_MACOS_UNSUPPORTED,
    }
}

#[derive(Clone, Copy)]
struct ArtifactStatusSpecInput {
    proof_entry_id: &'static str,
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    adapter_capability: V08SupportedAdapterCapability,
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

fn artifact_status_spec(input: ArtifactStatusSpecInput) -> EntrySpec {
    EntrySpec {
        proof_entry_id: input.proof_entry_id,
        runtime_boundary: input.runtime_boundary,
        platform: ParentPlatform::Windows,
        adapter_capability: input.adapter_capability,
        runtime_state: V08SupportedAdapterRuntimeState::ManualRequired,
        adapter_result: V08SupportedAdapterResult::ManualProofRequired,
        platform_support_state: V08SupportedAdapterPlatformSupportState::ManualRequired,
        target_identity_state: V08SupportedAdapterTargetIdentityState::InsufficientForBroadTarget,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::ManualRequired,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::ManualRequired,
        refusal_reason: V08SupportedAdapterRefusalReason::ManualArtifactRequired,
        evidence_refs: &[
            proof::REF_WINDOWS_ADAPTER_ARTIFACT_GATE,
            proof::REF_WINDOWS_ADAPTER_ARTIFACT_INGESTION,
        ],
        linked_proof_commands: &[
            proof::COMMAND_WINDOWS_ADAPTER_CAPABILITY_PROOF,
            proof::COMMAND_WINDOWS_ADAPTER_ARTIFACT_GATE,
            proof::COMMAND_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF,
        ],
        linked_proof_artifacts: &[
            proof::ARTIFACT_WINDOWS_ADAPTER_CAPABILITY_PROOF,
            proof::ARTIFACT_WINDOWS_ADAPTER_ARTIFACT_GATE,
            proof::ARTIFACT_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF,
        ],
        manual_proof_requirements: input.manual_proof_requirements,
        claim_boundary: input.claim_boundary,
        fallback_behavior: input.fallback_behavior,
    }
}

fn implemented_spec(input: ImplementedSpecInput) -> EntrySpec {
    let ImplementedSpecInput {
        proof_entry_id,
        runtime_boundary,
        adapter_capability,
        target_identity_state,
        rollback_reference_state,
        evidence_refs,
        linked_proof_commands,
        linked_proof_artifacts,
        claim_boundary,
        fallback_behavior,
    } = input;

    EntrySpec {
        proof_entry_id,
        runtime_boundary,
        platform: ParentPlatform::Windows,
        adapter_capability,
        runtime_state: V08SupportedAdapterRuntimeState::ImplementedBoundary,
        adapter_result: V08SupportedAdapterResult::SupportedBoundaryProved,
        platform_support_state: V08SupportedAdapterPlatformSupportState::SupportedOnWindows,
        target_identity_state,
        rollback_reference_state,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::AuditReferenceBacked,
        refusal_reason: V08SupportedAdapterRefusalReason::None,
        evidence_refs,
        linked_proof_commands,
        linked_proof_artifacts,
        manual_proof_requirements: &[],
        claim_boundary,
        fallback_behavior,
    }
}

fn manual_spec(input: ManualSpecInput) -> EntrySpec {
    let ManualSpecInput {
        proof_entry_id,
        runtime_boundary,
        platform,
        adapter_capability,
        target_identity_state,
        manual_proof_requirements,
        claim_boundary,
        fallback_behavior,
    } = input;

    EntrySpec {
        proof_entry_id,
        runtime_boundary,
        platform,
        adapter_capability,
        runtime_state: V08SupportedAdapterRuntimeState::ManualRequired,
        adapter_result: V08SupportedAdapterResult::ManualProofRequired,
        platform_support_state: V08SupportedAdapterPlatformSupportState::ManualRequired,
        target_identity_state,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::ManualRequired,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::ManualRequired,
        refusal_reason: V08SupportedAdapterRefusalReason::ManualArtifactRequired,
        evidence_refs: &[],
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements,
        claim_boundary,
        fallback_behavior,
    }
}

fn mobile_manual_spec(
    proof_entry_id: ProofEntryId,
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    platform: ParentPlatform,
    manual_proof_requirements: StaticTextRefs,
) -> EntrySpec {
    manual_spec(ManualSpecInput {
        proof_entry_id: proof_entry_id.0,
        runtime_boundary,
        platform,
        adapter_capability: V08SupportedAdapterCapability::MobileChildControlAdapter,
        target_identity_state: V08SupportedAdapterTargetIdentityState::UnsupportedPlatformTarget,
        manual_proof_requirements: manual_proof_requirements.0,
        claim_boundary: proof::CLAIM_MOBILE_MANUAL,
        fallback_behavior: proof::FALLBACK_MOBILE_MANUAL,
    })
}

fn entry_from_spec(
    spec: &EntrySpec,
    generated_at: GeneratedAtTextRef<'_>,
) -> V08SupportedAdapterRuntimeProofEntry {
    V08SupportedAdapterRuntimeProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: spec.proof_entry_id.to_string(),
        runtime_boundary: spec.runtime_boundary,
        platform: spec.platform,
        adapter_capability: spec.adapter_capability,
        runtime_state: spec.runtime_state,
        adapter_result: spec.adapter_result,
        platform_support_state: spec.platform_support_state,
        target_identity_state: spec.target_identity_state,
        rollback_reference_state: spec.rollback_reference_state,
        audit_reference_state: spec.audit_reference_state,
        refusal_reason: spec.refusal_reason,
        evidence_refs: StaticTextRefs(spec.evidence_refs)
            .0
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        linked_proof_commands: StaticTextRefs(spec.linked_proof_commands)
            .0
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        linked_proof_artifacts: StaticTextRefs(spec.linked_proof_artifacts)
            .0
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        manual_proof_requirements: StaticTextRefs(spec.manual_proof_requirements)
            .0
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        claim_boundary: spec.claim_boundary.to_string(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        broad_installed_app_blocking_claimed: false,
        network_domain_blocking_claimed: false,
        exact_active_tab_enforcement_claimed: false,
        notification_delivery_claimed: false,
        tamper_hardening_claimed: false,
        mobile_control_claimed: false,
        unsupported_platform_behavior_claimed: false,
        last_checked_at: generated_at.0.to_string(),
    }
}
