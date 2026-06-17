use ocentra_parent_agent_core::validate_enforcement_policy_dispatch_read_model;
use ocentra_parent_agent_protocol::{
    constants::v08_enforcement_policy_dispatch as dispatch, policy_constants,
    EnforcementAdapterKind, EnforcementCapabilityState, EnforcementMode,
    EnforcementPolicyDispatchApprovalState, EnforcementPolicyDispatchCapabilityMatrixRow,
    EnforcementPolicyDispatchIntent, EnforcementPolicyDispatchOutcomeState,
    EnforcementPolicyDispatchProofLevel, EnforcementPolicyDispatchReadModel,
    EnforcementPolicyDispatchReadModelEntry, EnforcementPolicyDispatchRejectionReason,
    EnforcementPolicyDispatchSourceState, EnforcementPolicyDispatchTimerState,
    ParentActionReference, ParentActorReference, ParentActorRole, ParentDeviceReference,
    ParentEvidenceReference, ParentEvidenceReferenceKind, ParentPlatform, PolicyAction,
    PolicyTarget, PolicyTargetType, V08EnforcementProductControlParentAction,
    V08EnforcementProductControlSurface,
};

pub(crate) fn v08_enforcement_policy_dispatch_read_model(
    generated_at: &str,
) -> EnforcementPolicyDispatchReadModel {
    let read_model = EnforcementPolicyDispatchReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: dispatch::READ_MODEL_ID.to_string(),
        generated_at: generated_at.to_string(),
        entries: policy_dispatch_entries(generated_at),
    };

    validate_enforcement_policy_dispatch_read_model(&read_model).unwrap();
    read_model
}

fn policy_dispatch_entries(generated_at: &str) -> Vec<EnforcementPolicyDispatchReadModelEntry> {
    let mut entries = implemented_dispatch_entries(generated_at);
    entries.push(ask_parent_dry_run_dispatch_entry(generated_at));
    entries.push(report_only_dispatch_entry(generated_at));
    entries.push(manual_required_dispatch_entry(generated_at));
    entries.push(stale_policy_version_rejected_entry(generated_at));
    entries.push(missing_source_rejected_entry(generated_at));
    entries.push(scaffold_dispatch_entry(generated_at));
    entries
}

fn implemented_dispatch_entries(
    generated_at: &str,
) -> Vec<EnforcementPolicyDispatchReadModelEntry> {
    vec![
        dispatch_entry(
            generated_at,
            DispatchEntryInput {
                intent_id: dispatch::INTENT_OWNED_PROCESS_TIME_LIMIT,
                matrix_id: dispatch::MATRIX_OWNED_PROCESS_IMPLEMENTED,
                surface: V08EnforcementProductControlSurface::WindowsOwnedProcessTimeLimit,
                adapter_kind: EnforcementAdapterKind::ProcessControl,
                requested_action: V08EnforcementProductControlParentAction::BlockScopedProcess,
                mode: EnforcementMode::TerminateProcess,
                capability_state: EnforcementCapabilityState::Supported,
                proof_level: EnforcementPolicyDispatchProofLevel::Implemented,
                outcome_state: EnforcementPolicyDispatchOutcomeState::DispatchReady,
                rejection_reason: EnforcementPolicyDispatchRejectionReason::None,
                source_state: EnforcementPolicyDispatchSourceState::Ready,
                approval_state: EnforcementPolicyDispatchApprovalState::NotRequired,
                timer_state: EnforcementPolicyDispatchTimerState::Active,
                child_reason_code: dispatch::CHILD_REASON_TIME_LIMIT,
                target_type: PolicyTargetType::App,
                target_value: dispatch::TARGET_OWNED_PROCESS_DEMO,
                evidence_reference_id: dispatch::EVIDENCE_APP_SESSION_OWNED_PROCESS,
                dry_run: false,
            },
        ),
        dispatch_entry(
            generated_at,
            DispatchEntryInput {
                intent_id: dispatch::INTENT_APP_GAME_SESSION_HANDOFF,
                matrix_id: dispatch::MATRIX_APP_GAME_TIME_LIMIT_IMPLEMENTED,
                surface: V08EnforcementProductControlSurface::WindowsAppTimeLimitLifecycle,
                adapter_kind: EnforcementAdapterKind::ProcessControl,
                requested_action: V08EnforcementProductControlParentAction::TimeLimit,
                mode: EnforcementMode::TimeLimit,
                capability_state: EnforcementCapabilityState::Supported,
                proof_level: EnforcementPolicyDispatchProofLevel::Implemented,
                outcome_state: EnforcementPolicyDispatchOutcomeState::DispatchReady,
                rejection_reason: EnforcementPolicyDispatchRejectionReason::None,
                source_state: EnforcementPolicyDispatchSourceState::Ready,
                approval_state: EnforcementPolicyDispatchApprovalState::Pending,
                timer_state: EnforcementPolicyDispatchTimerState::RestartRecovered,
                child_reason_code: dispatch::CHILD_REASON_BONUS_TIME,
                target_type: PolicyTargetType::App,
                target_value: dispatch::TARGET_APP_GAME_LAUNCHER,
                evidence_reference_id: dispatch::EVIDENCE_APP_GAME_SESSION_SUMMARY,
                dry_run: false,
            },
        ),
    ]
}

