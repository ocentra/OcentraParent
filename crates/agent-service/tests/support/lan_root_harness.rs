macro_rules! declare_lan_root_harness {
    () => {
        #[path = "../support/lan_test_websocket.rs"]
        mod websocket;
    };
}
