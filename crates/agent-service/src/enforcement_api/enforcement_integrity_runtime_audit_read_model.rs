use ocentra_parent_agent_protocol::{
    constants::v08_enforcement_integrity_runtime_audit as proof, policy_constants, ParentPlatform,
    V08EnforcementIntegrityRuntimeAuditAuditState, V08EnforcementIntegrityRuntimeAuditChildState,
    V08EnforcementIntegrityRuntimeAuditEntry, V08EnforcementIntegrityRuntimeAuditExecution,
    V08EnforcementIntegrityRuntimeAuditIntegrityState, V08EnforcementIntegrityRuntimeAuditIntentState,
    V08EnforcementIntegrityRuntimeAuditReadModel, V08EnforcementIntegrityRuntimeAuditResult,
    V08EnforcementIntegrityRuntimeAuditRollbackState, V08EnforcementIntegrityRuntimeAuditSurface,
    V08EnforcementIntegrityRuntimeAuditTimerState,
};

pub(crate) fn v08_enforcement_integrity_runtime_audit_read_model(
    generated_at: &str,
) -> V08EnforcementIntegrityRuntimeAuditReadModel {
    V08EnforcementIntegrityRuntimeAuditReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![
            proof::SOURCE_SUPPORTED_ADAPTER_RUNTIME_PROOF.to_string(),
            proof::SOURCE_POLICY_DISPATCH_PROOF.to_string(),
            proof::SOURCE_PRODUCT_CONTROL_SPINE.to_string(),
            proof::SOURCE_ENFORCEMENT_AUDIT_JOURNAL.to_string(),
            proof::SOURCE_TIMER_RECOVERY_STATE.to_string(),
        ],
        entries: entry_specs()
            .iter()
            .map(|spec| entry_from_spec(spec, generated_at))
            .collect(),
    }
}

struct EntrySpec {
    audit_entry_id: &'static str,
    surface: V08EnforcementIntegrityRuntimeAuditSurface,
    platform: ParentPlatform,
    result: V08EnforcementIntegrityRuntimeAuditResult,
    execution: V08EnforcementIntegrityRuntimeAuditExecution,
    intent_state: V08EnforcementIntegrityRuntimeAuditIntentState,
    timer_state: V08EnforcementIntegrityRuntimeAuditTimerState,
    rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState,
    child_state: V08EnforcementIntegrityRuntimeAuditChildState,
    integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState,
    audit_state: V08EnforcementIntegrityRuntimeAuditAuditState,
    policy_decision_refs: &'static [&'static str],
    evidence_refs: &'static [&'static str],
    adapter_outcome_refs: &'static [&'static str],
    audit_refs: &'static [&'static str],
    rollback_refs: &'static [&'static str],
    timer_refs: &'static [&'static str],
    child_status_refs: &'static [&'static str],
    integrity_refs: &'static [&'static str],
    parent_intent_refs: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
    boundary: &'static str,
}

struct SupportedSpecInput {
    audit_entry_id: &'static str,
    result: V08EnforcementIntegrityRuntimeAuditResult,
    timer_state: V08EnforcementIntegrityRuntimeAuditTimerState,
    rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState,
    child_state: V08EnforcementIntegrityRuntimeAuditChildState,
    parent_intent_refs: &'static [&'static str],
    boundary: &'static str,
}

struct NoExecutionSpecInput {
    audit_entry_id: &'static str,
    surface: V08EnforcementIntegrityRuntimeAuditSurface,
    result: V08EnforcementIntegrityRuntimeAuditResult,
    execution: V08EnforcementIntegrityRuntimeAuditExecution,
    intent_state: V08EnforcementIntegrityRuntimeAuditIntentState,
    child_status_refs: &'static [&'static str],
    evidence_refs: &'static [&'static str],
    boundary: &'static str,
}

struct IntegrityUnavailableSpecInput {
    audit_entry_id: &'static str,
    surface: V08EnforcementIntegrityRuntimeAuditSurface,
    execution: V08EnforcementIntegrityRuntimeAuditExecution,
    timer_state: V08EnforcementIntegrityRuntimeAuditTimerState,
    integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState,
    manual_proof_requirements: &'static [&'static str],
    boundary: &'static str,
}

