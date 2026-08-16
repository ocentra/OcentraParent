use ocentra_parent_agent_protocol::activity::policy::ParentActorReference;
use ocentra_parent_agent_protocol::activity::policy::ParentActorRole;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::schema_domain_mirrors::family::ParentActionReference;

use crate::activity_api::GeneratedAtText;

use super::{helpers, EnforcementTimerPayloadError, EnforcementTimerTextRef};

pub(crate) fn parent_action_reference(
    payload: &LogFields,
    observed_at: &GeneratedAtText,
) -> Result<ParentActionReference, EnforcementTimerPayloadError> {
    Ok(ParentActionReference {
        action_reference_id: required_string(
            payload,
            EnforcementTimerTextRef(constants::field::PARENT_ACTION_REFERENCE_ID),
        )?
        .0
        .to_string(),
        actor: parent_actor(payload)?,
        policy_version: required_string(
            payload,
            EnforcementTimerTextRef(constants::field::POLICY_VERSION),
        )?
        .0
        .to_string(),
        created_at: helpers::optional_string(
            payload,
            EnforcementTimerTextRef(constants::field::PARENT_ACTION_CREATED_AT),
        )
        .map(|value| value.0)
        .unwrap_or_else(|| observed_at.0.clone()),
    })
}

fn required_string<'a>(
    payload: &'a LogFields,
    field: EnforcementTimerTextRef<'_>,
) -> Result<EnforcementTimerTextRef<'a>, EnforcementTimerPayloadError> {
    match payload.get(field.0) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Ok(EnforcementTimerTextRef(value.trim()))
        }
        _ => Err(EnforcementTimerPayloadError::ParentActionRequired),
    }
}

fn parent_actor(payload: &LogFields) -> Result<ParentActorReference, EnforcementTimerPayloadError> {
    Ok(ParentActorReference {
        actor_id: required_string(
            payload,
            EnforcementTimerTextRef(constants::field::PARENT_ACTOR_ID),
        )?
        .0
        .to_string(),
        role: parent_actor_role(required_string(
            payload,
            EnforcementTimerTextRef(constants::field::PARENT_ACTOR_ROLE),
        )?)?,
    })
}

fn parent_actor_role(
    value: EnforcementTimerTextRef<'_>,
) -> Result<ParentActorRole, EnforcementTimerPayloadError> {
    match value.0 {
        policy_constants::ACTOR_ROLE_PARENT => Ok(ParentActorRole::Parent),
        policy_constants::ACTOR_ROLE_GUARDIAN => Ok(ParentActorRole::Guardian),
        policy_constants::ACTOR_ROLE_SYSTEM => Ok(ParentActorRole::System),
        _ => Err(EnforcementTimerPayloadError::ParentActionRequired),
    }
}
