use crate::lan_pairing::LanPairingRuntime;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingAuditEventType;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;

#[path = "../lan_pairing_controller_lease_validation.rs"]
mod lease_validation;

use ocentra_parent_agent_core::trusted_device_registry::controller_lease::LanControllerLeaseMutation;

#[derive(Clone, Debug)]
pub(crate) struct LanControllerLeaseState {
    pub(crate) controller_lease_id: String,
    pub(crate) controller_device_id: String,
    pub(crate) parent_actor_id: String,
    pub(crate) expires_at: String,
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
        LanControllerLeaseMutation::Renew,
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
        LanControllerLeaseMutation::Release,
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