fn report_only_dispatch_entry(generated_at: &str) -> EnforcementPolicyDispatchReadModelEntry {
    dispatch_entry(
        generated_at,
        DispatchEntryInput {
            intent_id: dispatch::INTENT_UNMANAGED_BROWSER_REPORT_ONLY,
            matrix_id: dispatch::MATRIX_UNMANAGED_BROWSER_REPORT_ONLY,
            surface: V08EnforcementProductControlSurface::WindowsUnmanagedBrowserProcessFallback,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            requested_action: V08EnforcementProductControlParentAction::ReportOnly,
            mode: EnforcementMode::ObserveOnly,
            capability_state: EnforcementCapabilityState::Degraded,
            proof_level: EnforcementPolicyDispatchProofLevel::ReportOnly,
            outcome_state: EnforcementPolicyDispatchOutcomeState::ReportOnly,
            rejection_reason: EnforcementPolicyDispatchRejectionReason::None,
            source_state: EnforcementPolicyDispatchSourceState::Ready,
            approval_state: EnforcementPolicyDispatchApprovalState::NotRequired,
            timer_state: EnforcementPolicyDispatchTimerState::NotRequired,
            child_reason_code: dispatch::CHILD_REASON_BROWSER_REPORT_ONLY,
            target_type: PolicyTargetType::Site,
            target_value: dispatch::TARGET_UNMANAGED_BROWSER_PROCESS,
            evidence_reference_id: dispatch::EVIDENCE_UNMANAGED_BROWSER_PROCESS,
            dry_run: true,
        },
    )
}

fn ask_parent_dry_run_dispatch_entry(
    generated_at: &str,
) -> EnforcementPolicyDispatchReadModelEntry {
    dispatch_entry(
        generated_at,
        DispatchEntryInput {
            intent_id: dispatch::INTENT_ASK_PARENT_DRY_RUN,
            matrix_id: dispatch::MATRIX_ASK_PARENT_DRY_RUN,
            surface: V08EnforcementProductControlSurface::WindowsAppTimeLimitLifecycle,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            requested_action: V08EnforcementProductControlParentAction::AskParent,
            mode: EnforcementMode::ObserveOnly,
            capability_state: EnforcementCapabilityState::Supported,
            proof_level: EnforcementPolicyDispatchProofLevel::Scaffold,
            outcome_state: EnforcementPolicyDispatchOutcomeState::DryRunOnly,
            rejection_reason: EnforcementPolicyDispatchRejectionReason::None,
            source_state: EnforcementPolicyDispatchSourceState::Ready,
            approval_state: EnforcementPolicyDispatchApprovalState::Pending,
            timer_state: EnforcementPolicyDispatchTimerState::NotRequired,
            child_reason_code: dispatch::CHILD_REASON_ASK_PARENT_REVIEW,
            target_type: PolicyTargetType::App,
            target_value: dispatch::TARGET_ASK_PARENT_REVIEW,
            evidence_reference_id: dispatch::EVIDENCE_APP_GAME_SESSION_SUMMARY,
            dry_run: true,
        },
    )
}

