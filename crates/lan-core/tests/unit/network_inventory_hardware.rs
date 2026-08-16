use serde_json::json;

use ocentra_lan_core::network_inventory_hardware::{
    linux_identity::{
        linux_dns_servers_from_resolv_conf_text, preferred_linux_local_network_identity,
        preferred_windows_local_network_identity,
    },
    network_identity_support::{ignored_interface_reason, LocalNetworkInterfaceIgnoreReason},
    LocalNetworkIdentity,
};

#[test]
fn windows_prefers_default_gateway_interface_and_skips_virtual_candidates() {
    let identity = preferred_windows_local_network_identity(&[
        json!({
            "IPAddress": "172.26.32.1",
            "PrefixLength": 20,
            "InterfaceAlias": "vEthernet (WSL)",
            "MacAddress": "00-15-5d-11-22-33",
            "DefaultGateway": "172.26.32.1"
        }),
        json!({
            "IPAddress": "192.168.2.42",
            "PrefixLength": 24,
            "InterfaceAlias": "Ethernet 2",
            "MacAddress": "54-27-1e-97-c3-31",
            "DefaultGateway": "192.168.2.1",
            "DnsServers": ["192.168.2.1", "1.1.1.1"],
            "DhcpServer": "192.168.2.1",
            "Ipv6Prefixes": ["2001:db8::42/64", "fe80::42/64"]
        }),
        json!({
            "IPAddress": "192.168.2.77",
            "PrefixLength": 24,
            "InterfaceAlias": "Wi-Fi",
            "MacAddress": "aa-bb-cc-dd-ee-ff"
        }),
    ]);

    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.network_interface.as_deref()),
        Some(Some("Ethernet 2"))
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.default_gateway.as_deref()),
        Some(Some("192.168.2.1"))
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.wifi_ssid.as_deref()),
        Some(None)
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.ipv4_cidr.as_deref()),
        Some(Some("192.168.2.42/24"))
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.broadcast_address.as_deref()),
        Some(Some("192.168.2.255"))
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.dns_servers.clone()),
        Some(vec!["192.168.2.1".to_string(), "1.1.1.1".to_string()])
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.dhcp_server.as_deref()),
        Some(Some("192.168.2.1"))
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.ipv6_prefixes.clone()),
        Some(vec!["2001:db8::42/64".to_string()])
    );
}

#[test]
fn linux_prefers_default_route_interface_and_captures_gateway_and_cidr() {
    let identity = preferred_linux_local_network_identity(
        &[json!({
            "dst": "default",
            "gateway": "192.168.2.1",
            "dev": "wlp0s20f3"
        })],
        &[
            json!({
                "ifname": "docker0",
                "address": "02:42:0a:9d:00:01",
                "addr_info": [{
                    "family": "inet",
                    "local": "10.157.0.1",
                    "prefixlen": 16,
                    "scope": "global"
                }]
            }),
            json!({
                "ifname": "eth0",
                "address": "10:20:30:40:50:60",
                "addr_info": [{
                    "family": "inet",
                    "local": "192.168.2.24",
                    "prefixlen": 24,
                    "scope": "global"
                }]
            }),
            json!({
                "ifname": "wlp0s20f3",
                "address": "54:27:1e:97:c3:31",
                "addr_info": [
                    {
                        "family": "inet",
                        "local": "192.168.2.42",
                        "prefixlen": 24,
                        "scope": "global"
                    },
                    {
                        "family": "inet6",
                        "local": "fe80::1234",
                        "prefixlen": 64,
                        "scope": "link"
                    },
                    {
                        "family": "inet6",
                        "local": "2001:db8::42",
                        "prefixlen": 64,
                        "scope": "global"
                    }
                ]
            }),
        ],
        &[
            "192.168.2.1".to_string(),
            "2001:4860:4860::8888".to_string(),
        ],
    );

    assert_eq!(
        identity,
        Some(LocalNetworkIdentity {
            ip_address: Some("192.168.2.42".to_string()),
            mac_address: Some("54-27-1e-97-c3-31".to_string()),
            network_interface: Some("wlp0s20f3".to_string()),
            wifi_ssid: None,
            default_gateway: Some("192.168.2.1".to_string()),
            ipv4_cidr: Some("192.168.2.42/24".to_string()),
            dns_servers: vec![
                "192.168.2.1".to_string(),
                "2001:4860:4860::8888".to_string(),
            ],
            dhcp_server: None,
            broadcast_address: Some("192.168.2.255".to_string()),
            ipv6_prefixes: vec!["2001:db8::42/64".to_string()],
        })
    );
}

