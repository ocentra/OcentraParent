use super::*;
use ocentra_lan_core::network_inventory::service_identity::http::parse_certificate_subject;
use std::net::TcpStream;

#[test]
fn probe_response_parser_collects_sanitized_http_title_header_redirect_and_links() {
    let listener = TcpListener::bind("127.0.0.1:0").value_or_unreachable();
    let port = listener.local_addr().value_or_unreachable().port();
    let request_count = Arc::new(AtomicUsize::new(0));
    let request_path = Arc::new(Mutex::new(None::<String>));
    let request_path_clone = Arc::clone(&request_path);
    let request_count_clone = Arc::clone(&request_count);

    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().value_or_unreachable();
        request_count_clone.fetch_add(1, Ordering::SeqCst);
        let request = read_request(&mut stream);
        *request_path_clone.lock().value_or_unreachable() = request
            .0
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .map(|value| value.to_string());

        let body = "<html><head><title> Demo\nPanel </title></head><body><a href=\"/child\">child</a></body></html>";
        let response = format!(
            "HTTP/1.1 302 Found\r\nServer: test-banner\r\nX-Powered-By: test-stack\r\nLocation: /admin/login\r\nLink: </metadata>; rel=\"service-desc\"\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes()).value_or_unreachable();
    });

    let observation = probe_service_identity_on_target(
        "127.0.0.1",
        ProbeTarget {
            port,
            transport: ProbeTransport::Http,
            request_paths: &["/"],
        },
    );

    server.join().value_or_unreachable();

    let observation = observation.value_or_unreachable();

    assert_eq!(observation.status_code, Some(302));
    assert_eq!(observation.title.as_deref(), Some("Demo Panel"));
    assert_eq!(observation.server_header.as_deref(), Some("test-banner"));
    assert_eq!(observation.banner.as_deref(), Some("test-stack"));
    assert_eq!(
        observation.redirect_location.as_deref(),
        Some("/admin/login")
    );
    assert_eq!(
        observation.descriptor_links,
        vec!["</metadata>; rel=\"service-desc\"".to_string()]
    );
    assert_eq!(
        request_count.load(Ordering::SeqCst),
        1,
        "probe must not crawl beyond the initial request"
    );
    assert_eq!(
        request_path.lock().value_or_unreachable().as_deref(),
        Some("/")
    );
}

#[test]
fn probe_response_parser_rejects_traversal_references_and_invalid_header_text() {
    let traversal = parse_probe_observation(
        b"HTTP/1.1 302 Found\r\nLocation: /../../secret?x=1\r\nLink: </../../metadata>; rel=\"service-desc\"\r\nContent-Length: 0\r\n\r\n",
        None,
    )
    .value_or_unreachable();

    assert_eq!(traversal.status_code, Some(302));
    assert!(traversal.redirect_location.is_none());
    assert!(traversal.descriptor_links.is_empty());

    assert!(parse_probe_observation(
        b"HTTP/1.1 200 OK\r\nServer: \xff\xfe\r\nContent-Length: 0\r\n\r\n",
        None,
    )
    .is_none());
}

#[test]
fn probe_response_parser_strips_nested_title_markup_before_weak_evidence_application() {
    let body = "<html><head><title>Trusted <span>Panel\u{0007}\n</span><script>alert(1)</script></title></head><body>ok</body></html>";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );

    let observation = parse_probe_observation(response.as_bytes(), None).value_or_unreachable();
    let title = observation.title.clone().value_or_unreachable();
    assert_eq!(title, "Trusted Panel alert(1)");

    let mut device = bounded_probe_devices()
        .into_iter()
        .next()
        .value_or_unreachable();
    apply_service_identity_probe(&mut device, observation);

    assert_eq!(
        device.agent_status.as_deref(),
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS)
    );
    assert_eq!(device.platform, constants::lan_pairing::PLATFORM_UNKNOWN);
    assert!(device.hostname.is_none());
    assert!(device
        .service_identity_probe_evidence
        .iter()
        .any(|evidence| {
            evidence.evidence_kind == LanServiceIdentityProbeEvidenceKind::HtmlTitle
                && evidence.value == title
        }));
}

