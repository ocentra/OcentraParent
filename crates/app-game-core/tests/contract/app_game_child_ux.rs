use ocentra_app_game_core::app_game_child_ux::build_app_game_child_ux_notice;
use ocentra_app_game_core::app_game_child_ux_types::{
    AppGameChildAdapterActionRef, AppGameChildReasonRef, AppGameChildStatusRef,
    AppGameChildUxAction, AppGameChildUxCapabilityState, AppGameChildUxInput,
    AppGameChildUxNoticeState, AppGameChildUxRequestState, AppGameChildUxSubjectKind,
    AppGameChildUxTextToken,
};
use ocentra_app_game_core::app_game_policy_evaluator_runtime::types::{
    AppGamePolicyRuntimeAdapterDispatchState, AppGamePolicyRuntimeDecision,
    AppGamePolicyRuntimeDecisionReason, AppGamePolicyRuntimeDecisionState,
};
use ocentra_app_game_core::app_game_policy_target_compiler::references::{
    AppGamePolicyAuditRef, AppGamePolicyEvidenceRef, AppGamePolicyRuleRef,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn game_warning_uses_controlled_family_rule_copy_without_adapter_claim() -> Result<(), EventingError>
{
    let notice = build_app_game_child_ux_notice(input(
        AppGameChildUxSubjectKind::Game,
        AppGamePolicyRuntimeDecisionState::WarnOnly,
        AppGameChildUxRequestState::None,
        AppGameChildUxCapabilityState::Available,
    ))?;
    assert_eq!(
        notice.state,
        AppGameChildUxNoticeState::GameTimeAlmostFinished
    );
    assert_eq!(
        notice.text_token,
        AppGameChildUxTextToken::FamilyRuleGameTimeAlmostFinished
    );
    assert!(!notice.adapter_dispatch_claimed);
    Ok(())
}

#[test]
fn approval_needed_requires_evidence_reason_and_status_refs() -> Result<(), EventingError> {
    let notice = build_app_game_child_ux_notice(input(
        AppGameChildUxSubjectKind::App,
        AppGamePolicyRuntimeDecisionState::AskParent,
        AppGameChildUxRequestState::ApprovalNeeded,
        AppGameChildUxCapabilityState::Available,
    ))?;
    assert_eq!(notice.state, AppGameChildUxNoticeState::NewAppNeedsApproval);
    assert_eq!(notice.action, AppGameChildUxAction::AskParent);

    let mut missing = input(
        AppGameChildUxSubjectKind::App,
        AppGamePolicyRuntimeDecisionState::AskParent,
        AppGameChildUxRequestState::ApprovalNeeded,
        AppGameChildUxCapabilityState::Available,
    );
    missing.child_reason_refs.clear();
    assert!(matches!(
        build_app_game_child_ux_notice(missing),
        Err(EventingError::InvalidValue {
            field: "app_game.child_ux.ask_parent_refs",
            ..
        })
    ));
    Ok(())
}

#[test]
fn request_submitted_approved_and_denied_states_are_explicit() -> Result<(), EventingError> {
    for (request_state, notice_state) in [
        (
            AppGameChildUxRequestState::Submitted,
            AppGameChildUxNoticeState::RequestSubmitted,
        ),
        (
            AppGameChildUxRequestState::Approved,
            AppGameChildUxNoticeState::RequestApproved,
        ),
        (
            AppGameChildUxRequestState::Denied,
            AppGameChildUxNoticeState::RequestDenied,
        ),
    ] {
        let notice = build_app_game_child_ux_notice(input(
            AppGameChildUxSubjectKind::Game,
            AppGamePolicyRuntimeDecisionState::Observe,
            request_state,
            AppGameChildUxCapabilityState::Available,
        ))?;
        assert_eq!(notice.state, notice_state);
        assert_eq!(notice.action, AppGameChildUxAction::None);
    }
    Ok(())
}

#[test]
fn manual_required_and_unavailable_states_are_honest() -> Result<(), EventingError> {
    let manual = build_app_game_child_ux_notice(input(
        AppGameChildUxSubjectKind::App,
        AppGamePolicyRuntimeDecisionState::Observe,
        AppGameChildUxRequestState::None,
        AppGameChildUxCapabilityState::ManualRequired,
    ))?;
    assert_eq!(manual.state, AppGameChildUxNoticeState::ManualRequired);
    let unavailable = build_app_game_child_ux_notice(input(
        AppGameChildUxSubjectKind::Game,
        AppGamePolicyRuntimeDecisionState::Observe,
        AppGameChildUxRequestState::None,
        AppGameChildUxCapabilityState::Unavailable,
    ))?;
    assert_eq!(unavailable.state, AppGameChildUxNoticeState::Unavailable);
    Ok(())
}

#[test]
fn child_ux_rejects_adapter_action_refs_in_every_state() {
    let mut invalid = input(
        AppGameChildUxSubjectKind::App,
        AppGamePolicyRuntimeDecisionState::ManualRequired,
        AppGameChildUxRequestState::None,
        AppGameChildUxCapabilityState::ManualRequired,
    );
    invalid.adapter_action_ref = Some(
        AppGameChildAdapterActionRef::parse("adapter-action-private")
            .expect_value("adapter action ref"),
    );
    assert!(matches!(
        build_app_game_child_ux_notice(invalid),
        Err(EventingError::InvalidValue {
            field: "app_game.child_ux.adapter_action_ref",
            ..
        })
    ));
}

fn input(
    subject_kind: AppGameChildUxSubjectKind,
    runtime_state: AppGamePolicyRuntimeDecisionState,
    request_state: AppGameChildUxRequestState,
    capability_state: AppGameChildUxCapabilityState,
) -> AppGameChildUxInput {
    AppGameChildUxInput {
        subject_kind,
        runtime_decision: runtime_decision(runtime_state),
        request_state,
        capability_state,
        policy_rule_ref: AppGamePolicyRuleRef::parse("family-rule-1").expect_value("rule ref"),
        evidence_refs: vec![
            AppGamePolicyEvidenceRef::parse("evidence-child-ux").expect_value("evidence ref")
        ],
        child_reason_refs: vec![
            AppGameChildReasonRef::parse("child-reason-1").expect_value("reason ref")
        ],
        child_status_refs: vec![
            AppGameChildStatusRef::parse("child-status-1").expect_value("status ref")
        ],
        adapter_action_ref: None,
    }
}

fn runtime_decision(state: AppGamePolicyRuntimeDecisionState) -> AppGamePolicyRuntimeDecision {
    AppGamePolicyRuntimeDecision {
        state,
        reason: AppGamePolicyRuntimeDecisionReason::WithinBudget,
        consumed_seconds: 50,
        effective_budget_seconds: 60,
        remaining_seconds: 10,
        counted_session_refs: Vec::new(),
        excluded_session_refs: Vec::new(),
        timer_ref: None,
        bonus_approval_ref: None,
        audit_ref: AppGamePolicyAuditRef::parse("audit-child-ux").expect_value("audit ref"),
        adapter_dispatch_state: AppGamePolicyRuntimeAdapterDispatchState::NotDispatched,
    }
}
