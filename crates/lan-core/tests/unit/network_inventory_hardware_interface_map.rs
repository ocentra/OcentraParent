use serde_json::json;

use ocentra_lan_core::network_inventory_hardware::{
    linux_identity::{
        linux_local_network_interface_map, preferred_linux_local_network_identity,
        windows_local_network_interface_map,
    },
    network_identity_support::{stable_interface_id, LocalNetworkInterfaceIgnoreReason},
    LocalNetworkInterfaceClassification,
};

#[test]
fn windows_interface_map_preserves_normalized_interface_metadata_and_addresses() {
    let map = windows_local_network_interface_map(&[json!({
        "InterfaceAlias": "Wi-Fi",
        "Description": "Intel Wireless",
        "InterfaceIndex": 12,
        "MacAddress": "54-27-1e-97-c3-31",
        "IPAddress": ["192.168.2.42", "fe80::42", "2001:db8::42"],
        "PrefixLength": 24,
        "DefaultGateway": "192.168.2.1",
        "DnsServers": ["192.168.2.1", "2001:4860:4860::8888"],
        "DhcpServer": "192.168.2.1",
        "BroadcastAddress": "192.168.2.255",
        "Ipv6Prefixes": ["2001:db8::42/64", "fe80::42/64"],
        "Status": "Up",
        "IsLoopback": false,
        "WifiSsid": "Home",
        "WifiSignalPercent": 87,
        "InterfaceType": "Wireless"
    })]);

    let interface_map = &map.interfaces;
    assert_eq!(interface_map.len(), 1);
    let interface = &interface_map[0];
    assert_eq!(interface.id, "mac:54-27-1e-97-c3-31");
    assert_eq!(interface.name, "Wi-Fi");
    assert_eq!(interface.description.as_deref(), Some("Intel Wireless"));
    assert_eq!(interface.index, Some(12));
    assert_eq!(interface.mac_address.as_deref(), Some("54-27-1e-97-c3-31"));
    assert_eq!(
        interface.ip_addresses,
        vec![
            "192.168.2.42".to_string(),
            "fe80::42".to_string(),
            "2001:db8::42".to_string(),
        ]
    );
    assert_eq!(interface.default_gateway.as_deref(), Some("192.168.2.1"));
    assert_eq!(
        interface.dns_servers,
        vec![
            "192.168.2.1".to_string(),
            "2001:4860:4860::8888".to_string(),
        ]
    );
    assert_eq!(interface.dhcp_server.as_deref(), Some("192.168.2.1"));
    assert_eq!(
        interface.broadcast_address.as_deref(),
        Some("192.168.2.255")
    );
    assert_eq!(interface.ipv4_cidr.as_deref(), Some("192.168.2.42/24"));
    assert_eq!(interface.ipv6_prefixes, vec!["2001:db8::42/64".to_string()]);
    assert!(interface.is_up);
    assert!(interface.is_connected);
    assert!(!interface.is_loopback);
    assert!(!interface.is_link_local_only);
    assert_eq!(
        interface.classification,
        LocalNetworkInterfaceClassification::Physical
    );
    assert_eq!(interface.wifi_ssid.as_deref(), Some("Home"));
    assert_eq!(interface.wifi_signal_percent, Some(87));
    assert!(interface.has_default_route);
    assert_eq!(
        map.recommended_interface_id.as_deref(),
        Some("mac:54-27-1e-97-c3-31")
    );
}

#[test]
fn linux_interface_map_merges_records_without_losing_addresses_or_state() {
    let map = linux_local_network_interface_map(
        &[json!({
            "dst": "default",
            "gateway": "192.168.2.1",
            "dev": "enp0s31f6"
        })],
        &[
            json!({
                "ifname": "enp0s31f6",
                "ifindex": 2,
                "address": "54:27:1e:97:c3:31",
                "operstate": "UP",
                "addr_info": [{
                    "family": "inet",
                    "local": "192.168.2.42",
                    "prefixlen": 24,
                    "scope": "global"
                }]
            }),
            json!({
                "ifname": "enp0s31f6",
                "description": "Intel Ethernet",
                "addr_info": [{
                    "family": "inet",
                    "local": "192.168.2.43",
                    "prefixlen": 24,
                    "scope": "global"
                }]
            }),
        ],
        &["192.168.2.1".to_string()],
    );

    assert_eq!(map.interfaces.len(), 1);
    let interface = &map.interfaces[0];
    assert_eq!(interface.name, "enp0s31f6");
    assert_eq!(interface.id, "mac:54-27-1e-97-c3-31");
    assert_eq!(interface.description.as_deref(), Some("Intel Ethernet"));
    assert_eq!(interface.index, Some(2));
    assert_eq!(
        interface.ip_addresses,
        vec!["192.168.2.42".to_string(), "192.168.2.43".to_string()]
    );
    assert!(interface.is_up);
    assert!(interface.is_connected);
    assert!(interface.has_default_route);
    assert_eq!(
        map.recommended_interface_id.as_deref(),
        Some(interface.id.as_str())
    );
}

