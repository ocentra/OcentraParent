use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::constants::enforcement;
use ocentra_parent_agent_protocol::constants::v08_os_adapter_product_proof as proof;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofAuditState;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofEntry;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofParentOverrideState;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofSurface;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofTimerRecoveryState;
use ocentra_parent_agent_protocol::enforcement_readiness::EnforcementReadinessState;
use ocentra_parent_agent_protocol::policy_constants;

use crate::test_text::{count_for_display, test_ok, TestResult, TestText};
use crate::{
    enforcement_os_adapter_product_proof_read_model::{
        v08_os_adapter_product_proof_read_model, GeneratedAtTextRef,
    },
    test_invariants::require_some,
};

#[path = "enforcement_os_adapter_product_proof_read_model_tests/product_control_spine_tests.rs"]
mod product_control_spine_tests;

#[test]
fn product_proof_read_model_captures_v0_8_adapter_boundaries() {
    let read_model = v08_os_adapter_product_proof_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));
    let readiness_counts = count_readiness(&read_model.entries);

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 12);
    assert_eq!(
        readiness_count(&readiness_counts, enforcement::READINESS_IMPLEMENTED),
        expected_implemented_count()
    );
    assert_eq!(
        readiness_count(&readiness_counts, enforcement::READINESS_MANUAL_REQUIRED),
        expected_manual_required_count()
    );
    assert_eq!(
        readiness_count(&readiness_counts, enforcement::READINESS_UNAVAILABLE),
        expected_unavailable_count()
    );
    assert_eq!(
        readiness_count(&readiness_counts, enforcement::READINESS_NOT_CLAIMED),
        1
    );
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.claim_upgrade_allowed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.broad_blocking_claimed && !entry.exact_url_claimed));
}

#[test]
fn product_proof_read_model_preserves_lifecycle_and_audit_states() {
    let read_model = v08_os_adapter_product_proof_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));
    let restart = entry_for(
        &read_model.entries,
        V08OsAdapterProductProofSurface::RestartRecovery,
    );
    let parent_cancel = entry_for(
        &read_model.entries,
        V08OsAdapterProductProofSurface::ParentCancelOverride,
    );
    let audit = entry_for(
        &read_model.entries,
        V08OsAdapterProductProofSurface::AuditCustody,
    );
    let rollback = entry_for(
        &read_model.entries,
        V08OsAdapterProductProofSurface::RollbackArtifactGate,
    );

    assert_eq!(
        restart.timer_recovery_state,
        expected_restart_recovery_state()
    );
    assert_eq!(
        parent_cancel.parent_override_state,
        expected_parent_override_state()
    );
    assert_eq!(audit.audit_state, expected_audit_state());
    assert_eq!(
        rollback.linked_artifact_gate_entry_ids,
        vec![constants::windows_adapter_artifact_gate::ENTRY_ID_ROLLBACK_AUDIT_TARGET.to_string()]
    );
    assert_eq!(
        rollback.readiness_state,
        EnforcementReadinessState::ManualRequired
    );
}

#[test]
fn product_proof_read_model_serializes_for_runtime_preview() -> TestResult {
    let read_model = v08_os_adapter_product_proof_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));
    let serialized = test_ok(
        serde_json::to_value(read_model),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    let reparsed = test_ok(serde_json::from_value::<
        ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofReadModel,
    >(serialized),
    constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    let unmanaged_exact = entry_for(
        &reparsed.entries,
        V08OsAdapterProductProofSurface::UnmanagedBrowserExactEvidence,
    );

    assert_eq!(reparsed.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(
        unmanaged_exact.readiness_state,
        EnforcementReadinessState::NotClaimed
    );
    assert_eq!(
        unmanaged_exact.audit_state,
        V08OsAdapterProductProofAuditState::Unavailable
    );
    assert!(unmanaged_exact.linked_artifact_gate_entry_ids.contains(
        &constants::windows_adapter_artifact_gate::ENTRY_ID_UNMANAGED_BROWSER_TARGET.to_string()
    ));

    Ok(())
}

fn entry_for(
    entries: &[V08OsAdapterProductProofEntry],
    surface: V08OsAdapterProductProofSurface,
) -> &V08OsAdapterProductProofEntry {
    require_some(
        entries.iter().find(|entry| entry.surface == surface),
        proof::READ_MODEL_ID,
    )
}

fn count_readiness(entries: &[V08OsAdapterProductProofEntry]) -> BTreeMap<TestText, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(TestText::from_display(
                entry.readiness_state.as_protocol_str(),
            ))
            .or_default() += 1;
        counts
    })
}

fn readiness_count(counts: &BTreeMap<TestText, usize>, state: impl std::fmt::Display) -> usize {
    count_for_display(counts, state)
}

#[cfg(windows)]
fn expected_implemented_count() -> usize {
    6
}

#[cfg(not(windows))]
fn expected_implemented_count() -> usize {
    0
}

#[cfg(windows)]
fn expected_manual_required_count() -> usize {
    5
}

#[cfg(not(windows))]
fn expected_manual_required_count() -> usize {
    1
}

#[cfg(windows)]
fn expected_unavailable_count() -> usize {
    0
}

#[cfg(not(windows))]
fn expected_unavailable_count() -> usize {
    10
}

#[cfg(windows)]
fn expected_restart_recovery_state() -> V08OsAdapterProductProofTimerRecoveryState {
    V08OsAdapterProductProofTimerRecoveryState::RestartRecovered
}

#[cfg(not(windows))]
fn expected_restart_recovery_state() -> V08OsAdapterProductProofTimerRecoveryState {
    V08OsAdapterProductProofTimerRecoveryState::Unavailable
}

#[cfg(windows)]
fn expected_parent_override_state() -> V08OsAdapterProductProofParentOverrideState {
    V08OsAdapterProductProofParentOverrideState::CancelSupported
}

#[cfg(not(windows))]
fn expected_parent_override_state() -> V08OsAdapterProductProofParentOverrideState {
    V08OsAdapterProductProofParentOverrideState::Unavailable
}

#[cfg(windows)]
fn expected_audit_state() -> V08OsAdapterProductProofAuditState {
    V08OsAdapterProductProofAuditState::Journaled
}

#[cfg(not(windows))]
fn expected_audit_state() -> V08OsAdapterProductProofAuditState {
    V08OsAdapterProductProofAuditState::Unavailable
}
