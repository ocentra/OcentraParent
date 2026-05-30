use std::collections::BTreeMap;

use crate::{
    constants::{self, v08_os_adapter_product_proof as proof_constants},
    EnforcementCapabilityState, EnforcementReadinessState, EnforcementResultStatus,
    EnforcementRollbackState, V08OsAdapterProductProofAuditState, V08OsAdapterProductProofEntry,
    V08OsAdapterProductProofParentOverrideState, V08OsAdapterProductProofReadModel,
    V08OsAdapterProductProofSurface, V08OsAdapterProductProofTimerRecoveryState,
};

#[test]
fn product_proof_surfaces_have_stable_protocol_strings() {
    let surfaces = [
        V08OsAdapterProductProofSurface::OwnedProcessTerminate,
        V08OsAdapterProductProofSurface::AppTimeLimitLifecycle,
        V08OsAdapterProductProofSurface::BroadAppBlocking,
        V08OsAdapterProductProofSurface::NetworkDomainBlocking,
        V08OsAdapterProductProofSurface::ManagedBrowserServiceCommand,
        V08OsAdapterProductProofSurface::ManagedBrowserExactUrl,
        V08OsAdapterProductProofSurface::UnmanagedBrowserProcessOnly,
        V08OsAdapterProductProofSurface::UnmanagedBrowserExactEvidence,
        V08OsAdapterProductProofSurface::RestartRecovery,
        V08OsAdapterProductProofSurface::ParentCancelOverride,
        V08OsAdapterProductProofSurface::AuditCustody,
        V08OsAdapterProductProofSurface::RollbackArtifactGate,
    ];
    let serialized =
        serde_json::to_value(surfaces).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized
            .as_array()
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
            .len(),
        12
    );
    assert_eq!(
        surfaces[0].as_protocol_str(),
        proof_constants::SURFACE_OWNED_PROCESS_TERMINATE
    );
    assert_eq!(
        surfaces[11].as_protocol_str(),
        proof_constants::SURFACE_ROLLBACK_ARTIFACT_GATE
    );
}

#[test]
fn product_proof_lifecycle_states_serialize_as_contract_values() {
    assert_eq!(
        V08OsAdapterProductProofTimerRecoveryState::RestartRecovered.as_protocol_str(),
        proof_constants::TIMER_STATE_RESTART_RECOVERED
    );
    assert_eq!(
        V08OsAdapterProductProofAuditState::Journaled.as_protocol_str(),
        proof_constants::AUDIT_STATE_JOURNALED
    );
    assert_eq!(
        V08OsAdapterProductProofParentOverrideState::CancelSupported.as_protocol_str(),
        proof_constants::PARENT_OVERRIDE_CANCEL_SUPPORTED
    );
}

#[test]
fn product_proof_read_model_serializes_claim_flags_for_runtime_preview() {
    let read_model = V08OsAdapterProductProofReadModel {
        schema_version: crate::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof_constants::READ_MODEL_ID.to_string(),
        generated_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![proof_constants::READ_MODEL_ID.to_string()],
        entries: vec![
            product_entry(
                proof_constants::ENTRY_ID_OWNED_PROCESS_TERMINATE,
                V08OsAdapterProductProofSurface::OwnedProcessTerminate,
                EnforcementReadinessState::Implemented,
                EnforcementCapabilityState::Supported,
                EnforcementResultStatus::ActuallyEnforced,
            ),
            product_entry(
                proof_constants::ENTRY_ID_BROAD_APP_BLOCKING,
                V08OsAdapterProductProofSurface::BroadAppBlocking,
                EnforcementReadinessState::ManualRequired,
                EnforcementCapabilityState::ManualRequired,
                EnforcementResultStatus::Unavailable,
            ),
        ],
    };
    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let reparsed = serde_json::from_value::<V08OsAdapterProductProofReadModel>(serialized)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let readiness_counts = count_readiness(&reparsed.entries);

    assert_eq!(reparsed.read_model_id, proof_constants::READ_MODEL_ID);
    assert_eq!(
        readiness_counts[crate::constants::enforcement::READINESS_IMPLEMENTED],
        1
    );
    assert_eq!(
        readiness_counts[crate::constants::enforcement::READINESS_MANUAL_REQUIRED],
        1
    );
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.claim_upgrade_allowed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.broad_blocking_claimed && !entry.exact_url_claimed));
}

fn product_entry(
    proof_entry_id: &str,
    surface: V08OsAdapterProductProofSurface,
    readiness_state: EnforcementReadinessState,
    capability_state: EnforcementCapabilityState,
    result_status: EnforcementResultStatus,
) -> V08OsAdapterProductProofEntry {
    V08OsAdapterProductProofEntry {
        schema_version: crate::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: proof_entry_id.to_string(),
        surface,
        platform: crate::ParentPlatform::Windows,
        adapter_kind: crate::EnforcementAdapterKind::ProcessControl,
        capability_state,
        readiness_state,
        proof_level: crate::EnforcementReadinessProofLevel::RealServiceProof,
        runtime_owner: crate::EnforcementReadinessRuntimeOwner::RustService,
        supported_modes: Vec::new(),
        result_status,
        rollback_state: EnforcementRollbackState::NotRequired,
        timer_recovery_state: V08OsAdapterProductProofTimerRecoveryState::NotRequired,
        audit_state: V08OsAdapterProductProofAuditState::Journaled,
        parent_override_state: V08OsAdapterProductProofParentOverrideState::NotRequired,
        linked_readiness_ids: Vec::new(),
        linked_capability_entry_ids: Vec::new(),
        linked_artifact_gate_entry_ids: Vec::new(),
        capability_requirement: proof_constants::CAPABILITY_OWNED_PROCESS.to_string(),
        proof_requirement: proof_constants::PROOF_OWNED_PROCESS.to_string(),
        claim_boundary: proof_constants::CLAIM_OWNED_PROCESS.to_string(),
        fallback_behavior: proof_constants::FALLBACK_OWNED_PROCESS.to_string(),
        claim_upgrade_allowed: false,
        broad_blocking_claimed: false,
        exact_url_claimed: false,
        last_checked_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}

fn count_readiness(entries: &[V08OsAdapterProductProofEntry]) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(entry.readiness_state.as_protocol_str())
            .or_default() += 1;
        counts
    })
}
