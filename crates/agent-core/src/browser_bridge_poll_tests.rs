use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
    thread,
};

use ocentra_parent_agent_protocol::{
    constants, ActivityEventKind, BrowserChannel, BrowserFamily, LogFieldValue,
};

use crate::{poll_chromium_bridge, BrowserBridgePollConfig, BrowserBridgePollError};

#[test]
fn poll_chromium_bridge_maps_page_targets_to_browser_activity_events() {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY,
    );
    let config = BrowserBridgePollConfig {
        endpoint,
        managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        process_id: constants::browser::PROCESS_ID_UNKNOWN,
        browser_family: BrowserFamily::UnknownChromium,
        browser_channel: BrowserChannel::Unknown,
    };

    let snapshot = poll_chromium_bridge(
        config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);

    assert_eq!(
        snapshot.browser_version,
        Some(constants::browser::DEVTOOLS_TEST_BROWSER_VERSION.to_string())
    );
    assert_eq!(snapshot.events.len(), 1);
    assert_eq!(snapshot.events[0].kind, ActivityEventKind::UrlObserved);
    assert_eq!(
        snapshot.events[0].fields[constants::field::URL],
        LogFieldValue::String(constants::activity_store::TEST_BROWSER_URL.to_string())
    );
}

#[test]
fn poll_chromium_bridge_rejects_non_loopback_endpoint() {
    let endpoint = SocketAddr::from((
        [192, 0, 2, 1],
        constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    ));
    let config = BrowserBridgePollConfig {
        endpoint,
        managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        process_id: constants::browser::PROCESS_ID_UNKNOWN,
        browser_family: BrowserFamily::UnknownChromium,
        browser_channel: BrowserChannel::Unknown,
    };

    let error = poll_chromium_bridge(
        config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserBridgePollError::NonLoopbackEndpoint);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_NON_LOOPBACK_ENDPOINT
    );
}

fn serve_devtools(version_body: &'static str, list_body: &'static str) -> SocketAddr {
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

fn devtools_response(body: &str) -> String {
    let mut response = String::from(constants::browser::HTTP_OK_PREFIX);
    response.push_str(constants::browser::HTTP_BODY_SEPARATOR);
    response.push_str(body);
    response
}
