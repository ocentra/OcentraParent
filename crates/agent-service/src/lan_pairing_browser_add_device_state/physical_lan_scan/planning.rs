use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

use ocentra_lan_core::network_inventory::{
    plan_lan_discovery_scan_until, plan_lan_discovery_scan_with_active_refresh_suppression,
    LanDiscoveryRefreshMode, LanDiscoveryScanPlan, LanNetworkInventoryDevice,
};
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

pub(super) fn plan_for_scan(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    deadline: Option<Instant>,
    cancellation: Option<&AtomicBool>,
) -> Option<LanDiscoveryScanPlan> {
    let plan = match (deadline, cancellation) {
        (Some(deadline), Some(cancellation)) => plan_lan_discovery_scan_until(
            identity_hint_devices,
            previous_devices,
            refresh_mode,
            active_refresh_suppression_devices,
            deadline,
            cancellation,
        ),
        _ => plan_lan_discovery_scan_with_active_refresh_suppression(
            identity_hint_devices,
            previous_devices,
            refresh_mode,
            active_refresh_suppression_devices,
        ),
    };
    if cancellation.is_some_and(|value| value.load(Ordering::Acquire))
        || deadline.is_some_and(|deadline| Instant::now() >= deadline)
    {
        return None;
    }
    Some(plan)
}
