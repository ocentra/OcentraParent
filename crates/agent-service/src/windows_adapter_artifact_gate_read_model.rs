use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants::{
    windows_adapter_artifact_gate as artifact_gate, windows_adapter_capability as windows_adapter,
};
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::windows_adapter_artifact_gate::{
    WindowsAdapterArtifactEvidence, WindowsAdapterArtifactGateDecision,
    WindowsAdapterArtifactGateEntry, WindowsAdapterArtifactGateProof, WindowsAdapterArtifactKind,
};
use ocentra_parent_agent_protocol::windows_adapter_capability::{
    WindowsAdapterCapabilityProof, WindowsAdapterCapabilityProofEntry,
    WindowsAdapterCapabilitySurface,
};

use crate::host_identity_read_model::GeneratedAtText;
use crate::windows_adapter_capability_read_model::windows_adapter_capability_proof;

const APP_REQUIRED_ARTIFACTS: &[WindowsAdapterArtifactKind] = &[
    WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence,
    WindowsAdapterArtifactKind::AdapterApplyResult,
    WindowsAdapterArtifactKind::AdapterRollbackResult,
    WindowsAdapterArtifactKind::AuditCustodyEvent,
];

const DOMAIN_REQUIRED_ARTIFACTS: &[WindowsAdapterArtifactKind] = &[
    WindowsAdapterArtifactKind::NetworkDomainFilterApply,
    WindowsAdapterArtifactKind::NetworkDomainFilterRollback,
    WindowsAdapterArtifactKind::AuditCustodyEvent,
];

const MANAGED_BROWSER_REQUIRED_ARTIFACTS: &[WindowsAdapterArtifactKind] = &[
    WindowsAdapterArtifactKind::ManagedBrowserExactUrlEvidence,
    WindowsAdapterArtifactKind::AuditCustodyEvent,
];

const ROLLBACK_AUDIT_REQUIRED_ARTIFACTS: &[WindowsAdapterArtifactKind] = &[
    WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence,
    WindowsAdapterArtifactKind::AdapterApplyResult,
    WindowsAdapterArtifactKind::AdapterRollbackResult,
    WindowsAdapterArtifactKind::AuditCustodyEvent,
];

#[derive(Clone, Copy)]
pub(crate) struct ArtifactGateGeneratedAtTextRef<'a>(pub(crate) &'a str);

