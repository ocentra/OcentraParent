#![forbid(unsafe_code)]

use std::{
    error::Error,
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{constants, AGENT_PROTOCOL_SCHEMA_VERSION};
use ocentra_schema::parent_ui_bridge::{
    ParentServiceHealthAuthenticationState, ParentServiceHealthReason, ParentServiceHealthRoute,
    ParentServiceHealthSnapshot, ParentServiceHealthState,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const STARTUP_TIMEOUT: Duration = Duration::from_secs(20);
const IO_TIMEOUT: Duration = Duration::from_secs(2);
const READ_ONLY_PAGE_HTML: &str = "<!doctype html><title>degraded read-only route</title>";

struct ServiceProcess {
    child: Child,
    root: PathBuf,
}

impl ServiceProcess {
    fn spawn(address: SocketAddr, root: PathBuf) -> TestResult<Self> {
        fs::create_dir_all(&root)?;
        let intervention_page_path = root.join("browser-intervention.html");
        fs::write(&intervention_page_path, READ_ONLY_PAGE_HTML)?;
        let child = Command::new(env!("CARGO_BIN_EXE_ocentra-parent-agent-service"))
            .current_dir(&root)
            .env(constants::env_var::AGENT_ADDR, address.to_string())
            .env(
                constants::env_var::ACTIVITY_DB_PATH,
                root.join("activity.sqlite"),
            )
            .env(
                constants::env_var::ACTIVITY_JOURNAL_PATH,
                root.join("activity.journal"),
            )
            .env(
                constants::env_var::ACTIVITY_JOURNAL_KEY_PATH,
                root.join("activity.key"),
            )
            .env(
                constants::env_var::NETWORK_RUNTIME_JOURNAL_PATH,
                root.join("network-runtime.journal"),
            )
            .env(constants::env_var::DEV_LOG_DIR, root.join("dev-log"))
            .env(
                constants::env_var::MANAGED_BROWSER_INTERVENTION_HTML_PATH,
                intervention_page_path,
            )
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(Self { child, root })
    }
}

impl Drop for ServiceProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn degraded_service_binds_read_only_routes_and_rejects_commands() -> TestResult {
    let address = reserve_loopback_address()?;
    let _service = ServiceProcess::spawn(address, unique_test_root()?)?;

    let health_response = wait_for_health(address)?;
    assert_eq!(status_line(&health_response)?, "HTTP/1.1 200 OK");
    let health: ParentServiceHealthSnapshot = serde_json::from_str(body(&health_response)?)?;
    assert_degraded_health(&health);

    let page_response = send_request(address, read_only_page_request(address))?;
    assert_eq!(status_line(&page_response)?, "HTTP/1.1 200 OK");
    assert_eq!(body(&page_response)?, READ_ONLY_PAGE_HTML);

    let websocket_response = send_request(address, websocket_request(address))?;
    assert_eq!(
        status_line(&websocket_response)?,
        "HTTP/1.1 503 Service Unavailable"
    );

    let health_after_rejection: ParentServiceHealthSnapshot =
        serde_json::from_str(body(&send_request(address, health_request(address))?)?)?;
    assert_eq!(health_after_rejection, health);
    Ok(())
}

fn assert_degraded_health(health: &ParentServiceHealthSnapshot) {
    assert_eq!(health.state, ParentServiceHealthState::Degraded);
    assert_eq!(health.route, Some(ParentServiceHealthRoute::Localhost));
    assert_eq!(
        health.protocol_schema_version,
        Some(AGENT_PROTOCOL_SCHEMA_VERSION)
    );
    assert_eq!(
        health.service_version.as_deref(),
        Some(env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(health.transport, None);
    assert_eq!(
        health.authentication_state,
        ParentServiceHealthAuthenticationState::Unavailable
    );
    assert_eq!(
        health.reason,
        ParentServiceHealthReason::RouteDependencyUnavailable
    );
    assert_eq!(health.trace.request_id, None);
    assert_eq!(health.trace.correlation_id, None);
    assert_eq!(health.trace.response_event_id, None);
    assert_eq!(health.trace.request_sent_at, None);
    assert_eq!(health.trace.response_sent_at, None);
}

fn reserve_loopback_address() -> TestResult<SocketAddr> {
    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))?;
    let address = listener.local_addr()?;
    drop(listener);
    Ok(address)
}

fn unique_test_root() -> TestResult<PathBuf> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!(
        "ocentra-agent-service-degraded-{}-{nonce}",
        std::process::id()
    )))
}

fn wait_for_health(address: SocketAddr) -> TestResult<String> {
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    loop {
        if let Ok(response) = send_request(address, health_request(address)) {
            if matches!(status_line(&response), Ok("HTTP/1.1 200 OK")) {
                return Ok(response);
            }
        }
        if Instant::now() >= deadline {
            return Err("agent service did not bind degraded health before the deadline".into());
        }
        thread::sleep(Duration::from_millis(25));
    }
}

fn send_request(address: SocketAddr, request: String) -> TestResult<String> {
    let mut stream = TcpStream::connect_timeout(&address, IO_TIMEOUT)?;
    stream.set_read_timeout(Some(IO_TIMEOUT))?;
    stream.set_write_timeout(Some(IO_TIMEOUT))?;
    stream.write_all(request.as_bytes())?;
    read_response(stream)
}

fn read_response(stream: TcpStream) -> TestResult<String> {
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    let mut content_length = 0;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Err("agent service closed before completing the HTTP headers".into());
        }
        response.push_str(&line);
        if line == "\r\n" {
            break;
        }
        if let Some(value) = header_value(&line, "content-length") {
            content_length = value.parse()?;
        }
    }
    let mut body = vec![0; content_length];
    reader.read_exact(&mut body)?;
    response.push_str(std::str::from_utf8(&body)?);
    Ok(response)
}

fn header_value<'a>(line: &'a str, expected_name: &str) -> Option<&'a str> {
    let (name, value) = line.split_once(':')?;
    name.eq_ignore_ascii_case(expected_name)
        .then(|| value.trim())
}

fn health_request(address: SocketAddr) -> String {
    get_request(constants::endpoint::HEALTH, address)
}

fn read_only_page_request(address: SocketAddr) -> String {
    get_request(constants::endpoint::BROWSER_INTERVENTION_PAGE, address)
}

fn get_request(path: &str, address: SocketAddr) -> String {
    format!("GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n")
}

fn websocket_request(address: SocketAddr) -> String {
    format!(
        "GET {} HTTP/1.1\r\nHost: {address}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n",
        constants::endpoint::DEV_WS,
    )
}

fn status_line(response: &str) -> TestResult<&str> {
    response
        .lines()
        .next()
        .ok_or_else(|| "HTTP response did not contain a status line".into())
}

fn body(response: &str) -> TestResult<&str> {
    response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .ok_or_else(|| "HTTP response did not contain a body separator".into())
}
