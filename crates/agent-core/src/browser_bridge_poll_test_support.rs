use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
    thread,
    time::Duration,
};

use ocentra_parent_agent_protocol::{constants, BrowserChannel, BrowserFamily, LogFieldValue};

use crate::{BrowserBridgeExpectedCustody, BrowserBridgePollConfig};

pub(crate) fn bridge_config(endpoint: SocketAddr) -> BrowserBridgePollConfig {
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

pub(crate) fn field_value_contains_raw_debugger_url(value: &LogFieldValue) -> bool {
    matches!(value, LogFieldValue::String(text) if text.contains(constants::browser::DEVTOOLS_TEST_RAW_DEBUGGER_URL))
}

pub(crate) fn serve_keep_alive_devtools(
    version_body: &'static str,
    list_body: &'static str,
) -> SocketAddr {
    let listener = TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT)
        .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
    let endpoint = listener
        .local_addr()
        .expect(constants::error::AGENT_ADDR_SOCKET_ADDRESS);

    thread::spawn(move || {
        for body in [version_body, list_body] {
            let (mut stream, _) = listener
                .accept()
                .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
            thread::spawn(move || {
                let mut request = [0; 1024];
                let _ = stream.read(&mut request);
                let response = devtools_content_length_response(body);
                stream
                    .write_all(response.as_bytes())
                    .expect(constants::error::AGENT_EVENT_SERIALIZES);
                thread::sleep(Duration::from_millis(
                    constants::browser::DEVTOOLS_TIMEOUT_MS + 250,
                ));
            });
        }
    });

    endpoint
}

pub(crate) fn serve_devtools(version_body: &'static str, list_body: &'static str) -> SocketAddr {
    let listener = TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT)
        .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
    let endpoint = listener
        .local_addr()
        .expect(constants::error::AGENT_ADDR_SOCKET_ADDRESS);

    thread::spawn(move || {
        for body in [version_body, list_body] {
            let (mut stream, _) = listener
                .accept()
                .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
            let mut request = [0; 1024];
            let _ = stream.read(&mut request);
            let response = devtools_response(body);
            stream
                .write_all(response.as_bytes())
                .expect(constants::error::AGENT_EVENT_SERIALIZES);
        }
    });

    endpoint
}

pub(crate) fn serve_devtools_owned(version_body: String, list_body: String) -> SocketAddr {
    let listener = TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT)
        .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
    let endpoint = listener
        .local_addr()
        .expect(constants::error::AGENT_ADDR_SOCKET_ADDRESS);

    thread::spawn(move || {
        for body in [version_body, list_body] {
            let (mut stream, _) = listener
                .accept()
                .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
            let mut request = [0; 1024];
            let _ = stream.read(&mut request);
            let response = devtools_response(&body);
            stream
                .write_all(response.as_bytes())
                .expect(constants::error::AGENT_EVENT_SERIALIZES);
        }
    });

    endpoint
}

pub(crate) fn serve_unresponsive_devtools() -> SocketAddr {
    let listener = TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT)
        .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
    let endpoint = listener
        .local_addr()
        .expect(constants::error::AGENT_ADDR_SOCKET_ADDRESS);

    thread::spawn(move || {
        let (mut stream, _) = listener
            .accept()
            .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
        let mut request = [0; 1024];
        let _ = stream.read(&mut request);
        thread::sleep(Duration::from_millis(
            constants::browser::DEVTOOLS_TIMEOUT_MS + 250,
        ));
    });

    endpoint
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
