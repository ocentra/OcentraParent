use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryInput;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecision;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecisionHandoffState;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy_context::ParentDeviceReference;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::EnforcementIntent;
use ocentra_parent_agent_protocol::enforcement::EnforcementIntentSource;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use super::field_access::evidence_references;
use super::field_access::optional_string;
use super::field_access::required_boolean;
use super::field_access::required_string;
use super::field_access::required_string_list;
use super::policy_action::policy_action;
use super::policy_target_type::policy_target_type;
use super::process_id::optional_process_id;
use super::trusted_delivery::{consume, TrustedDeliveryBinding, TrustedDeliveryDirectory};
use super::trusted_delivery_error::TrustedDeliveryError;
use super::EnforcementCommandPayload;
use super::EnforcementDeviceRefText;
use super::EnforcementFieldKey;
use super::EnforcementPayloadError;
use super::EnforcementPayloadIds;
use super::EnforcementPolicyPayload;
use super::EnforcementText;

fn parse_candidate_enforcement_command_payload(
    command: &AgentCommandEnvelope,
    observed_at: &EnforcementText,
) -> Result<EnforcementCommandPayload, EnforcementPayloadError> {
    let policy = parse_policy_payload(&command.payload, observed_at)?;
    let ids = parse_payload_ids(
        &command.payload,
        &EnforcementText(command.message_id.clone()),
    );
    let process_id = optional_process_id(&command.payload)?;
    let target = policy_target(&policy);
    let decision = policy_decision(&policy);
    let capability = crate::enforcement_capability::enforcement_capability_for_policy(
        policy.action,
        policy.target_type,
        &crate::enforcement_capability::EnforcementRequestedAtText(&policy.requested_at),
    );
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
        device_id: EnforcementDeviceRefText(command.target.device_id.clone()),
        platform: command.target.platform.clone(),
    })
}

pub(crate) fn parse_trusted_enforcement_command_payload(
    command: &AgentCommandEnvelope,
    observed_at: &EnforcementText,
    directory: &TrustedDeliveryDirectory,
) -> Result<EnforcementCommandPayload, TrustedDeliveryError> {
    let candidate = parse_candidate_enforcement_command_payload(command, observed_at)
        .map_err(TrustedDeliveryError::from)?;
    let binding = trusted_delivery_binding(command, &candidate);
    consume(directory, &binding)?;
    Ok(candidate)
}

fn trusted_delivery_binding(
    command: &AgentCommandEnvelope,
    candidate: &EnforcementCommandPayload,
) -> TrustedDeliveryBinding {
    TrustedDeliveryBinding::new(
        EnforcementText(command.message_id.clone()),
        candidate.device_id.clone(),
        candidate
            .input
            .decision
            .evidence_references
            .iter()
            .map(|reference| EnforcementText(reference.evidence_reference_id.clone()))
            .collect(),
        candidate.process_id,
        EnforcementText(candidate.input.intent.target.target_value.clone()),
        EnforcementText(candidate.input.decision.decision_id.clone()),
        EnforcementText(candidate.input.intent.intent_id.clone()),
    )
}

fn parse_policy_payload(
    payload: &LogFields,
    observed_at: &EnforcementText,
) -> Result<EnforcementPolicyPayload, EnforcementPayloadError> {
    let policy_decision_id = required_string(
        payload,
        EnforcementFieldKey(constants::field::POLICY_DECISION_ID),
    )?
    .0;
    let policy_version = match required_string(
        payload,
        EnforcementFieldKey(constants::field::POLICY_VERSION),
    ) {
        Ok(policy_version) => policy_version.0,
        Err(_) => return Err(EnforcementPayloadError::PolicyVersionRequired),
    };
    let target_id = required_string(payload, EnforcementFieldKey(constants::field::TARGET_ID))?.0;
    let target_value = required_string(
        payload,
        EnforcementFieldKey(constants::field::POLICY_TARGET_VALUE),
    )?
    .0;
    let target_type = policy_target_type(&required_string(
        payload,
        EnforcementFieldKey(constants::field::POLICY_TARGET_TYPE),
    )?)?;
    let action = policy_action(&required_string(
        payload,
        EnforcementFieldKey(constants::field::POLICY_ACTION),
    )?)?;
    let dry_run = required_boolean(
        payload,
        EnforcementFieldKey(constants::field::POLICY_DRY_RUN),
    )?;
    let reason_codes = required_string_list(
        payload,
        EnforcementFieldKey(constants::field::POLICY_REASON_CODES),
        EnforcementPayloadError::ReasonCodeRequired,
    )?;
    let rule_ids = required_string_list(
        payload,
        EnforcementFieldKey(constants::field::POLICY_RULE_IDS),
        EnforcementPayloadError::RuleIdRequired,
    )?;
    let evidence_references = evidence_references(
        payload,
        observed_at,
        EnforcementFieldKey(constants::field::EVIDENCE_REFERENCE_IDS),
    )?;
    let requested_at =
        optional_string(payload, EnforcementFieldKey(constants::field::REQUESTED_AT))
            .map(|value| value.0)
            .unwrap_or_else(|| observed_at.0.clone());

    Ok(EnforcementPolicyPayload {
        policy_decision_id,
        policy_version,
        target_id,
        target_value,
        target_type,
        action,
        dry_run,
        reason_codes: reason_codes.into_iter().map(|value| value.0).collect(),
        rule_ids: rule_ids.into_iter().map(|value| value.0).collect(),
        evidence_references,
        expires_at: optional_string(payload, EnforcementFieldKey(constants::field::EXPIRES_AT))
            .map(|value| value.0),
        local_ai_result_id: optional_string(
            payload,
            EnforcementFieldKey(constants::field::LOCAL_AI_RESULT_ID),
        )
        .map(|value| value.0),
        requested_at,
    })
}

