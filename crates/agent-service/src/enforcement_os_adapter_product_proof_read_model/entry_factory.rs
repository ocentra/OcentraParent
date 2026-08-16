use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants::v08_os_adapter_product_proof as proof;
use ocentra_parent_agent_protocol::constants::windows_adapter_capability;
use ocentra_parent_agent_protocol::enforcement::EnforcementCapabilityState;
use ocentra_parent_agent_protocol::enforcement::EnforcementResultStatus;
use ocentra_parent_agent_protocol::enforcement::EnforcementRollbackState;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofAuditState;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofEntry;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofParentOverrideState;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofSurface;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofTimerRecoveryState;
use ocentra_parent_agent_protocol::enforcement_readiness::EnforcementBroadAdapterCapability;
use ocentra_parent_agent_protocol::enforcement_readiness::EnforcementBroadAdapterReadinessEntry;
use ocentra_parent_agent_protocol::enforcement_readiness::EnforcementBroadOsAdapterReadinessMatrix;
use ocentra_parent_agent_protocol::enforcement_readiness::EnforcementReadinessProofLevel;
use ocentra_parent_agent_protocol::enforcement_readiness::EnforcementReadinessRuntimeOwner;
use ocentra_parent_agent_protocol::enforcement_readiness::EnforcementReadinessState;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::windows_adapter_artifact_gate::WindowsAdapterArtifactGateProof;
use ocentra_parent_agent_protocol::windows_adapter_capability::WindowsAdapterCapabilityProof;

use super::{EntryProofLinks, EntryProofText, EntrySpec, GeneratedAtTextRef, ProofEntryIdRef};
use ocentra_parent_agent_protocol::enforcement::EnforcementAdapterKind;

pub(super) fn manual_entry_spec<'a>(
    proof_entry_id: ProofEntryIdRef<'a>,
    surface: V08OsAdapterProductProofSurface,
    capability: EnforcementBroadAdapterCapability,
    adapter_kind: EnforcementAdapterKind,
    links: EntryProofLinks<'a>,
    text: EntryProofText<'a>,
) -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id: proof_entry_id.0,
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

pub(super) fn lifecycle_entry_spec<'a>(
    proof_entry_id: ProofEntryIdRef<'a>,
    surface: V08OsAdapterProductProofSurface,
    result_status: EnforcementResultStatus,
    timer_state: V08OsAdapterProductProofTimerRecoveryState,
    text: EntryProofText<'a>,
) -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id: proof_entry_id.0,
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

pub(super) fn entry_from_spec(
    spec: &EntrySpec<'_>,
    readiness: &EnforcementBroadOsAdapterReadinessMatrix,
    capability: &WindowsAdapterCapabilityProof,
    artifact_gate: &WindowsAdapterArtifactGateProof,
    generated_at: GeneratedAtTextRef<'_>,
) -> V08OsAdapterProductProofEntry {
    let primary = readiness_entry(readiness, spec.capability);
    assert_links(spec, capability, artifact_gate);
    let resolved = resolve_entry_state(primary, spec);

    V08OsAdapterProductProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: spec.proof_entry_id.to_string(),
        surface: spec.surface,
        platform: primary.platform,
        adapter_kind: spec.adapter_kind,
        capability_state: primary.capability_state,
        readiness_state: primary.readiness_state,
        proof_level: resolved.proof_level,
        runtime_owner: resolved.runtime_owner,
        supported_modes: primary.supported_modes.clone(),
        result_status: resolved.result_status,
        rollback_state: resolved.rollback_state,
        timer_recovery_state: resolved.timer_recovery_state,
        audit_state: resolved.audit_state,
        parent_override_state: resolved.parent_override_state,
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
        last_checked_at: generated_at.0.to_string(),
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
        .expect_value(proof::READ_MODEL_ID)
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
            .expect_value(proof::READ_MODEL_ID);
    }
    for entry_id in &spec.linked_artifact_gate_entry_ids {
        artifact_gate
            .entries
            .iter()
            .find(|entry| entry.gate_entry_id == **entry_id)
            .expect_value(proof::READ_MODEL_ID);
    }
}

struct ResolvedEntryState {
    proof_level: EnforcementReadinessProofLevel,
    runtime_owner: EnforcementReadinessRuntimeOwner,
    result_status: EnforcementResultStatus,
    rollback_state: EnforcementRollbackState,
    timer_recovery_state: V08OsAdapterProductProofTimerRecoveryState,
    audit_state: V08OsAdapterProductProofAuditState,
    parent_override_state: V08OsAdapterProductProofParentOverrideState,
}

fn resolve_entry_state(
    primary: &EnforcementBroadAdapterReadinessEntry,
    spec: &EntrySpec<'_>,
) -> ResolvedEntryState {
    let (
        proof_level,
        result_status,
        rollback_state,
        timer_recovery_state,
        audit_state,
        parent_override_state,
    ) = match primary.readiness_state {
        EnforcementReadinessState::Implemented => (
            primary.proof_level,
            spec.implemented_result,
            spec.implemented_rollback,
            spec.implemented_timer,
            V08OsAdapterProductProofAuditState::Journaled,
            spec.implemented_parent_override,
        ),
        EnforcementReadinessState::ManualRequired => (
            EnforcementReadinessProofLevel::ManualProofRequired,
            EnforcementResultStatus::Unavailable,
            EnforcementRollbackState::Unavailable,
            V08OsAdapterProductProofTimerRecoveryState::ManualRequired,
            V08OsAdapterProductProofAuditState::ManualRequired,
            V08OsAdapterProductProofParentOverrideState::ManualRequired,
        ),
        EnforcementReadinessState::NotClaimed => (
            EnforcementReadinessProofLevel::NotProved,
            EnforcementResultStatus::NoOp,
            EnforcementRollbackState::NotRequired,
            V08OsAdapterProductProofTimerRecoveryState::NotRequired,
            V08OsAdapterProductProofAuditState::Unavailable,
            V08OsAdapterProductProofParentOverrideState::Unavailable,
        ),
        _ => (
            EnforcementReadinessProofLevel::ManualProofRequired,
            EnforcementResultStatus::Unavailable,
            EnforcementRollbackState::Unavailable,
            V08OsAdapterProductProofTimerRecoveryState::Unavailable,
            V08OsAdapterProductProofAuditState::Unavailable,
            V08OsAdapterProductProofParentOverrideState::Unavailable,
        ),
    };
    let runtime_owner = if primary.capability_state == EnforcementCapabilityState::Supported
        || primary.readiness_state == EnforcementReadinessState::NotClaimed
    {
        spec.runtime_owner
    } else {
        EnforcementReadinessRuntimeOwner::ManualProof
    };

    ResolvedEntryState {
        proof_level,
        runtime_owner,
        result_status,
        rollback_state,
        timer_recovery_state,
        audit_state,
        parent_override_state,
    }
}
