use ocentra_parent_agent_core::enforcement_timer_state::EnforcementTimerTransitionIds;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

#[path = "enforcement_timer_payload/helpers.rs"]
mod helpers;
#[path = "enforcement_timer_payload/parent_action.rs"]
mod parent_action;

use crate::activity_api::GeneratedAtText;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementTimerText(pub(crate) String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementTimerTextRef<'a>(&'a str);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EnforcementTimerPayloadError {
    ParentActionRequired,
    ProcessIdRequired,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementTimerCommandPayload {
    pub command_correlation_id: EnforcementTimerText,
    pub command_sent_at: EnforcementTimerText,
    pub transition_ids: EnforcementTimerTransitionIds,
    pub expected_action_id: Option<EnforcementTimerText>,
    pub parent_override:
        Option<ocentra_parent_agent_protocol::schema_domain_mirrors::family::ParentActionReference>,
    pub process_id: Option<u32>,
    pub device_id: EnforcementTimerText,
    pub platform: EnforcementTimerText,
    pub source_peer_id: EnforcementTimerText,
    pub target_route: EnforcementTimerText,
}

pub(crate) fn parse_timer_recovery_payload(
    command: &AgentCommandEnvelope,
    observed_at: &GeneratedAtText,
) -> EnforcementTimerCommandPayload {
    EnforcementTimerCommandPayload {
        command_correlation_id: EnforcementTimerText(command.message_id.clone()),
        command_sent_at: EnforcementTimerText(command.sent_at.clone()),
        transition_ids: parse_transition_ids(
            &command.payload,
            EnforcementTimerTextRef(&command.message_id),
            observed_at,
        ),
        expected_action_id: helpers::optional_string(
            &command.payload,
            EnforcementTimerTextRef(constants::field::ENFORCEMENT_ACTION_ID),
        ),
        parent_override: None,
        process_id: None,
        device_id: EnforcementTimerText(command.target.device_id.clone()),
        platform: EnforcementTimerText(command.target.platform.clone()),
        source_peer_id: EnforcementTimerText(command.source.peer_id.clone()),
        target_route: target_route_text(&command.target.route),
    }
}

pub(crate) fn parse_timer_expiry_payload(
    command: &AgentCommandEnvelope,
    observed_at: &GeneratedAtText,
) -> Result<EnforcementTimerCommandPayload, EnforcementTimerPayloadError> {
    Ok(EnforcementTimerCommandPayload {
        command_correlation_id: EnforcementTimerText(command.message_id.clone()),
        command_sent_at: EnforcementTimerText(command.sent_at.clone()),
        transition_ids: parse_transition_ids(
            &command.payload,
            EnforcementTimerTextRef(&command.message_id),
            observed_at,
        ),
        expected_action_id: helpers::optional_string(
            &command.payload,
            EnforcementTimerTextRef(constants::field::ENFORCEMENT_ACTION_ID),
        ),
        parent_override: None,
        process_id: Some(helpers::required_process_id(&command.payload)?),
        device_id: EnforcementTimerText(command.target.device_id.clone()),
        platform: EnforcementTimerText(command.target.platform.clone()),
        source_peer_id: EnforcementTimerText(command.source.peer_id.clone()),
        target_route: target_route_text(&command.target.route),
    })
}

pub(crate) fn parse_parent_override_payload(
    command: &AgentCommandEnvelope,
    observed_at: &GeneratedAtText,
) -> Result<EnforcementTimerCommandPayload, EnforcementTimerPayloadError> {
    Ok(EnforcementTimerCommandPayload {
        command_correlation_id: EnforcementTimerText(command.message_id.clone()),
        command_sent_at: EnforcementTimerText(command.sent_at.clone()),
        transition_ids: parse_transition_ids(
            &command.payload,
            EnforcementTimerTextRef(&command.message_id),
            observed_at,
        ),
        expected_action_id: helpers::optional_string(
            &command.payload,
            EnforcementTimerTextRef(constants::field::ENFORCEMENT_ACTION_ID),
        ),
        parent_override: Some(parent_action::parent_action_reference(
            &command.payload,
            observed_at,
        )?),
        process_id: None,
        device_id: EnforcementTimerText(command.target.device_id.clone()),
        platform: EnforcementTimerText(command.target.platform.clone()),
        source_peer_id: EnforcementTimerText(command.source.peer_id.clone()),
        target_route: target_route_text(&command.target.route),
    })
}

fn target_route_text(
    route: &ocentra_parent_agent_protocol::transport::AgentRoute,
) -> EnforcementTimerText {
    let value = match route {
        ocentra_parent_agent_protocol::transport::AgentRoute::Localhost => {
            constants::value::DEVICE_RUNTIME_ROUTE_LOCALHOST
        }
        ocentra_parent_agent_protocol::transport::AgentRoute::LocalNetwork => {
            constants::value::DEVICE_RUNTIME_ROUTE_LOCAL_NETWORK
        }
        ocentra_parent_agent_protocol::transport::AgentRoute::CloudRelay => {
            constants::value::DEVICE_RUNTIME_ROUTE_CLOUD_RELAY
        }
    };
    EnforcementTimerText(value.to_string())
}

fn parse_transition_ids(
    payload: &LogFields,
    message_id: EnforcementTimerTextRef<'_>,
    observed_at: &GeneratedAtText,
) -> EnforcementTimerTransitionIds {
    EnforcementTimerTransitionIds {
        result_id: helpers::string_or_prefixed(
            payload,
            EnforcementTimerTextRef(constants::field::ENFORCEMENT_RESULT_ID),
            EnforcementTimerTextRef(constants::enforcement::RESULT_ID_PREFIX),
            message_id,
        )
        .0,
        audit_event_id: helpers::string_or_prefixed(
            payload,
            EnforcementTimerTextRef(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
            EnforcementTimerTextRef(constants::enforcement::AUDIT_EVENT_ID_PREFIX),
            message_id,
        )
        .0,
        timer_event_id: helpers::string_or_prefixed(
            payload,
            EnforcementTimerTextRef(constants::field::ENFORCEMENT_TIMER_EVENT_ID),
            EnforcementTimerTextRef(constants::enforcement::TIMER_EVENT_ID_PREFIX),
            message_id,
        )
        .0,
        observed_at: helpers::optional_string(
            payload,
            EnforcementTimerTextRef(constants::field::REQUESTED_AT),
        )
        .map(|value| value.0)
        .unwrap_or_else(|| observed_at.0.clone()),
    }
}
