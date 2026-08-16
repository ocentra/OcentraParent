use ocentra_app_game_core::app_game_policy_target_compiler::references::{
    AppGamePolicyAuditRef, AppGamePolicyAuthorityRef, AppGamePolicyCapabilityRef,
    AppGamePolicyCompileRequestId, AppGamePolicyCompiledDecisionId, AppGamePolicyDeviceId,
    AppGamePolicyEvidenceRef, AppGamePolicyLocalUserRef, AppGamePolicyRuleRef,
    AppGamePolicyScheduleRef, AppGamePolicyTargetRef,
};
use ocentra_app_game_core::app_game_policy_target_compiler::types::{
    AppGamePolicyCompilation, AppGamePolicyCompileRequest, AppGamePolicyCompilerAuthorityEvidence,
    AppGamePolicyCompilerAuthorityState, AppGamePolicyCompilerCapabilityEvidence,
    AppGamePolicyCompilerCapabilityState, AppGamePolicyCompilerContext,
    AppGamePolicyCompilerEvidence, AppGamePolicyCompilerEvidenceState,
    AppGamePolicyCompilerOutcomeState, AppGamePolicyCompilerProofKind,
    AppGamePolicyCompilerRedactionState, AppGamePolicyCompilerRejectionReason,
    AppGamePolicyCompilerRequestedAction, AppGamePolicyCompilerTarget,
    AppGamePolicyCompilerTraceBoundary, AppGamePolicyCompilerTraceOwner, AppGamePolicyTargetKind,
};
use ocentra_app_game_core::app_game_policy_target_compiler::{
    app_game_policy_target_compiler_rules_typescript, compile_app_game_policy_target,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn policy_compiler_accepts_specific_targets_only_with_identity_proof() {
    assert_ready(&compile(base_request(AppGamePolicyTargetKind::SpecificApp)));

    let mut missing = base_request(AppGamePolicyTargetKind::SpecificGame);
    remove_proof(&mut missing, AppGamePolicyCompilerProofKind::IdentityProof);
    assert_rejected(
        &compile(missing),
        AppGamePolicyCompilerRejectionReason::MissingIdentityProof,
    );
}

#[test]
fn policy_compiler_requires_unknown_and_category_proof_for_candidate_targets() {
    assert_ready(&compile(base_request(
        AppGamePolicyTargetKind::LauncherGameCandidate,
    )));

    let mut missing_unknown = base_request(AppGamePolicyTargetKind::UnknownApp);
    remove_proof(
        &mut missing_unknown,
        AppGamePolicyCompilerProofKind::UnknownStateProof,
    );
    assert_rejected(
        &compile(missing_unknown),
        AppGamePolicyCompilerRejectionReason::MissingUnknownStateProof,
    );

    let mut missing_category = base_request(AppGamePolicyTargetKind::RiskApp);
    remove_proof(
        &mut missing_category,
        AppGamePolicyCompilerProofKind::CategoryProof,
    );
    assert_rejected(
        &compile(missing_category),
        AppGamePolicyCompilerRejectionReason::MissingCategoryProof,
    );
}

#[test]
fn policy_compiler_covers_every_declared_target_family() {
    let identity_targets = [
        AppGamePolicyTargetKind::SpecificApp,
        AppGamePolicyTargetKind::PackageId,
        AppGamePolicyTargetKind::BundleId,
        AppGamePolicyTargetKind::AppUserModelId,
        AppGamePolicyTargetKind::DesktopEntryId,
        AppGamePolicyTargetKind::ExecutableHash,
        AppGamePolicyTargetKind::Publisher,
        AppGamePolicyTargetKind::SpecificGame,
        AppGamePolicyTargetKind::LauncherGameId,
        AppGamePolicyTargetKind::StoreGameId,
    ];
    let unknown_targets = [
        AppGamePolicyTargetKind::UnknownApp,
        AppGamePolicyTargetKind::NewApp,
        AppGamePolicyTargetKind::PortableApp,
        AppGamePolicyTargetKind::UnknownGame,
        AppGamePolicyTargetKind::NewGame,
        AppGamePolicyTargetKind::LauncherGameCandidate,
    ];
    let category_targets = [
        AppGamePolicyTargetKind::AppCategory,
        AppGamePolicyTargetKind::RiskApp,
        AppGamePolicyTargetKind::GameCategory,
        AppGamePolicyTargetKind::LauncherGameCandidate,
        AppGamePolicyTargetKind::MultiplayerGame,
        AppGamePolicyTargetKind::UgcGame,
        AppGamePolicyTargetKind::PurchaseCapableGame,
        AppGamePolicyTargetKind::MatureGame,
    ];

    assert_target_family_requires(
        &identity_targets,
        AppGamePolicyCompilerProofKind::IdentityProof,
        AppGamePolicyCompilerRejectionReason::MissingIdentityProof,
    );
    assert_target_family_requires(
        &unknown_targets,
        AppGamePolicyCompilerProofKind::UnknownStateProof,
        AppGamePolicyCompilerRejectionReason::MissingUnknownStateProof,
    );
    assert_target_family_requires(
        &category_targets,
        AppGamePolicyCompilerProofKind::CategoryProof,
        AppGamePolicyCompilerRejectionReason::MissingCategoryProof,
    );

    let mut covered = identity_targets.to_vec();
    covered.extend(unknown_targets);
    covered.extend(category_targets);
    covered.extend([
        AppGamePolicyTargetKind::AllNonSystemApps,
        AppGamePolicyTargetKind::AllGames,
    ]);
    covered.sort_by_key(|target| *target as u8);
    covered.dedup();
    assert_eq!(covered.len(), 25);
}

#[test]
fn policy_compiler_rejects_wrong_device_wrong_user_and_stale_evidence() {
    let mut wrong_device = base_request(AppGamePolicyTargetKind::SpecificApp);
    wrong_device.evidence[0].device_id = device_id("device-other");
    assert_rejected(
        &compile(wrong_device),
        AppGamePolicyCompilerRejectionReason::WrongDevice,
    );

    let mut wrong_user = base_request(AppGamePolicyTargetKind::SpecificApp);
    wrong_user.evidence[0].local_user_ref = local_user_ref("user-other");
    assert_rejected(
        &compile(wrong_user),
        AppGamePolicyCompilerRejectionReason::WrongLocalUser,
    );

    let mut stale = base_request(AppGamePolicyTargetKind::SpecificApp);
    stale.evidence[0].evidence_state = AppGamePolicyCompilerEvidenceState::Stale;
    assert_rejected(
        &compile(stale),
        AppGamePolicyCompilerRejectionReason::StaleEvidence,
    );
}

#[test]
fn policy_compiler_requires_schedule_proof_for_scheduled_rules() {
    let mut request = base_request(AppGamePolicyTargetKind::AllNonSystemApps);
    request.schedule_ref = Some(
        AppGamePolicyScheduleRef::parse("schedule-school-night").expect_value("valid schedule ref"),
    );
    remove_proof(&mut request, AppGamePolicyCompilerProofKind::ScheduleProof);

    assert_rejected(
        &compile(request),
        AppGamePolicyCompilerRejectionReason::MissingScheduleProof,
    );
}

#[test]
fn policy_compiler_keeps_unproved_hard_actions_manual_required() {
    for action in [
        AppGamePolicyCompilerRequestedAction::TerminateRunning,
        AppGamePolicyCompilerRequestedAction::HideApp,
        AppGamePolicyCompilerRequestedAction::SuspendApp,
        AppGamePolicyCompilerRequestedAction::ShieldApp,
    ] {
        let mut request = base_request(AppGamePolicyTargetKind::SpecificApp);
        request.requested_action = action;
        remove_proof(&mut request, AppGamePolicyCompilerProofKind::ApprovalProof);
        assert_manual(
            &compile(request),
            AppGamePolicyCompilerRejectionReason::HardActionManualRequired,
        );
    }

    let mut block = base_request(AppGamePolicyTargetKind::SpecificApp);
    block.requested_action = AppGamePolicyCompilerRequestedAction::BlockLaunch;
    block.capability_refs[0].capability_state =
        AppGamePolicyCompilerCapabilityState::ManualRequired;
    assert_manual(
        &compile(block),
        AppGamePolicyCompilerRejectionReason::BlockLaunchManualRequired,
    );

    let mut proved_block = base_request(AppGamePolicyTargetKind::SpecificApp);
    proved_block.requested_action = AppGamePolicyCompilerRequestedAction::BlockLaunch;
    assert_ready(&compile(proved_block));
}

#[test]
fn policy_compiler_fails_closed_for_missing_capability_and_authority() {
    let mut missing_capability = base_request(AppGamePolicyTargetKind::SpecificApp);
    missing_capability.capability_refs.clear();
    assert_rejected(
        &compile(missing_capability),
        AppGamePolicyCompilerRejectionReason::MissingCapabilityProof,
    );

    let mut missing_authority = base_request(AppGamePolicyTargetKind::SpecificApp);
    missing_authority.authority_refs.clear();
    remove_proof(
        &mut missing_authority,
        AppGamePolicyCompilerProofKind::AuthorityProof,
    );
    assert_manual(
        &compile(missing_authority),
        AppGamePolicyCompilerRejectionReason::MissingAuthorityProof,
    );
}

#[test]
fn policy_compiler_ready_output_is_dry_run_and_carries_only_input_refs() {
    let compilation = compile(base_request(AppGamePolicyTargetKind::SpecificApp));

    assert_ready(&compilation);
    assert!(compilation.decision.dry_run);
    assert_eq!(
        compilation
            .decision
            .evidence_refs
            .iter()
            .map(AppGamePolicyEvidenceRef::as_str)
            .collect::<Vec<_>>(),
        vec![
            "evidence-identity",
            "evidence-unknown",
            "evidence-category",
            "evidence-schedule",
            "evidence-approval",
            "evidence-authority",
            "evidence-capability",
        ]
    );
    assert_eq!(
        compilation.decision.rule_refs[0].as_str(),
        "rule-app-game-1"
    );
    assert_eq!(
        compilation.decision.capability_refs[0].as_str(),
        "capability-app-game-1"
    );
    assert_eq!(
        compilation.decision.authority_refs[0].as_str(),
        "authority-app-game-1"
    );
    assert_eq!(
        compilation.decision.audit_refs[0].as_str(),
        "audit-app-game-1"
    );
    assert_eq!(compilation.trace.run_id.as_str(), "audit-app-game-1");
    assert_eq!(
        compilation.trace.correlation_id.as_str(),
        "compile-request-app-game-1"
    );
    assert_eq!(
        compilation.trace.owner,
        AppGamePolicyCompilerTraceOwner::AppGameCore
    );
    assert_eq!(
        compilation.trace.boundary,
        AppGamePolicyCompilerTraceBoundary::PolicyTargetCompiler
    );
    assert_eq!(
        compilation.trace.redaction_state,
        AppGamePolicyCompilerRedactionState::OpaqueReferencesOnly
    );
    assert_eq!(compilation.trace.no_claim_reason, None);
}

#[test]
fn policy_compiler_rejects_unbound_capability_and_authority_evidence() {
    let mut capability = base_request(AppGamePolicyTargetKind::SpecificApp);
    capability.capability_refs[0].evidence_refs = vec![evidence_ref("not-in-request")];
    assert_rejected(
        &compile(capability),
        AppGamePolicyCompilerRejectionReason::UnboundCapabilityEvidence,
    );

    let mut authority = base_request(AppGamePolicyTargetKind::SpecificApp);
    authority.authority_refs[0].evidence_refs = vec![evidence_ref("not-in-request")];
    assert_rejected(
        &compile(authority),
        AppGamePolicyCompilerRejectionReason::UnboundAuthorityEvidence,
    );
}

#[test]
fn policy_compiler_requires_refs_for_concrete_targets_but_not_aggregate_targets() {
    let mut concrete = base_request(AppGamePolicyTargetKind::PackageId);
    concrete.target.target_ref = None;
    assert_rejected(
        &compile(concrete),
        AppGamePolicyCompilerRejectionReason::MissingTargetReference,
    );

    let mut aggregate = base_request(AppGamePolicyTargetKind::AllGames);
    aggregate.target.target_ref = None;
    assert_ready(&compile(aggregate));
}

#[test]
fn generated_app_game_policy_target_compiler_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-game-policy-target-compiler-rules.ts"
    );
    let generated = app_game_policy_target_compiler_rules_typescript();

    assert_eq!(checked_in, generated);
}

