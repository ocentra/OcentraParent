use ocentra_parent_agent_core::enforcement_policy_dispatch::validate_enforcement_policy_dispatch_read_model;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchOutcomeState;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchReadModel;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchRejectionReason;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchSourceState;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchTimerState;
use ocentra_parent_agent_protocol::policy_constants;

use super::test_text::{test_ok, TestResult};
use crate::enforcement_policy_dispatch_read_model::v08_enforcement_policy_dispatch_read_model;

#[test]
fn policy_dispatch_read_model_exposes_validation_and_non_claim_states() -> TestResult {
    let read_model =
        v08_enforcement_policy_dispatch_read_model(policy_constants::TEST_EVALUATED_AT);
    let validation = test_ok(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        constants::v08_enforcement_policy_dispatch::READ_MODEL_ID,
    )?;

    assert_eq!(
        read_model.read_model_id,
        constants::v08_enforcement_policy_dispatch::READ_MODEL_ID
    );
    assert_eq!(read_model.entries.len(), 8);
    assert_eq!(validation.dispatch_ready_count, 1);
    assert_eq!(validation.dry_run_only_count, 1);
    assert_eq!(validation.manual_required_count, 2);
    assert_eq!(validation.report_only_count, 1);
    assert_eq!(validation.rejected_count, 3);
    assert_eq!(validation.recovery_needed_count, 1);
    assert!(read_model.entries.iter().any(|entry| {
        entry.matrix_row.outcome_state == EnforcementPolicyDispatchOutcomeState::ManualRequired
    }));
    assert!(read_model.entries.iter().any(|entry| {
        entry.intent.requested_parent_action
            == ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlParentAction::AskParent
            && entry.matrix_row.outcome_state == EnforcementPolicyDispatchOutcomeState::DryRunOnly
            && entry.intent.dry_run
    }));
    assert!(read_model.entries.iter().any(|entry| {
        entry.matrix_row.rejection_reason
            == EnforcementPolicyDispatchRejectionReason::StalePolicyVersion
            && entry.intent.source_state == EnforcementPolicyDispatchSourceState::Stale
    }));
    assert!(read_model.entries.iter().any(|entry| {
        entry.matrix_row.rejection_reason
            == EnforcementPolicyDispatchRejectionReason::SourceNotReady
            && entry.intent.source_state == EnforcementPolicyDispatchSourceState::Missing
    }));
    assert!(read_model
        .entries
        .iter()
        .any(|entry| entry.timer_state == EnforcementPolicyDispatchTimerState::RestartRecovered));

    Ok(())
}

#[test]
fn policy_dispatch_read_model_preserves_reference_correlation_and_provenance() {
    let read_model =
        v08_enforcement_policy_dispatch_read_model(policy_constants::TEST_EVALUATED_AT);
    let entry = &read_model.entries[0];

    assert_eq!(
        entry.intent.policy_decision_id,
        "policy-dispatch-owned-process-time-limit"
    );
    assert_eq!(
        entry.intent.policy_decision_ref,
        "decision-dispatch-owned-process-time-limit"
    );
    assert_eq!(entry.intent.evidence_references.len(), 1);
    assert_eq!(
        entry.intent.evidence_references[0].kind,
        ParentEvidenceReferenceKind::ActivityEvent
    );
    assert_eq!(
        entry.intent.evidence_references[0].observed_at,
        policy_constants::TEST_EVALUATED_AT
    );
    assert_eq!(entry.audit_refs.len(), 1);
    assert_eq!(entry.timer_refs.len(), 1);
    assert!(entry.intent.approval_ref.is_none());
    assert_eq!(
        entry.dispatched_at.as_deref(),
        Some(policy_constants::TEST_EVALUATED_AT)
    );
}

#[test]
fn policy_dispatch_read_model_rejects_tampered_evidence_reference() {
    let mut read_model =
        v08_enforcement_policy_dispatch_read_model(policy_constants::TEST_EVALUATED_AT);
    read_model.entries[0].intent.evidence_references[0].evidence_reference_id =
        "not-an-owned-evidence-reference".to_string();

    assert_eq!(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        Err(EnforcementPolicyDispatchRejectionReason::MissingEvidence)
    );
}

#[test]
fn policy_dispatch_read_model_rejects_tampered_decision_correlation() {
    let mut read_model =
        v08_enforcement_policy_dispatch_read_model(policy_constants::TEST_EVALUATED_AT);
    read_model.entries[0].intent.policy_decision_ref = "decision-other-intent".to_string();

    assert_eq!(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        Err(EnforcementPolicyDispatchRejectionReason::MissingPolicyDecision)
    );
}
