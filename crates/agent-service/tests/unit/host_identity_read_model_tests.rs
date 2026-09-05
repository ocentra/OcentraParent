use std::collections::BTreeMap;
use std::primitive::str as TestStr;

use ocentra_parent_agent_protocol::constants::{self, enforcement, field, host_identity};
use ocentra_parent_agent_protocol::enforcement::EnforcementCapabilityState;
use ocentra_parent_agent_protocol::enforcement_readiness::{
    EnforcementReadinessProofLevel, EnforcementReadinessRuntimeOwner, EnforcementReadinessState,
};
use ocentra_parent_agent_protocol::host_identity::HostIdentityEvidenceKind;
use ocentra_parent_agent_protocol::policy_constants as policy;

use crate::host_identity_read_model::{host_identity_read_model, GeneratedAtText};
use crate::test_require_ok::require_ok;
use crate::test_require_some::require_some;

#[test]
fn host_identity_read_model_preserves_manual_and_unavailable_boundaries() {
    let model = host_identity_read_model(GeneratedAtText(policy::TEST_EVALUATED_AT.to_string()));
    let readiness_counts = count_readiness(&model.entries);
    let evidence_counts = count_evidence_classes(&model.entries);

    assert_eq!(model.read_model_id, host_identity::READ_MODEL_ID_V0_8);
    assert_eq!(model.entries.len(), 9);
    assert_eq!(readiness_counts[enforcement::READINESS_MANUAL_REQUIRED], 7);
    assert_eq!(readiness_counts[enforcement::READINESS_UNAVAILABLE], 1);
    assert_eq!(readiness_counts[enforcement::READINESS_NOT_CLAIMED], 1);
    assert_eq!(evidence_counts[host_identity::CLASS_INVENTORY], 2);
    assert_eq!(evidence_counts[host_identity::CLASS_PROCESS], 1);
    assert_eq!(evidence_counts[host_identity::CLASS_EXECUTABLE], 1);
    assert_eq!(evidence_counts[host_identity::CLASS_PACKAGE], 2);
    assert_eq!(evidence_counts[host_identity::CLASS_PUBLISHER_SIGNATURE], 1);
    assert_eq!(evidence_counts[host_identity::CLASS_ROLLBACK], 1);
    assert_eq!(evidence_counts[host_identity::CLASS_AUDIT], 1);
    assert!(model.entries.iter().all(|entry| {
        !entry.safe_for_broad_app_blocking
            && !entry.required_evidence_artifacts.is_empty()
            && !entry.acceptance_signals.is_empty()
    }));
}

#[test]
fn host_identity_read_model_keeps_unsupported_and_rollback_states_honest() {
    let model = host_identity_read_model(GeneratedAtText(policy::TEST_EVALUATED_AT.to_string()));
    let unsupported = entry_for(
        &model.entries,
        HostIdentityEvidenceKind::UnsupportedIdentity,
    );
    let rollback = entry_for(&model.entries, HostIdentityEvidenceKind::RollbackReadiness);

    assert_eq!(
        unsupported.capability_state,
        EnforcementCapabilityState::Unavailable
    );
    assert_eq!(
        unsupported.readiness_state,
        EnforcementReadinessState::Unavailable
    );
    assert_eq!(
        unsupported.proof_level,
        EnforcementReadinessProofLevel::ManualProofRequired
    );
    assert_eq!(
        unsupported.runtime_owner,
        EnforcementReadinessRuntimeOwner::ManualProof
    );
    assert_eq!(
        rollback.capability_state,
        EnforcementCapabilityState::ManualRequired
    );
    assert_eq!(
        rollback.readiness_state,
        EnforcementReadinessState::NotClaimed
    );
    assert_eq!(
        rollback.proof_level,
        EnforcementReadinessProofLevel::NotProved
    );
    assert_eq!(
        rollback.runtime_owner,
        EnforcementReadinessRuntimeOwner::NotImplemented
    );
}

#[test]
fn host_identity_read_model_serializes_for_runtime_preview_without_claiming_blocking() {
    let model = host_identity_read_model(GeneratedAtText(policy::TEST_EVALUATED_AT.to_string()));
    let serialized = require_ok(
        serde_json::to_value(model),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(
        serialized[field::READ_MODEL_ID],
        host_identity::READ_MODEL_ID_V0_8
    );
    assert_eq!(
        serialized[field::ENTRIES][0][field::CAPABILITY],
        enforcement::BROAD_CAPABILITY_BROAD_APP_BLOCKING
    );
    assert_eq!(
        serialized[field::ENTRIES][0][field::SAFE_FOR_BROAD_APP_BLOCKING],
        false
    );
    assert_eq!(
        serialized[field::ENTRIES][6][field::FALLBACK_BEHAVIOR],
        host_identity::FALLBACK_UNSUPPORTED_IDENTITY
    );
    assert_eq!(
        serialized[field::ENTRIES][7][field::FALLBACK_BEHAVIOR],
        host_identity::FALLBACK_ROLLBACK_READINESS
    );
}

fn entry_for(
    entries: &[ocentra_parent_agent_protocol::host_identity::HostIdentityReadModelEntry],
    evidence_kind: HostIdentityEvidenceKind,
) -> &ocentra_parent_agent_protocol::host_identity::HostIdentityReadModelEntry {
    require_some(
        entries
            .iter()
            .find(|entry| entry.evidence_kind == evidence_kind),
        host_identity::READ_MODEL_ID_V0_8,
    )
}

fn count_readiness(
    entries: &[ocentra_parent_agent_protocol::host_identity::HostIdentityReadModelEntry],
) -> BTreeMap<&'static TestStr, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(entry.readiness_state.as_protocol_str())
            .or_default() += 1;
        counts
    })
}

fn count_evidence_classes(
    entries: &[ocentra_parent_agent_protocol::host_identity::HostIdentityReadModelEntry],
) -> BTreeMap<&'static TestStr, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(entry.evidence_class.as_protocol_str())
            .or_default() += 1;
        counts
    })
}
