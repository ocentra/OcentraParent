use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingAuditEventType;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    event_builder::build_event,
    lan_pairing::{
        authority::{validate_registry_selection_intent, validate_write_authority},
        extend_log_fields, rejection_event, validate_command_target, LanPairingRuntime,
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
        observed_at: impl Into<LanPairingText>,
    ) -> Result<(), LanPairingRejectionReason> {
        let observed_at = observed_at.into();
        let lease = LanControllerLeaseState::from_intent(intent);
        if observed_at.0.as_str() > lease.expires_at.as_str() {
            return Err(LanPairingRejectionReason::ControllerLeaseExpired);
        }

        let mut active_lease = self.controller_lease.lock().map_err(|error| {
            let _ = error;
            LanPairingRejectionReason::Malformed
        })?;
        if active_lease
            .as_ref()
            .is_some_and(|active| observed_at.0.as_str() > active.expires_at.as_str())
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
        observed_at: impl Into<LanPairingText>,
    ) -> Result<(), LanPairingRejectionReason> {
        let observed_at = observed_at.into();
        let lease = LanControllerLeaseState::from_intent(intent);
        if observed_at.0.as_str() > lease.expires_at.as_str() {
            return Err(LanPairingRejectionReason::ControllerLeaseExpired);
        }

        let mut active_lease = self.controller_lease.lock().map_err(|error| {
            let _ = error;
            LanPairingRejectionReason::Malformed
        })?;
        clear_expired_lease(&mut active_lease, &observed_at);

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
        observed_at: impl Into<LanPairingText>,
    ) -> Result<(), LanPairingRejectionReason> {
        let observed_at = observed_at.into();
        let lease = LanControllerLeaseState::from_intent(intent);
        let mut active_lease = self.controller_lease.lock().map_err(|error| {
            let _ = error;
            LanPairingRejectionReason::Malformed
        })?;
        clear_expired_lease(&mut active_lease, &observed_at);

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
        observed_at: impl Into<LanPairingText>,
    ) -> Result<(), LanPairingRejectionReason> {
        let observed_at = observed_at.into();
        let lease = LanControllerLeaseState::from_intent(intent);
        if observed_at.0.as_str() > lease.expires_at.as_str() {
            return Err(LanPairingRejectionReason::ControllerLeaseExpired);
        }

        let mut active_lease = self.controller_lease.lock().map_err(|error| {
            let _ = error;
            LanPairingRejectionReason::Malformed
        })?;
        clear_expired_lease(&mut active_lease, &observed_at);

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
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    super::controller_lease_flow::controller_lease_lifecycle_command(
        runtime,
        origin,
        command,
        LanPairingAuditEventType::ControllerLeaseRenewed,
        super::controller_lease_flow::controller_lease_renew,
    )
}

pub(crate) fn controller_lease_release(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    super::controller_lease_flow::controller_lease_lifecycle_command(
        runtime,
        origin,
        command,
        LanPairingAuditEventType::ControllerLeaseReleased,
        super::controller_lease_flow::controller_lease_release,
    )
}

pub(crate) fn controller_lease_takeover(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    super::controller_lease_flow::controller_lease_takeover(runtime, origin, command)
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

fn clear_expired_lease(
    active_lease: &mut Option<LanControllerLeaseState>,
    observed_at: &LanPairingText,
) {
    if active_lease
        .as_ref()
        .is_some_and(|active| observed_at.0.as_str() > active.expires_at.as_str())
    {
        *active_lease = None;
    }
}
