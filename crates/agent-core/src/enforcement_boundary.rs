use ocentra_parent_agent_protocol::{
    constants::enforcement as enforcement_constants, EnforcementAction, EnforcementAdapterKind,
    EnforcementAdapterResultCode, EnforcementAuditEvent, EnforcementAuditEventKind,
    EnforcementCapabilityState, EnforcementCapabilityStatus, EnforcementIntent, EnforcementMode,
    EnforcementResult, EnforcementResultStatus, EnforcementRollbackState, EnforcementTimerEvent,
    EnforcementTimerEventKind, EnforcementUnavailableReason, ParentEvidenceReference,
    ParentPlatform, PolicyAction, PolicyDecision, PolicyTargetType,
};

mod enforcement_authorization;
mod enforcement_timer_event;
mod enforcement_unavailable_status;

pub use enforcement_authorization::{
    authorize_enforcement_boundary, EnforcementAuthorizationOutcome,
};

use enforcement_timer_event::timer_event;
use enforcement_unavailable_status::{
    adapter_unavailable_reason, build_unavailable_status, capability_unavailable_reason,
};

use super::enforcement_adapter::EnforcementAdapterOutcome;

#[derive(Clone, Debug, PartialEq)]
pub struct EnforcementBoundaryInput {
    pub intent: EnforcementIntent,
    pub decision: PolicyDecision,
    pub capability: EnforcementCapabilityStatus,
    pub action_id: String,
    pub result_id: String,
    pub audit_event_id: String,
    pub timer_event_id: String,
    pub rollback_token: Option<String>,
    pub policy_version: String,
    pub requested_at: String,
    pub completed_at: Option<String>,
    pub adapter_outcome: Option<EnforcementAdapterOutcome>,
    pub timer_event_kind: Option<EnforcementTimerEventKind>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementBoundaryOutcome {
    pub action: EnforcementAction,
    pub result: EnforcementResult,
    pub audit_event: EnforcementAuditEvent,
    pub timer_event: Option<EnforcementTimerEvent>,
    pub adapter_request: Option<EnforcementAdapterRequest>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementAdapterRequest {
    pub action_id: String,
    pub adapter_kind: EnforcementAdapterKind,
    pub mode: EnforcementMode,
}

struct EnforcementResultParts {
    status: EnforcementResultStatus,
    adapter_result_code: EnforcementAdapterResultCode,
    completed_at: Option<String>,
    unavailable_reason: Option<String>,
    failed_reason: Option<String>,
    rollback_token: Option<String>,
    rollback_state: EnforcementRollbackState,
    unavailable_status_reason: Option<EnforcementUnavailableReason>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnforcementBoundaryRejection {
    PolicyDecisionIdMismatch,
    PolicyActionMismatch,
    PolicyTargetMismatch,
    MissingPolicyEvidenceReference,
    UnsupportedEnforcementCapability,
    AdapterResultRequired,
}

impl EnforcementBoundaryRejection {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::PolicyDecisionIdMismatch => enforcement_constants::REJECTION_DECISION_ID_MISMATCH,
            Self::PolicyActionMismatch => {
                enforcement_constants::REJECTION_POLICY_ACTION_NOT_ENFORCEABLE
            }
            Self::PolicyTargetMismatch => enforcement_constants::REJECTION_TARGET_MISMATCH,
            Self::MissingPolicyEvidenceReference => {
                enforcement_constants::REJECTION_MISSING_EVIDENCE
            }
            Self::UnsupportedEnforcementCapability => {
                enforcement_constants::REJECTION_UNSUPPORTED_CAPABILITY
            }
            Self::AdapterResultRequired => enforcement_constants::REJECTION_ADAPTER_RESULT_REQUIRED,
        }
    }
}

pub fn evaluate_enforcement_boundary(
    input: EnforcementBoundaryInput,
) -> Result<EnforcementBoundaryOutcome, EnforcementBoundaryRejection> {
    validate_intent_decision(&input.intent, &input.decision)?;
    let mode = enforcement_mode(&input.intent)?;
    let action = enforcement_action(&input, mode);
    let result = enforcement_result(&input, &action)?;
    let timer_event = timer_event(&input, &action, &result);
    let audit_event = enforcement_audit_event(&input, &action, &result);
    let adapter_request = adapter_request(&action, &result);

    Ok(EnforcementBoundaryOutcome {
        action,
        result,
        audit_event,
        timer_event,
        adapter_request,
    })
}

fn validate_intent_decision(
    intent: &EnforcementIntent,
    decision: &PolicyDecision,
) -> Result<(), EnforcementBoundaryRejection> {
    if intent.policy_decision_id != decision.decision_id {
        return Err(EnforcementBoundaryRejection::PolicyDecisionIdMismatch);
    }
    if intent.requested_action != decision.action {
        return Err(EnforcementBoundaryRejection::PolicyActionMismatch);
    }
    if intent.evidence_references.is_empty() || decision.evidence_references.is_empty() {
        return Err(EnforcementBoundaryRejection::MissingPolicyEvidenceReference);
    }
    if !intent
        .evidence_references
        .iter()
        .all(|intent_ref| evidence_ref_is_in_decision(intent_ref, &decision.evidence_references))
    {
        return Err(EnforcementBoundaryRejection::MissingPolicyEvidenceReference);
    }

    Ok(())
}

fn enforcement_mode(
    intent: &EnforcementIntent,
) -> Result<EnforcementMode, EnforcementBoundaryRejection> {
    match intent.requested_action {
        PolicyAction::Allow | PolicyAction::Warn | PolicyAction::Unknown => {
            Ok(EnforcementMode::ObserveOnly)
        }
        PolicyAction::AskParent => Ok(EnforcementMode::AskParent),
        PolicyAction::TimeLimit => match intent.target.target_type {
            PolicyTargetType::App | PolicyTargetType::Process | PolicyTargetType::Device => {
                Ok(EnforcementMode::TimeLimit)
            }
            _ => Err(EnforcementBoundaryRejection::UnsupportedEnforcementCapability),
        },
        PolicyAction::Block => match intent.target.target_type {
            PolicyTargetType::Process => Ok(EnforcementMode::TerminateProcess),
            PolicyTargetType::App => Ok(EnforcementMode::BlockProcess),
            _ => Err(EnforcementBoundaryRejection::PolicyTargetMismatch),
        },
    }
}

fn enforcement_action(
    input: &EnforcementBoundaryInput,
    mode: EnforcementMode,
) -> EnforcementAction {
    EnforcementAction {
        schema_version: input.decision.schema_version.clone(),
        action_id: input.action_id.clone(),
        intent_id: input.intent.intent_id.clone(),
        policy_decision_id: input.decision.decision_id.clone(),
        policy_action: input.decision.action,
        adapter_kind: adapter_kind(mode),
        platform: capability_platform(&input.capability, &input.intent.device.platform),
        target: input.intent.target.clone(),
        mode,
        capability: input.capability.clone(),
        reason_codes: input.decision.reason_codes.clone(),
        evidence_references: input.intent.evidence_references.clone(),
        local_ai_result_id: input.decision.local_ai_result_id.clone(),
        parent_approval: input.intent.parent_approval.clone(),
        dry_run: input.decision.dry_run
            || input.capability.capability_state == EnforcementCapabilityState::DryRun,
        requested_at: input.requested_at.clone(),
        expires_at: input.decision.expires_at.clone(),
        rollback_token: input.rollback_token.clone(),
    }
}

fn enforcement_result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
) -> Result<EnforcementResult, EnforcementBoundaryRejection> {
    if action.dry_run {
        return Ok(dry_run_result(input, action));
    }
    if let Some(result) = capability_state_result(input, action) {
        return Ok(result);
    }
    if let Some(result) = no_adapter_action_result(input, action) {
        return Ok(result);
    }
    if !input.capability.supported_actions.contains(&action.mode) {
        return Ok(result(
            input,
            action,
            EnforcementResultParts {
                status: EnforcementResultStatus::Unavailable,
                adapter_result_code: EnforcementAdapterResultCode::AdapterUnavailable,
                completed_at: input.completed_at.clone(),
                unavailable_reason: Some(
                    enforcement_constants::UNAVAILABLE_UNSUPPORTED_ACTION.to_string(),
                ),
                failed_reason: None,
                rollback_token: action.rollback_token.clone(),
                rollback_state: EnforcementRollbackState::Unavailable,
                unavailable_status_reason: Some(EnforcementUnavailableReason::UnsupportedAction),
            },
        ));
    }

    let Some(adapter_outcome) = input.adapter_outcome.clone() else {
        return Err(EnforcementBoundaryRejection::AdapterResultRequired);
    };

    Ok(adapter_completed_result(input, action, adapter_outcome))
}

