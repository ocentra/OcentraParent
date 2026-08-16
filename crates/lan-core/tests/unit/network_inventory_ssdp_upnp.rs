use crate::support::{OptionTestExt as _, ResultTestExt as _};
use std::io::{Read, Write};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use ocentra_lan_core::network_inventory::ssdp_upnp::*;
use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;

fn spawn_http_server(
    response_body: Vec<u8>,
    expected_requests: usize,
) -> (std::net::SocketAddr, thread::JoinHandle<()>) {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).value_or_unreachable();
    let addr = listener.local_addr().value_or_unreachable();
    let handle = thread::spawn(move || {
        for _ in 0..expected_requests {
            let (mut stream, _) = listener.accept().value_or_unreachable();
            let mut request = [0_u8; 1024];
            let _ = stream.read(&mut request);
            let response_headers = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/xml\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                ""
            );
            stream
                .write_all(response_headers.as_bytes())
                .value_or_unreachable();
            stream.write_all(&response_body).value_or_unreachable();
        }
    });
    (addr, handle)
}

fn spawn_http_server_that_times_out() -> (std::net::SocketAddr, thread::JoinHandle<()>) {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).value_or_unreachable();
    let addr = listener.local_addr().value_or_unreachable();
    let handle = thread::spawn(move || {
        let (_stream, _) = listener.accept().value_or_unreachable();
        thread::sleep(Duration::from_millis(200));
    });
    (addr, handle)
}

fn spawn_udp_ssdp_responder(
    response: Vec<u8>,
) -> (std::net::SocketAddr, thread::JoinHandle<Vec<u8>>) {
    let socket = UdpSocket::bind(("127.0.0.1", 0)).value_or_unreachable();
    let addr = socket.local_addr().value_or_unreachable();
    let handle = thread::spawn(move || {
        let mut buf = [0_u8; 2048];
        let (size, source) = socket.recv_from(&mut buf).value_or_unreachable();
        let request = buf[..size].to_vec();
        socket.send_to(&response, source).value_or_unreachable();
        request
    });
    (addr, handle)
}

fn spawn_udp_ssdp_responder_sequence(
    responses: Vec<Vec<u8>>,
) -> (std::net::SocketAddr, thread::JoinHandle<Vec<Vec<u8>>>) {
    let socket = UdpSocket::bind(("127.0.0.1", 0)).value_or_unreachable();
    let addr = socket.local_addr().value_or_unreachable();
    let handle = thread::spawn(move || {
        let mut requests = Vec::new();
        for response in responses {
            let mut buf = [0_u8; 2048];
            let (size, source) = socket.recv_from(&mut buf).value_or_unreachable();
            requests.push(buf[..size].to_vec());
            socket.send_to(&response, source).value_or_unreachable();
        }
        requests
    });
    (addr, handle)
}

fn sample_description_xml(
    name: impl std::fmt::Display,
    device_type: impl std::fmt::Display,
    sdn: impl std::fmt::Display,
    manufacturer: impl std::fmt::Display,
    model_name: impl std::fmt::Display,
) -> Vec<u8> {
    format!(
        "<?xml version=\"1.0\"?>\n<root>\n  <device>\n    <friendlyName>{name}</friendlyName>\n    <manufacturer>{manufacturer}</manufacturer>\n    <modelName>{model_name}</modelName>\n    <deviceType>{device_type}</deviceType>\n    <UDN>{sdn}</UDN>\n  </device>\n</root>"
    )
    .into_bytes()
}

fn ssdp_response(
    location: impl std::fmt::Display,
    search_target: impl std::fmt::Display,
    usn: impl std::fmt::Display,
) -> Vec<u8> {
    format!(
        "HTTP/1.1 200 OK\r\nCACHE-CONTROL: max-age=1800\r\nLOCATION:  {location} \r\nST:  {search_target}\t\r\nUSN: {usn}\r\nSERVER: Linux/5.15 UPnP/1.0 Acme/1.0\r\n\r\n"
    )
    .into_bytes()
}

struct SsdpFixture<'a> {
    name: &'a str,
    friendly_name: &'a str,
    description_device_type: &'a str,
    response_device_type: &'a str,
    usn: &'a str,
    expected_platform: &'a str,
    infrastructure: bool,
    enrollable: bool,
}

