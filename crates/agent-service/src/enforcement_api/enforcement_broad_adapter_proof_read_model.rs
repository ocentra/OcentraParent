use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::constants::enforcement_broad_adapter_proof as proof;
use ocentra_parent_agent_protocol::constants::v08_browser_domain_adapter_proof as browser_proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeClaimState;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeEvidenceState;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofEntry;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofReadModel;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeSurface;
use ocentra_parent_agent_protocol::policy_constants;

#[derive(Clone, Copy)]
pub(crate) struct GeneratedAtTextRef<'a>(pub(crate) &'a str);

#[derive(Clone, Copy)]
struct ProofEntryId(pub(crate) &'static str);

pub(crate) fn v08_broad_adapter_proof_read_model<'a>(
    generated_at: impl Into<GeneratedAtTextRef<'a>>,
) -> V08BroadAdapterRuntimeProofReadModel {
    let generated_at = generated_at.into();
    V08BroadAdapterRuntimeProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.0.to_string(),
        source_read_model_ids: vec![
            proof::SOURCE_BROAD_OS_ADAPTER_PROOF.to_string(),
            proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF.to_string(),
            proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES.to_string(),
            proof::SOURCE_OS_ADAPTER_PRODUCT_PROOF.to_string(),
        ],
        entries: entry_specs()
            .iter()
            .map(|spec| entry_from_spec(spec, generated_at))
            .collect(),
    }
}

struct EntrySpec {
    proof_entry_id: &'static str,
    runtime_surface: V08BroadAdapterRuntimeSurface,
    platform: ParentPlatform,
    product_claim_state: V08BroadAdapterRuntimeClaimState,
    evidence_state: V08BroadAdapterRuntimeEvidenceState,
    source_proof_ids: &'static [&'static str],
    linked_proof_commands: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

#[derive(Clone, Copy)]
struct EvidenceRefs {
    source_proof_ids: &'static [&'static str],
    linked_proof_commands: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
}

