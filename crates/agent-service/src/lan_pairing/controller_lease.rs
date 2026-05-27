use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LanPairingRejectionReason,
    LanParentIntentEnvelope, LogLevel,
};

use crate::{
    event_builder::build_event,
    lan_pairing::{
        authority::{validate_registry_selection_intent, validate_write_authority},
        rejection_event, validate_command_target, LanPairingRuntime,
    },
    lan_pairing_audit::controller_lease_audit_fields,
    lan_pairing_payload::parse_intent,
    lan_pairing_status::pairing_status_event,
    time::timestamp_now,
};

#[derive(Clone, Debug)]
pub(crate) struct LanControllerLeaseState {
    pub(crate) controller_lease_id: String,
    pub(crate) controller_device_id: String,
    pub(crate) parent_actor_id: String,
    pub(crate) expires_at: String,
}

impl LanPairingRuntime {
    pub(crate) fn validate_controller_lease(
        &self,
        intent: &LanParentIntentEnvelope,
        observed_at: &str,
    ) -> Result<(), LanPairingRejectionReason> {
        let lease = LanControllerLeaseState::from_intent(intent);
        if observed_at > lease.expires_at.as_str() {
            return Err(LanPairingRejectionReason::ControllerLeaseExpired);
        }

        let mut active_lease = self
            .controller_lease
            .lock()
            .map_err(|_| LanPairingRejectionReason::Malformed)?;
        if active_lease
            .as_ref()
            .is_some_and(|active| observed_at > active.expires_at.as_str())
        {
            *active_lease = None;
        }

        match active_lease.as_ref() {
            Some(active) if active.matches(&lease) => Ok(()),
            Some(_) => Err(LanPairingRejectionReason::WrongController),
            None => {
                *active_lease = Some(lease);
                Ok(())
            }
        }
    }

    pub(crate) fn renew_controller_lease(
        &self,
        intent: &LanParentIntentEnvelope,
        observed_at: &str,
    ) -> Result<(), LanPairingRejectionReason> {
        let lease = LanControllerLeaseState::from_intent(intent);
        if observed_at > lease.expires_at.as_str() {
            return Err(LanPairingRejectionReason::ControllerLeaseExpired);
        }

        let mut active_lease = self
            .controller_lease
            .lock()
            .map_err(|_| LanPairingRejectionReason::Malformed)?;
        clear_expired_lease(&mut active_lease, observed_at);

        match active_lease.as_ref() {
            Some(active) if active.matches(&lease) => {
                *active_lease = Some(lease);
                Ok(())
            }
            Some(_) => Err(LanPairingRejectionReason::WrongController),
            None => {
                *active_lease = Some(lease);
                Ok(())
            }
        }
    }

    pub(crate) fn release_controller_lease(
        &self,
        intent: &LanParentIntentEnvelope,
        observed_at: &str,
    ) -> Result<(), LanPairingRejectionReason> {
        let lease = LanControllerLeaseState::from_intent(intent);
        let mut active_lease = self
            .controller_lease
            .lock()
            .map_err(|_| LanPairingRejectionReason::Malformed)?;
        clear_expired_lease(&mut active_lease, observed_at);

        match active_lease.as_ref() {
            Some(active) if active.matches(&lease) => {
                *active_lease = None;
                Ok(())
            }
            Some(_) => Err(LanPairingRejectionReason::WrongController),
            None => Err(LanPairingRejectionReason::ControllerLeaseMissing),
        }
    }

    pub(crate) fn takeover_controller_lease(
        &self,
        intent: &LanParentIntentEnvelope,
        observed_at: &str,
    ) -> Result<(), LanPairingRejectionReason> {
        let lease = LanControllerLeaseState::from_intent(intent);
        if observed_at > lease.expires_at.as_str() {
            return Err(LanPairingRejectionReason::ControllerLeaseExpired);
        }

        let mut active_lease = self
            .controller_lease
            .lock()
            .map_err(|_| LanPairingRejectionReason::Malformed)?;
        clear_expired_lease(&mut active_lease, observed_at);

        match active_lease.as_ref() {
            Some(active) if active.matches(&lease) => {
                *active_lease = Some(lease);
                Ok(())
            }
            Some(_) => Err(LanPairingRejectionReason::TakeoverDenied),
            None => {
                *active_lease = Some(lease);
                Ok(())
            }
        }
    }
}