fn ssdp_fixture_cases() -> [SsdpFixture<'static>; 4] {
    [
        SsdpFixture {
            name: "tv",
            friendly_name: "Living Room TV",
            description_device_type: "urn:schemas-upnp-org:device:MediaRenderer:1",
            response_device_type: "urn:schemas-upnp-org:device:MediaRenderer:1",
            usn: "uuid:tv-1::urn:schemas-upnp-org:device:MediaRenderer:1",
            expected_platform: "MediaRenderer",
            infrastructure: false,
            enrollable: true,
        },
        SsdpFixture {
            name: "roster",
            friendly_name: "Home Roster",
            description_device_type: "urn:schemas-upnp-org:device:InternetGatewayDevice:1",
            response_device_type: "urn:schemas-upnp-org:device:InternetGatewayDevice:1",
            usn: "uuid:roster-1::urn:schemas-upnp-org:device:InternetGatewayDevice:1",
            expected_platform: constants::lan_pairing::PLATFORM_ROUTER,
            infrastructure: true,
            enrollable: false,
        },
        SsdpFixture {
            name: "console",
            friendly_name: "Game Console",
            description_device_type: "urn:schemas-upnp-org:device:GameConsole:1",
            response_device_type: "urn:schemas-upnp-org:device:GameConsole:1",
            usn: "uuid:console-1::urn:schemas-upnp-org:device:GameConsole:1",
            expected_platform: "GameConsole",
            infrastructure: false,
            enrollable: true,
        },
        SsdpFixture {
            name: "printer",
            friendly_name: "Office Printer",
            description_device_type: "urn:schemas-upnp-org:device:Printer:1",
            response_device_type: "urn:schemas-upnp-org:device:Printer:1",
            usn: "uuid:printer-1::urn:schemas-upnp-org:device:Printer:1",
            expected_platform: "Printer",
            infrastructure: false,
            enrollable: true,
        },
    ]
}

fn assert_ssdp_record_fixture(
    record: &SsdpDiscoveryRecord,
    location: impl std::fmt::Display,
    fixture: &SsdpFixture<'_>,
) {
    let location = location.to_string();
    assert_eq!(record.response.location, location);
    assert_eq!(record.response.search_target, fixture.response_device_type);
    assert_eq!(record.response.usn, fixture.usn);
    assert_eq!(
        record.response.udn.as_deref(),
        Some(
            fixture
                .usn
                .trim_start_matches("uuid:")
                .split("::")
                .next()
                .value_or_unreachable()
        )
    );
    assert_eq!(
        record.response.device_type.as_deref(),
        Some(fixture.response_device_type)
    );
    assert_eq!(record.response.infrastructure, fixture.infrastructure);
    assert_eq!(record.response.enrollable, fixture.enrollable);

    let description = record.description.as_ref().value_or_unreachable();
    let expected_model_name = format!("{}-1000", fixture.name.to_uppercase());
    assert_eq!(description.friendly_name, fixture.friendly_name);
    assert_eq!(description.manufacturer.as_deref(), Some("Acme"));
    assert_eq!(
        description.model_name.as_deref(),
        Some(expected_model_name.as_str())
    );
    assert_eq!(
        description.device_type.as_deref(),
        Some(fixture.description_device_type)
    );
    assert_eq!(
        description.udn.as_deref(),
        Some(
            fixture
                .usn
                .trim_start_matches("uuid:")
                .split("::")
                .next()
                .value_or_unreachable()
        )
    );
    assert_eq!(description.description_url, location);
}

fn assert_ssdp_enriched_device(device: &LanNetworkInventoryDevice, fixture: &SsdpFixture<'_>) {
    assert_eq!(device.platform, fixture.expected_platform);
    assert_eq!(device.ip_address, "127.0.0.1");
    assert_eq!(device.reachability, LanPairingDeviceReachability::Online);
    assert_eq!(
        device.scan_sources,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP.to_string(),
        ]
    );
}

