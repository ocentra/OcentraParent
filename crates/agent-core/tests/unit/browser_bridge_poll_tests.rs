use ocentra_parent_agent_protocol::activity::ActivityEventKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{
    browser_bridge_poll_support::{
        bridge_config, ok, serve_keep_alive_devtools, try_serve_devtools, DevtoolsBodies,
        TestResult,
    },
    poll_chromium_bridge,
};

#[test]
fn poll_chromium_bridge_maps_page_targets_to_browser_activity_events() -> TestResult {
    let endpoint = try_serve_devtools(DevtoolsBodies {
        version_body: constants::browser::DEVTOOLS_TEST_VERSION_BODY.to_string(),
        list_body: constants::browser::DEVTOOLS_TEST_LIST_BODY.to_string(),
    })?;
    let config = bridge_config(endpoint);

    let snapshot = ok(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert_eq!(
        snapshot.browser_version,
        Some(constants::browser::DEVTOOLS_TEST_BROWSER_VERSION.to_string())
    );
    assert_eq!(snapshot.page_target_count, 1);
    assert_eq!(snapshot.events.len(), 1);
    assert_eq!(snapshot.events[0].kind, ActivityEventKind::UrlObserved);
    assert_eq!(
        snapshot.events[0].fields.get(constants::field::URL),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_URL.to_string()
        ))
    );
    assert_eq!(
        snapshot.events[0].fields.get(constants::field::TAB_ID),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_TAB_ID_FROM_TARGET.to_string()
        ))
    );
    assert_eq!(
        snapshot.events[0]
            .fields
            .get(constants::field::ACTIVE_STATE),
        Some(&LogFieldValue::String(
            constants::browser::ACTIVE_STATE_UNKNOWN.to_string()
        ))
    );
    assert_eq!(
        snapshot.events[0]
            .fields
            .get(constants::field::ACTIVE_PROOF_SOURCE),
        Some(&LogFieldValue::String(
            constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY.to_string()
        ))
    );
    assert_eq!(
        snapshot.events[0]
            .fields
            .get(constants::field::CAPABILITY_STATUS),
        Some(&LogFieldValue::String(
            constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY.to_string()
        ))
    );

    Ok(())
}

#[test]
fn poll_chromium_bridge_preserves_adapter_tab_and_window_ids() -> TestResult {
    let endpoint = try_serve_devtools(DevtoolsBodies {
        version_body: constants::browser::DEVTOOLS_TEST_VERSION_BODY.to_string(),
        list_body: constants::browser::DEVTOOLS_TEST_LIST_BODY_WITH_TAB_WINDOW.to_string(),
    })?;
    let config = bridge_config(endpoint);

    let snapshot = ok(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert_eq!(snapshot.page_target_count, 1);
    assert_eq!(
        snapshot.events[0].fields.get(constants::field::TAB_ID),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_TAB_ID.to_string()
        ))
    );
    assert_eq!(
        snapshot.events[0].fields.get(constants::field::WINDOW_ID),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_WINDOW_ID.to_string()
        ))
    );

    Ok(())
}

#[test]
fn poll_chromium_bridge_reports_empty_page_target_discovery() -> TestResult {
    let endpoint = try_serve_devtools(DevtoolsBodies {
        version_body: constants::browser::DEVTOOLS_TEST_VERSION_BODY.to_string(),
        list_body: constants::browser::DEVTOOLS_TEST_EMPTY_LIST_BODY.to_string(),
    })?;
    let config = bridge_config(endpoint);

    let snapshot = ok(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert_eq!(snapshot.page_target_count, 0);
    assert!(snapshot.events.is_empty());

    Ok(())
}

#[test]
fn poll_chromium_bridge_accepts_keep_alive_devtools_response_with_content_length() -> TestResult {
    let endpoint = serve_keep_alive_devtools(DevtoolsBodies {
        version_body: constants::browser::DEVTOOLS_TEST_VERSION_BODY.to_string(),
        list_body: constants::browser::DEVTOOLS_TEST_LIST_BODY.to_string(),
    })?;
    let config = bridge_config(endpoint);

    let snapshot = ok(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert_eq!(
        snapshot.browser_version,
        Some(constants::browser::DEVTOOLS_TEST_BROWSER_VERSION.to_string())
    );
    assert_eq!(snapshot.page_target_count, 1);
    assert_eq!(
        snapshot.events[0].fields.get(constants::field::TITLE),
        Some(&LogFieldValue::String(
            constants::browser::DEVTOOLS_TEST_PAGE_TITLE.to_string()
        ))
    );

    Ok(())
}
