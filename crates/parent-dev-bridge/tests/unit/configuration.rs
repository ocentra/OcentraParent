use std::{
    error::Error,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_dev_bridge::{
    parent_dev_bridge_address_from_configuration, parent_dev_bridge_log_fields,
    ParentDevBridgeFailure,
};
use ocentra_parent_logging_core::field::LogFieldValue;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const STARTUP_TIMEOUT: Duration = Duration::from_secs(10);
const IO_TIMEOUT: Duration = Duration::from_secs(2);
const PORTAL_ORIGIN: &str = "http://127.0.0.1:4478";
const UNTRUSTED_ORIGIN: &str = "http://untrusted.example:4478";

struct ParentDevBridgeProcess(Child);

impl ParentDevBridgeProcess {
    fn spawn(address: SocketAddr) -> TestResult<Self> {
        let child = Command::new(env!("CARGO_BIN_EXE_ocentra-parent-dev-bridge"))
            .env(
                constants::env_var::PARENT_DEV_BRIDGE_PORT,
                address.port().to_string(),
            )
            .env(constants::env_var::DEV_NETWORK_MODE, "loopback")
            .env(
                constants::env_var::AGENT_ALLOWED_ORIGINS,
                format!("{PORTAL_ORIGIN},http://localhost:4478"),
            )
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(Self(child))
    }
}

impl Drop for ParentDevBridgeProcess {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn configured_parent_dev_bridge_defaults_to_loopback() {
    let address = parent_dev_bridge_address_from_configuration(Some(4491), false);

    assert_eq!(
        address.map(|value| value.to_string()),
        Some("127.0.0.1:4491".to_string())
    );
}

#[test]
fn configured_parent_dev_bridge_uses_wildcard_for_lan_mode() {
    let address = parent_dev_bridge_address_from_configuration(Some(4491), true);

    assert_eq!(
        address.map(|value| value.to_string()),
        Some("0.0.0.0:4491".to_string())
    );
}

#[test]
fn configured_parent_dev_bridge_rejects_missing_or_invalid_ports() {
    assert_eq!(
        parent_dev_bridge_address_from_configuration(None, false),
        None
    );
}

#[test]
fn parent_dev_bridge_cors_allows_only_configured_portal_origin() -> TestResult {
    let address = reserve_loopback_address()?;
    let _bridge = ParentDevBridgeProcess::spawn(address)?;

    let allowed_response = wait_for_preflight(address, PORTAL_ORIGIN)?;
    assert_eq!(http_status_line(&allowed_response), Some("HTTP/1.1 200 OK"));
    assert_eq!(
        response_header(&allowed_response, "access-control-allow-origin"),
        Some(PORTAL_ORIGIN)
    );

    let rejected_response = send_preflight(address, UNTRUSTED_ORIGIN)?;
    assert_eq!(
        http_status_line(&rejected_response),
        Some("HTTP/1.1 200 OK")
    );
    assert_eq!(
        response_header(&rejected_response, "access-control-allow-origin"),
        None
    );
    Ok(())
}

#[test]
fn parent_dev_bridge_log_fields_include_bridge_address_when_present() {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4491);
    let fields = parent_dev_bridge_log_fields(
        Some(address),
        &ParentDevBridgeFailure::from_display(constants::error::PARENT_DEV_BRIDGE_RUNS),
    );

    assert_eq!(
        fields.get(constants::field::LOCAL_PORT),
        Some(&LogFieldValue::Number(4491.0))
    );
    assert_eq!(
        fields.get(constants::field::BRIDGE_ENDPOINT_REF),
        Some(&LogFieldValue::String(address.to_string()))
    );
    assert_eq!(
        fields.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::error::PARENT_DEV_BRIDGE_RUNS.to_string()
        ))
    );
}

#[test]
fn parent_dev_bridge_log_fields_omit_address_fields_when_absent() {
    let fields = parent_dev_bridge_log_fields(
        None,
        &ParentDevBridgeFailure::from_display(constants::error::PARENT_DEV_BRIDGE_RUNS),
    );

    assert_eq!(fields.get(constants::field::LOCAL_PORT), None);
    assert_eq!(fields.get(constants::field::BRIDGE_ENDPOINT_REF), None);
    assert_eq!(
        fields.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::error::PARENT_DEV_BRIDGE_RUNS.to_string()
        ))
    );
}

fn reserve_loopback_address() -> TestResult<SocketAddr> {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let address = listener.local_addr()?;
    drop(listener);
    Ok(address)
}

fn wait_for_preflight(address: SocketAddr, origin: &str) -> TestResult<String> {
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    loop {
        match send_preflight(address, origin) {
            Ok(response) => return Ok(response),
            Err(error) if Instant::now() < deadline => {
                let _ = error;
                thread::sleep(Duration::from_millis(25));
            }
            Err(error) => return Err(error),
        }
    }
}

fn send_preflight(address: SocketAddr, origin: &str) -> TestResult<String> {
    let mut stream = TcpStream::connect_timeout(&address, IO_TIMEOUT)?;
    stream.set_read_timeout(Some(IO_TIMEOUT))?;
    stream.set_write_timeout(Some(IO_TIMEOUT))?;
    stream.write_all(preflight_request(address, origin).as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}

fn preflight_request(address: SocketAddr, origin: &str) -> String {
    format!(
        "OPTIONS {} HTTP/1.1\r\nHost: {address}\r\nOrigin: {origin}\r\nAccess-Control-Request-Method: POST\r\nAccess-Control-Request-Headers: content-type\r\nConnection: close\r\n\r\n",
        ocentra_schema::parent_ui_bridge::PARENT_DEV_BRIDGE_LOAD_ROUTE_PATH
    )
}

fn http_status_line(response: &str) -> Option<&str> {
    response.lines().next()
}

fn response_header<'a>(response: &'a str, expected_name: &str) -> Option<&'a str> {
    response.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case(expected_name)
            .then(|| value.trim())
    })
}
