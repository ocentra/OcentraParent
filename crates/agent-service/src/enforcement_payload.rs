use ocentra_parent_agent_core::EnforcementBoundaryInput;
use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, EnforcementIntent, EnforcementIntentSource,
    LogFieldValue, LogFields, ParentDeviceReference, ParentEvidenceReference,
    ParentEvidenceReferenceKind, PolicyAction, PolicyDecision, PolicyDecisionHandoffState,
    PolicyTarget, PolicyTargetType,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EnforcementCommandPayload {
    pub input: EnforcementBoundaryInput,
    pub process_id: Option<u32>,
    pub device_id: String,
    pub platform: String,
}

#[derive(Clone, Debug, PartialEq)]
struct EnforcementPolicyPayload {
    policy_decision_id: String,
    policy_version: String,
    target_id: String,
    target_value: String,
    target_type: PolicyTargetType,
    action: PolicyAction,
    dry_run: bool,
    reason_codes: Vec<String>,
    rule_ids: Vec<String>,
    evidence_references: Vec<ParentEvidenceReference>,
    expires_at: Option<String>,
    local_ai_result_id: Option<String>,
    requested_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EnforcementPayloadIds {
    action_id: String,
    result_id: String,
    audit_event_id: String,
    timer_event_id: String,
    intent_id: String,
    rollback_token: Option<String>,
}

pub(crate) fn parse_enforcement_command_payload(
    command: &AgentCommandEnvelope,
    observed_at: &str,
) -> Result<EnforcementCommandPayload, &'static str> {
    let policy = parse_policy_payload(&command.payload, observed_at)?;
    let ids = parse_payload_ids(&command.payload, &command.message_id);
    let process_id = optional_process_id(&command.payload)?;
    let target = policy_target(&policy);
    let decision = policy_decision(&policy);
    let capability = ocentra_parent_agent_core::process_control_capability(&policy.requested_at);
    let intent = enforcement_intent(command, &policy, &ids, &target);
    let input = EnforcementBoundaryInput {
        intent,
        decision,
        capability,
        action_id: ids.action_id,
        result_id: ids.result_id,
        audit_event_id: ids.audit_event_id,
        timer_event_id: ids.timer_event_id,
        rollback_token: ids.rollback_token,
        policy_version: policy.policy_version,
        requested_at: policy.requested_at,
        completed_at: None,
        adapter_outcome: None,
        timer_event_kind: None,
    };

    Ok(EnforcementCommandPayload {
        input,
        process_id,
        device_id: command.target.device_id.clone(),
        platform: command.target.platform.clone(),
    })
}

fn parse_policy_payload(
    payload: &LogFields,
    observed_at: &str,
) -> Result<EnforcementPolicyPayload, &'static str> {
    let policy_decision_id =
        required_string(payload, constants::field::POLICY_DECISION_ID)?.to_string();
    let policy_version = required_string(payload, constants::field::POLICY_VERSION)
        .map_err(|_| constants::enforcement::REJECTION_POLICY_VERSION_REQUIRED)?
        .to_string();
    let target_id = required_string(payload, constants::field::TARGET_ID)?.to_string();
    let target_value = required_string(payload, constants::field::POLICY_TARGET_VALUE)?.to_string();
    let target_type = policy_target_type(required_string(
        payload,
        constants::field::POLICY_TARGET_TYPE,
    )?)?;
    let action = policy_action(required_string(payload, constants::field::POLICY_ACTION)?)?;
    let dry_run = required_boolean(payload, constants::field::POLICY_DRY_RUN)?;
    let reason_codes = required_string_list(
        payload,
        constants::field::POLICY_REASON_CODES,
        constants::enforcement::REJECTION_REASON_CODE_REQUIRED,
    )?;
    let rule_ids = required_string_list(
        payload,
        constants::field::POLICY_RULE_IDS,
        constants::enforcement::REJECTION_RULE_ID_REQUIRED,
    )?;
    let evidence_references = evidence_references(
        payload,
        observed_at,
        constants::field::EVIDENCE_REFERENCE_IDS,
    )?;
    let requested_at = optional_string(payload, constants::field::REQUESTED_AT)
        .unwrap_or_else(|| observed_at.to_string());

    Ok(EnforcementPolicyPayload {
        policy_decision_id,
        policy_version,
        target_id,
        target_value,
        target_type,
        action,
        dry_run,
        reason_codes,
        rule_ids,
        evidence_references,
        expires_at: optional_string(payload, constants::field::EXPIRES_AT),
        local_ai_result_id: optional_string(payload, constants::field::LOCAL_AI_RESULT_ID),
        requested_at,
    })
}

