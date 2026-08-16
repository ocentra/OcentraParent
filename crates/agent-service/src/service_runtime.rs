use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::{fields::fields_from_pairs, network::NetworkPolicy};

const STARTUP_LOG_CONTEXT_FIELD: &str = "context";
const STARTUP_LOG_CONTEXT_VALUE: &str = "startup";

struct StartupErrorReason(String);

pub async fn run_agent_service() {
    let network = NetworkPolicy::from_environment();
    let listener = match tokio::net::TcpListener::bind(network.bind_address()).await {
        Ok(listener) => listener,
        Err(error) => {
            let _ = crate::dev_log::write_agent_error(
                constants::error::LOCALHOST_BIND_SUCCEEDS,
                startup_error_log_fields(&network, StartupErrorReason(error.to_string())),
            );
            return;
        }
    };
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
    if let Err(error) = crate::network_runtime_delivery::initialize_network_runtime_spine().await {
        let _ = crate::dev_log::write_agent_error(
            constants::error::AGENT_SERVICE_RUNS,
            startup_error_log_fields(&network, StartupErrorReason(error.to_string())),
        );
        return;
    }
    let _screen_ai_service_event_runtime =
        match crate::screen_ai_service_event_subscription::ScreenAiServiceEventRuntime::start()
            .await
        {
            Ok(runtime) => runtime,
            Err(error) => {
                let _ = crate::dev_log::write_agent_error(
                    constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES,
                    startup_error_log_fields(&network, StartupErrorReason(error.to_string())),
                );
                return;
            }
        };

    if let Err(error) = axum::serve(listener, crate::app::router(network.clone())).await {
        let _ = crate::dev_log::write_agent_error(
            constants::error::AGENT_SERVICE_RUNS,
            startup_error_log_fields(&network, StartupErrorReason(error.to_string())),
        );
    }
}

pub fn startup_log_fields(network: &NetworkPolicy) -> LogFields {
    fields_from_pairs(vec![
        (
            STARTUP_LOG_CONTEXT_FIELD,
            LogFieldValue::String(STARTUP_LOG_CONTEXT_VALUE.to_string()),
        ),
        (
            constants::field::LOCAL_PORT,
            LogFieldValue::Number(f64::from(network.bind_address().port())),
        ),
    ])
}

fn startup_error_log_fields(network: &NetworkPolicy, reason: StartupErrorReason) -> LogFields {
    fields_from_pairs(vec![
        (
            STARTUP_LOG_CONTEXT_FIELD,
            LogFieldValue::String(STARTUP_LOG_CONTEXT_VALUE.to_string()),
        ),
        (
            constants::field::LOCAL_PORT,
            LogFieldValue::Number(f64::from(network.bind_address().port())),
        ),
        (constants::field::REASON, LogFieldValue::String(reason.0)),
    ])
}
