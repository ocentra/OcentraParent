use std::{
    io,
    sync::{atomic::AtomicBool, Arc},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::lan_pairing::LanPairingRuntime;

use super::{
    capability_store, listener_runtime, pipeline_health, reconciliation, refresh_signal_channel,
    service_owner::LanPassiveDiscoveryServiceOwner, LanPassiveDiscoveryServiceRuntime,
    PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL,
};

pub(super) fn start(runtime: &LanPairingRuntime) -> io::Result<LanPassiveDiscoveryServiceRuntime> {
    let pipeline_health = pipeline_health::LanPassiveDiscoveryPipelineHealth::starting();
    capability_store::record_starting(runtime, &pipeline_health.snapshot());
    let (sender, receiver) = refresh_signal_channel();
    let listener_state = Arc::downgrade(&runtime.passive_discovery_listener_state);
    let listener_join = listener_runtime::spawn(runtime, sender, pipeline_health.clone())
        .map_err(|error| record_listener_spawn_failure(runtime, &pipeline_health, error))?;
    let mut listener_join = Some(listener_join);
    let reconciliation_stop = Arc::new(AtomicBool::new(false));
    let reconciliation_join = spawn_reconciliation(
        runtime,
        receiver,
        &pipeline_health,
        &reconciliation_stop,
        &mut listener_join,
    )?;
    let listener_join = listener_join
        .take()
        .unwrap_or_else(|| std::process::abort());
    let owner = Arc::new(LanPassiveDiscoveryServiceOwner::new(
        listener_state,
        Some(listener_join),
        reconciliation_stop,
        reconciliation_join,
    ));
    Ok(LanPassiveDiscoveryServiceRuntime { _owner: owner })
}

fn spawn_reconciliation(
    runtime: &LanPairingRuntime,
    receiver: super::LanPassiveDiscoveryRefreshSignalReceiver,
    pipeline_health: &pipeline_health::LanPassiveDiscoveryPipelineHealth,
    reconciliation_stop: &Arc<AtomicBool>,
    listener_join: &mut Option<JoinHandle<()>>,
) -> io::Result<JoinHandle<()>> {
    match reconciliation::spawn(
        runtime.clone(),
        receiver,
        pipeline_health.clone(),
        Arc::clone(reconciliation_stop),
    ) {
        Ok(join) => Ok(join),
        Err(error) => {
            if let Some(listener) = listener_join.take() {
                stop_and_join_listener_or_abort(runtime, listener);
            }
            Err(record_reconciliation_spawn_failure(
                runtime,
                pipeline_health,
                error,
            ))
        }
    }
}

fn record_listener_spawn_failure(
    runtime: &LanPairingRuntime,
    pipeline_health: &pipeline_health::LanPassiveDiscoveryPipelineHealth,
    error: io::Error,
) -> io::Error {
    if let Ok(mut state) = runtime.passive_discovery_listener_state.lock() {
        state.stop();
    }
    record_spawn_failure(
        runtime,
        pipeline_health,
        pipeline_health::LanPassiveDiscoveryPipelineIssue::ListenerRuntimeSpawnFailed,
    );
    error
}

fn record_reconciliation_spawn_failure(
    runtime: &LanPairingRuntime,
    pipeline_health: &pipeline_health::LanPassiveDiscoveryPipelineHealth,
    error: io::Error,
) -> io::Error {
    record_spawn_failure(
        runtime,
        pipeline_health,
        pipeline_health::LanPassiveDiscoveryPipelineIssue::ReconciliationRuntimeSpawnFailed,
    );
    error
}

fn record_spawn_failure(
    runtime: &LanPairingRuntime,
    pipeline_health: &pipeline_health::LanPassiveDiscoveryPipelineHealth,
    issue: pipeline_health::LanPassiveDiscoveryPipelineIssue,
) {
    pipeline_health.record_failure(issue, PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL);
    let capability_store =
        capability_store::LanPassiveDiscoveryCapabilityStore::for_runtime(runtime);
    let _persisted = capability_store.save_pipeline_health(&pipeline_health.snapshot());
}

fn stop_and_join_listener_or_abort(runtime: &LanPairingRuntime, listener: JoinHandle<()>) {
    if let Ok(mut state) = runtime.passive_discovery_listener_state.lock() {
        state.stop();
    }
    let deadline = Instant::now() + reconciliation::RECONCILIATION_SHUTDOWN_TIMEOUT;
    while !listener.is_finished() && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(5));
    }
    if !listener.is_finished() {
        std::process::abort();
    }
    let _joined = listener.join();
}
