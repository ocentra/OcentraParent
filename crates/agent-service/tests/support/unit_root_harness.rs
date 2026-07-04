macro_rules! declare_agent_service_unit_root_harness {
    () => {
        #[path = "../support/activity_capture/mod.rs"]
        mod activity_capture;
        #[path = "../../src/activity_store_path.rs"]
        mod activity_store_path;
        #[path = "../../src/activity_surface_store.rs"]
        mod activity_surface_store;
        #[path = "../../src/event_builder.rs"]
        mod event_builder;
        #[path = "../../src/fields.rs"]
        mod fields;
        #[path = "../../src/json_contract.rs"]
        mod json_contract;
        #[path = "../../src/local_ai_provider_scheduler.rs"]
        mod local_ai_provider_scheduler;
        #[path = "../../src/local_ai_runtime_config.rs"]
        mod local_ai_runtime_config;
        #[path = "../../src/local_ai_runtime_status.rs"]
        mod local_ai_runtime_status;
        #[path = "../../src/parent_assistant_runtime.rs"]
        mod parent_assistant_runtime;
        #[path = "../support/test_text.rs"]
        mod test_text;
        #[path = "../../src/time.rs"]
        mod time;
    };
}

macro_rules! declare_agent_service_screen_ai_root_harness {
    () => {
        #[path = "../support/activity_capture/mod.rs"]
        mod activity_capture;
        #[path = "../support/activity_surface_read_models/mod.rs"]
        mod activity_surface_read_models;
        #[path = "../support/screen_ai_service_event_subscription/mod.rs"]
        mod screen_ai_service_event_subscription;
    };
}