#[test]
fn discovery_covers_tv_roster_console_and_printer_fixtures() {
    for fixture in ssdp_fixture_cases() {
        let xml = sample_description_xml(
            fixture.friendly_name,
            fixture.description_device_type,
            fixture.usn,
            "Acme",
            format!("{}-1000", fixture.name.to_uppercase()),
        );
        let (http_addr, http_handle) = spawn_http_server(xml, 2);
        let location = format!("http://127.0.0.1:{}/{}.xml", http_addr.port(), fixture.name);
        let (udp_addr, udp_handle) = spawn_udp_ssdp_responder(ssdp_response(
            &location,
            fixture.response_device_type,
            fixture.usn,
        ));

        let records = discover_ssdp_upnp_devices(
            "ssdp:all",
            udp_addr,
            Duration::from_millis(250),
            1,
            Duration::from_millis(250),
        )
        .value_or_unreachable();

        let request = udp_handle.join().value_or_unreachable();
        let request_text = String::from_utf8(request).value_or_unreachable();
        assert_eq!(
            request_text,
            format!(
                "M-SEARCH * HTTP/1.1\r\nHOST: {udp_addr}\r\nMAN: \"ssdp:discover\"\r\nMX: 1\r\nST: ssdp:all\r\nUSER-AGENT: ocentra-parent/lan-core\r\nCONNECTION: close\r\n\r\n"
            )
        );

        assert_eq!(records.len(), 1);
        assert_ssdp_record_fixture(&records[0], &location, &fixture);

        let (enrich_udp_addr, enrich_udp_handle) = spawn_udp_ssdp_responder(ssdp_response(
            &location,
            fixture.response_device_type,
            fixture.usn,
        ));
        let mut devices = vec![LanNetworkInventoryDevice {
            device_id: "lan-device-legacy".to_string(),
            label: "LAN 127.0.0.1".to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: "127.0.0.1".to_string(),
            mac_address: String::new(),
            hostname: None,
            network_interface: None,
            reachability: LanPairingDeviceReachability::Stale,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            observed_at: String::new(),
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        }];

        enrich_ssdp_upnp_devices_for_target(&mut devices, enrich_udp_addr);

        let enrich_request = enrich_udp_handle.join().value_or_unreachable();
        let enrich_request_text = String::from_utf8(enrich_request).value_or_unreachable();
        assert_eq!(
            enrich_request_text,
            format!(
                "M-SEARCH * HTTP/1.1\r\nHOST: {enrich_udp_addr}\r\nMAN: \"ssdp:discover\"\r\nMX: 1\r\nST: ssdp:all\r\nUSER-AGENT: ocentra-parent/lan-core\r\nCONNECTION: close\r\n\r\n"
            )
        );

        assert_eq!(devices.len(), 1);
        assert_ssdp_enriched_device(&devices[0], &fixture);

        http_handle.join().value_or_unreachable();
    }
}

#[test]
fn enrich_adds_ssdp_only_devices_as_agentless_hints() {
    let xml = sample_description_xml(
        "Living Room TV",
        "urn:schemas-upnp-org:device:MediaRenderer:1",
        "uuid:tv-1::urn:schemas-upnp-org:device:MediaRenderer:1",
        "Acme",
        "TV-1000",
    );
    let (http_addr, http_handle) = spawn_http_server(xml, 1);
    let location = format!("http://127.0.0.1:{}/tv.xml", http_addr.port());
    let (udp_addr, udp_handle) = spawn_udp_ssdp_responder(ssdp_response(
        &location,
        "urn:schemas-upnp-org:device:MediaRenderer:1",
        "uuid:tv-1::urn:schemas-upnp-org:device:MediaRenderer:1",
    ));

    let mut devices = Vec::new();
    enrich_ssdp_upnp_devices_for_target(&mut devices, udp_addr);

    let request = udp_handle.join().value_or_unreachable();
    let request_text = String::from_utf8(request).value_or_unreachable();
    assert_eq!(
        request_text,
        format!(
            "M-SEARCH * HTTP/1.1\r\nHOST: {udp_addr}\r\nMAN: \"ssdp:discover\"\r\nMX: 1\r\nST: ssdp:all\r\nUSER-AGENT: ocentra-parent/lan-core\r\nCONNECTION: close\r\n\r\n"
        )
    );
    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(device.device_id, "tv-1");
    assert_eq!(device.label, "Living Room TV");
    assert_eq!(device.platform, "MediaRenderer");
    assert_eq!(device.ip_address, "127.0.0.1");
    assert!(device.mac_address.is_empty());
    assert_eq!(device.hostname, None);
    assert_eq!(device.agent_status, None);
    assert_eq!(device.reachability, LanPairingDeviceReachability::Online);
    assert_eq!(
        device.scan_sources,
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP.to_string()]
    );

    http_handle.join().value_or_unreachable();
}

#[test]
fn ssdp_response_parser_normalizes_header_values_and_sdn_prefixes() {
    let response = b"HTTP/1.1 200 OK\r\nLoCaTiOn:  http://127.0.0.1:1900/device.xml \r\nsT:  urn:schemas-upnp-org:device:MediaRenderer:1\t\r\nuSn:  urn:uuid:device-1::urn:schemas-upnp-org:device:MediaRenderer:1  \r\n\r\n";

    let parsed = parse_ssdp_response(response).value_or_unreachable();
    assert_eq!(parsed.location, "http://127.0.0.1:1900/device.xml");
    assert_eq!(
        parsed.search_target,
        "urn:schemas-upnp-org:device:MediaRenderer:1"
    );
    assert_eq!(
        parsed.usn,
        "urn:uuid:device-1::urn:schemas-upnp-org:device:MediaRenderer:1"
    );
    assert_eq!(parsed.udn.as_deref(), Some("device-1"));
    assert_eq!(
        parsed.device_type.as_deref(),
        Some("urn:schemas-upnp-org:device:MediaRenderer:1")
    );
}

