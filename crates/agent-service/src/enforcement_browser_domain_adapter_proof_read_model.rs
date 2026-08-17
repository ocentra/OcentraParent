use ocentra_parent_agent_protocol::constants::v08_browser_domain_adapter_proof as proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterExecutionState;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofCapabilityName;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofCapabilityStatus;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofClaimState;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofEntry;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofEvidenceKind;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofReadModel;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofSurface;
use ocentra_parent_agent_protocol::policy_constants;

use crate::enforcement_browser_domain_adapter_app_control_proof_states::{
    app_control_state_specs, GeneratedAtTextRef,
};
use crate::enforcement_os_adapter_product_proof_read_model::product_control_spine::{
    GeneratedAtText, ProofEntryId,
};

pub(crate) fn v08_browser_domain_adapter_proof_read_model(
    generated_at: impl Into<GeneratedAtText>,
) -> V08BrowserDomainAdapterProofReadModel {
    let generated_at = generated_at.into();
    V08BrowserDomainAdapterProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.0.clone(),
        source_read_model_ids: vec![
            proof::SOURCE_BROAD_OS_PROOF.to_string(),
            proof::SOURCE_CROSS_PLATFORM_PROOF.to_string(),
            proof::SOURCE_OS_PRODUCT_PROOF.to_string(),
            proof::SOURCE_BROWSER_POLICY_RUNTIME.to_string(),
        ],
        windows_app_control_states: app_control_state_specs(GeneratedAtTextRef(
            generated_at.0.as_str(),
        )),
        entries: entry_specs()
            .iter()
            .map(|spec| entry_from_spec(spec, &generated_at))
            .collect(),
    }
}