fn entry_specs() -> Vec<EntrySpec> {
    vec![
        supported_spec(SupportedSpecInput {
            audit_entry_id: proof::ENTRY_APP_TIME_LIMIT_SUCCEEDED,
            result: V08EnforcementIntegrityRuntimeAuditResult::Succeeded,
            timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::ActiveTimerBacked,
            rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState::RollbackTokenBacked,
            child_state: V08EnforcementIntegrityRuntimeAuditChildState::ReasonRefBacked,
            parent_intent_refs: &[],
            boundary: "Owned-process app/game time-limit actions can execute only with policy, evidence, timer, rollback, child-reason, and audit references.",
        }),
        supported_spec(SupportedSpecInput {
            audit_entry_id: proof::ENTRY_APP_TIME_LIMIT_EXPIRED,
            result: V08EnforcementIntegrityRuntimeAuditResult::Expired,
            timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::ExpiredBacked,
            rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState::NotNeeded,
            child_state: V08EnforcementIntegrityRuntimeAuditChildState::StatusRefBacked,
            parent_intent_refs: &[],
            boundary: "Expiry is audit-backed by timer state and child-facing status refs; it does not imply broad app blocking.",
        }),
        supported_spec(SupportedSpecInput {
            audit_entry_id: proof::ENTRY_APP_TIME_LIMIT_ROLLED_BACK,
            result: V08EnforcementIntegrityRuntimeAuditResult::RolledBack,
            timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::RollbackBacked,
            rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState::RollbackCompleted,
            child_state: V08EnforcementIntegrityRuntimeAuditChildState::StatusRefBacked,
            parent_intent_refs: &[],
            boundary: "Rollback is a typed supported-boundary state with rollback refs and audit refs, not an implicit unblock claim for unproved adapters.",
        }),
        supported_spec(SupportedSpecInput {
            audit_entry_id: proof::ENTRY_PARENT_OVERRIDE_SUPERSEDED,
            result: V08EnforcementIntegrityRuntimeAuditResult::Superseded,
            timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::CancelledBacked,
            rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState::RollbackTokenBacked,
            child_state: V08EnforcementIntegrityRuntimeAuditChildState::ApprovalIntentBacked,
            parent_intent_refs: &[proof::REF_PARENT_OVERRIDE_INTENT],
            boundary: "Parent override supersedes a validated action through auditable intent refs owned by the agent runtime.",
        }),
        no_execution_spec(NoExecutionSpecInput {
            audit_entry_id: proof::ENTRY_DRY_RUN_NO_OP,
            surface: V08EnforcementIntegrityRuntimeAuditSurface::AppGameTimeLimit,
            result: V08EnforcementIntegrityRuntimeAuditResult::NoOp,
            execution: V08EnforcementIntegrityRuntimeAuditExecution::DryRunNoAdapterExecution,
            intent_state: V08EnforcementIntegrityRuntimeAuditIntentState::ObserveOnly,
            child_status_refs: &[],
            evidence_refs: &["policy-preview-ref"],
            boundary: "Dry-run and observe previews are audit-visible no-ops and must not execute adapters.",
        }),
        no_execution_spec(NoExecutionSpecInput {
            audit_entry_id: proof::ENTRY_STALE_POLICY_REJECTED,
            surface: V08EnforcementIntegrityRuntimeAuditSurface::AppGameTimeLimit,
            result: V08EnforcementIntegrityRuntimeAuditResult::Failed,
            execution: V08EnforcementIntegrityRuntimeAuditExecution::RejectedBeforeAdapter,
            intent_state: V08EnforcementIntegrityRuntimeAuditIntentState::RejectedStale,
            child_status_refs: &["child-status-stale-policy-ref"],
            evidence_refs: &["policy-preview-ref"],
            boundary: "Stale policy decisions reject before adapter execution and keep a child-facing reason/status ref.",
        }),
        no_execution_spec(NoExecutionSpecInput {
            audit_entry_id: proof::ENTRY_WRONG_DEVICE_REJECTED,
            surface: V08EnforcementIntegrityRuntimeAuditSurface::AppGameTimeLimit,
            result: V08EnforcementIntegrityRuntimeAuditResult::Failed,
            execution: V08EnforcementIntegrityRuntimeAuditExecution::RejectedBeforeAdapter,
            intent_state: V08EnforcementIntegrityRuntimeAuditIntentState::RejectedWrongDevice,
            child_status_refs: &["child-status-wrong-device-ref"],
            evidence_refs: &["policy-preview-ref"],
            boundary: "Wrong-device intents reject before adapter execution and remain auditable.",
        }),
        no_execution_spec(NoExecutionSpecInput {
            audit_entry_id: proof::ENTRY_NETWORK_OBSERVE_ONLY,
            surface: V08EnforcementIntegrityRuntimeAuditSurface::NetworkDomainObserveOnly,
            result: V08EnforcementIntegrityRuntimeAuditResult::ObserveOnly,
            execution: V08EnforcementIntegrityRuntimeAuditExecution::ObserveOnlyNoExecution,
            intent_state: V08EnforcementIntegrityRuntimeAuditIntentState::ObserveOnly,
            child_status_refs: &[],
            evidence_refs: &[proof::REF_NETWORK_FLOW_SUMMARY],
            boundary: "Network/domain runtime state is observe-only over stored flow evidence; host DNS/filter enforcement is not executed.",
        }),
        manual_spec(
            proof::ENTRY_HOST_NETWORK_MANUAL,
            V08EnforcementIntegrityRuntimeAuditSurface::HostNetworkDomainFilter,
            &[proof::REQUIREMENT_HOST_DNS_OR_FILTER_APPLY, proof::REQUIREMENT_HOST_FILTER_ROLLBACK],
            "Host network/domain filtering remains manual-required until apply, rollback, and audit artifacts exist.",
        ),
        unavailable_spec(IntegrityUnavailableSpecInput {
            audit_entry_id: proof::ENTRY_PERMISSION_LOSS,
            surface: V08EnforcementIntegrityRuntimeAuditSurface::IntegrityHeartbeat,
            execution: V08EnforcementIntegrityRuntimeAuditExecution::UnavailableNoExecution,
            timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::Unavailable,
            integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState::PermissionMissing,
            manual_proof_requirements: &[proof::REQUIREMENT_PERMISSION_RESTORE, proof::REQUIREMENT_OPERATOR_PERMISSION_STATE],
            boundary: "Permission loss is explicit unavailable state and must not be reported as enforcement success.",
        }),
        unavailable_spec(IntegrityUnavailableSpecInput {
            audit_entry_id: proof::ENTRY_ADAPTER_UNAVAILABLE,
            surface: V08EnforcementIntegrityRuntimeAuditSurface::IntegrityHeartbeat,
            execution: V08EnforcementIntegrityRuntimeAuditExecution::RecoveryNeededNoExecution,
            timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::RecoveryNeeded,
            integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState::AdapterUnavailable,
            manual_proof_requirements: &[proof::REQUIREMENT_ADAPTER_RECOVERY, proof::REQUIREMENT_SERVICE_RESTART_RECOVERY],
            boundary: "Adapter recovery is explicit recovery-needed/unavailable state where persisted restart recovery is not proved.",
        }),
        unavailable_spec(IntegrityUnavailableSpecInput {
            audit_entry_id: proof::ENTRY_STALE_HEARTBEAT,
            surface: V08EnforcementIntegrityRuntimeAuditSurface::IntegrityHeartbeat,
            execution: V08EnforcementIntegrityRuntimeAuditExecution::UnavailableNoExecution,
            timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::Unavailable,
            integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState::StaleHeartbeat,
            manual_proof_requirements: &[proof::REQUIREMENT_FRESH_HEARTBEAT, proof::REQUIREMENT_PARENT_VISIBLE_STALE_ALERT],
            boundary: "Stale heartbeat is parent-visible degraded integrity state, not anti-tamper hardening.",
        }),
        unsupported_spec(),
        manual_tamper_spec(),
    ]
}

