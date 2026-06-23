use ocentra_parent_agent_protocol::constants::enforcement_broad_adapter_proof as proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeClaimState;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeEvidenceState;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofEntry;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofReadModel;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeSurface;
use ocentra_parent_agent_protocol::policy_constants;

pub(crate) fn v08_broad_adapter_proof_read_model(
    generated_at: &str,
) -> V08BroadAdapterRuntimeProofReadModel {
    V08BroadAdapterRuntimeProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.to_string(),
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
            proof::ENTRY_ID_OWNED_PROCESS_TIMER,
            V08BroadAdapterRuntimeSurface::WindowsOwnedProcessAndTimerRuntimeBoundary,
            ParentPlatform::Windows,
            V08BroadAdapterRuntimeClaimState::ImplementedBoundary,
            V08BroadAdapterRuntimeEvidenceState::CompositeRuntimeProof,
            linked_evidence(
                &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_PRODUCT_PROOF,
                ],
                &[
                    proof::COMMAND_BROAD_OS_ADAPTER_PROOF,
                    proof::COMMAND_OS_ADAPTER_PRODUCT_PROOF_CARGO,
                ],
                &[
                    proof::ARTIFACT_BROAD_OS_ADAPTER_PROOF,
                    proof::ARTIFACT_OS_ADAPTER_PRODUCT_PROOF_SERVICE,
                ],
            ),
            boundary_text(
                proof::CLAIM_OWNED_PROCESS_TIMER,
                proof::FALLBACK_OWNED_PROCESS_TIMER,
            ),
        ),
        entry_spec(
            proof::ENTRY_ID_MANAGED_BROWSER_SESSION,
            V08BroadAdapterRuntimeSurface::WindowsManagedBrowserSessionRuntimeBoundary,
            ParentPlatform::Windows,
            V08BroadAdapterRuntimeClaimState::ImplementedBoundary,
            V08BroadAdapterRuntimeEvidenceState::CompositeRuntimeProof,
            linked_evidence(
                &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF,
                ],
                &[
                    proof::COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF,
                    proof::COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF_CARGO,
                ],
                &[
                    proof::ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF,
                    proof::ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF_SERVICE,
                ],
            ),
            boundary_text(
                proof::CLAIM_MANAGED_BROWSER_SESSION,
                proof::FALLBACK_MANAGED_BROWSER_SESSION,
            ),
        ),
    ]
}