fn parse_payload_ids(payload: &LogFields, message_id: &str) -> EnforcementPayloadIds {
    let rollback_token = optional_string(payload, constants::field::ROLLBACK_TOKEN).or_else(|| {
        Some(prefixed_id(
            constants::enforcement::ROLLBACK_TOKEN_PREFIX,
            message_id,
        ))
    });
    let action_id = string_or_prefixed(
        payload,
        constants::field::ENFORCEMENT_ACTION_ID,
        constants::enforcement::ACTION_ID_PREFIX,
        message_id,
    );
    let result_id = string_or_prefixed(
        payload,
        constants::field::ENFORCEMENT_RESULT_ID,
        constants::enforcement::RESULT_ID_PREFIX,
        message_id,
    );
    let audit_event_id = string_or_prefixed(
        payload,
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID,
        constants::enforcement::AUDIT_EVENT_ID_PREFIX,
        message_id,
    );
    let timer_event_id = string_or_prefixed(
        payload,
        constants::field::ENFORCEMENT_TIMER_EVENT_ID,
        constants::enforcement::TIMER_EVENT_ID_PREFIX,
        message_id,
    );
    let intent_id = string_or_prefixed(
        payload,
        constants::field::ENFORCEMENT_INTENT_ID,
        constants::enforcement::INTENT_ID_PREFIX,
        message_id,
    );

    EnforcementPayloadIds {
        action_id,
        result_id,
        audit_event_id,
        timer_event_id,
        intent_id,
        rollback_token,
    }
}

fn policy_target(policy: &EnforcementPolicyPayload) -> PolicyTarget {
    PolicyTarget {
        target_id: policy.target_id.clone(),
        target_type: policy.target_type,
        target_value: policy.target_value.clone(),
    }
}

fn policy_decision(policy: &EnforcementPolicyPayload) -> PolicyDecision {
    PolicyDecision {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        decision_id: policy.policy_decision_id.clone(),
        action: policy.action,
        reason_codes: policy.reason_codes.clone(),
        evidence_references: policy.evidence_references.clone(),
        rule_ids: policy.rule_ids.clone(),
        local_ai_result_id: policy.local_ai_result_id.clone(),
        dry_run: policy.dry_run,
        enforcement_handoff_state: PolicyDecisionHandoffState::HandedOff,
        expires_at: policy.expires_at.clone(),
    }
}

fn enforcement_intent(
    command: &AgentCommandEnvelope,
    policy: &EnforcementPolicyPayload,
    ids: &EnforcementPayloadIds,
    target: &PolicyTarget,
) -> EnforcementIntent {
    EnforcementIntent {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        intent_id: ids.intent_id.clone(),
        source: EnforcementIntentSource::ParentPortal,
        actor: None,
        device: ParentDeviceReference {
            device_id: command.target.device_id.clone(),
            child_profile_id: optional_string(&command.payload, constants::field::PROFILE_ID),
            label: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
        },
        policy_decision_id: policy.policy_decision_id.clone(),
        target: target.clone(),
        requested_action: policy.action,
        evidence_references: policy.evidence_references.clone(),
        parent_approval: None,
        idempotency_key: idempotency_key(&policy.policy_decision_id, &policy.target_id),
    }
}

fn required_string<'a>(payload: &'a LogFields, field: &str) -> Result<&'a str, &'static str> {
    match payload.get(field) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => Ok(value.trim()),
        _ => Err(constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID),
    }
}

fn optional_string(payload: &LogFields, field: &str) -> Option<String> {
    match payload.get(field) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(value.trim().to_string())
        }
        _ => None,
    }
}

