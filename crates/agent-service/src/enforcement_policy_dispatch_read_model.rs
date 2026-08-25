use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_core::enforcement_policy_dispatch::validate_enforcement_policy_dispatch_read_model;
use ocentra_parent_agent_protocol::activity::policy::ParentActorReference;
use ocentra_parent_agent_protocol::activity::policy::ParentActorRole;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::activity::policy_context::ParentDeviceReference;
use ocentra_parent_agent_protocol::constants::v08_enforcement_policy_dispatch as dispatch;
use ocentra_parent_agent_protocol::enforcement::EnforcementAdapterKind;
use ocentra_parent_agent_protocol::enforcement::EnforcementCapabilityState;
use ocentra_parent_agent_protocol::enforcement::EnforcementMode;
use ocentra_parent_agent_protocol::enforcement::ParentActionReference;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchApprovalState;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchCapabilityMatrixRow;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchIntent;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchOutcomeState;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchProofLevel;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchReadModel;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchReadModelEntry;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchRejectionReason;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchSourceState;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchTimerState;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlParentAction;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSurface;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::schema_domain_mirrors::family::ParentActorReference as MirrorParentActorReference;
use ocentra_parent_agent_protocol::schema_domain_mirrors::family::ParentActorRole as MirrorParentActorRole;

#[path = "enforcement_policy_dispatch_read_model/policy_action.rs"]
mod policy_action;

use self::policy_action::policy_action_for;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DispatchText(pub(crate) String);

impl<T> From<T> for DispatchText
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for DispatchText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

pub(crate) fn v08_enforcement_policy_dispatch_read_model(
    generated_at: impl Into<DispatchText>,
) -> EnforcementPolicyDispatchReadModel {
    let generated_at = generated_at.into();
    let read_model = EnforcementPolicyDispatchReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: dispatch::READ_MODEL_ID.to_string(),
        generated_at: generated_at.0.clone(),
        entries: policy_dispatch_entries(&generated_at),
    };

    validate_enforcement_policy_dispatch_read_model(&read_model)
        .expect_value(dispatch::READ_MODEL_ID);
    read_model
}

