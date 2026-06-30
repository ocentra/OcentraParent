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

pub type TestResult = Result<(), String>;

pub fn ok<T, E: Debug>(result: Result<T, E>, context: &str) -> Result<T, String> {
    result.map_err(|error| format!("{context}: {error:?}"))
}

pub fn err<T, E: Debug>(result: Result<T, E>, context: &str) -> Result<E, String> {
    match result {
        Ok(_) => Err(format!("{context}: expected error")),
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

pub fn serve_keep_alive_devtools(
    version_body: &'static str,
    list_body: &'static str,
) -> Result<SocketAddr, String> {
    let listener = ok(
        TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT),
        constants::error::LOCALHOST_BIND_SUCCEEDS,
    )?;
    let endpoint = ok(
        listener.local_addr(),
        constants::error::AGENT_ADDR_SOCKET_ADDRESS,
    )?;

    thread::spawn(move || {
        for body in [version_body, list_body] {
            let Ok((mut stream, _)) = listener.accept() else {
                return;
            };
            thread::spawn(move || {
                let mut request = [0; 1024];
                let _ = stream.read(&mut request);
                let response = devtools_content_length_response(body);
                let _ = stream.write_all(response.as_bytes());
                thread::sleep(Duration::from_millis(
                    constants::browser::DEVTOOLS_TIMEOUT_MS + 250,
                ));
            });
        }
    });

    Ok(endpoint)
}

pub fn serve_devtools(version_body: &'static str, list_body: &'static str) -> SocketAddr {
    let Ok(endpoint) = try_serve_devtools(version_body, list_body) else {
        std::process::abort();
    };

    endpoint
}

pub fn try_serve_devtools(
    version_body: &'static str,
    list_body: &'static str,
) -> Result<SocketAddr, String> {
    let listener = ok(
        TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT),
        constants::error::LOCALHOST_BIND_SUCCEEDS,
    )?;
    let endpoint = ok(
        listener.local_addr(),
        constants::error::AGENT_ADDR_SOCKET_ADDRESS,
    )?;

    thread::spawn(move || {
        for body in [version_body, list_body] {
            let Ok((mut stream, _)) = listener.accept() else {
                return;
            };
            let mut request = [0; 1024];
            let _ = stream.read(&mut request);
            let response = devtools_response(body);
            let _ = stream.write_all(response.as_bytes());
        }
    });

    Ok(endpoint)
}

pub fn serve_devtools_owned(version_body: String, list_body: String) -> SocketAddr {
    let Ok(endpoint) = try_serve_devtools_owned(version_body, list_body) else {
        std::process::abort();
    };

    endpoint
}

pub fn try_serve_devtools_owned(
    version_body: String,
    list_body: String,
) -> Result<SocketAddr, String> {
    let listener = ok(
        TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT),
        constants::error::LOCALHOST_BIND_SUCCEEDS,
    )?;
    let endpoint = ok(
        listener.local_addr(),
        constants::error::AGENT_ADDR_SOCKET_ADDRESS,
    )?;

    thread::spawn(move || {
        for body in [version_body, list_body] {
            let Ok((mut stream, _)) = listener.accept() else {
                return;
            };
            let mut request = [0; 1024];
            let _ = stream.read(&mut request);
            let response = devtools_response(&body);
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

pub fn try_serve_unresponsive_devtools() -> Result<SocketAddr, String> {
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

fn devtools_response(body: &str) -> String {
    let mut response = String::from(constants::browser::HTTP_OK_PREFIX);
    response.push_str(constants::browser::HTTP_BODY_SEPARATOR);
    response.push_str(body);
    response
}

fn devtools_content_length_response(body: &str) -> String {
    let mut response = String::from(constants::browser::HTTP_OK_PREFIX);
    response.push_str(constants::browser::HTTP_LINE_SEPARATOR);
    response.push_str(constants::browser::HTTP_HEADER_CONTENT_LENGTH);
    response.push(' ');
    response.push_str(&body.len().to_string());
    response.push_str(constants::browser::HTTP_BODY_SEPARATOR);
    response.push_str(body);
    response
}