#[test]
fn probe_response_parser_normalizes_backslash_references() {
    let observation = parse_probe_observation(
        b"HTTP/1.1 200 OK\r\nLink: <\\metadata\\service-desc>; rel=\"service-desc\"\r\nContent-Length: 0\r\n\r\n",
        None,
    )
    .value_or_unreachable();

    assert_eq!(
        observation.descriptor_links,
        vec!["</metadata/service-desc>; rel=\"service-desc\"".to_string()]
    );
}

#[test]
fn probe_response_reader_rejects_oversized_responses() {
    let mut response = b"HTTP/1.1 200 OK\r\nContent-Length: 40000\r\n\r\n".to_vec();
    response.extend(std::iter::repeat_n(
        b'a',
        SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES + 1,
    ));

    assert!(read_probe_response(&mut Cursor::new(response)).is_none());
}

#[test]
fn service_identity_probe_stops_when_scan_budget_is_exhausted() {
    let targets = vec![ProbeTarget {
        port: 80,
        transport: ProbeTransport::Http,
        request_paths: &["/"],
    }];

    assert!(probe_service_identity(
        "127.0.0.1",
        Some("camera-1"),
        &targets,
        ServiceIdentityProbeSettings::default(),
        Instant::now(),
        None,
    )
    .is_none());
}

#[test]
fn probe_service_identity_continues_after_refused_target() {
    let listener = TcpListener::bind("127.0.0.1:0").value_or_unreachable();
    let port = listener.local_addr().value_or_unreachable().port();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().value_or_unreachable();
        let request = read_request(&mut stream);
        assert_eq!(request.0.lines().next(), Some("GET / HTTP/1.1"));
        let body = "<html><head><title>Later target</title></head><body>ok</body></html>";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes()).value_or_unreachable();
    });
    let targets = [
        ProbeTarget {
            port: 0,
            transport: ProbeTransport::Http,
            request_paths: &["/"],
        },
        ProbeTarget {
            port,
            transport: ProbeTransport::Http,
            request_paths: &["/"],
        },
    ];

    let observation = probe_service_identity(
        "127.0.0.1",
        Some("later-device"),
        &targets,
        ServiceIdentityProbeSettings::default(),
        Instant::now() + Duration::from_millis(SERVICE_IDENTITY_PROBE_SCAN_BUDGET_MS),
        None,
    )
    .value_or_unreachable();

    server.join().value_or_unreachable();

    assert_eq!(observation.title.as_deref(), Some("Later target"));
}

#[test]
fn probe_response_parser_collects_tls_certificate_subject() {
    let cert = generate_simple_self_signed(vec!["service.local".into()]).value_or_unreachable();
    let cert_der = cert.cert.der().clone();
    let certificate_subject = parse_certificate_subject(&cert_der).value_or_unreachable();
    let observation = parse_probe_observation(
        b"HTTP/1.1 200 OK\r\nServer: tls-banner\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: 70\r\n\r\n<html><head><title>Secure Control</title></head><body>ok</body></html>",
        Some(certificate_subject.clone()),
    )
    .value_or_unreachable();
    assert_eq!(observation.title.as_deref(), Some("Secure Control"));
    assert_eq!(observation.server_header.as_deref(), Some("tls-banner"));
    assert_eq!(certificate_subject, "CN=rcgen self signed cert");
    assert_eq!(
        observation.certificate_subject.as_deref(),
        Some(certificate_subject.as_str())
    );
}