fn policy_dispatch_entries(
    generated_at: &DispatchText,
) -> Vec<EnforcementPolicyDispatchReadModelEntry> {
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
    generated_at: &DispatchText,
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
                capability_state: EnforcementCapabilityState::ManualRequired,
                proof_level: EnforcementPolicyDispatchProofLevel::ManualRequired,
                outcome_state: EnforcementPolicyDispatchOutcomeState::ManualRequired,
                rejection_reason: EnforcementPolicyDispatchRejectionReason::AdapterManualRequired,
                source_state: EnforcementPolicyDispatchSourceState::Ready,
                approval_state: EnforcementPolicyDispatchApprovalState::ManualRequired,
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

fn report_only_dispatch_entry(
    generated_at: &DispatchText,
) -> EnforcementPolicyDispatchReadModelEntry {
    let generated_at = DispatchText(generated_at.to_string());
    dispatch_entry(
        &generated_at,
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
    generated_at: &DispatchText,
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

fn manual_required_dispatch_entry(
    generated_at: &DispatchText,
) -> EnforcementPolicyDispatchReadModelEntry {
    let generated_at = DispatchText(generated_at.to_string());
    dispatch_entry(
        &generated_at,
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
    generated_at: &DispatchText,
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

fn missing_source_rejected_entry(
    generated_at: &DispatchText,
) -> EnforcementPolicyDispatchReadModelEntry {
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

fn scaffold_dispatch_entry(generated_at: &DispatchText) -> EnforcementPolicyDispatchReadModelEntry {
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

#[derive(Clone, Copy)]
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
    generated_at: &DispatchText,
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
        audit_refs: vec![
            prefixed(
                DispatchText(dispatch::PREFIX_AUDIT.to_string()),
                &DispatchText(input.intent_id.to_string()),
            )
            .0,
        ],
        timer_refs: timer_refs(
            &DispatchText(input.intent_id.to_string()),
            input.timer_state,
        )
        .into_iter()
        .map(|text| text.0)
        .collect(),
        child_reason_code: input.child_reason_code.to_string(),
        reason_codes: vec![input.child_reason_code.to_string()],
        dispatched_at: dispatched_at_for(input.outcome_state, generated_at).map(|text| text.0),
        next_check_at: next_check_at_for(input.timer_state, generated_at).map(|text| text.0),
    }
}

fn dispatch_intent(
    generated_at: &DispatchText,
    input: &DispatchEntryInput,
) -> EnforcementPolicyDispatchIntent {
    let intent_id = DispatchText(input.intent_id.to_string());
    let policy_prefix = DispatchText(dispatch::PREFIX_POLICY.to_string());
    let decision_prefix = DispatchText(dispatch::PREFIX_DECISION.to_string());
    let target_prefix = DispatchText(dispatch::PREFIX_TARGET.to_string());
    let schedule_prefix = DispatchText(dispatch::PREFIX_SCHEDULE.to_string());
    EnforcementPolicyDispatchIntent {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        intent_id: intent_id.0.clone(),
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
        policy_decision_id: prefixed(policy_prefix, &intent_id).0,
        policy_decision_ref: prefixed(decision_prefix, &intent_id).0,
        policy_version: dispatch::POLICY_VERSION_V0_8_DISPATCH.to_string(),
        target: PolicyTarget {
            target_id: prefixed(target_prefix, &intent_id).0,
            target_type: input.target_type,
            target_value: input.target_value.to_string(),
        },
        requested_policy_action: policy_action_for(input.requested_action),
        requested_parent_action: input.requested_action,
        schedule_ref: prefixed(schedule_prefix, &intent_id).0,
        evidence_references: vec![ParentEvidenceReference {
            evidence_reference_id: input.evidence_reference_id.to_string(),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: generated_at.0.clone(),
        }],
        approval_ref: approval_ref_for(generated_at, input),
        route_ref: dispatch::LOCAL_DEV_AGENT_ROUTE_REF.to_string(),
        source_state: input.source_state,
        dry_run: input.dry_run,
        requested_at: generated_at.0.clone(),
    }
}

fn approval_ref_for(
    generated_at: &DispatchText,
    input: &DispatchEntryInput,
) -> Option<ParentActionReference> {
    if input.approval_state == EnforcementPolicyDispatchApprovalState::NotRequired {
        return None;
    }

    Some(ParentActionReference {
        action_reference_id: prefixed(
            DispatchText(dispatch::PREFIX_APPROVAL.to_string()),
            &DispatchText(input.intent_id.to_string()),
        )
        .0,
        actor: MirrorParentActorReference {
            actor_id: dispatch::PARENT_ACTOR_PRIMARY_ID.to_string(),
            role: MirrorParentActorRole::Parent,
        },
        policy_version: dispatch::POLICY_VERSION_V0_8_DISPATCH.to_string(),
        created_at: generated_at.0.clone(),
    })
}

fn timer_refs(
    intent_id: &DispatchText,
    timer_state: EnforcementPolicyDispatchTimerState,
) -> Vec<DispatchText> {
    if timer_state == EnforcementPolicyDispatchTimerState::NotRequired {
        return Vec::new();
    }

    vec![prefixed(
        DispatchText(dispatch::PREFIX_TIMER.to_string()),
        intent_id,
    )]
}

fn prefixed(prefix: DispatchText, value: &DispatchText) -> DispatchText {
    let mut output = prefix.0;
    output.push_str(&value.0);
    DispatchText(output)
}

fn dispatched_at_for(
    outcome_state: EnforcementPolicyDispatchOutcomeState,
    generated_at: &DispatchText,
) -> Option<DispatchText> {
    if outcome_state == EnforcementPolicyDispatchOutcomeState::DispatchReady {
        return Some(DispatchText(generated_at.0.clone()));
    }

    None
}

fn next_check_at_for(
    timer_state: EnforcementPolicyDispatchTimerState,
    generated_at: &DispatchText,
) -> Option<DispatchText> {
    match timer_state {
        EnforcementPolicyDispatchTimerState::Active
        | EnforcementPolicyDispatchTimerState::RestartRecovered
        | EnforcementPolicyDispatchTimerState::RecoveryNeeded => {
            Some(DispatchText(generated_at.0.clone()))
        }
        EnforcementPolicyDispatchTimerState::NotRequired
        | EnforcementPolicyDispatchTimerState::Expired
        | EnforcementPolicyDispatchTimerState::Cancelled
        | EnforcementPolicyDispatchTimerState::RollbackCompleted => None,
    }
}