pub(crate) fn controller_lease_renew(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    controller_lease_lifecycle_command(
        runtime,
        origin,
        command,
        constants::value::LAN_AUDIT_CONTROLLER_LEASE_RENEWED,
        |runtime, intent, observed_at| runtime.renew_controller_lease(intent, observed_at),
    )
}

pub(crate) fn controller_lease_release(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    controller_lease_lifecycle_command(
        runtime,
        origin,
        command,
        constants::value::LAN_AUDIT_CONTROLLER_LEASE_RELEASED,
        |runtime, intent, observed_at| runtime.release_controller_lease(intent, observed_at),
    )
}

pub(crate) fn controller_lease_takeover(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let observed_origin = origin.as_deref();
    match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_write_authority(&intent))
            .and_then(|()| validate_registry_selection_intent(&runtime, observed_origin, &intent))
            .and_then(|()| runtime.takeover_controller_lease(&intent, timestamp_now().as_str()))
        {
            Ok(()) => {
                let audit_fields = controller_lease_audit_fields(
                    &command,
                    &intent,
                    observed_origin,
                    constants::value::LAN_AUDIT_CONTROLLER_LEASE_TAKEOVER_ACCEPTED,
                    None,
                );
                let mut event = pairing_status_event(&runtime, command);
                event.payload.extend(audit_fields);
                event
            }
            Err(reason) => {
                let audit_type = if reason == LanPairingRejectionReason::TakeoverDenied {
                    constants::value::LAN_AUDIT_CONTROLLER_LEASE_TAKEOVER_REJECTED
                } else {
                    constants::value::LAN_AUDIT_CONTROL_REJECTED
                };
                let payload = controller_lease_audit_fields(
                    &command,
                    &intent,
                    observed_origin,
                    audit_type,
                    Some(&reason),
                );
                build_event(
                    constants::event_id::COMMAND_REJECTED,
                    &command.message_id,
                    command.source,
                    AgentEventName::AgentCommandRejected,
                    LogLevel::Warn,
                    payload,
                    None,
                )
            }
        },
        Err(reason) => rejection_event(command, reason, None, observed_origin),
    }
}

fn controller_lease_lifecycle_command(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
    audit_event_type: &'static str,
    apply: impl Fn(
        &LanPairingRuntime,
        &LanParentIntentEnvelope,
        &str,
    ) -> Result<(), LanPairingRejectionReason>,
) -> AgentEventEnvelope {
    let observed_origin = origin.as_deref();
    match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_write_authority(&intent))
            .and_then(|()| validate_registry_selection_intent(&runtime, observed_origin, &intent))
            .and_then(|()| apply(&runtime, &intent, timestamp_now().as_str()))
        {
            Ok(()) => {
                let audit_fields = controller_lease_audit_fields(
                    &command,
                    &intent,
                    observed_origin,
                    audit_event_type,
                    None,
                );
                let mut event = pairing_status_event(&runtime, command);
                event.payload.extend(audit_fields);
                event
            }
            Err(reason) => rejection_event(command, reason, Some(&intent), observed_origin),
        },
        Err(reason) => rejection_event(command, reason, None, observed_origin),
    }
}

impl LanControllerLeaseState {
    pub(crate) fn from_intent(intent: &LanParentIntentEnvelope) -> Self {
        Self {
            controller_lease_id: intent.controller_lease_id.clone(),
            controller_device_id: intent.controller_device_id.clone(),
            parent_actor_id: intent.parent_actor_id.clone(),
            expires_at: intent.controller_lease_expires_at.clone(),
        }
    }

    fn matches(&self, other: &Self) -> bool {
        self.controller_lease_id == other.controller_lease_id
            && self.controller_device_id == other.controller_device_id
            && self.parent_actor_id == other.parent_actor_id
    }
}

fn clear_expired_lease(active_lease: &mut Option<LanControllerLeaseState>, observed_at: &str) {
    if active_lease
        .as_ref()
        .is_some_and(|active| observed_at > active.expires_at.as_str())
    {
        *active_lease = None;
    }
}