#[test]
fn interface_map_marks_down_disconnected_loopback_and_non_physical_classes() {
    let map = windows_local_network_interface_map(&[
        json!({
            "InterfaceAlias": "Ethernet Down",
            "IPAddress": "192.168.2.20",
            "PrefixLength": 24,
            "Status": "Down",
            "InterfaceType": "Ethernet"
        }),
        json!({
            "InterfaceAlias": "Wi-Fi Disconnected",
            "IPAddress": "192.168.2.21",
            "PrefixLength": 24,
            "Status": "Disconnected",
            "InterfaceType": "Wireless"
        }),
        json!({
            "InterfaceAlias": "Loopback",
            "IPAddress": "127.0.0.1",
            "Status": "Up",
            "IsLoopback": true,
            "InterfaceType": "Loopback"
        }),
        json!({
            "InterfaceAlias": "Ethernet Link Local",
            "IPAddress": "169.254.10.20",
            "PrefixLength": 16,
            "Status": "Up",
            "InterfaceType": "Ethernet"
        }),
        json!({
            "InterfaceAlias": "docker0",
            "IPAddress": "10.0.0.1",
            "PrefixLength": 24,
            "Status": "Up"
        }),
        json!({
            "InterfaceAlias": "tailscale0",
            "IPAddress": "100.64.0.1",
            "PrefixLength": 10,
            "Status": "Up"
        }),
        json!({
            "InterfaceAlias": "vEthernet (Default Switch)",
            "IPAddress": "172.20.0.1",
            "PrefixLength": 20,
            "Status": "Up"
        }),
    ]);

    let find = |name: &str| {
        map.interfaces
            .iter()
            .find(|interface| interface.name == name)
    };
    assert_eq!(
        find("Ethernet Down").map(|interface| interface.ignored_reason),
        Some(Some(LocalNetworkInterfaceIgnoreReason::Down))
    );
    assert_eq!(
        find("Ethernet Down").map(|interface| interface.classification),
        Some(LocalNetworkInterfaceClassification::Physical)
    );
    assert_eq!(
        find("Wi-Fi Disconnected").map(|interface| interface.ignored_reason),
        Some(Some(LocalNetworkInterfaceIgnoreReason::Disconnected))
    );
    assert_eq!(
        find("Loopback").map(|interface| interface.ignored_reason),
        Some(Some(LocalNetworkInterfaceIgnoreReason::Loopback))
    );
    assert_eq!(
        find("Loopback").map(|interface| interface.classification),
        Some(LocalNetworkInterfaceClassification::Loopback)
    );
    assert_eq!(
        find("Ethernet Link Local").map(|interface| interface.is_link_local_only),
        Some(true)
    );
    assert_eq!(
        find("Ethernet Link Local").map(|interface| interface.ignored_reason),
        Some(Some(LocalNetworkInterfaceIgnoreReason::LinkLocalOnly))
    );
    assert_eq!(
        find("Ethernet Link Local").map(|interface| interface.classification),
        Some(LocalNetworkInterfaceClassification::LinkLocalOnly)
    );
    assert_eq!(
        find("docker0").map(|interface| interface.classification),
        Some(LocalNetworkInterfaceClassification::Container)
    );
    assert_eq!(
        find("tailscale0").map(|interface| interface.classification),
        Some(LocalNetworkInterfaceClassification::VpnOrTunnel)
    );
    assert_eq!(
        find("vEthernet (Default Switch)").map(|interface| interface.classification),
        Some(LocalNetworkInterfaceClassification::Virtual)
    );
    assert_eq!(map.recommended_interface_id, None);
}