fn supported_spec(input: SupportedSpecInput) -> EntrySpec {
    EntrySpec {
        audit_entry_id: input.audit_entry_id,
        surface: V08EnforcementIntegrityRuntimeAuditSurface::AppGameTimeLimit,
        platform: ParentPlatform::Windows,
        result: input.result,
        execution: V08EnforcementIntegrityRuntimeAuditExecution::ExecutedSupportedBoundary,
        intent_state: V08EnforcementIntegrityRuntimeAuditIntentState::Validated,
        timer_state: input.timer_state,
        rollback_state: input.rollback_state,
        child_state: input.child_state,
        integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState::Running,
        audit_state: V08EnforcementIntegrityRuntimeAuditAuditState::AuditBacked,
        policy_decision_refs: &[proof::REF_POLICY_DECISION],
        evidence_refs: &[proof::REF_APP_SESSION_EVIDENCE, proof::REF_OWNED_PROCESS_IDENTITY],
        adapter_outcome_refs: &[proof::REF_ADAPTER_OUTCOME],
        audit_refs: &[proof::REF_ENFORCEMENT_AUDIT],
        rollback_refs: &["rollback-token-ref"],
        timer_refs: &[proof::REF_TIMER_STATE],
        child_status_refs: &[proof::REF_CHILD_STATUS],
        integrity_refs: &[proof::REF_INTEGRITY_HEARTBEAT],
        parent_intent_refs: input.parent_intent_refs,
        manual_proof_requirements: &[],
        boundary: input.boundary,
    }
}