fn dry_run_result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
) -> EnforcementResult {
    result(
        input,
        action,
        EnforcementResultParts {
            status: EnforcementResultStatus::WouldEnforce,
            adapter_result_code: EnforcementAdapterResultCode::DryRunNoAction,
            completed_at: input.completed_at.clone(),
            unavailable_reason: None,
            failed_reason: None,
            rollback_token: action.rollback_token.clone(),
            rollback_state: EnforcementRollbackState::NotRequired,
            unavailable_status_reason: None,
        },
    )
}

fn capability_state_result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
) -> Option<EnforcementResult> {
    match input.capability.capability_state {
        EnforcementCapabilityState::Unavailable => {
            let unavailable_reason = capability_unavailable_reason(&input.capability);
            let adapter_result_code = match unavailable_reason {
                EnforcementUnavailableReason::UnsupportedPlatform => {
                    EnforcementAdapterResultCode::UnsupportedPlatform
                }
                EnforcementUnavailableReason::AdapterError => {
                    EnforcementAdapterResultCode::AdapterFailed
                }
                EnforcementUnavailableReason::UnsupportedAction
                | EnforcementUnavailableReason::MissingPermission
                | EnforcementUnavailableReason::MissingDependency
                | EnforcementUnavailableReason::AdapterUnavailable => {
                    EnforcementAdapterResultCode::AdapterUnavailable
                }
            };
            Some(result(
                input,
                action,
                EnforcementResultParts {
                    status: EnforcementResultStatus::Unavailable,
                    adapter_result_code,
                    completed_at: input.completed_at.clone(),
                    unavailable_reason: Some(unavailable_reason.as_protocol_str().to_string()),
                    failed_reason: None,
                    rollback_token: action.rollback_token.clone(),
                    rollback_state: EnforcementRollbackState::Unavailable,
                    unavailable_status_reason: Some(unavailable_reason),
                },
            ))
        }
        EnforcementCapabilityState::ObserveOnly => Some(result(
            input,
            action,
            EnforcementResultParts {
                status: EnforcementResultStatus::NoOp,
                adapter_result_code: EnforcementAdapterResultCode::LeftRunningObserveOnly,
                completed_at: input.completed_at.clone(),
                unavailable_reason: None,
                failed_reason: None,
                rollback_token: action.rollback_token.clone(),
                rollback_state: EnforcementRollbackState::NotRequired,
                unavailable_status_reason: None,
            },
        )),
        EnforcementCapabilityState::Supported
        | EnforcementCapabilityState::Degraded
        | EnforcementCapabilityState::DryRun => None,
    }
}

