#![forbid(unsafe_code)]

#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../../src/screen_settings_api.rs"]
mod screen_settings_api;
#[path = "screen_settings_api_tests.rs"]
mod screen_settings_api_tests;
#[path = "../../src/screen_settings_payload.rs"]
mod screen_settings_payload;
#[path = "../../src/screen_settings_request.rs"]
mod screen_settings_request;
#[path = "../../src/screen_settings_runtime.rs"]
mod screen_settings_runtime;
#[path = "screen_settings_runtime_tests.rs"]
mod screen_settings_runtime_tests;
#[path = "../../src/screen_settings_store.rs"]
mod screen_settings_store;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../../src/time.rs"]
mod time;

#[test]
fn screen_settings_harness_links_env_runtime_and_shared_helpers() {
    let portal = event_builder::portal_peer();
    assert_eq!(
        portal.peer_id,
        ocentra_parent_agent_protocol::constants::peer::PORTAL_DEV
    );

    let value = json_contract::serialize_json_value(serde_json::json!({"screen": "settings"}));
    assert_eq!(value["screen"], "settings");

    let store_path = screen_settings_store::screen_settings_store_path_from_env();
    assert!(store_path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some());

    let runtime = screen_settings_runtime::ScreenSettingsRuntime::from_env();
    assert!(format!("{runtime:?}").contains("ScreenSettingsRuntime"));

    assert_eq!(
        time::timestamp_from_epoch_seconds(0),
        "1970-01-01T00:00:00.000Z"
    );
    assert_eq!(
        time::timestamp_after_epoch_seconds(20, 2),
        "1970-01-01T00:00:22.000Z"
    );
}
