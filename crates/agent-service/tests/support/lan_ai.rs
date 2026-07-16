use std::fmt::Display;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::lan_pairing::LanPairingRuntime;
use crate::lan_pairing_runtime_state::job_leases::LanAiJobLeaseState;
use crate::lan_pairing_runtime_state::provider_heartbeat::LanAiProviderHeartbeatState;
use crate::lan_runtime_test_support::TestLeaseState;
use crate::test_text::TestText;

pub(crate) fn mark_selected_offline_for_test(runtime: &LanPairingRuntime) -> bool {
    runtime
        .registry
        .lock()
        .map(|mut registry| registry.mark_selected_offline(constants::lan_pairing::OBSERVED_AT))
        .unwrap_or(false)
}

pub(crate) fn mark_selected_stale_for_test(runtime: &LanPairingRuntime) -> bool {
    runtime
        .registry
        .lock()
        .map(|mut registry| registry.mark_selected_stale(constants::lan_pairing::EXPIRED_AT))
        .unwrap_or(false)
}

pub(crate) fn mark_lan_ai_provider_heartbeat_stale_for_test(runtime: &LanPairingRuntime) {
    record_lan_ai_provider_heartbeat_state_for_test(
        runtime,
        constants::lan_pairing::EXPIRED_AT,
        LanPairingDeviceReachability::Stale,
    );
}

pub(crate) fn mark_lan_ai_provider_heartbeat_offline_for_test(runtime: &LanPairingRuntime) {
    record_lan_ai_provider_heartbeat_state_for_test(
        runtime,
        constants::lan_pairing::OBSERVED_AT,
        LanPairingDeviceReachability::Offline,
    );
}

pub(crate) fn seed_lan_ai_job_lease_for_test(
    runtime: &LanPairingRuntime,
    job_id: impl Display,
    lease_state: impl Display,
    attempt_count: u64,
    expires_at: impl Display,
) {
    let job_id = job_id.to_string();
    let expires_at = expires_at.to_string();
    let lease_state = TestText::from_display(lease_state);
    let lease_state = normalized_lan_ai_lease_state(&lease_state);
    let Ok(mut leases) = runtime.lan_ai_job_leases.lock() else {
        return;
    };
    leases.retain(|lease| lease.job_id != job_id);
    leases.push(LanAiJobLeaseState {
        job_id,
        claim_id: lan_ai_claim_id().0,
        lease_id: lan_ai_lease_id().0,
        lease_state: match lease_state {
            TestLeaseState::Claimed => constants::value::LAN_AI_LEASE_STATE_CLAIMED,
            TestLeaseState::Completed => constants::value::LAN_AI_LEASE_STATE_COMPLETED,
            TestLeaseState::DuplicateRejected => {
                constants::value::LAN_AI_LEASE_STATE_DUPLICATE_REJECTED
            }
            TestLeaseState::ExpiredRequeued => {
                constants::value::LAN_AI_LEASE_STATE_EXPIRED_REQUEUED
            }
            TestLeaseState::DeadLettered => constants::value::LAN_AI_LEASE_STATE_DEAD_LETTERED,
        },
        attempt_count,
        expires_at,
        dead_letter_reason: None,
    });
}

fn record_lan_ai_provider_heartbeat_state_for_test(
    runtime: &LanPairingRuntime,
    observed_at: impl Display,
    reachability: LanPairingDeviceReachability,
) {
    let Ok(mut state) = runtime.lan_ai_provider_heartbeat.lock() else {
        return;
    };
    *state = Some(LanAiProviderHeartbeatState {
        observed_at: observed_at.to_string(),
        reachability,
    });
}

fn normalized_lan_ai_lease_state(lease_state: &TestText) -> TestLeaseState {
    [
        (
            constants::value::LAN_AI_LEASE_STATE_CLAIMED,
            TestLeaseState::Claimed,
        ),
        (
            constants::value::LAN_AI_LEASE_STATE_COMPLETED,
            TestLeaseState::Completed,
        ),
        (
            constants::value::LAN_AI_LEASE_STATE_DUPLICATE_REJECTED,
            TestLeaseState::DuplicateRejected,
        ),
        (
            constants::value::LAN_AI_LEASE_STATE_EXPIRED_REQUEUED,
            TestLeaseState::ExpiredRequeued,
        ),
        (
            constants::value::LAN_AI_LEASE_STATE_DEAD_LETTERED,
            TestLeaseState::DeadLettered,
        ),
    ]
    .into_iter()
    .find_map(|(value, state)| (lease_state.as_str() == value).then_some(state))
    .unwrap_or(TestLeaseState::Claimed)
}

fn lan_ai_claim_id() -> TestText {
    TestText::from_display(format!(
        "{}{}",
        constants::lan_pairing::LAN_AI_CLAIM_ID_PREFIX,
        constants::lan_pairing::LAN_AI_JOB_ID
    ))
}

fn lan_ai_lease_id() -> TestText {
    TestText::from_display(format!(
        "{}{}",
        constants::lan_pairing::LAN_AI_LEASE_ID_PREFIX,
        constants::lan_pairing::LAN_AI_JOB_ID
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lan_ai_helpers_are_linked() {
        let _ = mark_selected_offline_for_test;
        let _ = mark_selected_stale_for_test;
        let _ = mark_lan_ai_provider_heartbeat_stale_for_test;
        let _ = mark_lan_ai_provider_heartbeat_offline_for_test;
        let _ = |runtime: &LanPairingRuntime| {
            seed_lan_ai_job_lease_for_test(runtime, "job", "claimed", 1, "2026-06-24T00:00:00Z")
        };
        let lease_state = TestText::from_display(constants::value::LAN_AI_LEASE_STATE_CLAIMED);
        let _ = normalized_lan_ai_lease_state(&lease_state);
        let _ = lan_ai_claim_id;
        let _ = lan_ai_lease_id;
    }
}
