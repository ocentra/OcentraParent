use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryUdpListenerIssueKind;

use super::{
    LanPassiveDiscoveryRuntimeAvailability, LanPassiveDiscoverySourceAvailability,
    LanPassiveDiscoverySourceCapability,
};
use crate::lan_pairing_runtime_state::passive_discovery::pipeline_health::{
    LanPassiveDiscoveryPipelineHealthSnapshot, LanPassiveDiscoveryPipelineState,
};

pub(super) fn for_sources(
    sources: &[LanPassiveDiscoverySourceCapability],
    active_listener_count: usize,
    pipeline_health: &LanPassiveDiscoveryPipelineHealthSnapshot,
) -> LanPassiveDiscoveryRuntimeAvailability {
    match pipeline_health.state {
        LanPassiveDiscoveryPipelineState::Starting => {
            return LanPassiveDiscoveryRuntimeAvailability::Starting;
        }
        LanPassiveDiscoveryPipelineState::RetryScheduled => {
            return LanPassiveDiscoveryRuntimeAvailability::Unavailable;
        }
        LanPassiveDiscoveryPipelineState::Stopped => {
            return LanPassiveDiscoveryRuntimeAvailability::Stopped;
        }
        LanPassiveDiscoveryPipelineState::Healthy => {}
    }
    if active_listener_count == sources.len() && !sources.is_empty() {
        return LanPassiveDiscoveryRuntimeAvailability::Available;
    }
    if active_listener_count > 0 {
        return LanPassiveDiscoveryRuntimeAvailability::Degraded;
    }
    if sources.iter().any(source_requires_apple_manual_action) {
        return LanPassiveDiscoveryRuntimeAvailability::ManualRequired;
    }
    if sources.iter().all(source_is_pending_without_issue) {
        return LanPassiveDiscoveryRuntimeAvailability::Starting;
    }
    LanPassiveDiscoveryRuntimeAvailability::Unavailable
}

fn source_requires_apple_manual_action(source: &LanPassiveDiscoverySourceCapability) -> bool {
    source.issue.as_ref().is_some_and(|issue| {
        issue.kind == LanPassiveDiscoveryUdpListenerIssueKind::AppleLocalNetworkPermissionRequired
    })
}

fn source_is_pending_without_issue(source: &LanPassiveDiscoverySourceCapability) -> bool {
    source.availability == LanPassiveDiscoverySourceAvailability::PendingBind
        && source.issue.is_none()
}