struct EntrySpec {
    proof_entry_id: ProofEntryId,
    surface: V08BrowserDomainAdapterProofSurface,
    platform: ParentPlatform,
    capability: V08BrowserDomainAdapterProofCapabilityName,
    capability_status: V08BrowserDomainAdapterProofCapabilityStatus,
    evidence_kind: V08BrowserDomainAdapterProofEvidenceKind,
    product_claim_state: V08BrowserDomainAdapterProofClaimState,
    adapter_execution_state: V08BrowserDomainAdapterExecutionState,
    linked_proof_commands: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

#[derive(Clone, Copy)]
struct ProofEvidence {
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
    let mut specs = Vec::new();
    specs.extend(implemented_specs());
    specs.extend(manual_and_unmanaged_gap_specs());
    specs.extend(network_specs());
    specs.extend(unsupported_target_specs());
    specs
}

fn implemented_specs() -> Vec<EntrySpec> {
    vec![
        implemented_spec(
            ProofEntryId(proof::ENTRY_ID_UNMANAGED_TERMINATE),
            V08BrowserDomainAdapterProofSurface::WindowsUnmanagedBrowserTerminateBoundary,
            V08BrowserDomainAdapterProofCapabilityName::UnmanagedBrowserDetection,
            V08BrowserDomainAdapterProofEvidenceKind::UnmanagedBrowser,
            linked_evidence(ProofEvidence {
                linked_proof_commands: &[proof::COMMAND_UNMANAGED_BROWSER_PROOF],
                linked_proof_artifacts: &[proof::ARTIFACT_UNMANAGED_BROWSER_PROOF],
                manual_proof_requirements: &[],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_UNMANAGED_TERMINATE,
                fallback_behavior: proof::FALLBACK_UNMANAGED_TERMINATE,
            }),
        ),
        implemented_spec(
            ProofEntryId(proof::ENTRY_ID_AUDIT_VISIBILITY),
            V08BrowserDomainAdapterProofSurface::WindowsAuditVisibilityBoundary,
            V08BrowserDomainAdapterProofCapabilityName::LocalStorage,
            V08BrowserDomainAdapterProofEvidenceKind::Audit,
            linked_evidence(ProofEvidence {
                linked_proof_commands: &[proof::COMMAND_APP_TIME_LIMIT_PROOF],
                linked_proof_artifacts: &[proof::ARTIFACT_APP_TIME_LIMIT_PROOF],
                manual_proof_requirements: &[],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_AUDIT_VISIBILITY,
                fallback_behavior: proof::FALLBACK_AUDIT_VISIBILITY,
            }),
        ),
        implemented_spec(
            ProofEntryId(proof::ENTRY_ID_RESTART_RECOVERY),
            V08BrowserDomainAdapterProofSurface::WindowsRestartRecoveryVisibilityBoundary,
            V08BrowserDomainAdapterProofCapabilityName::AppTimeLimit,
            V08BrowserDomainAdapterProofEvidenceKind::RestartRecovery,
            linked_evidence(ProofEvidence {
                linked_proof_commands: &[proof::COMMAND_APP_TIME_LIMIT_PROOF],
                linked_proof_artifacts: &[proof::ARTIFACT_APP_TIME_LIMIT_PROOF],
                manual_proof_requirements: &[],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_RESTART_RECOVERY,
                fallback_behavior: proof::FALLBACK_RESTART_RECOVERY,
            }),
        ),
        implemented_spec(
            ProofEntryId(proof::ENTRY_ID_BROWSER_POLICY_ROLLBACK),
            V08BrowserDomainAdapterProofSurface::WindowsBrowserPolicyRollbackVisibility,
            V08BrowserDomainAdapterProofCapabilityName::ManagedBrowserControl,
            V08BrowserDomainAdapterProofEvidenceKind::Rollback,
            linked_evidence(ProofEvidence {
                linked_proof_commands: &[proof::COMMAND_BROWSER_POLICY_ROLLBACK_TEST],
                linked_proof_artifacts: &[proof::ARTIFACT_BROWSER_POLICY_ROLLBACK_TEST],
                manual_proof_requirements: &[],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_BROWSER_POLICY_ROLLBACK,
                fallback_behavior: proof::FALLBACK_BROWSER_POLICY_ROLLBACK,
            }),
        ),
    ]
}

fn manual_and_unmanaged_gap_specs() -> Vec<EntrySpec> {
    vec![
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_MANAGED_INTERVENTION),
            V08BrowserDomainAdapterProofSurface::WindowsManagedBrowserInterventionState,
            ParentPlatform::Windows,
            V08BrowserDomainAdapterProofCapabilityName::ManagedBrowserControl,
            V08BrowserDomainAdapterProofEvidenceKind::ManagedBrowser,
            manual_evidence(ProofEvidence {
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: MANAGED_INTERVENTION_MANUAL_REQUIREMENTS,
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_MANAGED_INTERVENTION,
                fallback_behavior: proof::FALLBACK_MANAGED_INTERVENTION,
            }),
        ),
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_MANAGED_EXACT_URL),
            V08BrowserDomainAdapterProofSurface::WindowsManagedBrowserExactUrlManual,
            ParentPlatform::Windows,
            V08BrowserDomainAdapterProofCapabilityName::ManagedBrowserControl,
            V08BrowserDomainAdapterProofEvidenceKind::ManagedBrowser,
            manual_evidence(ProofEvidence {
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_ACTIVE_TAB,
                    proof::REQUIREMENT_EXACT_URL_APPLY,
                    proof::REQUIREMENT_ROLLBACK,
                    proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_MANAGED_EXACT_URL,
                fallback_behavior: proof::FALLBACK_MANAGED_EXACT_URL,
            }),
        ),
        degraded_spec(
            ProofEntryId(proof::ENTRY_ID_UNMANAGED_WARN),
            V08BrowserDomainAdapterProofSurface::WindowsUnmanagedBrowserWarnNoop,
            V08BrowserDomainAdapterProofCapabilityName::UnmanagedBrowserDetection,
            V08BrowserDomainAdapterProofEvidenceKind::UnmanagedBrowser,
            degraded_evidence(ProofEvidence {
                linked_proof_commands: &[proof::COMMAND_UNMANAGED_BROWSER_PROOF],
                linked_proof_artifacts: &[proof::ARTIFACT_UNMANAGED_WARN_EVENT],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_WARNING_DELIVERY,
                    proof::REQUIREMENT_BROWSER_INTEGRATION,
                ],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_UNMANAGED_WARN,
                fallback_behavior: proof::FALLBACK_UNMANAGED_WARN,
            }),
        ),
        not_claimed_spec(
            ProofEntryId(proof::ENTRY_ID_UNMANAGED_EXACT_EVIDENCE),
            V08BrowserDomainAdapterProofSurface::WindowsUnmanagedBrowserExactEvidenceNotClaimed,
            ParentPlatform::Windows,
            V08BrowserDomainAdapterProofCapabilityName::UnmanagedBrowserDetection,
            V08BrowserDomainAdapterProofEvidenceKind::UnmanagedBrowser,
            manual_evidence(ProofEvidence {
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_MANAGED_PROFILE,
                    proof::REQUIREMENT_BROWSER_EXTENSION,
                    proof::REQUIREMENT_ACTIVE_TAB_CUSTODY,
                ],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_UNMANAGED_EXACT_EVIDENCE,
                fallback_behavior: proof::FALLBACK_UNMANAGED_EXACT_EVIDENCE,
            }),
        ),
    ]
}