fn no_adapter_action_result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
) -> Option<EnforcementResult> {
    matches!(
        action.mode,
        EnforcementMode::ObserveOnly | EnforcementMode::AskParent | EnforcementMode::TimeLimit
    )
    .then(|| {
        result(
            input,
            action,
            EnforcementResultParts {
                status: EnforcementResultStatus::NoOp,
                adapter_result_code: EnforcementAdapterResultCode::NoOp,
                completed_at: input.completed_at.clone(),
                unavailable_reason: None,
                failed_reason: None,
                rollback_token: action.rollback_token.clone(),
                rollback_state: EnforcementRollbackState::NotRequired,
                unavailable_status_reason: None,
            },
        )
    })
}

fn adapter_completed_result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
    adapter_outcome: EnforcementAdapterOutcome,
) -> EnforcementResult {
    let unavailable_status_reason = adapter_unavailable_reason(&adapter_outcome);
    result(
        input,
        action,
        EnforcementResultParts {
            status: adapter_outcome.status,
            adapter_result_code: adapter_outcome.adapter_result_code,
            completed_at: adapter_outcome.completed_at,
            unavailable_reason: adapter_outcome.unavailable_reason,
            failed_reason: adapter_outcome.failed_reason,
            rollback_token: adapter_outcome
                .rollback_token
                .or_else(|| action.rollback_token.clone()),
            rollback_state: adapter_outcome.rollback_state,
            unavailable_status_reason,
        },
    )
}

