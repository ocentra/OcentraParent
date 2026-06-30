use ocentra_parent_agent_service::dev_log::{
    write_agent_debug, write_agent_error, write_agent_info, write_agent_warn,
};

#[cfg(test)]
mod clippy_linkage {
    #[test]
    fn time_helpers_are_linked() {
        let _ = crate::time::timestamp_from_epoch_seconds;
        let _ = crate::time::timestamp_after_epoch_seconds;
    }
}

#[path = "unit/app_game_adapter_host_capabilities.rs"]
mod app_game_adapter_host_capabilities_tests;
#[path = "unit/browser_evidence_payload.rs"]
mod browser_evidence_payload_tests;
#[path = "unit/dev_log.rs"]
mod dev_log;
#[path = "unit/event_builder.rs"]
mod event_builder_tests;
#[path = "../src/fields.rs"]
mod fields;
#[path = "unit/network.rs"]
mod network_tests;
#[path = "unit/screen_ai_policy_refs.rs"]
mod screen_ai_policy_refs;
#[path = "unit/snapshot.rs"]
mod snapshot_tests;
#[path = "../src/time.rs"]
mod time;