const MANAGED_INTERVENTION_MANUAL_REQUIREMENTS: &[&str] = &[
    proof::REQUIREMENT_MANAGED_PROFILE,
    proof::REQUIREMENT_ACTIVE_TAB,
    proof::REQUIREMENT_ROLLBACK,
    proof::REQUIREMENT_AUDIT_CUSTODY,
];

fn network_specs() -> Vec<EntrySpec> {
    vec![
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_NETWORK_FILTER_MANUAL),
            V08BrowserDomainAdapterProofSurface::WindowsNetworkDomainFilterManual,
            ParentPlatform::Windows,
            V08BrowserDomainAdapterProofCapabilityName::NetworkDomainBlocking,
            V08BrowserDomainAdapterProofEvidenceKind::NetworkDomain,
            manual_evidence(ProofEvidence {
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_NETWORK_FILTER,
                    proof::REQUIREMENT_DNS_VPN_APPLY,
                    proof::REQUIREMENT_ROLLBACK,
                    proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_NETWORK_FILTER_MANUAL,
                fallback_behavior: proof::FALLBACK_NETWORK_FILTER_MANUAL,
            }),
        ),
        unavailable_spec(
            ProofEntryId(proof::ENTRY_ID_NETWORK_ADAPTER_UNAVAILABLE),
            V08BrowserDomainAdapterProofSurface::WindowsNetworkDomainAdapterUnavailable,
            ParentPlatform::Windows,
            V08BrowserDomainAdapterProofCapabilityName::NetworkDomainBlocking,
            V08BrowserDomainAdapterProofEvidenceKind::NetworkDomain,
            manual_evidence(ProofEvidence {
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_SERVICE_UNAVAILABLE,
                    proof::REQUIREMENT_ADAPTER_INSTALL,
                    proof::REQUIREMENT_OPERATOR_RETRY,
                ],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_NETWORK_ADAPTER_UNAVAILABLE,
                fallback_behavior: proof::FALLBACK_NETWORK_ADAPTER_UNAVAILABLE,
            }),
        ),
    ]
}

fn unsupported_target_specs() -> Vec<EntrySpec> {
    let mut specs = Vec::new();
    specs.extend(unsupported_desktop_target_specs());
    specs.extend(unsupported_mobile_target_specs());
    specs
}

fn unsupported_desktop_target_specs() -> Vec<EntrySpec> {
    vec![
        unavailable_spec(
            ProofEntryId(proof::ENTRY_ID_LINUX_ADAPTER),
            V08BrowserDomainAdapterProofSurface::LinuxBrowserDomainAdapterUnavailable,
            ParentPlatform::Linux,
            V08BrowserDomainAdapterProofCapabilityName::ManagedBrowserControl,
            V08BrowserDomainAdapterProofEvidenceKind::UnsupportedTarget,
            manual_evidence(ProofEvidence {
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_LINUX_SERVICE,
                    proof::REQUIREMENT_LINUX_ADAPTER,
                ],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_LINUX_ADAPTER,
                fallback_behavior: proof::FALLBACK_LINUX_ADAPTER,
            }),
        ),
        unavailable_spec(
            ProofEntryId(proof::ENTRY_ID_MACOS_ADAPTER),
            V08BrowserDomainAdapterProofSurface::MacosBrowserDomainAdapterUnavailable,
            ParentPlatform::Macos,
            V08BrowserDomainAdapterProofCapabilityName::ManagedBrowserControl,
            V08BrowserDomainAdapterProofEvidenceKind::UnsupportedTarget,
            manual_evidence(ProofEvidence {
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_MACOS_PERMISSION,
                    proof::REQUIREMENT_MACOS_ADAPTER,
                ],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_MACOS_ADAPTER,
                fallback_behavior: proof::FALLBACK_MACOS_ADAPTER,
            }),
        ),
    ]
}