fn required_boolean(payload: &LogFields, field: &str) -> Result<bool, &'static str> {
    match payload.get(field) {
        Some(LogFieldValue::Boolean(value)) => Ok(*value),
        _ => Err(constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID),
    }
}

fn required_string_list(
    payload: &LogFields,
    field: &str,
    error: &'static str,
) -> Result<Vec<String>, &'static str> {
    let values = split_list(required_string(payload, field)?);
    if values.is_empty() {
        return Err(error);
    }
    Ok(values)
}

fn evidence_references(
    payload: &LogFields,
    observed_at: &str,
    field: &str,
) -> Result<Vec<ParentEvidenceReference>, &'static str> {
    let references = required_string_list(
        payload,
        field,
        constants::enforcement::REJECTION_MISSING_EVIDENCE,
    )?
    .into_iter()
    .map(|evidence_reference_id| ParentEvidenceReference {
        evidence_reference_id,
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: observed_at.to_string(),
    })
    .collect::<Vec<_>>();

    Ok(references)
}

fn split_list(value: &str) -> Vec<String> {
    value
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn optional_process_id(payload: &LogFields) -> Result<Option<u32>, &'static str> {
    match payload.get(constants::field::PROCESS_ID) {
        Some(LogFieldValue::Number(value))
            if value.is_finite()
                && *value > 0.0
                && value.fract() == 0.0
                && *value <= f64::from(u32::MAX) =>
        {
            Ok(Some(*value as u32))
        }
        Some(_) => Err(constants::enforcement::REJECTION_PROCESS_ID_REQUIRED),
        None => Ok(None),
    }
}

fn policy_action(value: &str) -> Result<PolicyAction, &'static str> {
    match value {
        policy_constants::ACTION_ALLOW => Ok(PolicyAction::Allow),
        policy_constants::ACTION_WARN => Ok(PolicyAction::Warn),
        policy_constants::ACTION_BLOCK => Ok(PolicyAction::Block),
        policy_constants::ACTION_TIME_LIMIT => Ok(PolicyAction::TimeLimit),
        policy_constants::ACTION_ASK_PARENT => Ok(PolicyAction::AskParent),
        policy_constants::ACTION_UNKNOWN => Ok(PolicyAction::Unknown),
        _ => Err(constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID),
    }
}

fn policy_target_type(value: &str) -> Result<PolicyTargetType, &'static str> {
    match value {
        policy_constants::TARGET_TYPE_APP => Ok(PolicyTargetType::App),
        policy_constants::TARGET_TYPE_PROCESS => Ok(PolicyTargetType::Process),
        policy_constants::TARGET_TYPE_WINDOW => Ok(PolicyTargetType::Window),
        policy_constants::TARGET_TYPE_DOMAIN => Ok(PolicyTargetType::Domain),
        policy_constants::TARGET_TYPE_SITE => Ok(PolicyTargetType::Site),
        policy_constants::TARGET_TYPE_CATEGORY => Ok(PolicyTargetType::Category),
        policy_constants::TARGET_TYPE_VIDEO => Ok(PolicyTargetType::Video),
        policy_constants::TARGET_TYPE_CHANNEL => Ok(PolicyTargetType::Channel),
        policy_constants::TARGET_TYPE_ACTIVITY_TYPE => Ok(PolicyTargetType::ActivityType),
        policy_constants::TARGET_TYPE_DEVICE => Ok(PolicyTargetType::Device),
        _ => Err(constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID),
    }
}

fn string_or_prefixed(payload: &LogFields, field: &str, prefix: &str, suffix: &str) -> String {
    optional_string(payload, field).unwrap_or_else(|| prefixed_id(prefix, suffix))
}

fn prefixed_id(prefix: &str, suffix: &str) -> String {
    let mut value = String::from(prefix);
    value.push_str(suffix);
    value
}

fn idempotency_key(policy_decision_id: &str, target_id: &str) -> String {
    let mut value = String::from(policy_decision_id);
    value.push(constants::delimiter::COLON);
    value.push_str(target_id);
    value
}