#[test]
fn msearch_request_clamps_mx_and_normalizes_search_target() {
    let request = build_msearch_request(
        " \r\nssdp:all\t",
        "239.255.255.250:1900".parse().value_or_unreachable(),
        99,
    );
    let request_text = String::from_utf8(request).value_or_unreachable();

    assert_eq!(
        request_text,
        "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: 5\r\nST: ssdp:all\r\nUSER-AGENT: ocentra-parent/lan-core\r\nCONNECTION: close\r\n\r\n"
            .to_string()
    );
}

#[test]
fn roster_ssdp_response_is_visible_but_non_enrollable() {
    let xml = sample_description_xml(
        "Home Roster",
        "urn:schemas-upnp-org:device:InternetGatewayDevice:1",
        "uuid:roster-1",
        "RosterCo",
        "RTR-1",
    );
    let (http_addr, http_handle) = spawn_http_server(xml, 1);
    let location = format!("http://127.0.0.1:{}/roster.xml", http_addr.port());
    let response = ssdp_response(
        &location,
        "urn:schemas-upnp-org:device:InternetGatewayDevice:1",
        "uuid:roster-1::urn:schemas-upnp-org:device:InternetGatewayDevice:1",
    );
    let parsed = parse_ssdp_response(&response).value_or_unreachable();

    assert_eq!(parsed.location, location);
    assert!(parsed.infrastructure);
    assert!(!parsed.enrollable);
    assert_eq!(
        parsed.device_type.as_deref(),
        Some("urn:schemas-upnp-org:device:InternetGatewayDevice:1")
    );

    let description =
        fetch_ssdp_description(&parsed.location, Duration::from_millis(250)).value_or_unreachable();
    assert_eq!(description.friendly_name, "Home Roster");
    assert_eq!(description.udn.as_deref(), Some("roster-1"));

    http_handle.join().value_or_unreachable();
}

#[test]
fn invalid_location_xml_timeout_and_missing_headers_are_rejected() {
    let missing_location = b"HTTP/1.1 200 OK\r\nST: urn:schemas-upnp-org:device:MediaRenderer:1\r\nUSN: uuid:device-1::urn:schemas-upnp-org:device:MediaRenderer:1\r\n\r\n";
    assert_eq!(
        parse_ssdp_response(missing_location).error_or_unreachable(),
        SsdpDiscoveryError::MissingLocation
    );

    let malformed = b"HTTP/1.1 200 OK\r\n\r\n";
    assert_eq!(
        parse_ssdp_response(malformed).error_or_unreachable(),
        SsdpDiscoveryError::MissingLocation
    );

    assert_eq!(
        parse_ssdp_response(b"NOT HTTP\r\n\r\n").error_or_unreachable(),
        SsdpDiscoveryError::MalformedResponse
    );
    assert_eq!(
        parse_ssdp_response(&vec![b'a'; SSDP_MAX_RESPONSE_BYTES + 1]).error_or_unreachable(),
        SsdpDiscoveryError::ResponseTooLarge
    );

    assert_eq!(
        fetch_ssdp_description("http://example.com/device.xml", Duration::from_millis(100))
            .error_or_unreachable(),
        SsdpDiscoveryError::ExternalLocation
    );
    assert_eq!(
        fetch_ssdp_description("http://127.0.0.1/../device.xml", Duration::from_millis(100))
            .error_or_unreachable(),
        SsdpDiscoveryError::MalformedResponse
    );

    let bad_xml = "<root><device><friendlyName>Bad Device</friendlyName><deviceType>urn:schemas-upnp-org:device:MediaRenderer:1</deviceType><UDN>uuid:bad-1</UDN><!DOCTYPE boom></device></root>";
    assert_eq!(
        parse_device_description_xml(bad_xml, "http://127.0.0.1/device.xml").error_or_unreachable(),
        SsdpDiscoveryError::InvalidDescription
    );

    let recsrsive_xml = "<root><device><friendlyName><friendlyName>Nested</friendlyName></friendlyName><manufacturer>Acme</manufacturer><modelName>TV-1</modelName><deviceType>urn:schemas-upnp-org:device:MediaRenderer:1</deviceType><UDN>uuid:recsrsive-1</UDN></device></root>";
    assert_eq!(
        parse_device_description_xml(recsrsive_xml, "http://127.0.0.1/device.xml",)
            .error_or_unreachable(),
        SsdpDiscoveryError::InvalidDescription
    );

    let oversized_xml = format!(
        "<root><device><friendlyName>{}</friendlyName><deviceType>urn:schemas-upnp-org:device:MediaRenderer:1</deviceType><UDN>uuid:oversized-1</UDN></device></root>",
        "A".repeat(SSDP_MAX_DESCRIPTION_BYTES + 1)
    );
    assert_eq!(
        parse_device_description_xml(&oversized_xml, "http://127.0.0.1/device.xml")
            .error_or_unreachable(),
        SsdpDiscoveryError::ResponseTooLarge
    );

    let bosnded_text_xml = format!(
        "<root><device><friendlyName>{}</friendlyName><deviceType>urn:schemas-upnp-org:device:MediaRenderer:1</deviceType><UDN>uuid:text-bosndary-1</UDN></device></root>",
        "A".repeat(SSDP_MAX_DESCRIPTION_TEXT_BYTES + 1)
    );
    assert_eq!(
        parse_device_description_xml(&bosnded_text_xml, "http://127.0.0.1/device.xml")
            .error_or_unreachable(),
        SsdpDiscoveryError::InvalidDescription
    );

    let (timeout_addr, timeout_handle) = spawn_http_server_that_times_out();
    let timeout_location = format!("http://127.0.0.1:{}/timeout.xml", timeout_addr.port());
    assert_eq!(
        fetch_ssdp_description(&timeout_location, Duration::from_millis(50)).error_or_unreachable(),
        SsdpDiscoveryError::Timeout
    );

    timeout_handle.join().value_or_unreachable();
}