#[test]
fn manual_interface_selection_can_override_default_without_invalid_fallback() {
    let map = windows_local_network_interface_map(&[
        json!({
            "InterfaceAlias": "Ethernet",
            "IPAddress": "192.168.2.42",
            "PrefixLength": 24,
            "DefaultGateway": "192.168.2.1",
            "Status": "Up",
            "InterfaceType": "Ethernet"
        }),
        json!({
            "InterfaceAlias": "vEthernet (WSL)",
            "MacAddress": "00-15-5d-11-22-33",
            "IPAddress": "172.26.32.1",
            "PrefixLength": 20,
            "Status": "Up",
            "InterfaceType": "Virtual"
        }),
    ]);

    let manual = map.select_interface(Some("vEthernet (WSL)"));
    assert_eq!(
        manual.map(|interface| interface.classification),
        Some(LocalNetworkInterfaceClassification::Wsl)
    );
    assert_eq!(
        manual.map(|interface| interface.ignored_reason),
        Some(Some(LocalNetworkInterfaceIgnoreReason::Wsl))
    );
    assert_eq!(
        manual.map(|interface| interface.ip_addresses.clone()),
        Some(vec!["172.26.32.1".to_string()])
    );
    assert_eq!(
        map.selected_identity(Some("vEthernet (WSL)"))
            .and_then(|identity| identity.ip_address),
        Some("172.26.32.1".to_string())
    );
    assert_eq!(map.select_interface(Some("missing-interface")), None);
    assert_eq!(
        map.select_interface(None)
            .map(|interface| interface.name.as_str()),
        Some("Ethernet")
    );
}

#[test]
fn stable_interface_id_prefers_mac_then_index_then_name() {
    assert_eq!(
        stable_interface_id("Ethernet", Some(12), Some("AA-BB-CC-DD-EE-FF")),
        Some("mac:aa-bb-cc-dd-ee-ff".to_string())
    );
    assert_eq!(
        stable_interface_id("Ethernet", Some(12), None),
        Some("index:12".to_string())
    );
    assert_eq!(
        stable_interface_id(" Wi-Fi ", None, None),
        Some("name:wi-fi".to_string())
    );
    assert_eq!(stable_interface_id("", Some(0), None), None);
}

#[test]
fn unknown_state_is_fail_closed_but_observed_partial_merge_becomes_eligible() {
    let unknown_map = windows_local_network_interface_map(&[json!({
        "InterfaceAlias": "Ethernet",
        "InterfaceIndex": 12,
        "IPAddress": "192.168.2.42",
        "PrefixLength": 24
    })]);

    let unknown = &unknown_map.interfaces[0];
    assert!(unknown.is_up);
    assert!(unknown.is_connected);
    assert!(!unknown.state_observed);
    assert!(!unknown.is_eligible_by_default());
    assert_eq!(unknown_map.recommended_interface_id.as_deref(), None);

    let merged_map = windows_local_network_interface_map(&[
        json!({
            "InterfaceAlias": "Ethernet",
            "InterfaceIndex": 12,
            "IPAddress": "192.168.2.42",
            "PrefixLength": 24
        }),
        json!({
            "InterfaceAlias": "Ethernet",
            "InterfaceIndex": 12,
            "IPAddress": "192.168.2.43",
            "PrefixLength": 24,
            "Status": "Up"
        }),
    ]);

    let merged = &merged_map.interfaces[0];
    assert!(merged.state_observed);
    assert!(merged.is_up);
    assert!(merged.is_connected);
    assert!(merged.is_eligible_by_default());
    assert_eq!(
        merged_map.recommended_interface_id.as_deref(),
        Some(merged.id.as_str())
    );
}

#[test]
fn windows_media_connect_state_distinguishes_connected_disconnected_and_unknown() {
    let connected = windows_local_network_interface_map(&[json!({
        "InterfaceAlias": "Ethernet",
        "IPAddress": "192.168.2.42",
        "PrefixLength": 24,
        "MediaConnectState": 1
    })]);
    assert!(connected.interfaces[0].state_observed);
    assert!(connected.interfaces[0].is_up);
    assert!(connected.interfaces[0].is_connected);
    assert!(connected.interfaces[0].is_eligible_by_default());

    let net_connected = windows_local_network_interface_map(&[json!({
        "InterfaceAlias": "Ethernet",
        "IPAddress": "192.168.2.42",
        "PrefixLength": 24,
        "NetConnectionStatus": 2
    })]);
    assert!(net_connected.interfaces[0].state_observed);
    assert!(net_connected.interfaces[0].is_up);
    assert!(net_connected.interfaces[0].is_connected);

    let disconnected = windows_local_network_interface_map(&[json!({
        "InterfaceAlias": "Ethernet",
        "IPAddress": "192.168.2.42",
        "PrefixLength": 24,
        "MediaConnectState": 2
    })]);
    assert!(disconnected.interfaces[0].state_observed);
    assert!(!disconnected.interfaces[0].is_up);
    assert!(!disconnected.interfaces[0].is_connected);
    assert!(!disconnected.interfaces[0].is_eligible_by_default());

    let unknown = windows_local_network_interface_map(&[json!({
        "InterfaceAlias": "Ethernet",
        "IPAddress": "192.168.2.42",
        "PrefixLength": 24,
        "MediaConnectState": 0
    })]);
    assert!(!unknown.interfaces[0].state_observed);
    assert!(!unknown.interfaces[0].is_eligible_by_default());
}

