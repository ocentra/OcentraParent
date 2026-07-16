use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use std::fmt::Display;

use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanAiJobLeaseState {
    pub(crate) job_id: String,
    pub(crate) claim_id: String,
    pub(crate) lease_id: String,
    pub(crate) lease_state: &'static str,
    pub(crate) attempt_count: u64,
    pub(crate) expires_at: String,
    pub(crate) dead_letter_reason: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LanAiJobLeaseTransition {
    Claimed(LanAiJobLeaseState),
    DuplicateCompleted(LanAiJobLeaseState),
    DuplicateActiveRejected(LanAiJobLeaseState),
    ExpiredRequeued(LanAiJobLeaseState),
    DeadLettered(LanAiJobLeaseState),
}

impl LanPairingRuntime {
    pub(crate) fn claim_lan_ai_job_lease(
        &self,
        job_id: &impl Display,
    ) -> Result<LanAiJobLeaseTransition, LanPairingRejectionReason> {
        let job_id = LanPairingText(job_id.to_string());
        let now: LanPairingText = timestamp_now::<String>().into();
        let mut leases = self.lan_ai_job_leases.lock().map_err(|error| {
            let _ = error;
            LanPairingRejectionReason::Malformed
        })?;
        match leases.iter_mut().find(|lease| lease.job_id == job_id.0) {
            Some(lease) => Ok(transition_existing_lease(lease, &now)),
            None => {
                let lease = new_claimed_lease(&job_id);
                leases.push(lease.clone());
                Ok(LanAiJobLeaseTransition::Claimed(lease))
            }
        }
    }

    pub(crate) fn complete_lan_ai_job_lease(
        &self,
        job_id: &impl Display,
    ) -> Option<LanAiJobLeaseState> {
        let job_id = LanPairingText(job_id.to_string());
        self.lan_ai_job_leases.lock().ok().and_then(|mut leases| {
            leases
                .iter_mut()
                .find(|lease| lease.job_id == job_id.0)
                .map(|lease| {
                    lease.lease_state = constants::value::LAN_AI_LEASE_STATE_COMPLETED;
                    lease.dead_letter_reason = None;
                    lease.clone()
                })
        })
    }
}

fn transition_existing_lease(
    lease: &mut LanAiJobLeaseState,
    now: &LanPairingText,
) -> LanAiJobLeaseTransition {
    match lease.lease_state {
        constants::value::LAN_AI_LEASE_STATE_COMPLETED => {
            LanAiJobLeaseTransition::DuplicateCompleted(lease.clone())
        }
        constants::value::LAN_AI_LEASE_STATE_DEAD_LETTERED => {
            LanAiJobLeaseTransition::DeadLettered(lease.clone())
        }
        _ if lease.expires_at.as_str() > now.0.as_str() => {
            lease.lease_state = constants::value::LAN_AI_LEASE_STATE_DUPLICATE_REJECTED;
            LanAiJobLeaseTransition::DuplicateActiveRejected(lease.clone())
        }
        _ => transition_expired_lease(lease),
    }
}

fn transition_expired_lease(lease: &mut LanAiJobLeaseState) -> LanAiJobLeaseTransition {
    lease.attempt_count += 1;
    if lease.attempt_count >= constants::lan_pairing::LAN_AI_MAX_LEASE_ATTEMPTS as u64 {
        lease.lease_state = constants::value::LAN_AI_LEASE_STATE_DEAD_LETTERED;
        lease.dead_letter_reason = Some(constants::value::LAN_AI_DEAD_LETTER_REASON_MAX_ATTEMPTS);
        LanAiJobLeaseTransition::DeadLettered(lease.clone())
    } else {
        lease.lease_state = constants::value::LAN_AI_LEASE_STATE_EXPIRED_REQUEUED;
        lease.dead_letter_reason = None;
        LanAiJobLeaseTransition::ExpiredRequeued(lease.clone())
    }
}

fn new_claimed_lease(job_id: &LanPairingText) -> LanAiJobLeaseState {
    LanAiJobLeaseState {
        job_id: job_id.to_string(),
        claim_id: lan_ai_claim_id(job_id).0,
        lease_id: lan_ai_lease_id(job_id).0,
        lease_state: constants::value::LAN_AI_LEASE_STATE_CLAIMED,
        attempt_count: 1,
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        dead_letter_reason: None,
    }
}

fn lan_ai_claim_id(job_id: &LanPairingText) -> LanPairingText {
    let mut claim_id = String::from(constants::lan_pairing::LAN_AI_CLAIM_ID_PREFIX);
    claim_id.push_str(job_id.0.as_str());
    LanPairingText(claim_id)
}

fn lan_ai_lease_id(job_id: &LanPairingText) -> LanPairingText {
    let mut lease_id = String::from(constants::lan_pairing::LAN_AI_LEASE_ID_PREFIX);
    lease_id.push_str(job_id.0.as_str());
    LanPairingText(lease_id)
}