fn unsupported_mobile_target_specs() -> Vec<EntrySpec> {
    vec![
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_ANDROID_ADAPTER),
            V08BrowserDomainAdapterProofSurface::AndroidBrowserDomainAdapterManual,
            ParentPlatform::Android,
            V08BrowserDomainAdapterProofCapabilityName::VpnDnsFiltering,
            V08BrowserDomainAdapterProofEvidenceKind::UnsupportedTarget,
            manual_evidence(ProofEvidence {
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_ANDROID_VPN_DNS,
                    proof::REQUIREMENT_ANDROID_DEVICE_OWNER,
                    proof::REQUIREMENT_ANDROID_PACKAGE,
                ],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_ANDROID_ADAPTER,
                fallback_behavior: proof::FALLBACK_ANDROID_ADAPTER,
            }),
        ),
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_IOS_ADAPTER),
            V08BrowserDomainAdapterProofSurface::IosBrowserDomainAdapterManual,
            ParentPlatform::Ios,
            V08BrowserDomainAdapterProofCapabilityName::NetworkExtension,
            V08BrowserDomainAdapterProofEvidenceKind::UnsupportedTarget,
            manual_evidence(ProofEvidence {
                linked_proof_commands: &[],
                linked_proof_artifacts: &[],
                manual_proof_requirements: &[
                    proof::REQUIREMENT_IOS_NETWORK_EXTENSION,
                    proof::REQUIREMENT_IOS_FAMILY_DEVICE,
                    proof::REQUIREMENT_IOS_TESTFLIGHT,
                ],
            }),
            boundary_text(BoundaryText {
                claim_boundary: proof::CLAIM_IOS_ADAPTER,
                fallback_behavior: proof::FALLBACK_IOS_ADAPTER,
            }),
        ),
    ]
}

fn linked_evidence(evidence: ProofEvidence) -> ProofEvidence {
    evidence
}

fn degraded_evidence(evidence: ProofEvidence) -> ProofEvidence {
    evidence
}

fn manual_evidence(evidence: ProofEvidence) -> ProofEvidence {
    evidence
}

fn boundary_text(text: BoundaryText) -> BoundaryText {
    text
}