#[test]
fn enrich_service_identity_probes_is_bounded_by_concurrency() {
    let _env_lock = agent_addr_env_lock();
    let listener = TcpListener::bind("127.0.0.2:0").value_or_unreachable();
    let port = listener.local_addr().value_or_unreachable().port();
    let active = Arc::new(AtomicUsize::new(0));
    let max_active = Arc::new(AtomicUsize::new(0));
    let server = spawn_bounded_probe_server(listener, Arc::clone(&active), Arc::clone(&max_active));

    let previous_agent_addr = env::var(constants::env_var::AGENT_ADDR).ok();
    env::set_var(constants::env_var::AGENT_ADDR, format!("127.0.0.1:{port}"));

    let mut devices = bounded_probe_devices();
    for device in &mut devices {
        device.ip_address = "127.0.0.2".to_string();
    }

    enrich_service_identity_probes(
        &mut devices,
        &[],
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE),
        None,
    );

    if let Some(previous_agent_addr) = previous_agent_addr {
        env::set_var(constants::env_var::AGENT_ADDR, previous_agent_addr);
    } else {
        env::remove_var(constants::env_var::AGENT_ADDR);
    }

    server.join().value_or_unreachable();

    assert!(devices
        .iter()
        .all(|device| is_service_identity_probe_status(device.agent_status.as_deref())));
    assert!(devices.iter().all(|device| {
        device
            .service_identity_probe_evidence
            .iter()
            .any(|evidence| {
                evidence.evidence_kind == LanServiceIdentityProbeEvidenceKind::HtmlTitle
            })
    }));
    let observed_max = max_active.load(Ordering::SeqCst);
    assert!(
        observed_max >= 2,
        "probe fan-out should exercise real concurrency, observed {observed_max}"
    );
    assert!(
        observed_max <= 4,
        "probe fan-out must stay within the configured concurrency ceiling, observed {observed_max}"
    );
}

fn spawn_bounded_probe_server(
    listener: TcpListener,
    active: Arc<AtomicUsize>,
    max_active: Arc<AtomicUsize>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut handlers = Vec::new();
        for _ in 0..5 {
            let (stream, _) = listener.accept().value_or_unreachable();
            handlers.push(spawn_bounded_probe_handler(
                stream,
                Arc::clone(&active),
                Arc::clone(&max_active),
            ));
        }

        join_bounded_probe_handlers(handlers);
    })
}

fn spawn_bounded_probe_handler(
    stream: TcpStream,
    active: Arc<AtomicUsize>,
    max_active: Arc<AtomicUsize>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        run_bounded_probe_handler(stream, active.as_ref(), max_active.as_ref());
    })
}

fn run_bounded_probe_handler(
    mut stream: TcpStream,
    active: &AtomicUsize,
    max_active: &AtomicUsize,
) {
    let current = active.fetch_add(1, Ordering::SeqCst) + 1;
    update_max_active(max_active, current);

    let _ = read_request(&mut stream);
    thread::sleep(Duration::from_millis(150));
    let body = "<html><head><title>Bounded</title></head><body>ok</body></html>";
    let response = format!(
        "HTTP/1.1 200 OK\r\nServer: bounded\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
    active.fetch_sub(1, Ordering::SeqCst);
}

fn update_max_active(max_active: &AtomicUsize, current: usize) {
    loop {
        let observed = max_active.load(Ordering::SeqCst);
        if current <= observed {
            return;
        }
        if max_active
            .compare_exchange(observed, current, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            return;
        }
    }
}

fn join_bounded_probe_handlers(handlers: Vec<thread::JoinHandle<()>>) {
    for handler in handlers {
        handler.join().value_or_unreachable();
    }
}

fn bounded_probe_devices() -> Vec<LanNetworkInventoryDevice> {
    (0..5)
        .map(|index| LanNetworkInventoryDevice {
            device_id: format!("lan-device-{index}"),
            label: format!("device-{index}"),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: "127.0.0.1".to_string(),
            mac_address: format!("AA-BB-CC-DD-EE-{index:02X}"),
            hostname: None,
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            ],
            observed_at: String::new(),
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        })
        .collect::<Vec<_>>()
}