fn no_execution_spec(input: NoExecutionSpecInput) -> EntrySpec {
    EntrySpec {
        audit_entry_id: input.audit_entry_id,
        surface: input.surface,
        platform: ParentPlatform::Windows,
        result: input.result,
        execution: input.execution,
        intent_state: input.intent_state,
        timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::NotApplicable,
        rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState::NotNeeded,
        child_state: if input.child_status_refs.is_empty() {
            V08EnforcementIntegrityRuntimeAuditChildState::NotClaimed
        } else {
            V08EnforcementIntegrityRuntimeAuditChildState::ReasonRefBacked
        },
        integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState::Running,
        audit_state: V08EnforcementIntegrityRuntimeAuditAuditState::AuditBacked,
        policy_decision_refs: &[],
        evidence_refs: input.evidence_refs,
        adapter_outcome_refs: &[],
        audit_refs: &[proof::REF_ENFORCEMENT_AUDIT],
        rollback_refs: &[],
        timer_refs: &[],
        child_status_refs: input.child_status_refs,
        integrity_refs: &[proof::REF_INTEGRITY_HEARTBEAT],
        parent_intent_refs: &[],
        manual_proof_requirements: &[],
        boundary: input.boundary,
    }
}

fn manual_spec(
    audit_entry_id: &'static str,
    surface: V08EnforcementIntegrityRuntimeAuditSurface,
    manual_proof_requirements: &'static [&'static str],
    boundary: &'static str,
) -> EntrySpec {
    EntrySpec {
        audit_entry_id,
        surface,
        platform: ParentPlatform::Windows,
        result: V08EnforcementIntegrityRuntimeAuditResult::ManualRequired,
        execution: V08EnforcementIntegrityRuntimeAuditExecution::ManualRequiredNoExecution,
        intent_state: V08EnforcementIntegrityRuntimeAuditIntentState::RejectedUnsupported,
        timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::Unavailable,
        rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState::ManualRequired,
        child_state: V08EnforcementIntegrityRuntimeAuditChildState::ManualRequired,
        integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState::NotApplicable,
        audit_state: V08EnforcementIntegrityRuntimeAuditAuditState::ManualRequired,
        policy_decision_refs: &[],
        evidence_refs: &[],
        adapter_outcome_refs: &[],
        audit_refs: &[],
        rollback_refs: &[],
        timer_refs: &[],
        child_status_refs: &[],
        integrity_refs: &[],
        parent_intent_refs: &[],
        manual_proof_requirements,
        boundary,
    }
}

fn unavailable_spec(input: IntegrityUnavailableSpecInput) -> EntrySpec {
    EntrySpec {
        audit_entry_id: input.audit_entry_id,
        surface: input.surface,
        platform: ParentPlatform::Windows,
        result: V08EnforcementIntegrityRuntimeAuditResult::Unavailable,
        execution: input.execution,
        intent_state: V08EnforcementIntegrityRuntimeAuditIntentState::Validated,
        timer_state: input.timer_state,
        rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState::Unavailable,
        child_state: V08EnforcementIntegrityRuntimeAuditChildState::ReasonRefBacked,
        integrity_state: input.integrity_state,
        audit_state: V08EnforcementIntegrityRuntimeAuditAuditState::AuditBacked,
        policy_decision_refs: &[proof::REF_POLICY_DECISION],
        evidence_refs: &[],
        adapter_outcome_refs: &[],
        audit_refs: &[proof::REF_ENFORCEMENT_AUDIT],
        rollback_refs: &[],
        timer_refs: if input.timer_state
            == V08EnforcementIntegrityRuntimeAuditTimerState::RecoveryNeeded
        {
            &[proof::REF_TIMER_RECOVERY_NEEDED]
        } else {
            &[]
        },
        child_status_refs: &["child-status-unavailable-ref"],
        integrity_refs: &["integrity-state-ref"],
        parent_intent_refs: &[],
        manual_proof_requirements: input.manual_proof_requirements,
        boundary: input.boundary,
    }
}