fn manual_required_dispatch_entry(generated_at: &str) -> EnforcementPolicyDispatchReadModelEntry {
    dispatch_entry(
        generated_at,
        DispatchEntryInput {
            intent_id: dispatch::INTENT_NETWORK_DOMAIN_MANUAL_REQUIRED,
            matrix_id: dispatch::MATRIX_NETWORK_DOMAIN_MANUAL_REQUIRED,
            surface: V08EnforcementProductControlSurface::WindowsNetworkDomainBlocking,
            adapter_kind: EnforcementAdapterKind::NetworkControl,
            requested_action: V08EnforcementProductControlParentAction::ReportOnly,
            mode: EnforcementMode::TemporaryBlock,
            capability_state: EnforcementCapabilityState::ManualRequired,
            proof_level: EnforcementPolicyDispatchProofLevel::ManualRequired,
            outcome_state: EnforcementPolicyDispatchOutcomeState::ManualRequired,
            rejection_reason: EnforcementPolicyDispatchRejectionReason::AdapterManualRequired,
            source_state: EnforcementPolicyDispatchSourceState::Ready,
            approval_state: EnforcementPolicyDispatchApprovalState::ManualRequired,
            timer_state: EnforcementPolicyDispatchTimerState::NotRequired,
            child_reason_code: dispatch::CHILD_REASON_MANUAL_REQUIRED,
            target_type: PolicyTargetType::Domain,
            target_value: dispatch::TARGET_EXAMPLE_DOMAIN,
            evidence_reference_id: dispatch::EVIDENCE_NETWORK_FLOW_DOMAIN_SUMMARY,
            dry_run: false,
        },
    )
}

fn stale_policy_version_rejected_entry(
    generated_at: &str,
) -> EnforcementPolicyDispatchReadModelEntry {
    dispatch_entry(
        generated_at,
        DispatchEntryInput {
            intent_id: dispatch::INTENT_STALE_POLICY_VERSION_REJECTED,
            matrix_id: dispatch::MATRIX_STALE_POLICY_VERSION_REJECTED,
            surface: V08EnforcementProductControlSurface::WindowsAppTimeLimitLifecycle,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            requested_action: V08EnforcementProductControlParentAction::TimeLimit,
            mode: EnforcementMode::TimeLimit,
            capability_state: EnforcementCapabilityState::Supported,
            proof_level: EnforcementPolicyDispatchProofLevel::Scaffold,
            outcome_state: EnforcementPolicyDispatchOutcomeState::Rejected,
            rejection_reason: EnforcementPolicyDispatchRejectionReason::StalePolicyVersion,
            source_state: EnforcementPolicyDispatchSourceState::Stale,
            approval_state: EnforcementPolicyDispatchApprovalState::NotRequired,
            timer_state: EnforcementPolicyDispatchTimerState::NotRequired,
            child_reason_code: dispatch::CHILD_REASON_STALE_POLICY_VERSION,
            target_type: PolicyTargetType::App,
            target_value: dispatch::TARGET_APP_GAME_LAUNCHER,
            evidence_reference_id: dispatch::EVIDENCE_POLICY_DECISION_STALE,
            dry_run: false,
        },
    )
}

fn missing_source_rejected_entry(generated_at: &str) -> EnforcementPolicyDispatchReadModelEntry {
    dispatch_entry(
        generated_at,
        DispatchEntryInput {
            intent_id: dispatch::INTENT_MISSING_SOURCE_REJECTED,
            matrix_id: dispatch::MATRIX_MISSING_SOURCE_REJECTED,
            surface: V08EnforcementProductControlSurface::WindowsAppTimeLimitLifecycle,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            requested_action: V08EnforcementProductControlParentAction::TimeLimit,
            mode: EnforcementMode::TimeLimit,
            capability_state: EnforcementCapabilityState::Supported,
            proof_level: EnforcementPolicyDispatchProofLevel::Scaffold,
            outcome_state: EnforcementPolicyDispatchOutcomeState::Rejected,
            rejection_reason: EnforcementPolicyDispatchRejectionReason::SourceNotReady,
            source_state: EnforcementPolicyDispatchSourceState::Missing,
            approval_state: EnforcementPolicyDispatchApprovalState::NotRequired,
            timer_state: EnforcementPolicyDispatchTimerState::NotRequired,
            child_reason_code: dispatch::CHILD_REASON_SOURCE_NOT_READY,
            target_type: PolicyTargetType::App,
            target_value: dispatch::TARGET_POLICY_SOURCE_MISSING,
            evidence_reference_id: dispatch::EVIDENCE_POLICY_SOURCE_MISSING,
            dry_run: false,
        },
    )
}

