use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason;
use ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity;
use ocentra_parent_agent_protocol::constants::lan_pairing as lan_pairing_constants;

use super::LanPassiveDiscoveryLocalNetworkChangeTrigger;

pub(super) fn local_network_change_triggers(
    previous_identity: Option<&LanPassiveRuntimeLocalNetworkIdentity>,
    current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
) -> Vec<LanPassiveDiscoveryLocalNetworkChangeTrigger> {
    let Some(previous_identity) = previous_identity else {
        return Vec::new();
    };

    let mut triggers = Vec::new();
    triggers.extend(interface_change_triggers(
        previous_identity,
        current_identity,
    ));

    if let Some(trigger) = wifi_ssid_change_trigger(previous_identity, current_identity) {
        triggers.push(trigger);
    }
    if let Some(trigger) = ip_address_change_trigger(previous_identity, current_identity) {
        triggers.push(trigger);
    }
    if let Some(trigger) = default_gateway_change_trigger(previous_identity, current_identity) {
        triggers.push(trigger);
    }

    triggers
}

fn interface_change_triggers(
    previous_identity: &LanPassiveRuntimeLocalNetworkIdentity,
    current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
) -> Vec<LanPassiveDiscoveryLocalNetworkChangeTrigger> {
    if previous_identity.network_interface == current_identity.network_interface {
        return Vec::new();
    }

    let mut triggers = Vec::new();

    if let Some(previous_interface) = previous_identity.network_interface.as_ref() {
        let mut summary =
            String::from(lan_pairing_constants::PASSIVE_DISCOVERY_INTERFACE_DOWN_PREFIX);
        summary.push_str(previous_interface);
        triggers.push(LanPassiveDiscoveryLocalNetworkChangeTrigger {
            reason: LanPassiveDiscoveryTriggerReason::InterfaceDown,
            summary,
        });
    }

    if let Some(current_interface) = current_identity.network_interface.as_ref() {
        let mut summary =
            String::from(lan_pairing_constants::PASSIVE_DISCOVERY_INTERFACE_UP_PREFIX);
        summary.push_str(current_interface);
        triggers.push(LanPassiveDiscoveryLocalNetworkChangeTrigger {
            reason: LanPassiveDiscoveryTriggerReason::InterfaceUp,
            summary,
        });
    }

    triggers
}

fn wifi_ssid_change_trigger(
    previous_identity: &LanPassiveRuntimeLocalNetworkIdentity,
    current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
) -> Option<LanPassiveDiscoveryLocalNetworkChangeTrigger> {
    if previous_identity.network_interface != current_identity.network_interface
        || previous_identity.wifi_ssid == current_identity.wifi_ssid
        || (previous_identity.wifi_ssid.is_none() && current_identity.wifi_ssid.is_none())
    {
        return None;
    }

    let mut summary =
        String::from(lan_pairing_constants::PASSIVE_DISCOVERY_WIFI_SSID_CHANGED_PREFIX);
    summary.push_str(
        previous_identity
            .wifi_ssid
            .as_deref()
            .unwrap_or(lan_pairing_constants::PASSIVE_DISCOVERY_NONE),
    );
    summary.push_str(lan_pairing_constants::PASSIVE_DISCOVERY_VALUE_CHANGE_SEPARATOR);
    summary.push_str(
        current_identity
            .wifi_ssid
            .as_deref()
            .unwrap_or(lan_pairing_constants::PASSIVE_DISCOVERY_NONE),
    );

    Some(LanPassiveDiscoveryLocalNetworkChangeTrigger {
        reason: LanPassiveDiscoveryTriggerReason::WifiSsidChanged,
        summary,
    })
}

fn ip_address_change_trigger(
    previous_identity: &LanPassiveRuntimeLocalNetworkIdentity,
    current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
) -> Option<LanPassiveDiscoveryLocalNetworkChangeTrigger> {
    if previous_identity.ip_address == current_identity.ip_address {
        return None;
    }

    let mut summary =
        String::from(lan_pairing_constants::PASSIVE_DISCOVERY_IP_ADDRESS_CHANGED_PREFIX);
    summary.push_str(
        previous_identity
            .ip_address
            .as_deref()
            .unwrap_or(lan_pairing_constants::PASSIVE_DISCOVERY_NONE),
    );
    summary.push_str(lan_pairing_constants::PASSIVE_DISCOVERY_VALUE_CHANGE_SEPARATOR);
    summary.push_str(
        current_identity
            .ip_address
            .as_deref()
            .unwrap_or(lan_pairing_constants::PASSIVE_DISCOVERY_NONE),
    );

    Some(LanPassiveDiscoveryLocalNetworkChangeTrigger {
        reason: LanPassiveDiscoveryTriggerReason::IpAddressChanged,
        summary,
    })
}

fn default_gateway_change_trigger(
    previous_identity: &LanPassiveRuntimeLocalNetworkIdentity,
    current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
) -> Option<LanPassiveDiscoveryLocalNetworkChangeTrigger> {
    if previous_identity.default_gateway == current_identity.default_gateway {
        return None;
    }

    let mut summary =
        String::from(lan_pairing_constants::PASSIVE_DISCOVERY_DEFAULT_GATEWAY_CHANGED_PREFIX);
    summary.push_str(
        previous_identity
            .default_gateway
            .as_deref()
            .unwrap_or(lan_pairing_constants::PASSIVE_DISCOVERY_NONE),
    );
    summary.push_str(lan_pairing_constants::PASSIVE_DISCOVERY_VALUE_CHANGE_SEPARATOR);
    summary.push_str(
        current_identity
            .default_gateway
            .as_deref()
            .unwrap_or(lan_pairing_constants::PASSIVE_DISCOVERY_NONE),
    );

    Some(LanPassiveDiscoveryLocalNetworkChangeTrigger {
        reason: LanPassiveDiscoveryTriggerReason::DefaultGatewayChanged,
        summary,
    })
}