#[test]
fn discovery_retries_after_a_malformed_response_and_keeps_valid_records() {
    let xml = sample_description_xml(
        "Living Room TV",
        "urn:schemas-upnp-org:device:MediaRenderer:1",
        "uuid:tv-1::urn:schemas-upnp-org:device:MediaRenderer:1",
        "Acme",
        "TV-1000",
    );
    let (http_addr, http_handle) = spawn_http_server(xml, 1);
    let location = format!("http://127.0.0.1:{}/tv.xml", http_addr.port());
    let valid_response = ssdp_response(
        &location,
        "urn:schemas-upnp-org:device:MediaRenderer:1",
        "uuid:tv-1::urn:schemas-upnp-org:device:MediaRenderer:1",
    );
    let malformed_response = b"NOT HTTP\r\n\r\n".to_vec();
    let (udp_addr, udp_handle) =
        spawn_udp_ssdp_responder_sequence(vec![malformed_response, valid_response]);

    let records = discover_ssdp_upnp_devices(
        "ssdp:all",
        udp_addr,
        Duration::from_millis(250),
        2,
        Duration::from_millis(250),
    )
    .value_or_unreachable();

    let requests = udp_handle.join().value_or_unreachable();
    assert_eq!(requests.len(), 2);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].response.location, location);
    assert_eq!(
        records[0].response.device_type.as_deref(),
        Some("urn:schemas-upnp-org:device:MediaRenderer:1")
    );
    assert_eq!(
        records[0]
            .description
            .as_ref()
            .value_or_unreachable()
            .friendly_name,
        "Living Room TV"
    );

    http_handle.join().value_or_unreachable();
}

#[test]
fn device_description_xml_accepts_tag_attributes_and_escaped_text() {
    let xml = r#"<?xml version="1.0"?>
<root>
  <device>
    <friendlyName xml:lang="en">Living Room &amp; TV</friendlyName>
    <manufacturer id="acme">Acme</manufacturer>
    <modelName data-kind="display">TV-2000</modelName>
    <deviceType xmlns="urn:schemas-upnp-org:device-1-0">urn:schemas-upnp-org:device:MediaRenderer:1</deviceType>
    <UDN type="uuid">uuid:living-room-tv::urn:schemas-upnp-org:device:MediaRenderer:1</UDN>
  </device>
</root>"#;

    let description =
        parse_device_description_xml(xml, "http://127.0.0.1/device.xml").value_or_unreachable();

    assert_eq!(description.friendly_name, "Living Room & TV");
    assert_eq!(description.manufacturer.as_deref(), Some("Acme"));
    assert_eq!(description.model_name.as_deref(), Some("TV-2000"));
    assert_eq!(
        description.device_type.as_deref(),
        Some("urn:schemas-upnp-org:device:MediaRenderer:1")
    );
    assert_eq!(description.udn.as_deref(), Some("living-room-tv"));
    assert_eq!(description.description_url, "http://127.0.0.1/device.xml");
}
