macro_rules! declare_lan_root_harness {
    () => {
        #[path = "../support/lan_root_app.rs"]
        mod app;
        #[path = "../../src/event_builder.rs"]
        mod event_builder;
        #[path = "../../src/fields.rs"]
        mod fields;
        #[path = "../../src/json_contract.rs"]
        mod json_contract;
        #[path = "../support/lan_pairing_mod.rs"]
        mod lan_pairing;
        #[path = "../support/lan_pairing_audit_mod.rs"]
        mod lan_pairing_audit;
        #[path = "../../src/lan_pairing_browser_add_device_scan.rs"]
        mod lan_pairing_browser_add_device_scan;
        #[path = "../support/lan_pairing_browser_add_device_state_mod.rs"]
        mod lan_pairing_browser_add_device_state;
        #[path = "../../src/lan_pairing_browser_runtime.rs"]
        mod lan_pairing_browser_runtime;
        #[path = "../../src/lan_pairing_payload.rs"]
        mod lan_pairing_payload;
        #[path = "../support/lan_pairing_runtime_state_mod.rs"]
        mod lan_pairing_runtime_state;
        #[path = "../../src/lan_pairing_status.rs"]
        mod lan_pairing_status;
        #[path = "../../src/lan_runtime_stream_api.rs"]
        mod lan_runtime_stream_api;
        #[path = "../../src/lan_runtime_stream_payload.rs"]
        mod lan_runtime_stream_payload;
        #[path = "../support/lan_runtime_test_support.rs"]
        mod lan_runtime_test_support;
        #[path = "../support/lan_test_websocket_dispatch.rs"]
        mod lan_test_websocket_dispatch;
        #[path = "../support/test_text.rs"]
        mod test_text;
        #[path = "../../src/time.rs"]
        mod time;
        #[path = "../support/lan_test_websocket.rs"]
        mod websocket;
    };
}