#[derive(Clone, Copy)]
struct BoundaryText {
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

fn entry_specs() -> Vec<EntrySpec> {
    let mut specs = implemented_boundary_specs();
    specs.extend(windows_manual_gate_specs());
    specs.push(managed_exact_url_gate_spec());
    specs.push(unmanaged_exact_evidence_gap_spec());
    specs.extend(desktop_platform_gap_specs());
    specs.extend(mobile_platform_gap_specs());
    specs
}

fn implemented_boundary_specs() -> Vec<EntrySpec> {
    vec![
        entry_spec(
            ProofEntryId(proof::ENTRY_ID_OWNED_PROCESS_TIMER),
            V08BroadAdapterRuntimeSurface::WindowsOwnedProcessAndTimerRuntimeBoundary,
            ParentPlatform::Windows,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            EvidenceRefs {
                source_proof_ids: &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_PRODUCT_PROOF,
                ],
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: &[
                    enforcement_constants::ARTIFACT_APP_TIME_LIMIT_EXECUTOR,
                    proof::REQUIREMENT_ROLLBACK,
                    proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
            },
            BoundaryText {
                claim_boundary: proof::CLAIM_OWNED_PROCESS_TIMER,
                fallback_behavior: proof::FALLBACK_OWNED_PROCESS_TIMER,
            },
        ),
        entry_spec(
            ProofEntryId(proof::ENTRY_ID_MANAGED_BROWSER_SESSION),
            V08BroadAdapterRuntimeSurface::WindowsManagedBrowserSessionRuntimeBoundary,
            ParentPlatform::Windows,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            EvidenceRefs {
                source_proof_ids: &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF,
                ],
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: &[
                    browser_proof::REQUIREMENT_MANAGED_PROFILE,
                    browser_proof::REQUIREMENT_ACTIVE_TAB,
                    browser_proof::REQUIREMENT_ROLLBACK,
                    browser_proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
            },
            BoundaryText {
                claim_boundary: proof::CLAIM_MANAGED_BROWSER_SESSION,
                fallback_behavior: proof::FALLBACK_MANAGED_BROWSER_SESSION,
            },
        ),
    ]
}

fn windows_manual_gate_specs() -> Vec<EntrySpec> {
    vec![
        entry_spec(
            ProofEntryId(proof::ENTRY_ID_BROAD_INSTALLED_APP_GATE),
            V08BroadAdapterRuntimeSurface::WindowsBroadInstalledAppRuntimeGate,
            ParentPlatform::Windows,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            EvidenceRefs {
                source_proof_ids: &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
                ],
                linked_proof_commands: &[proof::COMMAND_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                linked_proof_artifacts: &[proof::ARTIFACT_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_SAME_APP_IDENTITY,
                    proof::REQUIREMENT_HOST_BLOCK_APPLY,
                    proof::REQUIREMENT_ROLLBACK,
                    proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
            },
            BoundaryText {
                claim_boundary: proof::CLAIM_BROAD_INSTALLED_APP_GATE,
                fallback_behavior: proof::FALLBACK_BROAD_INSTALLED_APP_GATE,
            },
        ),
        entry_spec(
            ProofEntryId(proof::ENTRY_ID_NETWORK_DOMAIN_GATE),
            V08BroadAdapterRuntimeSurface::WindowsNetworkDomainRuntimeGate,
            ParentPlatform::Windows,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            EvidenceRefs {
                source_proof_ids: &[
                    proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
                ],
                linked_proof_commands: &[proof::COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF],
                linked_proof_artifacts: &[proof::ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_HOST_DNS_OR_FILTER_APPLY,
                    proof::REQUIREMENT_ROLLBACK,
                    proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
            },
            BoundaryText {
                claim_boundary: proof::CLAIM_NETWORK_DOMAIN_GATE,
                fallback_behavior: proof::FALLBACK_NETWORK_DOMAIN_GATE,
            },
        ),
    ]
}

fn managed_exact_url_gate_spec() -> EntrySpec {
    entry_spec(
        ProofEntryId(proof::ENTRY_ID_MANAGED_EXACT_URL_GATE),
        V08BroadAdapterRuntimeSurface::WindowsManagedBrowserExactUrlRuntimeGate,
        ParentPlatform::Windows,
        V08BroadAdapterRuntimeClaimState::ManualRequired,
        V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
        EvidenceRefs {
            source_proof_ids: &[
                proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF,
                proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
            ],
            linked_proof_commands: &[proof::COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF],
            linked_proof_artifacts: &[proof::ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF],
            manual_proof_requirements: &[
                proof::REQUIREMENT_ACTIVE_TAB,
                proof::REQUIREMENT_EXACT_URL_APPLY,
                proof::REQUIREMENT_ROLLBACK,
            ],
        },
        BoundaryText {
            claim_boundary: proof::CLAIM_MANAGED_EXACT_URL_GATE,
            fallback_behavior: proof::FALLBACK_MANAGED_EXACT_URL_GATE,
        },
    )
}

fn unmanaged_exact_evidence_gap_spec() -> EntrySpec {
    entry_spec(
        ProofEntryId(proof::ENTRY_ID_UNMANAGED_EXACT_EVIDENCE_GAP),
        V08BroadAdapterRuntimeSurface::WindowsUnmanagedBrowserExactEvidenceRuntimeGap,
        ParentPlatform::Windows,
        V08BroadAdapterRuntimeClaimState::NotClaimed,
        V08BroadAdapterRuntimeEvidenceState::NotImplemented,
        EvidenceRefs {
            source_proof_ids: &[
                proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF,
            ],
            linked_proof_commands: &[proof::COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF],
            linked_proof_artifacts: &[proof::ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF],
            manual_proof_requirements: &[proof::REQUIREMENT_BROWSER_INTEGRATION],
        },
        BoundaryText {
            claim_boundary: proof::CLAIM_UNMANAGED_EXACT_EVIDENCE_GAP,
            fallback_behavior: proof::FALLBACK_UNMANAGED_EXACT_EVIDENCE_GAP,
        },
    )
}

fn desktop_platform_gap_specs() -> Vec<EntrySpec> {
    vec![
        entry_spec(
            ProofEntryId(proof::ENTRY_ID_LINUX_UNAVAILABLE),
            V08BroadAdapterRuntimeSurface::LinuxHostRuntimeUnavailable,
            ParentPlatform::Linux,
            V08BroadAdapterRuntimeClaimState::Unavailable,
            V08BroadAdapterRuntimeEvidenceState::TargetUnavailable,
            EvidenceRefs {
                source_proof_ids: &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF,
                ],
                linked_proof_commands: &[proof::COMMAND_BROAD_OS_ADAPTER_PROOF],
                linked_proof_artifacts: &[proof::ARTIFACT_BROAD_OS_ADAPTER_PROOF],
                manual_proof_requirements: &[proof::REQUIREMENT_LINUX_HOST],
            },
            BoundaryText {
                claim_boundary: proof::CLAIM_LINUX_UNAVAILABLE,
                fallback_behavior: proof::FALLBACK_LINUX_UNAVAILABLE,
            },
        ),
        entry_spec(
            ProofEntryId(proof::ENTRY_ID_MACOS_MANUAL_GATE),
            V08BroadAdapterRuntimeSurface::MacosHostRuntimeManualGate,
            ParentPlatform::Macos,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            EvidenceRefs {
                source_proof_ids: &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
                ],
                linked_proof_commands: &[proof::COMMAND_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                linked_proof_artifacts: &[proof::ARTIFACT_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                manual_proof_requirements: &[proof::REQUIREMENT_MACOS_HOST],
            },
            BoundaryText {
                claim_boundary: proof::CLAIM_MACOS_MANUAL_GATE,
                fallback_behavior: proof::FALLBACK_MACOS_MANUAL_GATE,
            },
        ),
    ]
}

fn mobile_platform_gap_specs() -> Vec<EntrySpec> {
    vec![
        entry_spec(
            ProofEntryId(proof::ENTRY_ID_ANDROID_MANUAL_GATE),
            V08BroadAdapterRuntimeSurface::AndroidMobileRuntimeManualGate,
            ParentPlatform::Android,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            EvidenceRefs {
                source_proof_ids: &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
                ],
                linked_proof_commands: &[proof::COMMAND_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                linked_proof_artifacts: &[proof::ARTIFACT_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_ANDROID_DEVICE_OWNER,
                    proof::REQUIREMENT_ANDROID_USAGE_STATS,
                    proof::REQUIREMENT_ANDROID_ACCESSIBILITY_VPN_DNS,
                    proof::REQUIREMENT_ANDROID_PACKAGE_LIFECYCLE,
                ],
            },
            BoundaryText {
                claim_boundary: proof::CLAIM_ANDROID_MANUAL_GATE,
                fallback_behavior: proof::FALLBACK_ANDROID_MANUAL_GATE,
            },
        ),
        entry_spec(
            ProofEntryId(proof::ENTRY_ID_IOS_MANUAL_GATE),
            V08BroadAdapterRuntimeSurface::IosMobileRuntimeManualGate,
            ParentPlatform::Ios,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            EvidenceRefs {
                source_proof_ids: &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
                ],
                linked_proof_commands: &[proof::COMMAND_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                linked_proof_artifacts: &[proof::ARTIFACT_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_IOS_FAMILY_CONTROLS,
                    proof::REQUIREMENT_IOS_DEVICE_ACTIVITY,
                    proof::REQUIREMENT_IOS_NETWORK_EXTENSION,
                    proof::REQUIREMENT_IOS_SIGNING_TESTFLIGHT,
                ],
            },
            BoundaryText {
                claim_boundary: proof::CLAIM_IOS_MANUAL_GATE,
                fallback_behavior: proof::FALLBACK_IOS_MANUAL_GATE,
            },
        ),
    ]
}

fn entry_spec(
    proof_entry_id: ProofEntryId,
    runtime_surface: V08BroadAdapterRuntimeSurface,
    platform: ParentPlatform,
    product_claim_state: V08BroadAdapterRuntimeClaimState,
    evidence_state: V08BroadAdapterRuntimeEvidenceState,
    evidence: EvidenceRefs,
    text: BoundaryText,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof_entry_id.0,
        runtime_surface,
        platform,
        product_claim_state,
        evidence_state,
        source_proof_ids: evidence.source_proof_ids,
        linked_proof_commands: evidence.linked_proof_commands,
        linked_proof_artifacts: evidence.linked_proof_artifacts,
        manual_proof_requirements: evidence.manual_proof_requirements,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn entry_from_spec(
    spec: &EntrySpec,
    generated_at: GeneratedAtTextRef<'_>,
) -> V08BroadAdapterRuntimeProofEntry {
    V08BroadAdapterRuntimeProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: spec.proof_entry_id.to_string(),
        runtime_surface: spec.runtime_surface,
        platform: spec.platform,
        product_claim_state: spec.product_claim_state,
        evidence_state: spec.evidence_state,
        source_proof_ids: spec
            .source_proof_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        linked_proof_commands: spec
            .linked_proof_commands
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        linked_proof_artifacts: spec
            .linked_proof_artifacts
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        manual_proof_requirements: spec
            .manual_proof_requirements
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        claim_boundary: spec.claim_boundary.to_string(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        broad_installed_app_blocking_claimed: false,
        network_domain_blocking_claimed: false,
        managed_browser_exact_url_claimed: false,
        unmanaged_browser_exact_evidence_claimed: false,
        unsupported_platform_claimed: false,
        mobile_privilege_claimed: false,
        last_checked_at: generated_at.0.to_string(),
    }
}
