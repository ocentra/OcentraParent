use std::sync::atomic::AtomicBool;
use std::time::Instant;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use crate::network_inventory::active_refresh::{
    targeted_arp_refresh_evidence_for_identity,
    targeted_arp_refresh_evidence_for_scan_plan_until as targeted_arp_refresh_evidence_for_scan_plan_until_impl,
};
use crate::network_inventory_hardware::local_network_identity;

use super::super::{
    LanDiscoveryRefreshMode, LanDiscoveryScanPlan, LanNetworkInventoryDevice,
    LanTargetedArpRefreshEvidence,
};

pub(crate) fn targeted_arp_refresh_evidence_for_scan(
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
) -> Vec<LanTargetedArpRefreshEvidence> {
    if refresh_mode != LanDiscoveryRefreshMode::ActiveSubnetRefresh {
        return Vec::new();
    }
    let identity = local_network_identity();
    targeted_arp_refresh_evidence_for_identity(
        identity.as_ref(),
        active_refresh_suppression_devices,
        previous_devices,
    )
}

pub(crate) fn targeted_arp_refresh_evidence_for_scan_plan_until(
    scan_plan: &LanDiscoveryScanPlan,
    previous_devices: &[LanNetworkInventoryDevice],
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    cancellation: Option<&AtomicBool>,
    outer_deadline: Option<Instant>,
) -> Vec<LanTargetedArpRefreshEvidence> {
    targeted_arp_refresh_evidence_for_scan_plan_until_impl(
        scan_plan,
        previous_devices,
        active_refresh_suppression_devices,
        cancellation,
        outer_deadline,
    )
}