#[test]
fn linux_skips_link_local_only_and_falls_back_to_first_viable_interface() {
    let identity = preferred_linux_local_network_identity(
        &[],
        &[
            json!({
                "ifname": "enp0s31f6",
                "address": "10:20:30:40:50:60",
                "addr_info": [{
                    "family": "inet",
                    "local": "169.254.10.20",
                    "prefixlen": 16,
                    "scope": "link"
                }]
            }),
            json!({
                "ifname": "wlp0s20f3",
                "address": "54:27:1e:97:c3:31",
                "addr_info": [{
                    "family": "inet",
                    "local": "192.168.2.88",
                    "prefixlen": 24,
                    "scope": "global"
                }]
            }),
        ],
        &[],
    );

    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.network_interface.as_deref()),
        Some(Some("wlp0s20f3"))
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.wifi_ssid.as_deref()),
        Some(None)
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.default_gateway.as_deref()),
        Some(None)
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.ipv4_cidr.as_deref()),
        Some(Some("192.168.2.88/24"))
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.broadcast_address.as_deref()),
        Some(Some("192.168.2.255"))
    );
    assert_eq!(
        identity.as_ref().map(|identity| identity.dns_servers.len()),
        Some(0)
    );
    assert_eq!(
        identity
            .as_ref()
            .map(|identity| identity.ipv6_prefixes.len()),
        Some(0)
    );
}

#[test]
fn linux_resolv_conf_parser_filters_loopback_nameservers_and_keeps_real_servers() {
    let dns_servers = linux_dns_servers_from_resolv_conf_text(
        "\
# generated by systemd-resolved
nameserver 127.0.0.53
nameserver 192.168.2.1
search lan
nameserver 2001:4860:4860::8888
nameserver invalid
",
    );

    assert_eq!(
        dns_servers,
        vec![
            "192.168.2.1".to_string(),
            "2001:4860:4860::8888".to_string(),
        ]
    );
}

#[test]
fn interface_ignore_reasons_are_explicit_for_non_household_links() {
    assert_eq!(
        ignored_interface_reason(""),
        Some(LocalNetworkInterfaceIgnoreReason::EmptyName)
    );
    assert_eq!(
        ignored_interface_reason("lo"),
        Some(LocalNetworkInterfaceIgnoreReason::Loopback)
    );
    assert_eq!(
        ignored_interface_reason("vEthernet (WSL)"),
        Some(LocalNetworkInterfaceIgnoreReason::VirtualEthernet)
    );
    assert_eq!(
        ignored_interface_reason("docker0"),
        Some(LocalNetworkInterfaceIgnoreReason::ContainerBridge)
    );
    assert_eq!(
        ignored_interface_reason("br-1aa2bb3cc4dd"),
        Some(LocalNetworkInterfaceIgnoreReason::ContainerBridge)
    );
    assert_eq!(
        ignored_interface_reason("vboxnet0"),
        Some(LocalNetworkInterfaceIgnoreReason::VirtualMachineBridge)
    );
    assert_eq!(
        ignored_interface_reason("tailscale0"),
        Some(LocalNetworkInterfaceIgnoreReason::VpnOrTunnel)
    );
    assert_eq!(
        ignored_interface_reason("ztmj4yq1"),
        Some(LocalNetworkInterfaceIgnoreReason::ZeroTier)
    );
    assert_eq!(ignored_interface_reason("wlp0s20f3"), None);
    assert_eq!(ignored_interface_reason("Ethernet 2"), None);
}

#[test]
fn windows_identity_keeps_wifi_ssid_only_for_wireless_aliases() {
    let wireless_identity = preferred_windows_local_network_identity(&[json!({
        "IPAddress": "192.168.2.77",
        "PrefixLength": 24,
        "InterfaceAlias": "Wi-Fi",
        "MacAddress": "aa-bb-cc-dd-ee-ff",
        "WifiSsid": "Home-Wifi"
    })]);

    assert_eq!(
        wireless_identity
            .as_ref()
            .map(|identity| identity.wifi_ssid.as_deref()),
        Some(Some("Home-Wifi"))
    );

    let wired_identity = preferred_windows_local_network_identity(&[json!({
        "IPAddress": "192.168.2.42",
        "PrefixLength": 24,
        "InterfaceAlias": "Ethernet 2",
        "MacAddress": "54-27-1e-97-c3-31",
        "WifiSsid": "Should-Not-Survive"
    })]);

    assert_eq!(
        wired_identity
            .as_ref()
            .map(|identity| identity.wifi_ssid.as_deref()),
        Some(None)
    );
}