fn compile(request: AppGamePolicyCompileRequest) -> AppGamePolicyCompilation {
    compile_app_game_policy_target(
        request,
        AppGamePolicyCompilerContext {
            compiled_decision_id: AppGamePolicyCompiledDecisionId::parse(
                "compiled-decision-app-game-1",
            )
            .expect_value("valid compiled decision id"),
            audit_ref: AppGamePolicyAuditRef::parse("audit-app-game-1")
                .expect_value("valid audit ref"),
        },
    )
}

fn base_request(target_kind: AppGamePolicyTargetKind) -> AppGamePolicyCompileRequest {
    let device_id = device_id("device-app-game-1");
    let local_user_ref = local_user_ref("local-user-app-game-1");
    let evidence = [
        (
            AppGamePolicyCompilerProofKind::IdentityProof,
            "evidence-identity",
        ),
        (
            AppGamePolicyCompilerProofKind::UnknownStateProof,
            "evidence-unknown",
        ),
        (
            AppGamePolicyCompilerProofKind::CategoryProof,
            "evidence-category",
        ),
        (
            AppGamePolicyCompilerProofKind::ScheduleProof,
            "evidence-schedule",
        ),
        (
            AppGamePolicyCompilerProofKind::ApprovalProof,
            "evidence-approval",
        ),
        (
            AppGamePolicyCompilerProofKind::AuthorityProof,
            "evidence-authority",
        ),
        (
            AppGamePolicyCompilerProofKind::CapabilityProof,
            "evidence-capability",
        ),
    ]
    .into_iter()
    .map(|(proof_kind, reference)| AppGamePolicyCompilerEvidence {
        evidence_ref: evidence_ref(reference),
        proof_kind,
        evidence_state: AppGamePolicyCompilerEvidenceState::Active,
        device_id: device_id.clone(),
        local_user_ref: local_user_ref.clone(),
    })
    .collect::<Vec<_>>();

    AppGamePolicyCompileRequest {
        compile_request_id: AppGamePolicyCompileRequestId::parse("compile-request-app-game-1")
            .expect_value("valid compile request id"),
        rule_ref: AppGamePolicyRuleRef::parse("rule-app-game-1").expect_value("valid rule ref"),
        device_id,
        local_user_ref,
        target: AppGamePolicyCompilerTarget {
            target_kind,
            target_ref: Some(
                AppGamePolicyTargetRef::parse("target-app-game-1").expect_value("valid target ref"),
            ),
        },
        requested_action: AppGamePolicyCompilerRequestedAction::TimeLimit,
        schedule_ref: None,
        capability_refs: vec![AppGamePolicyCompilerCapabilityEvidence {
            capability_ref: AppGamePolicyCapabilityRef::parse("capability-app-game-1")
                .expect_value("valid capability ref"),
            capability_state: AppGamePolicyCompilerCapabilityState::Supported,
            evidence_refs: vec![evidence_ref("evidence-capability")],
        }],
        authority_refs: vec![AppGamePolicyCompilerAuthorityEvidence {
            authority_ref: AppGamePolicyAuthorityRef::parse("authority-app-game-1")
                .expect_value("valid authority ref"),
            authority_state: AppGamePolicyCompilerAuthorityState::Proved,
            evidence_refs: vec![evidence_ref("evidence-authority")],
        }],
        evidence,
    }
}