fn unsupported_spec() -> EntrySpec {
    EntrySpec {
        audit_entry_id: proof::ENTRY_MOBILE_UNSUPPORTED,
        surface: V08EnforcementIntegrityRuntimeAuditSurface::MobileChildControl,
        platform: ParentPlatform::Ios,
        result: V08EnforcementIntegrityRuntimeAuditResult::Unsupported,
        execution: V08EnforcementIntegrityRuntimeAuditExecution::UnsupportedNoExecution,
        intent_state: V08EnforcementIntegrityRuntimeAuditIntentState::RejectedUnsupported,
        timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::Unavailable,
        rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState::Unavailable,
        child_state: V08EnforcementIntegrityRuntimeAuditChildState::Unsupported,
        integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState::NotApplicable,
        audit_state: V08EnforcementIntegrityRuntimeAuditAuditState::Unavailable,
        policy_decision_refs: &[],
        evidence_refs: &[],
        adapter_outcome_refs: &[],
        audit_refs: &[],
        rollback_refs: &[],
        timer_refs: &[],
        child_status_refs: &[],
        integrity_refs: &[],
        parent_intent_refs: &[],
        manual_proof_requirements: &[
            proof::REQUIREMENT_IOS_FAMILY_CONTROLS,
            proof::REQUIREMENT_IOS_DEVICE_ACTIVITY,
        ],
        boundary: "Mobile child control remains unsupported without platform entitlement and device proof.",
    }
}

fn manual_tamper_spec() -> EntrySpec {
    let mut spec = manual_spec(
        proof::ENTRY_TAMPER_MANUAL,
        V08EnforcementIntegrityRuntimeAuditSurface::TamperUninstallSignal,
        &[
            proof::REQUIREMENT_SERVICE_MANAGER_STOP_PROOF,
            proof::REQUIREMENT_UNINSTALL_DETECTION_ARTIFACT,
            proof::REQUIREMENT_SECURITY_REVIEW,
        ],
        "Tamper/uninstall is represented as detectable/manual-required state only; no stealth, persistence, or anti-tamper hardening is claimed.",
    );
    spec.integrity_state = V08EnforcementIntegrityRuntimeAuditIntegrityState::TamperSignalManualRequired;
    spec
}

fn entry_from_spec(
    spec: &EntrySpec,
    generated_at: &str,
) -> V08EnforcementIntegrityRuntimeAuditEntry {
    V08EnforcementIntegrityRuntimeAuditEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        audit_entry_id: spec.audit_entry_id.to_string(),
        surface: spec.surface,
        platform: spec.platform,
        result: spec.result,
        execution: spec.execution,
        intent_state: spec.intent_state,
        timer_state: spec.timer_state,
        rollback_state: spec.rollback_state,
        child_state: spec.child_state,
        integrity_state: spec.integrity_state,
        audit_state: spec.audit_state,
        policy_decision_refs: to_strings(spec.policy_decision_refs),
        evidence_refs: to_strings(spec.evidence_refs),
        adapter_outcome_refs: to_strings(spec.adapter_outcome_refs),
        audit_refs: to_strings(spec.audit_refs),
        rollback_refs: to_strings(spec.rollback_refs),
        timer_refs: to_strings(spec.timer_refs),
        child_status_refs: to_strings(spec.child_status_refs),
        integrity_refs: to_strings(spec.integrity_refs),
        parent_intent_refs: to_strings(spec.parent_intent_refs),
        manual_proof_requirements: to_strings(spec.manual_proof_requirements),
        boundary: spec.boundary.to_string(),
        broad_installed_app_blocking_claimed: false,
        host_network_domain_blocking_claimed: false,
        exact_active_tab_enforcement_claimed: false,
        notification_delivery_claimed: false,
        tamper_hardening_claimed: false,
        mobile_privilege_claimed: false,
        stealth_persistence_claimed: false,
        privilege_escalation_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

fn to_strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