fn scaffold_dispatch_entry(generated_at: &str) -> EnforcementPolicyDispatchReadModelEntry {
    dispatch_entry(
        generated_at,
        DispatchEntryInput {
            intent_id: dispatch::INTENT_TAMPER_ALERT_SCAFFOLD,
            matrix_id: dispatch::MATRIX_TAMPER_SCAFFOLD,
            surface: V08EnforcementProductControlSurface::WindowsTamperUninstallAlerts,
            adapter_kind: EnforcementAdapterKind::TimerControl,
            requested_action: V08EnforcementProductControlParentAction::Observe,
            mode: EnforcementMode::ObserveOnly,
            capability_state: EnforcementCapabilityState::Unavailable,
            proof_level: EnforcementPolicyDispatchProofLevel::Scaffold,
            outcome_state: EnforcementPolicyDispatchOutcomeState::Rejected,
            rejection_reason: EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved,
            source_state: EnforcementPolicyDispatchSourceState::Unavailable,
            approval_state: EnforcementPolicyDispatchApprovalState::NotRequired,
            timer_state: EnforcementPolicyDispatchTimerState::RecoveryNeeded,
            child_reason_code: dispatch::CHILD_REASON_INTEGRITY_PROOF,
            target_type: PolicyTargetType::Device,
            target_value: dispatch::LOCAL_DEV_AGENT_DEVICE_ID,
            evidence_reference_id: dispatch::EVIDENCE_INTEGRITY_HEARTBEAT_GAP,
            dry_run: false,
        },
    )
}

struct DispatchEntryInput {
    intent_id: &'static str,
    matrix_id: &'static str,
    surface: V08EnforcementProductControlSurface,
    adapter_kind: EnforcementAdapterKind,
    requested_action: V08EnforcementProductControlParentAction,
    mode: EnforcementMode,
    capability_state: EnforcementCapabilityState,
    proof_level: EnforcementPolicyDispatchProofLevel,
    outcome_state: EnforcementPolicyDispatchOutcomeState,
    rejection_reason: EnforcementPolicyDispatchRejectionReason,
    source_state: EnforcementPolicyDispatchSourceState,
    approval_state: EnforcementPolicyDispatchApprovalState,
    timer_state: EnforcementPolicyDispatchTimerState,
    child_reason_code: &'static str,
    target_type: PolicyTargetType,
    target_value: &'static str,
    evidence_reference_id: &'static str,
    dry_run: bool,
}

fn dispatch_entry(
    generated_at: &str,
    input: DispatchEntryInput,
) -> EnforcementPolicyDispatchReadModelEntry {
    EnforcementPolicyDispatchReadModelEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        intent: dispatch_intent(generated_at, &input),
        matrix_row: EnforcementPolicyDispatchCapabilityMatrixRow {
            matrix_id: input.matrix_id.to_string(),
            surface: input.surface,
            platform: ParentPlatform::Windows,
            adapter_kind: input.adapter_kind,
            requested_action: input.requested_action,
            mode: input.mode,
            capability_state: input.capability_state,
            proof_level: input.proof_level,
            outcome_state: input.outcome_state,
            rejection_reason: input.rejection_reason,
            source_state: input.source_state,
            child_reason_code: input.child_reason_code.to_string(),
        },
        approval_state: input.approval_state,
        timer_state: input.timer_state,
        audit_refs: vec![prefixed(dispatch::PREFIX_AUDIT, input.intent_id)],
        timer_refs: timer_refs(input.intent_id, input.timer_state),
        child_reason_code: input.child_reason_code.to_string(),
        reason_codes: vec![input.child_reason_code.to_string()],
        dispatched_at: dispatched_at_for(input.outcome_state, generated_at),
        next_check_at: next_check_at_for(input.timer_state, generated_at),
    }
}

