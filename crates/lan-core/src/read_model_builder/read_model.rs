use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceReadModel, LanDiscoveryEventHistoryState,
};

use super::canonical_household_device_spine::canonical_household_devices;
use super::history::discovery_event_history;
use super::production_household_proof::production_household_proof_summary;
use super::scan::scan_summary;
use super::signed_discovery_relay_spine::signed_discovery_relay_spine_summary;
use super::LanAddDeviceReadModelInput;
use crate::read_model::{audit_check_labels, honest_non_claims, lan_discovery_source_matrix};

pub(super) fn build_lan_add_device_read_model(
    input: LanAddDeviceReadModelInput,
) -> LanBrowserAddDeviceReadModel {
    let scan_summary = scan_summary(&input.discovered_devices);
    let canonical_household_devices = canonical_household_devices(
        &input.discovered_devices,
        &input.trusted_device_registry,
        &input.household_device_decisions,
        &input.generated_at,
    );
    let production_household_proof = production_household_proof_summary(
        &input.generated_at,
        input.physical_household_lan_state.clone(),
        &scan_summary,
        &input.trusted_device_registry,
        &input.household_device_decisions,
        &input.selected_device_readiness,
    );
    let signed_discovery_relay_spine = signed_discovery_relay_spine_summary(
        &input.generated_at,
        input.physical_household_lan_state.clone(),
        &scan_summary,
        &input.trusted_device_registry,
        &input.household_device_decisions,
        &input.selected_device_readiness,
    );

    let service_state = unavailable_state_if_missing(input.service_data_available);
    let platform_state = unavailable_state_if_missing(input.platform_data_available);
    let discovery_event_history_state =
        discovery_event_history_state(input.service_data_available, input.platform_data_available);
    let discovery_event_history = discovery_event_history(
        &input.generated_at,
        &discovery_event_history_state,
        &input.physical_household_lan_state,
        &input.selected_device_readiness,
        &canonical_household_devices,
        &input.discovered_devices,
    );

    LanBrowserAddDeviceReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: input.generated_at.clone(),
        discovery_source: input.discovery_source,
        add_device_state: service_state
            .clone()
            .unwrap_or_else(|| input.add_device_state.clone()),
        local_service_discovery_state: service_state
            .unwrap_or_else(|| input.local_service_discovery_state.clone()),
        physical_household_lan_state: platform_state
            .unwrap_or_else(|| input.physical_household_lan_state.clone()),
        cloud_relay_state: input.cloud_relay_state,
        scan_summary: scan_summary.clone(),
        discovered_devices: input.discovered_devices,
        discovery_event_history,
        canonical_household_devices,
        pairing_requests: input.pairing_requests,
        trusted_device_registry: input.trusted_device_registry,
        household_device_decisions: input.household_device_decisions,
        production_household_proof: Some(production_household_proof),
        signed_discovery_relay_spine: Some(signed_discovery_relay_spine),
        lan_discovery_source_matrix: Some(lan_discovery_source_matrix(
            &input.generated_at,
            &scan_summary,
        )),
        trusted_device_ids: input.trusted_device_ids,
        revoked_device_ids: input.revoked_device_ids,
        selected_device_readiness: input.selected_device_readiness,
        controller_authority: input.controller_authority,
        observer_authority: input.observer_authority,
        route_requirement_labels: constants::lan_pairing::ROUTE_REQUIREMENTS
            .iter()
            .map(|requirement| (*requirement).to_string())
            .collect(),
        audit_check_labels: audit_check_labels(),
        honest_non_claims: honest_non_claims(),
    }
}

fn unavailable_state_if_missing(available: bool) -> Option<LanPairingProductionDiscoveryState> {
    if available {
        None
    } else {
        Some(LanPairingProductionDiscoveryState::Unavailable)
    }
}

fn discovery_event_history_state(
    service_data_available: bool,
    platform_data_available: bool,
) -> LanDiscoveryEventHistoryState {
    if service_data_available || platform_data_available {
        LanDiscoveryEventHistoryState::Empty
    } else {
        LanDiscoveryEventHistoryState::Unavailable
    }
}