fn remove_proof(
    request: &mut AppGamePolicyCompileRequest,
    proof_kind: AppGamePolicyCompilerProofKind,
) {
    request
        .evidence
        .retain(|entry| entry.proof_kind != proof_kind);
}

fn assert_target_family_requires(
    targets: &[AppGamePolicyTargetKind],
    proof_kind: AppGamePolicyCompilerProofKind,
    reason: AppGamePolicyCompilerRejectionReason,
) {
    for target in targets {
        let mut request = base_request(*target);
        remove_proof(&mut request, proof_kind);
        assert_rejected(&compile(request), reason);
    }
}

fn assert_ready(compilation: &AppGamePolicyCompilation) {
    assert_eq!(
        compilation.decision.outcome_state,
        AppGamePolicyCompilerOutcomeState::DryRunReady
    );
    assert_eq!(
        compilation.decision.rejection_reason,
        AppGamePolicyCompilerRejectionReason::None
    );
}

fn assert_rejected(
    compilation: &AppGamePolicyCompilation,
    reason: AppGamePolicyCompilerRejectionReason,
) {
    assert_eq!(
        compilation.decision.outcome_state,
        AppGamePolicyCompilerOutcomeState::Rejected
    );
    assert_eq!(compilation.decision.rejection_reason, reason);
    assert_eq!(compilation.trace.no_claim_reason, Some(reason));
}

fn assert_manual(
    compilation: &AppGamePolicyCompilation,
    reason: AppGamePolicyCompilerRejectionReason,
) {
    assert_eq!(
        compilation.decision.outcome_state,
        AppGamePolicyCompilerOutcomeState::ManualRequired
    );
    assert_eq!(compilation.decision.rejection_reason, reason);
    assert_eq!(compilation.trace.no_claim_reason, Some(reason));
}

fn device_id(value: &str) -> AppGamePolicyDeviceId {
    AppGamePolicyDeviceId::parse(value).expect_value("valid device id")
}

fn local_user_ref(value: &str) -> AppGamePolicyLocalUserRef {
    AppGamePolicyLocalUserRef::parse(value).expect_value("valid local user ref")
}

fn evidence_ref(value: &str) -> AppGamePolicyEvidenceRef {
    AppGamePolicyEvidenceRef::parse(value).expect_value("valid evidence ref")
}