fn result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
    parts: EnforcementResultParts,
) -> EnforcementResult {
    EnforcementResult {
        schema_version: input.decision.schema_version.clone(),
        result_id: input.result_id.clone(),
        action_id: action.action_id.clone(),
        status: parts.status,
        adapter_result_code: parts.adapter_result_code,
        started_at: input.requested_at.clone(),
        completed_at: parts.completed_at,
        rollback_token: parts.rollback_token,
        rollback_state: parts.rollback_state,
        unavailable_reason: parts.unavailable_reason,
        unavailable_status: parts.unavailable_status_reason.map(|reason| {
            build_unavailable_status(&input.decision.schema_version, &input.capability, reason)
        }),
        failed_reason: parts.failed_reason,
        next_check_at: action.expires_at.clone(),
        capability: input.capability.clone(),
    }
}

fn enforcement_audit_event(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
    result: &EnforcementResult,
) -> EnforcementAuditEvent {
    EnforcementAuditEvent {
        schema_version: input.decision.schema_version.clone(),
        audit_event_id: input.audit_event_id.clone(),
        audit_event_kind: audit_kind(result.status),
        action: action.clone(),
        result: result.clone(),
        capability: result.capability.clone(),
        unavailable_status: result.unavailable_status.clone(),
        policy_version: input.policy_version.clone(),
        evidence_references: input.intent.evidence_references.clone(),
        actor: input.intent.actor.clone(),
        parent_override: input.intent.parent_approval.clone(),
        journal_sequence: None,
        observed_at: input
            .completed_at
            .clone()
            .unwrap_or_else(|| input.requested_at.clone()),
    }
}

fn adapter_request(
    action: &EnforcementAction,
    result: &EnforcementResult,
) -> Option<EnforcementAdapterRequest> {
    if action.dry_run
        || result.status != EnforcementResultStatus::WouldEnforce
        || matches!(
            action.mode,
            EnforcementMode::ObserveOnly | EnforcementMode::AskParent | EnforcementMode::TimeLimit
        )
    {
        return None;
    }

    Some(EnforcementAdapterRequest {
        action_id: action.action_id.clone(),
        adapter_kind: action.adapter_kind,
        mode: action.mode,
    })
}

fn evidence_ref_is_in_decision(
    intent_ref: &ParentEvidenceReference,
    decision_refs: &[ParentEvidenceReference],
) -> bool {
    decision_refs
        .iter()
        .any(|decision_ref| decision_ref.evidence_reference_id == intent_ref.evidence_reference_id)
}

fn adapter_kind(mode: EnforcementMode) -> EnforcementAdapterKind {
    match mode {
        EnforcementMode::TerminateProcess
        | EnforcementMode::BlockProcess
        | EnforcementMode::TemporaryBlock
        | EnforcementMode::TimeLimit => EnforcementAdapterKind::ProcessControl,
        EnforcementMode::AskParent | EnforcementMode::ObserveOnly => {
            EnforcementAdapterKind::TimerControl
        }
    }
}

fn audit_kind(status: EnforcementResultStatus) -> EnforcementAuditEventKind {
    match status {
        EnforcementResultStatus::WouldEnforce => EnforcementAuditEventKind::Attempted,
        EnforcementResultStatus::ActuallyEnforced => EnforcementAuditEventKind::Succeeded,
        EnforcementResultStatus::Unavailable => EnforcementAuditEventKind::Unavailable,
        EnforcementResultStatus::Failed => EnforcementAuditEventKind::Failed,
        EnforcementResultStatus::Expired => EnforcementAuditEventKind::Expired,
        EnforcementResultStatus::RolledBack => EnforcementAuditEventKind::RollbackCompleted,
        EnforcementResultStatus::Superseded => EnforcementAuditEventKind::Cancelled,
        EnforcementResultStatus::NoOp => EnforcementAuditEventKind::Attempted,
    }
}

fn capability_platform(
    capability: &EnforcementCapabilityStatus,
    device_platform: &str,
) -> ParentPlatform {
    if capability.capability_state != EnforcementCapabilityState::Unavailable {
        return capability.platform;
    }

    match device_platform {
        enforcement_constants::PLATFORM_WINDOWS => Some(ParentPlatform::Windows),
        enforcement_constants::PLATFORM_LINUX => Some(ParentPlatform::Linux),
        enforcement_constants::PLATFORM_MACOS => Some(ParentPlatform::Macos),
        enforcement_constants::PLATFORM_ANDROID => Some(ParentPlatform::Android),
        enforcement_constants::PLATFORM_IOS => Some(ParentPlatform::Ios),
        _ => None,
    }
    .unwrap_or(capability.platform)
}