#[derive(Clone, Copy)]
struct GateEntryId(&'static str);

#[derive(Clone, Copy)]
struct ProductClaimBoundary(&'static str);

#[derive(Clone, Copy)]
struct RefusalReason(&'static str);

pub(crate) fn windows_adapter_artifact_gate_proof(
    generated_at: ArtifactGateGeneratedAtTextRef<'_>,
) -> WindowsAdapterArtifactGateProof {
    evaluate_windows_adapter_artifact_gate(generated_at, &[])
}

pub(crate) fn evaluate_windows_adapter_artifact_gate(
    generated_at: ArtifactGateGeneratedAtTextRef<'_>,
    artifacts: &[WindowsAdapterArtifactEvidence],
) -> WindowsAdapterArtifactGateProof {
    let capability = windows_adapter_capability_proof(GeneratedAtText(generated_at.0.to_string()));

    WindowsAdapterArtifactGateProof {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: artifact_gate::READ_MODEL_ID_V0_8.to_string(),
        generated_at: generated_at.0.to_string(),
        capability_read_model_id: windows_adapter::READ_MODEL_ID_V0_8.to_string(),
        entries: artifact_gate_entries(&capability, artifacts, generated_at),
    }
}

fn artifact_gate_entries(
    capability: &WindowsAdapterCapabilityProof,
    artifacts: &[WindowsAdapterArtifactEvidence],
    generated_at: ArtifactGateGeneratedAtTextRef<'_>,
) -> Vec<WindowsAdapterArtifactGateEntry> {
    vec![
        artifact_gate_entry(&app_target_spec(), capability, artifacts, generated_at),
        artifact_gate_entry(
            &domain_network_target_spec(),
            capability,
            artifacts,
            generated_at,
        ),
        artifact_gate_entry(
            &managed_browser_target_spec(),
            capability,
            artifacts,
            generated_at,
        ),
        artifact_gate_entry(
            &unmanaged_browser_target_spec(),
            capability,
            artifacts,
            generated_at,
        ),
        artifact_gate_entry(
            &unsupported_os_target_spec(),
            capability,
            artifacts,
            generated_at,
        ),
        artifact_gate_entry(
            &rollback_audit_target_spec(),
            capability,
            artifacts,
            generated_at,
        ),
    ]
}

struct ArtifactGateSpec {
    gate_entry_id: GateEntryId,
    surface: WindowsAdapterCapabilitySurface,
    required_artifact_kinds: &'static [WindowsAdapterArtifactKind],
    product_claim_boundary: ProductClaimBoundary,
    refusal_reason: RefusalReason,
    unsupported_surface: bool,
}

fn gate_spec(
    gate_entry_id: GateEntryId,
    surface: WindowsAdapterCapabilitySurface,
    required_artifact_kinds: &'static [WindowsAdapterArtifactKind],
    product_claim_boundary: ProductClaimBoundary,
    refusal_reason: RefusalReason,
    unsupported_surface: bool,
) -> ArtifactGateSpec {
    ArtifactGateSpec {
        gate_entry_id,
        surface,
        required_artifact_kinds,
        product_claim_boundary,
        refusal_reason,
        unsupported_surface,
    }
}

fn app_target_spec() -> ArtifactGateSpec {
    gate_spec(
        GateEntryId(artifact_gate::ENTRY_ID_APP_TARGET),
        WindowsAdapterCapabilitySurface::AppTarget,
        APP_REQUIRED_ARTIFACTS,
        ProductClaimBoundary(artifact_gate::CLAIM_BOUNDARY_APP_TARGET),
        RefusalReason(artifact_gate::REFUSAL_MISSING_APP_ARTIFACTS),
        false,
    )
}

fn domain_network_target_spec() -> ArtifactGateSpec {
    gate_spec(
        GateEntryId(artifact_gate::ENTRY_ID_DOMAIN_NETWORK_TARGET),
        WindowsAdapterCapabilitySurface::DomainNetworkTarget,
        DOMAIN_REQUIRED_ARTIFACTS,
        ProductClaimBoundary(artifact_gate::CLAIM_BOUNDARY_DOMAIN_NETWORK_TARGET),
        RefusalReason(artifact_gate::REFUSAL_MISSING_DOMAIN_ARTIFACTS),
        false,
    )
}

fn managed_browser_target_spec() -> ArtifactGateSpec {
    gate_spec(
        GateEntryId(artifact_gate::ENTRY_ID_MANAGED_BROWSER_TARGET),
        WindowsAdapterCapabilitySurface::ManagedBrowserTarget,
        MANAGED_BROWSER_REQUIRED_ARTIFACTS,
        ProductClaimBoundary(artifact_gate::CLAIM_BOUNDARY_MANAGED_BROWSER_TARGET),
        RefusalReason(artifact_gate::REFUSAL_MISSING_MANAGED_BROWSER_ARTIFACTS),
        false,
    )
}

fn unmanaged_browser_target_spec() -> ArtifactGateSpec {
    gate_spec(
        GateEntryId(artifact_gate::ENTRY_ID_UNMANAGED_BROWSER_TARGET),
        WindowsAdapterCapabilitySurface::UnmanagedBrowserTarget,
        &[],
        ProductClaimBoundary(artifact_gate::CLAIM_BOUNDARY_UNMANAGED_BROWSER_TARGET),
        RefusalReason(artifact_gate::REFUSAL_UNMANAGED_BROWSER_PROCESS_ONLY),
        true,
    )
}

fn unsupported_os_target_spec() -> ArtifactGateSpec {
    gate_spec(
        GateEntryId(artifact_gate::ENTRY_ID_UNSUPPORTED_OS_TARGET),
        WindowsAdapterCapabilitySurface::UnsupportedOsTarget,
        &[],
        ProductClaimBoundary(artifact_gate::CLAIM_BOUNDARY_UNSUPPORTED_OS_TARGET),
        RefusalReason(artifact_gate::REFUSAL_UNSUPPORTED_OS),
        true,
    )
}

fn rollback_audit_target_spec() -> ArtifactGateSpec {
    gate_spec(
        GateEntryId(artifact_gate::ENTRY_ID_ROLLBACK_AUDIT_TARGET),
        WindowsAdapterCapabilitySurface::RollbackAuditTarget,
        ROLLBACK_AUDIT_REQUIRED_ARTIFACTS,
        ProductClaimBoundary(artifact_gate::CLAIM_BOUNDARY_ROLLBACK_AUDIT_TARGET),
        RefusalReason(artifact_gate::REFUSAL_MISSING_ROLLBACK_AUDIT_ARTIFACTS),
        false,
    )
}

fn artifact_gate_entry(
    spec: &ArtifactGateSpec,
    capability: &WindowsAdapterCapabilityProof,
    artifacts: &[WindowsAdapterArtifactEvidence],
    generated_at: ArtifactGateGeneratedAtTextRef<'_>,
) -> WindowsAdapterArtifactGateEntry {
    let capability_entry = capability_entry_for(capability, spec.surface);
    let present_artifacts = present_artifacts(spec, artifacts);
    let missing_artifact_kinds = missing_artifact_kinds(spec, &present_artifacts);
    let decision = gate_decision(spec, &missing_artifact_kinds);
    let ready_for_manual_review =
        decision == WindowsAdapterArtifactGateDecision::ReadyForManualReview;

    WindowsAdapterArtifactGateEntry {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        gate_entry_id: spec.gate_entry_id.0.to_string(),
        capability_entry_id: capability_entry.proof_entry_id.clone(),
        surface: spec.surface,
        required_artifact_kinds: spec.required_artifact_kinds.to_vec(),
        present_artifact_ids: present_artifacts
            .iter()
            .map(|artifact| artifact.artifact_id.clone())
            .collect(),
        missing_artifact_kinds,
        refusal_reasons: refusal_reasons(spec, ready_for_manual_review)
            .into_iter()
            .map(|reason| reason.0.to_string())
            .collect(),
        decision,
        ready_for_manual_review,
        claim_upgrade_allowed: false,
        product_claim_boundary: spec.product_claim_boundary.0.to_string(),
        last_checked_at: generated_at.0.to_string(),
    }
}

fn capability_entry_for(
    capability: &WindowsAdapterCapabilityProof,
    surface: WindowsAdapterCapabilitySurface,
) -> &WindowsAdapterCapabilityProofEntry {
    capability
        .entries
        .iter()
        .find(|entry| entry.surface == surface)
        .expect_value(artifact_gate::READ_MODEL_ID_V0_8)
}

fn present_artifacts<'a>(
    spec: &ArtifactGateSpec,
    artifacts: &'a [WindowsAdapterArtifactEvidence],
) -> Vec<&'a WindowsAdapterArtifactEvidence> {
    let mut present = Vec::new();
    for required_kind in spec.required_artifact_kinds {
        if let Some(artifact) = artifacts.iter().find(|artifact| {
            artifact.surface == spec.surface
                && artifact.artifact_kind == *required_kind
                && artifact_satisfies_kind(artifact)
        }) {
            present.push(artifact);
        }
    }
    present
}

fn missing_artifact_kinds(
    spec: &ArtifactGateSpec,
    present_artifacts: &[&WindowsAdapterArtifactEvidence],
) -> Vec<WindowsAdapterArtifactKind> {
    spec.required_artifact_kinds
        .iter()
        .copied()
        .filter(|required_kind| {
            !present_artifacts
                .iter()
                .any(|artifact| artifact.artifact_kind == *required_kind)
        })
        .collect()
}

fn artifact_satisfies_kind(artifact: &WindowsAdapterArtifactEvidence) -> bool {
    !artifact.artifact_id.is_empty()
        && !artifact.subject_ref.is_empty()
        && artifact
            .custody_event_id
            .as_deref()
            .is_some_and(|custody_event_id| !custody_event_id.is_empty())
}

fn gate_decision(
    spec: &ArtifactGateSpec,
    missing_artifact_kinds: &[WindowsAdapterArtifactKind],
) -> WindowsAdapterArtifactGateDecision {
    if spec.unsupported_surface {
        WindowsAdapterArtifactGateDecision::RefusedUnsupportedSurface
    } else if missing_artifact_kinds.is_empty() {
        WindowsAdapterArtifactGateDecision::ReadyForManualReview
    } else {
        WindowsAdapterArtifactGateDecision::RefusedMissingArtifacts
    }
}

fn refusal_reasons(spec: &ArtifactGateSpec, ready_for_manual_review: bool) -> Vec<RefusalReason> {
    if ready_for_manual_review {
        Vec::new()
    } else {
        vec![spec.refusal_reason]
    }
}