fn parse_payload_ids(payload: &LogFields, message_id: &EnforcementText) -> EnforcementPayloadIds {
    let rollback_token = optional_string(
        payload,
        EnforcementFieldKey(constants::field::ROLLBACK_TOKEN),
    )
    .map(|value| value.0)
    .or_else(|| {
        Some(
            prefixed_id(
                &EnforcementText(constants::enforcement::ROLLBACK_TOKEN_PREFIX.to_string()),
                message_id,
            )
            .0,
        )
    });
    let action_id = string_or_prefixed(
        payload,
        EnforcementFieldKey(constants::field::ENFORCEMENT_ACTION_ID),
        &EnforcementText(constants::enforcement::ACTION_ID_PREFIX.to_string()),
        message_id,
    );
    let result_id = string_or_prefixed(
        payload,
        EnforcementFieldKey(constants::field::ENFORCEMENT_RESULT_ID),
        &EnforcementText(constants::enforcement::RESULT_ID_PREFIX.to_string()),
        message_id,
    );
    let audit_event_id = string_or_prefixed(
        payload,
        EnforcementFieldKey(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
        &EnforcementText(constants::enforcement::AUDIT_EVENT_ID_PREFIX.to_string()),
        message_id,
    );
    let timer_event_id = string_or_prefixed(
        payload,
        EnforcementFieldKey(constants::field::ENFORCEMENT_TIMER_EVENT_ID),
        &EnforcementText(constants::enforcement::TIMER_EVENT_ID_PREFIX.to_string()),
        message_id,
    );
    let intent_id = string_or_prefixed(
        payload,
        EnforcementFieldKey(constants::field::ENFORCEMENT_INTENT_ID),
        &EnforcementText(constants::enforcement::INTENT_ID_PREFIX.to_string()),
        message_id,
    );

    EnforcementPayloadIds {
        action_id: action_id.0,
        result_id: result_id.0,
        audit_event_id: audit_event_id.0,
        timer_event_id: timer_event_id.0,
        intent_id: intent_id.0,
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
            child_profile_id: optional_string(
                &command.payload,
                EnforcementFieldKey(constants::field::PROFILE_ID),
            )
            .map(|value| value.0),
            label: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
        },
        policy_decision_id: policy.policy_decision_id.clone(),
        target: target.clone(),
        requested_action: policy.action,
        evidence_references: policy.evidence_references.clone(),
        parent_approval: None,
        idempotency_key: idempotency_key(
            &EnforcementText(policy.policy_decision_id.clone()),
            &EnforcementText(policy.target_id.clone()),
        )
        .0,
    }
}

fn string_or_prefixed(
    payload: &LogFields,
    field: EnforcementFieldKey,
    prefix: &EnforcementText,
    suffix: &EnforcementText,
) -> EnforcementText {
    optional_string(payload, field).unwrap_or_else(|| prefixed_id(prefix, suffix))
}

fn prefixed_id(prefix: &EnforcementText, suffix: &EnforcementText) -> EnforcementText {
    let mut value = prefix.0.clone();
    value.push_str(&suffix.0);
    EnforcementText(value)
}

fn idempotency_key(
    policy_decision_id: &EnforcementText,
    target_id: &EnforcementText,
) -> EnforcementText {
    let mut value = policy_decision_id.0.clone();
    value.push(constants::delimiter::COLON);
    value.push_str(&target_id.0);
    EnforcementText(value)
}