#[test]
fn interface_merge_upgrades_index_identity_to_mac_identity() {
    let map = windows_local_network_interface_map(&[
        json!({
            "InterfaceAlias": "Ethernet",
            "InterfaceIndex": 12,
            "IPAddress": "192.168.2.42",
            "PrefixLength": 24,
            "Status": "Up"
        }),
        json!({
            "InterfaceAlias": "Ethernet",
            "InterfaceIndex": 12,
            "MacAddress": "54-27-1e-97-c3-31",
            "IPAddress": "192.168.2.43",
            "PrefixLength": 24,
            "Status": "Up"
        }),
    ]);

    assert_eq!(map.interfaces.len(), 1);
    assert_eq!(map.interfaces[0].id, "mac:54-27-1e-97-c3-31");
    assert_eq!(
        map.interfaces[0].ip_addresses,
        vec!["192.168.2.42".to_string(), "192.168.2.43".to_string()]
    );
    assert_eq!(
        map.recommended_interface_id.as_deref(),
        Some("mac:54-27-1e-97-c3-31")
    );
}

#[test]
fn interface_address_parser_rejects_invalid_prefixes_and_family_mismatches() {
    let invalid_windows = windows_local_network_interface_map(&[json!({
        "InterfaceAlias": "Ethernet",
        "IPAddress": [
            "192.168.2.42/33",
            "2001:db8::42/129",
            "2001:db8::43/not-a-prefix"
        ],
        "Status": "Up"
    })]);

    assert_eq!(invalid_windows.interfaces.len(), 1);
    assert!(invalid_windows.interfaces[0].ip_addresses.is_empty());
    assert_eq!(invalid_windows.recommended_interface_id, None);

    let malformed_windows = windows_local_network_interface_map(&[json!({
        "InterfaceAlias": "Ethernet",
        "IPAddress": "192.168.2.42",
        "PrefixLength": "not-a-prefix",
        "Status": "Up"
    })]);
    assert!(malformed_windows.interfaces.is_empty());

    let invalid_linux = linux_local_network_interface_map(
        &[],
        &[json!({
            "ifname": "eth0",
            "operstate": "UP",
            "addr_info": [
                {
                    "family": "inet",
                    "local": "2001:db8::42",
                    "prefixlen": 24
                },
                {
                    "family": "inet6",
                    "local": "192.168.2.42",
                    "prefixlen": 24
                },
                {
                    "family": "inet6",
                    "local": "2001:db8::43",
                    "prefixlen": 129
                }
            ]
        })],
        &[],
    );

    assert_eq!(invalid_linux.interfaces.len(), 1);
    assert!(invalid_linux.interfaces[0].ip_addresses.is_empty());
    assert_eq!(invalid_linux.recommended_interface_id, None);
}

#[test]
fn interface_merge_retains_conservative_non_physical_classification() {
    let map = windows_local_network_interface_map(&[
        json!({
            "InterfaceAlias": "Ethernet",
            "InterfaceIndex": 12,
            "IPAddress": "192.168.2.42",
            "PrefixLength": 24,
            "Status": "Up",
            "InterfaceType": "Ethernet"
        }),
        json!({
            "InterfaceAlias": "Ethernet",
            "InterfaceIndex": 12,
            "IPAddress": "192.168.2.43",
            "PrefixLength": 24,
            "Status": "Up",
            "InterfaceType": "Virtual"
        }),
    ]);

    assert_eq!(map.interfaces.len(), 1);
    assert_eq!(
        map.interfaces[0].classification,
        LocalNetworkInterfaceClassification::Virtual
    );
    assert_eq!(map.recommended_interface_id, None);
}

#[test]
fn preferred_identity_skips_recommended_ipv6_only_interface_for_ipv4() {
    let route_records = Vec::new();
    let address_records = vec![
        json!({
            "ifname": "enp6s0",
            "operstate": "UP",
            "addr_info": [{
                "family": "inet6",
                "local": "2001:db8::6",
                "prefixlen": 64,
                "scope": "global"
            }]
        }),
        json!({
            "ifname": "enp7s0",
            "operstate": "UP",
            "addr_info": [{
                "family": "inet",
                "local": "192.168.2.77",
                "prefixlen": 24,
                "scope": "global"
            }]
        }),
    ];
    let map = linux_local_network_interface_map(&route_records, &address_records, &[]);

    assert_eq!(map.interfaces.len(), 2);
    assert_eq!(map.recommended_interface_id.as_deref(), Some("name:enp6s0"));
    assert_eq!(
        preferred_linux_local_network_identity(&route_records, &address_records, &[])
            .and_then(|identity| identity.network_interface),
        Some("enp7s0".to_string())
    );
}