fn windows_manual_gate_specs() -> Vec<EntrySpec> {
    vec![
        entry_spec(
            proof::ENTRY_ID_BROAD_INSTALLED_APP_GATE,
            V08BroadAdapterRuntimeSurface::WindowsBroadInstalledAppRuntimeGate,
            ParentPlatform::Windows,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            manual_evidence(
                &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
                ],
                &[proof::COMMAND_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                &[proof::ARTIFACT_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                &[
                    proof::REQUIREMENT_SAME_APP_IDENTITY,
                    proof::REQUIREMENT_HOST_BLOCK_APPLY,
                    proof::REQUIREMENT_ROLLBACK,
                    proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
            ),
            boundary_text(
                proof::CLAIM_BROAD_INSTALLED_APP_GATE,
                proof::FALLBACK_BROAD_INSTALLED_APP_GATE,
            ),
        ),
        entry_spec(
            proof::ENTRY_ID_NETWORK_DOMAIN_GATE,
            V08BroadAdapterRuntimeSurface::WindowsNetworkDomainRuntimeGate,
            ParentPlatform::Windows,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            manual_evidence(
                &[
                    proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
                ],
                &[proof::COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF],
                &[proof::ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF],
                &[
                    proof::REQUIREMENT_HOST_DNS_OR_FILTER_APPLY,
                    proof::REQUIREMENT_ROLLBACK,
                    proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
            ),
            boundary_text(
                proof::CLAIM_NETWORK_DOMAIN_GATE,
                proof::FALLBACK_NETWORK_DOMAIN_GATE,
            ),
        ),
    ]
}

fn managed_exact_url_gate_spec() -> EntrySpec {
    entry_spec(
        proof::ENTRY_ID_MANAGED_EXACT_URL_GATE,
        V08BroadAdapterRuntimeSurface::WindowsManagedBrowserExactUrlRuntimeGate,
        ParentPlatform::Windows,
        V08BroadAdapterRuntimeClaimState::ManualRequired,
        V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
        manual_evidence(
            &[
                proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF,
                proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
            ],
            &[proof::COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF],
            &[proof::ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF],
            &[
                proof::REQUIREMENT_ACTIVE_TAB,
                proof::REQUIREMENT_EXACT_URL_APPLY,
                proof::REQUIREMENT_ROLLBACK,
            ],
        ),
        boundary_text(
            proof::CLAIM_MANAGED_EXACT_URL_GATE,
            proof::FALLBACK_MANAGED_EXACT_URL_GATE,
        ),
    )
}

fn unmanaged_exact_evidence_gap_spec() -> EntrySpec {
    entry_spec(
        proof::ENTRY_ID_UNMANAGED_EXACT_EVIDENCE_GAP,
        V08BroadAdapterRuntimeSurface::WindowsUnmanagedBrowserExactEvidenceRuntimeGap,
        ParentPlatform::Windows,
        V08BroadAdapterRuntimeClaimState::NotClaimed,
        V08BroadAdapterRuntimeEvidenceState::NotImplemented,
        manual_evidence(
            &[
                proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF,
            ],
            &[proof::COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF],
            &[proof::ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF],
            &[proof::REQUIREMENT_BROWSER_INTEGRATION],
        ),
        boundary_text(
            proof::CLAIM_UNMANAGED_EXACT_EVIDENCE_GAP,
            proof::FALLBACK_UNMANAGED_EXACT_EVIDENCE_GAP,
        ),
    )
}

fn desktop_platform_gap_specs() -> Vec<EntrySpec> {
    vec![
        entry_spec(
            proof::ENTRY_ID_LINUX_UNAVAILABLE,
            V08BroadAdapterRuntimeSurface::LinuxHostRuntimeUnavailable,
            ParentPlatform::Linux,
            V08BroadAdapterRuntimeClaimState::Unavailable,
            V08BroadAdapterRuntimeEvidenceState::TargetUnavailable,
            manual_evidence(
                &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF,
                ],
                &[proof::COMMAND_BROAD_OS_ADAPTER_PROOF],
                &[proof::ARTIFACT_BROAD_OS_ADAPTER_PROOF],
                &[proof::REQUIREMENT_LINUX_HOST],
            ),
            boundary_text(
                proof::CLAIM_LINUX_UNAVAILABLE,
                proof::FALLBACK_LINUX_UNAVAILABLE,
            ),
        ),
        entry_spec(
            proof::ENTRY_ID_MACOS_MANUAL_GATE,
            V08BroadAdapterRuntimeSurface::MacosHostRuntimeManualGate,
            ParentPlatform::Macos,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            manual_evidence(
                &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
                ],
                &[proof::COMMAND_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                &[proof::ARTIFACT_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                &[proof::REQUIREMENT_MACOS_HOST],
            ),
            boundary_text(
                proof::CLAIM_MACOS_MANUAL_GATE,
                proof::FALLBACK_MACOS_MANUAL_GATE,
            ),
        ),
    ]
}

fn mobile_platform_gap_specs() -> Vec<EntrySpec> {
    vec![
        entry_spec(
            proof::ENTRY_ID_ANDROID_MANUAL_GATE,
            V08BroadAdapterRuntimeSurface::AndroidMobileRuntimeManualGate,
            ParentPlatform::Android,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            manual_evidence(
                &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
                ],
                &[proof::COMMAND_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                &[proof::ARTIFACT_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                &[
                    proof::REQUIREMENT_ANDROID_DEVICE_OWNER,
                    proof::REQUIREMENT_ANDROID_USAGE_STATS,
                    proof::REQUIREMENT_ANDROID_ACCESSIBILITY_VPN_DNS,
                    proof::REQUIREMENT_ANDROID_PACKAGE_LIFECYCLE,
                ],
            ),
            boundary_text(
                proof::CLAIM_ANDROID_MANUAL_GATE,
                proof::FALLBACK_ANDROID_MANUAL_GATE,
            ),
        ),
        entry_spec(
            proof::ENTRY_ID_IOS_MANUAL_GATE,
            V08BroadAdapterRuntimeSurface::IosMobileRuntimeManualGate,
            ParentPlatform::Ios,
            V08BroadAdapterRuntimeClaimState::ManualRequired,
            V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            manual_evidence(
                &[
                    proof::SOURCE_BROAD_OS_ADAPTER_PROOF,
                    proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES,
                ],
                &[proof::COMMAND_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                &[proof::ARTIFACT_OS_ADAPTER_MANUAL_ARTIFACT_GATES],
                &[
                    proof::REQUIREMENT_IOS_FAMILY_CONTROLS,
                    proof::REQUIREMENT_IOS_DEVICE_ACTIVITY,
                    proof::REQUIREMENT_IOS_NETWORK_EXTENSION,
                    proof::REQUIREMENT_IOS_SIGNING_TESTFLIGHT,
                ],
            ),
            boundary_text(
                proof::CLAIM_IOS_MANUAL_GATE,
                proof::FALLBACK_IOS_MANUAL_GATE,
            ),
        ),
    ]
}

fn linked_evidence(
    source_proof_ids: &'static [&'static str],
    linked_proof_commands: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
) -> EvidenceRefs {
    EvidenceRefs {
        source_proof_ids,
        linked_proof_commands,
        linked_proof_artifacts,
        manual_proof_requirements: &[],
    }
}

fn manual_evidence(
    source_proof_ids: &'static [&'static str],
    linked_proof_commands: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
) -> EvidenceRefs {
    EvidenceRefs {
        source_proof_ids,
        linked_proof_commands,
        linked_proof_artifacts,
        manual_proof_requirements,
    }
}

fn boundary_text(claim_boundary: &'static str, fallback_behavior: &'static str) -> BoundaryText {
    BoundaryText {
        claim_boundary,
        fallback_behavior,
    }
}

fn entry_spec(
    proof_entry_id: &'static str,
    runtime_surface: V08BroadAdapterRuntimeSurface,
    platform: ParentPlatform,
    product_claim_state: V08BroadAdapterRuntimeClaimState,
    evidence_state: V08BroadAdapterRuntimeEvidenceState,
    evidence: EvidenceRefs,
    text: BoundaryText,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
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

fn entry_from_spec(spec: &EntrySpec, generated_at: &str) -> V08BroadAdapterRuntimeProofEntry {
    V08BroadAdapterRuntimeProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: spec.proof_entry_id.to_string(),
        runtime_surface: spec.runtime_surface,
        platform: spec.platform,
        product_claim_state: spec.product_claim_state,
        evidence_state: spec.evidence_state,
        source_proof_ids: to_strings(spec.source_proof_ids),
        linked_proof_commands: to_strings(spec.linked_proof_commands),
        linked_proof_artifacts: to_strings(spec.linked_proof_artifacts),
        manual_proof_requirements: to_strings(spec.manual_proof_requirements),
        claim_boundary: spec.claim_boundary.to_string(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        broad_installed_app_blocking_claimed: false,
        network_domain_blocking_claimed: false,
        managed_browser_exact_url_claimed: false,
        unmanaged_browser_exact_evidence_claimed: false,
        unsupported_platform_claimed: false,
        mobile_privilege_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

fn to_strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
