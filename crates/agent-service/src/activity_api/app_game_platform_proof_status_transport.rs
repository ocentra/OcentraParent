use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use std::sync::Arc;

use super::app_game_adapter_execution_readiness_payload::GeneratedAtText;
use super::app_game_adapter_host_capabilities::HostCapabilitySignals;
use super::app_game_platform_probe_cache::PlatformProbeCache;
use super::app_game_platform_proof_status_payload::{
    app_game_platform_proof_status_payload,
    app_game_platform_proof_status_read_model_from_preflights,
};
use crate::websocket::{WebsocketPeerProvenance, WebsocketPlatformProbeDispatcher};
use crate::{event_builder::build_event, time::timestamp_now};
use ocentra_parent_screen_capture_adapter::linux_foreground_source::LinuxForegroundSourcePreflight;

pub(crate) fn platform_probe_dispatcher(
    cache: PlatformProbeCache,
) -> Arc<WebsocketPlatformProbeDispatcher> {
    Arc::new(move |command, provenance| {
        let cache = cache.clone();
        Box::pin(async move {
            build_activity_app_game_platform_proof_status_report(command, cache, provenance).await
        })
    })
}

pub(crate) async fn build_activity_app_game_platform_proof_status_report(
    command: AgentCommandEnvelope,
    probe_cache: PlatformProbeCache,
    provenance: WebsocketPeerProvenance,
) -> AgentEventEnvelope {
    let generated_at = GeneratedAtText(timestamp_now());
    let _ = provenance;
    // ADB is a direct, request-time host capability signal. Only the Docker
    // preflight is server-owned and cached; keeping these lifetimes separate
    // prevents a five-minute cache from turning device presence into stale
    // authority or visibility.
    let host_capabilities = HostCapabilitySignals::detect();
    let linux_docker_host_preflight = probe_cache.snapshot();
    // The live Linux foreground tool path is intentionally not spawned from a
    // request. No retained subprocess owner can guarantee custody across
    // setsid/pid-namespace escapes, so this production handler stays
    // unavailable until an owned single-flight worker with a real OS custody
    // primitive exists. Docker host visibility is an independent bounded
    // preflight and does not mint foreground-source authority.
    let linux_preflight = LinuxForegroundSourcePreflight::unavailable();
    let read_model = app_game_platform_proof_status_read_model_from_preflights(
        generated_at,
        &host_capabilities,
        &linux_preflight,
        &linux_docker_host_preflight,
    );
    build_event(
        ocentra_parent_agent_protocol::constants::event_id::ACTIVITY_APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGamePlatformProofStatusReadModelReported,
        LogLevel::Info,
        app_game_platform_proof_status_payload(&read_model),
        None,
    )
}
