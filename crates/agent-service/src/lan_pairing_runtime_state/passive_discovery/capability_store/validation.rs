use super::{
    LanPassiveDiscoveryRuntimeCapability, LanPassiveDiscoverySourceAvailability,
    LanPassiveDiscoverySourceCapability, CAPABILITY_SCHEMA_VERSION,
};
use crate::lan_pairing_runtime_state::passive_discovery::pipeline_health::LanPassiveDiscoveryPipelineState;

#[path = "validation/pipeline.rs"]
mod pipeline;

pub(super) fn validate_and_rederive(
    mut capability: LanPassiveDiscoveryRuntimeCapability,
) -> Option<LanPassiveDiscoveryRuntimeCapability> {
    if capability.schema_version != CAPABILITY_SCHEMA_VERSION
        || capability.process_id != std::process::id()
        || !pipeline::is_coherent(&capability.pipeline_health)
    {
        return None;
    }

    let expected_sources = super::super::passive_discovery_udp_sources();
    if capability.sources.len() != expected_sources.len() {
        return None;
    }
    let mut ordered_sources = Vec::with_capacity(expected_sources.len());
    for expected_source in expected_sources {
        let mut matching = capability
            .sources
            .iter()
            .filter(|source| source.source == *expected_source);
        let source = matching.next()?.clone();
        if matching.next().is_some()
            || !source_is_coherent(&source, &capability.pipeline_health.state)
        {
            return None;
        }
        ordered_sources.push(source);
    }

    let active_listener_count = ordered_sources
        .iter()
        .filter(|source| source.availability == LanPassiveDiscoverySourceAvailability::Listening)
        .count();
    let availability = super::availability::for_sources(
        &ordered_sources,
        active_listener_count,
        &capability.pipeline_health,
    );
    if capability.expected_listener_count != expected_sources.len()
        || capability.active_listener_count != active_listener_count
        || capability.availability != availability
    {
        return None;
    }

    capability.sources = ordered_sources;
    capability.expected_listener_count = expected_sources.len();
    capability.active_listener_count = active_listener_count;
    capability.availability = availability;
    Some(capability)
}

fn source_is_coherent(
    source: &LanPassiveDiscoverySourceCapability,
    pipeline_state: &LanPassiveDiscoveryPipelineState,
) -> bool {
    if source
        .issue
        .as_ref()
        .is_some_and(|issue| issue.source != source.source)
    {
        return false;
    }
    match source.availability {
        LanPassiveDiscoverySourceAvailability::PendingBind
        | LanPassiveDiscoverySourceAvailability::Listening => {
            source.consecutive_failures == 0
                && source.retry_delay_millis.is_none()
                && source.issue.is_none()
                && *pipeline_state != LanPassiveDiscoveryPipelineState::Stopped
        }
        LanPassiveDiscoverySourceAvailability::RetryScheduled => {
            source.consecutive_failures > 0
                && source.retry_delay_millis.is_some()
                && source.issue.is_some()
                && *pipeline_state != LanPassiveDiscoveryPipelineState::Stopped
        }
        LanPassiveDiscoverySourceAvailability::Stopped => {
            source.consecutive_failures == 0
                && source.retry_delay_millis.is_none()
                && source.issue.is_none()
                && *pipeline_state == LanPassiveDiscoveryPipelineState::Stopped
        }
    }
}
