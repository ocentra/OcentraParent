use ocentra_parent_agent_protocol::{constants, LogFieldValue, LogFields};

use crate::{fields::fields_from_pairs, network::NetworkPolicy};

pub async fn run_agent_service() {
    let network = NetworkPolicy::from_environment();
    let listener = tokio::net::TcpListener::bind(network.bind_address())
        .await
        .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
    let _ = crate::dev_log::write_agent_info(
        constants::dev_log_message::AGENT_SERVICE_STARTED,
        startup_log_fields(&network),
    );
    crate::activity_capture::spawn_startup_activity_capture();
    crate::screen_ai_cadence_runtime::spawn_screen_ai_cadence_runtime();
    crate::screen_ai_foreground_runtime::spawn_screen_ai_foreground_runtime();
    crate::screen_ai_analysis_runtime::spawn_screen_ai_analysis_runtime();
    crate::screen_ai_retention_sweeper_runtime::spawn_screen_ai_retention_sweeper_runtime();
    crate::screen_ai_service_event_subscription::live_view_service_runtime::spawn_screen_live_view_worker_runtime();
    let _screen_ai_service_event_runtime =
        crate::screen_ai_service_event_subscription::ScreenAiServiceEventRuntime::start()
            .await
            .expect(constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES);

    axum::serve(listener, crate::app::router(network))
        .await
        .expect(constants::error::AGENT_SERVICE_RUNS);
}

pub(crate) fn startup_log_fields(network: &NetworkPolicy) -> LogFields {
    fields_from_pairs(vec![
        ("context", LogFieldValue::String("startup".to_string())),
        (
            constants::field::LOCAL_PORT,
            LogFieldValue::Number(f64::from(network.bind_address().port())),
        ),
    ])
}

#[cfg(test)]
#[path = "../tests/unit/service_runtime.rs"]
mod service_runtime_tests;
