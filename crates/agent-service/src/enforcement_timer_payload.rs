use ocentra_parent_agent_core::EnforcementTimerTransitionIds;
use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, LogFieldValue, LogFields,
    ParentActionReference, ParentActorReference, ParentActorRole,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementTimerCommandPayload {
    pub transition_ids: EnforcementTimerTransitionIds,
    pub expected_action_id: Option<String>,
    pub parent_override: Option<ParentActionReference>,
    pub process_id: Option<u32>,
    pub device_id: String,
    pub platform: String,
}

pub(crate) fn parse_timer_recovery_payload(
    command: &AgentCommandEnvelope,
    observed_at: &str,
) -> EnforcementTimerCommandPayload {
    EnforcementTimerCommandPayload {
        transition_ids: parse_transition_ids(&command.payload, &command.message_id, observed_at),
        expected_action_id: optional_string(
            &command.payload,
            constants::field::ENFORCEMENT_ACTION_ID,
        ),
        parent_override: None,
        process_id: None,
        device_id: command.target.device_id.clone(),
        platform: command.target.platform.clone(),
    }
}

pub(crate) fn parse_timer_expiry_payload(
    command: &AgentCommandEnvelope,
    observed_at: &str,
) -> Result<EnforcementTimerCommandPayload, &'static str> {
    Ok(EnforcementTimerCommandPayload {
        transition_ids: parse_transition_ids(&command.payload, &command.message_id, observed_at),
        expected_action_id: optional_string(
            &command.payload,
            constants::field::ENFORCEMENT_ACTION_ID,
        ),
        parent_override: None,
        process_id: Some(required_process_id(&command.payload)?),
        device_id: command.target.device_id.clone(),
        platform: command.target.platform.clone(),
    })
}

pub(crate) fn parse_parent_override_payload(
    command: &AgentCommandEnvelope,
    observed_at: &str,
) -> Result<EnforcementTimerCommandPayload, &'static str> {
    Ok(EnforcementTimerCommandPayload {
        transition_ids: parse_transition_ids(&command.payload, &command.message_id, observed_at),
        expected_action_id: optional_string(
            &command.payload,
            constants::field::ENFORCEMENT_ACTION_ID,
        ),
        parent_override: Some(parent_action_reference(&command.payload, observed_at)?),
        process_id: None,
        device_id: command.target.device_id.clone(),
        platform: command.target.platform.clone(),
    })
}

fn parse_transition_ids(
    payload: &LogFields,
    message_id: &str,
    observed_at: &str,
) -> EnforcementTimerTransitionIds {
    EnforcementTimerTransitionIds {
        result_id: string_or_prefixed(
            payload,
            constants::field::ENFORCEMENT_RESULT_ID,
            constants::enforcement::RESULT_ID_PREFIX,
            message_id,
        ),
        audit_event_id: string_or_prefixed(
            payload,
            constants::field::ENFORCEMENT_AUDIT_EVENT_ID,
            constants::enforcement::AUDIT_EVENT_ID_PREFIX,
            message_id,
        ),
        timer_event_id: string_or_prefixed(
            payload,
            constants::field::ENFORCEMENT_TIMER_EVENT_ID,
            constants::enforcement::TIMER_EVENT_ID_PREFIX,
            message_id,
        ),
        observed_at: optional_string(payload, constants::field::REQUESTED_AT)
            .unwrap_or_else(|| observed_at.to_string()),
    }
}

fn parent_action_reference(
    payload: &LogFields,
    observed_at: &str,
) -> Result<ParentActionReference, &'static str> {
    Ok(ParentActionReference {
        action_reference_id: required_string(
            payload,
            constants::field::PARENT_ACTION_REFERENCE_ID,
        )?
        .to_string(),
        actor: parent_actor(payload)?,
        policy_version: required_string(payload, constants::field::POLICY_VERSION)?.to_string(),
        created_at: optional_string(payload, constants::field::PARENT_ACTION_CREATED_AT)
            .unwrap_or_else(|| observed_at.to_string()),
    })
}

fn parent_actor(payload: &LogFields) -> Result<ParentActorReference, &'static str> {
    Ok(ParentActorReference {
        actor_id: required_string(payload, constants::field::PARENT_ACTOR_ID)?.to_string(),
        role: parent_actor_role(required_string(
            payload,
            constants::field::PARENT_ACTOR_ROLE,
        )?)?,
    })
}

fn parent_actor_role(value: &str) -> Result<ParentActorRole, &'static str> {
    match value {
        policy_constants::ACTOR_ROLE_PARENT => Ok(ParentActorRole::Parent),
        policy_constants::ACTOR_ROLE_GUARDIAN => Ok(ParentActorRole::Guardian),
        policy_constants::ACTOR_ROLE_SYSTEM => Ok(ParentActorRole::System),
        _ => Err(constants::enforcement::REJECTION_PARENT_ACTION_REQUIRED),
    }
}

fn required_string<'a>(payload: &'a LogFields, field: &str) -> Result<&'a str, &'static str> {
    match payload.get(field) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => Ok(value.trim()),
        _ => Err(constants::enforcement::REJECTION_PARENT_ACTION_REQUIRED),
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

fn required_process_id(payload: &LogFields) -> Result<u32, &'static str> {
    match payload.get(constants::field::PROCESS_ID) {
        Some(LogFieldValue::Number(value))
            if value.is_finite()
                && *value > 0.0
                && value.fract() == 0.0
                && *value <= f64::from(u32::MAX) =>
        {
            Ok(*value as u32)
        }
        _ => Err(constants::enforcement::REJECTION_PROCESS_ID_REQUIRED),
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
