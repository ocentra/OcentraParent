use std::thread;
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use super::probe::probe_service_identity;
use super::targets::service_identity_probe_targets;
use super::{
    apply_service_identity_probe, runtime_service_identity_probe_settings,
    should_probe_service_identity, AllowedSnmpResponseObserver, LanNetworkInventoryDevice,
    LanServiceIdentityProbeObservation, ProbeTarget, ServiceIdentityProbeSettings,
    SERVICE_IDENTITY_PROBE_MAX_CONCURRENCY, SERVICE_IDENTITY_PROBE_SCAN_BUDGET_MS,
};

struct ProbeCandidate {
    index: usize,
    ip_address: String,
    device_id: String,
}

struct ProbeResult {
    index: usize,
    probe_match: Option<LanServiceIdentityProbeObservation>,
}

pub(super) fn enrich_service_identity_probes(
    devices: &mut [LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) {
    let Some(selected_interface) = selected_interface.filter(|value| !value.trim().is_empty())
    else {
        return;
    };
    let targets = service_identity_probe_targets();
    if targets.is_empty() {
        return;
    }
    let deadline = Instant::now() + Duration::from_millis(SERVICE_IDENTITY_PROBE_SCAN_BUDGET_MS);
    let candidates = probe_candidates(devices, probe_suppression_devices, selected_interface);
    let settings = runtime_service_identity_probe_settings();
    for batch in candidates.chunks(SERVICE_IDENTITY_PROBE_MAX_CONCURRENCY) {
        if Instant::now() >= deadline {
            break;
        }
        apply_probe_results(
            devices,
            probe_batch(
                batch,
                &targets,
                settings,
                deadline,
                allowed_snmp_response_observer,
            ),
        );
    }
}

fn probe_candidates(
    devices: &[LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: &str,
) -> Vec<ProbeCandidate> {
    devices
        .iter()
        .enumerate()
        .filter(|(_, device)| {
            should_probe_service_identity(device, probe_suppression_devices, selected_interface)
        })
        .map(|(index, device)| ProbeCandidate {
            index,
            ip_address: device.ip_address.clone(),
            device_id: device.device_id.clone(),
        })
        .collect()
}

fn probe_batch(
    batch: &[ProbeCandidate],
    targets: &[ProbeTarget],
    settings: ServiceIdentityProbeSettings,
    deadline: Instant,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) -> Vec<ProbeResult> {
    thread::scope(|scope| {
        let handles = spawn_probe_handles(
            scope,
            batch,
            targets,
            settings,
            deadline,
            allowed_snmp_response_observer,
        );
        handles
            .into_iter()
            .filter_map(|handle| handle.join().ok())
            .collect()
    })
}

fn spawn_probe_handles<'scope, 'env>(
    scope: &'scope thread::Scope<'scope, 'env>,
    batch: &'scope [ProbeCandidate],
    targets: &'scope [ProbeTarget],
    settings: ServiceIdentityProbeSettings,
    deadline: Instant,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'env>,
) -> Vec<thread::ScopedJoinHandle<'scope, ProbeResult>> {
    batch
        .iter()
        .map(|candidate| {
            scope.spawn(move || ProbeResult {
                index: candidate.index,
                probe_match: probe_service_identity(
                    &candidate.ip_address,
                    Some(candidate.device_id.as_str()),
                    targets,
                    settings,
                    deadline,
                    allowed_snmp_response_observer,
                ),
            })
        })
        .collect()
}

fn apply_probe_results(devices: &mut [LanNetworkInventoryDevice], probe_results: Vec<ProbeResult>) {
    for ProbeResult { index, probe_match } in probe_results {
        let Some(probe_match) = probe_match else {
            continue;
        };
        let Some(device) = devices.get_mut(index) else {
            continue;
        };
        apply_service_identity_probe(device, probe_match);
    }
}
