use chrono::{DateTime, Utc};

use crate::lan_pairing_runtime_state::passive_discovery::pipeline_health::{
    LanPassiveDiscoveryPipelineHealthSnapshot, LanPassiveDiscoveryPipelineState,
};

struct CapabilityTimestamp<'a>(&'a str);

pub(super) fn is_coherent(health: &LanPassiveDiscoveryPipelineHealthSnapshot) -> bool {
    if !last_success_is_valid(health) {
        return false;
    }
    match health.state {
        LanPassiveDiscoveryPipelineState::Starting => {
            health.consecutive_failures == 0
                && health.retry_delay_millis.is_none()
                && health.issue.is_none()
                && health.last_succeeded_at.is_none()
        }
        LanPassiveDiscoveryPipelineState::Healthy => {
            health.consecutive_failures == 0
                && health.retry_delay_millis.is_none()
                && health.issue.is_none()
                && health.last_succeeded_at.is_some()
        }
        LanPassiveDiscoveryPipelineState::RetryScheduled => {
            health.consecutive_failures > 0
                && health.retry_delay_millis.is_some()
                && health.issue.is_some()
        }
        LanPassiveDiscoveryPipelineState::Stopped => {
            health.consecutive_failures == 0
                && health.retry_delay_millis.is_none()
                && health.issue.is_none()
        }
    }
}

fn last_success_is_valid(health: &LanPassiveDiscoveryPipelineHealthSnapshot) -> bool {
    health
        .last_succeeded_at
        .as_deref()
        .map(|value| timestamp_is_not_in_future(&CapabilityTimestamp(value)))
        .unwrap_or(true)
}

fn timestamp_is_not_in_future(value: &CapabilityTimestamp<'_>) -> bool {
    DateTime::parse_from_rfc3339(value.0)
        .map(|timestamp| timestamp.with_timezone(&Utc) <= Utc::now())
        .unwrap_or(false)
}
