use std::{
    fmt::Debug,
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
    thread,
    time::Duration,
};

use ocentra_parent_agent_core::browser_bridge_poll::{
    BrowserBridgeExpectedCustody, BrowserBridgePollConfig,
};
use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

#[derive(Debug)]
pub struct TestError(String);

impl std::fmt::Display for TestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for TestError {}

#[derive(Clone)]
pub struct DevtoolsBodies {
    pub version_body: String,
    pub list_body: String,
}

pub type TestResult = Result<(), TestError>;

pub fn ok<T, E: Debug>(
    result: Result<T, E>,
    context: impl std::fmt::Display,
) -> Result<T, TestError> {
    result.map_err(|error| TestError(format!("{context}: {error:?}")))
}

pub fn err<T, E: Debug>(
    result: Result<T, E>,
    context: impl std::fmt::Display,
) -> Result<E, TestError> {
    match result {
        Ok(_) => Err(TestError(format!("{context}: expected error"))),
        Err(error) => Ok(error),
    }
}

pub fn bridge_config(endpoint: SocketAddr) -> BrowserBridgePollConfig {
    BrowserBridgePollConfig {
        endpoint,
        managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        process_id: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        browser_family: BrowserFamily::Chrome,
        browser_channel: BrowserChannel::Stable,
        expected_custody: BrowserBridgeExpectedCustody {
            bridge_port: endpoint.port(),
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
            browser_family: BrowserFamily::Chrome,
            browser_channel: BrowserChannel::Stable,
            session_fresh_until: constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string(),
        },
    }
}

pub fn field_value_contains_raw_debugger_url(value: &LogFieldValue) -> bool {
    matches!(value, LogFieldValue::String(text) if text.contains(constants::browser::DEVTOOLS_TEST_RAW_DEBUGGER_URL))
}

pub fn serve_keep_alive_devtools(bodies: DevtoolsBodies) -> Result<SocketAddr, TestError> {
    let listener = ok(
        TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT),
        constants::error::LOCALHOST_BIND_SUCCEEDS,
    )?;
    let endpoint = ok(
        listener.local_addr(),
        constants::error::AGENT_ADDR_SOCKET_ADDRESS,
    )?;

    thread::spawn(move || {
        for body in [bodies.version_body, bodies.list_body] {
            let Ok((mut stream, _)) = listener.accept() else {
                return;
            };
            thread::spawn(move || {
                let mut request = [0; 1024];
                let _ = stream.read(&mut request);
                let mut response = String::from(constants::browser::HTTP_OK_PREFIX);
                response.push_str(constants::browser::HTTP_LINE_SEPARATOR);
                response.push_str(constants::browser::HTTP_HEADER_CONTENT_LENGTH);
                response.push(' ');
                response.push_str(&body.len().to_string());
                response.push_str(constants::browser::HTTP_BODY_SEPARATOR);
                response.push_str(&body);
                let _ = stream.write_all(response.as_bytes());
                thread::sleep(Duration::from_millis(
                    constants::browser::DEVTOOLS_TIMEOUT_MS + 250,
                ));
            });
        }
    });

    Ok(endpoint)
}

pub fn serve_devtools(bodies: DevtoolsBodies) -> SocketAddr {
    let Ok(endpoint) = try_serve_devtools(bodies) else {
        std::process::abort();
    };

    endpoint
}

pub fn try_serve_devtools(bodies: DevtoolsBodies) -> Result<SocketAddr, TestError> {
    let listener = ok(
        TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT),
        constants::error::LOCALHOST_BIND_SUCCEEDS,
    )?;
    let endpoint = ok(
        listener.local_addr(),
        constants::error::AGENT_ADDR_SOCKET_ADDRESS,
    )?;

    thread::spawn(move || {
        for body in [bodies.version_body, bodies.list_body] {
            let Ok((mut stream, _)) = listener.accept() else {
                return;
            };
            let mut request = [0; 1024];
            let _ = stream.read(&mut request);
            let mut response = String::from(constants::browser::HTTP_OK_PREFIX);
            response.push_str(constants::browser::HTTP_BODY_SEPARATOR);
            response.push_str(&body);
            let _ = stream.write_all(response.as_bytes());
        }
    });

    Ok(endpoint)
}

pub fn serve_devtools_owned(bodies: DevtoolsBodies) -> SocketAddr {
    let Ok(endpoint) = try_serve_devtools_owned(bodies) else {
        std::process::abort();
    };

    endpoint
}

pub fn try_serve_devtools_owned(bodies: DevtoolsBodies) -> Result<SocketAddr, TestError> {
    let listener = ok(
        TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT),
        constants::error::LOCALHOST_BIND_SUCCEEDS,
    )?;
    let endpoint = ok(
        listener.local_addr(),
        constants::error::AGENT_ADDR_SOCKET_ADDRESS,
    )?;

    thread::spawn(move || {
        for body in [bodies.version_body, bodies.list_body] {
            let Ok((mut stream, _)) = listener.accept() else {
                return;
            };
            let mut request = [0; 1024];
            let _ = stream.read(&mut request);
            let mut response = String::from(constants::browser::HTTP_OK_PREFIX);
            response.push_str(constants::browser::HTTP_BODY_SEPARATOR);
            response.push_str(&body);
            let _ = stream.write_all(response.as_bytes());
        }
    });

    Ok(endpoint)
}

pub fn serve_unresponsive_devtools() -> SocketAddr {
    let Ok(endpoint) = try_serve_unresponsive_devtools() else {
        std::process::abort();
    };

    endpoint
}

pub fn try_serve_unresponsive_devtools() -> Result<SocketAddr, TestError> {
    let listener = ok(
        TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT),
        constants::error::LOCALHOST_BIND_SUCCEEDS,
    )?;
    let endpoint = ok(
        listener.local_addr(),
        constants::error::AGENT_ADDR_SOCKET_ADDRESS,
    )?;

    thread::spawn(move || {
        let Ok((mut stream, _)) = listener.accept() else {
            return;
        };
        let mut request = [0; 1024];
        let _ = stream.read(&mut request);
        thread::sleep(Duration::from_millis(
            constants::browser::DEVTOOLS_TIMEOUT_MS + 250,
        ));
    });

    Ok(endpoint)
}