fn implemented_spec(
    proof_entry_id: ProofEntryId,
    surface: V08BrowserDomainAdapterProofSurface,
    capability: V08BrowserDomainAdapterProofCapabilityName,
    evidence_kind: V08BrowserDomainAdapterProofEvidenceKind,
    evidence: ProofEvidence,
    text: BoundaryText,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
        surface,
        platform: ParentPlatform::Windows,
        capability,
        capability_status: V08BrowserDomainAdapterProofCapabilityStatus::Implemented,
        evidence_kind,
        product_claim_state: V08BrowserDomainAdapterProofClaimState::ImplementedBoundary,
        adapter_execution_state: V08BrowserDomainAdapterExecutionState::ExecutesRealService,
        linked_proof_commands: evidence.linked_proof_commands,
        linked_proof_artifacts: evidence.linked_proof_artifacts,
        manual_proof_requirements: evidence.manual_proof_requirements,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn degraded_spec(
    proof_entry_id: ProofEntryId,
    surface: V08BrowserDomainAdapterProofSurface,
    capability: V08BrowserDomainAdapterProofCapabilityName,
    evidence_kind: V08BrowserDomainAdapterProofEvidenceKind,
    evidence: ProofEvidence,
    text: BoundaryText,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
        surface,
        platform: ParentPlatform::Windows,
        capability,
        capability_status: V08BrowserDomainAdapterProofCapabilityStatus::Supported,
        evidence_kind,
        product_claim_state: V08BrowserDomainAdapterProofClaimState::DegradedBoundary,
        adapter_execution_state: V08BrowserDomainAdapterExecutionState::ReturnsDegradedNoop,
        linked_proof_commands: evidence.linked_proof_commands,
        linked_proof_artifacts: evidence.linked_proof_artifacts,
        manual_proof_requirements: evidence.manual_proof_requirements,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn manual_spec(
    proof_entry_id: ProofEntryId,
    surface: V08BrowserDomainAdapterProofSurface,
    platform: ParentPlatform,
    capability: V08BrowserDomainAdapterProofCapabilityName,
    evidence_kind: V08BrowserDomainAdapterProofEvidenceKind,
    evidence: ProofEvidence,
    text: BoundaryText,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
        surface,
        platform,
        capability,
        capability_status: V08BrowserDomainAdapterProofCapabilityStatus::ManualRequired,
        evidence_kind,
        product_claim_state: V08BrowserDomainAdapterProofClaimState::ManualRequired,
        adapter_execution_state: V08BrowserDomainAdapterExecutionState::ReturnsManualRequired,
        linked_proof_commands: evidence.linked_proof_commands,
        linked_proof_artifacts: evidence.linked_proof_artifacts,
        manual_proof_requirements: evidence.manual_proof_requirements,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn unavailable_spec(
    proof_entry_id: ProofEntryId,
    surface: V08BrowserDomainAdapterProofSurface,
    platform: ParentPlatform,
    capability: V08BrowserDomainAdapterProofCapabilityName,
    evidence_kind: V08BrowserDomainAdapterProofEvidenceKind,
    evidence: ProofEvidence,
    text: BoundaryText,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
        surface,
        platform,
        capability,
        capability_status: V08BrowserDomainAdapterProofCapabilityStatus::Unavailable,
        evidence_kind,
        product_claim_state: V08BrowserDomainAdapterProofClaimState::Unavailable,
        adapter_execution_state: V08BrowserDomainAdapterExecutionState::ReturnsUnavailable,
        linked_proof_commands: evidence.linked_proof_commands,
        linked_proof_artifacts: evidence.linked_proof_artifacts,
        manual_proof_requirements: evidence.manual_proof_requirements,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn not_claimed_spec(
    proof_entry_id: ProofEntryId,
    surface: V08BrowserDomainAdapterProofSurface,
    platform: ParentPlatform,
    capability: V08BrowserDomainAdapterProofCapabilityName,
    evidence_kind: V08BrowserDomainAdapterProofEvidenceKind,
    evidence: ProofEvidence,
    text: BoundaryText,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
        surface,
        platform,
        capability,
        capability_status: V08BrowserDomainAdapterProofCapabilityStatus::NotImplemented,
        evidence_kind,
        product_claim_state: V08BrowserDomainAdapterProofClaimState::NotClaimed,
        adapter_execution_state: V08BrowserDomainAdapterExecutionState::NotInvoked,
        linked_proof_commands: evidence.linked_proof_commands,
        linked_proof_artifacts: evidence.linked_proof_artifacts,
        manual_proof_requirements: evidence.manual_proof_requirements,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn entry_from_spec(
    spec: &EntrySpec,
    generated_at: &GeneratedAtText,
) -> V08BrowserDomainAdapterProofEntry {
    V08BrowserDomainAdapterProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: spec.proof_entry_id.0.to_string(),
        surface: spec.surface,
        platform: spec.platform,
        capability: spec.capability,
        capability_status: spec.capability_status,
        evidence_kind: spec.evidence_kind,
        product_claim_state: spec.product_claim_state,
        adapter_execution_state: spec.adapter_execution_state,
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
        managed_exact_url_claimed: false,
        unmanaged_exact_url_claimed: false,
        network_domain_blocking_claimed: false,
        broad_browser_control_claimed: false,
        unsupported_os_claimed: false,
        last_checked_at: generated_at.0.clone(),
    }
}