fn dispatch_intent(
    generated_at: &str,
    input: &DispatchEntryInput,
) -> EnforcementPolicyDispatchIntent {
    EnforcementPolicyDispatchIntent {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        intent_id: input.intent_id.to_string(),
        actor: ParentActorReference {
            actor_id: dispatch::PARENT_ACTOR_PRIMARY_ID.to_string(),
            role: ParentActorRole::Parent,
        },
        device: ParentDeviceReference {
            device_id: dispatch::LOCAL_DEV_AGENT_DEVICE_ID.to_string(),
            child_profile_id: Some(dispatch::LOCAL_DEV_CHILD_PROFILE_ID.to_string()),
            label: dispatch::LOCAL_DEV_CHILD_DEVICE_LABEL.to_string(),
            platform: dispatch::WINDOWS_PLATFORM.to_string(),
        },
        policy_decision_id: prefixed(dispatch::PREFIX_POLICY, input.intent_id),
        policy_decision_ref: prefixed(dispatch::PREFIX_DECISION, input.intent_id),
        policy_version: dispatch::POLICY_VERSION_V0_8_DISPATCH.to_string(),
        target: PolicyTarget {
            target_id: prefixed(dispatch::PREFIX_TARGET, input.intent_id),
            target_type: input.target_type,
            target_value: input.target_value.to_string(),
        },
        requested_policy_action: policy_action_for(input.requested_action),
        requested_parent_action: input.requested_action,
        schedule_ref: prefixed(dispatch::PREFIX_SCHEDULE, input.intent_id),
        evidence_references: vec![ParentEvidenceReference {
            evidence_reference_id: input.evidence_reference_id.to_string(),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: generated_at.to_string(),
        }],
        approval_ref: approval_ref_for(generated_at, input),
        route_ref: dispatch::LOCAL_DEV_AGENT_ROUTE_REF.to_string(),
        source_state: input.source_state,
        dry_run: input.dry_run,
        requested_at: generated_at.to_string(),
    }
}

fn policy_action_for(action: V08EnforcementProductControlParentAction) -> PolicyAction {
    match action {
        V08EnforcementProductControlParentAction::Warn => PolicyAction::Warn,
        V08EnforcementProductControlParentAction::TimeLimit => PolicyAction::TimeLimit,
        V08EnforcementProductControlParentAction::BlockScopedProcess => PolicyAction::Block,
        V08EnforcementProductControlParentAction::AskParent => PolicyAction::AskParent,
        V08EnforcementProductControlParentAction::Observe
        | V08EnforcementProductControlParentAction::DryRunPreview
        | V08EnforcementProductControlParentAction::ReportOnly => PolicyAction::Allow,
    }
}

fn approval_ref_for(
    generated_at: &str,
    input: &DispatchEntryInput,
) -> Option<ParentActionReference> {
    if input.approval_state == EnforcementPolicyDispatchApprovalState::NotRequired {
        return None;
    }

    Some(ParentActionReference {
        action_reference_id: prefixed(dispatch::PREFIX_APPROVAL, input.intent_id),
        actor: ParentActorReference {
            actor_id: dispatch::PARENT_ACTOR_PRIMARY_ID.to_string(),
            role: ParentActorRole::Parent,
        },
        policy_version: dispatch::POLICY_VERSION_V0_8_DISPATCH.to_string(),
        created_at: generated_at.to_string(),
    })
}

fn timer_refs(intent_id: &str, timer_state: EnforcementPolicyDispatchTimerState) -> Vec<String> {
    if timer_state == EnforcementPolicyDispatchTimerState::NotRequired {
        return Vec::new();
    }

    vec![prefixed(dispatch::PREFIX_TIMER, intent_id)]
}

fn prefixed(prefix: &str, value: &str) -> String {
    let mut output = String::from(prefix);
    output.push_str(value);
    output
}

fn dispatched_at_for(
    outcome_state: EnforcementPolicyDispatchOutcomeState,
    generated_at: &str,
) -> Option<String> {
    if outcome_state == EnforcementPolicyDispatchOutcomeState::DispatchReady {
        return Some(generated_at.to_string());
    }

    None
}

fn next_check_at_for(
    timer_state: EnforcementPolicyDispatchTimerState,
    generated_at: &str,
) -> Option<String> {
    match timer_state {
        EnforcementPolicyDispatchTimerState::Active
        | EnforcementPolicyDispatchTimerState::RestartRecovered
        | EnforcementPolicyDispatchTimerState::RecoveryNeeded => Some(generated_at.to_string()),
        EnforcementPolicyDispatchTimerState::NotRequired
        | EnforcementPolicyDispatchTimerState::Expired
        | EnforcementPolicyDispatchTimerState::Cancelled
        | EnforcementPolicyDispatchTimerState::RollbackCompleted => None,
    }
}
