use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingAuditEventType, LanPairingOptionalText, LanPairingText, LanParentIntentEnvelope,
};
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use crate::lan_pairing::{
    lan_ai_job_lease_events::{
        lan_ai_job_completed_event, lan_ai_job_duplicate_rejected_event,
        lan_ai_job_lease_state_event,
    },
    LanPairingRuntime,
};
use crate::lan_pairing_runtime_state::job_leases::LanAiJobLeaseTransition;

use super::super::fields::lan_ai_job_id;
use super::lan_ai_rejection_event;

pub(super) fn lan_ai_job_transition_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
    requested_capability: &LanPairingText,
) -> AgentEventEnvelope {
    let job_id = lan_ai_job_id(&command, intent);
    match runtime.claim_lan_ai_job_lease(&job_id.0) {
        Ok(LanAiJobLeaseTransition::Claimed(lease)) => {
            let completed_lease = runtime
                .complete_lan_ai_job_lease(&job_id.0)
                .unwrap_or(lease);
            lan_ai_job_completed_event(
                runtime,
                command,
                intent,
                origin,
                requested_capability,
                &completed_lease,
            )
        }
        Ok(LanAiJobLeaseTransition::DuplicateCompleted(lease)) => lan_ai_job_completed_event(
            runtime,
            command,
            intent,
            origin,
            requested_capability,
            &lease,
        ),
        Ok(LanAiJobLeaseTransition::DuplicateActiveRejected(lease)) => {
            lan_ai_job_duplicate_rejected_event(runtime, command, intent, origin, &lease)
        }
        Ok(LanAiJobLeaseTransition::ExpiredRequeued(lease))
        | Ok(LanAiJobLeaseTransition::DeadLettered(lease)) => {
            lan_ai_job_lease_state_event(runtime, command, intent, origin, &lease)
        }
        Err(reason) => lan_ai_rejection_event(
            runtime,
            command,
            &reason,
            Some(intent),
            origin,
            LanPairingAuditEventType::LanAiJobRejected,
        ),
    }
}
