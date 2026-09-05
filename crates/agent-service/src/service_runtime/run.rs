use super::StartupErrorReason;
use crate::{network::NetworkPolicy, service_runtime::initialize_network_runtime};
use ocentra_parent_agent_protocol::constants;
use std::net::SocketAddr;

pub async fn run_agent_service() {
    let network = NetworkPolicy::from_environment();
    let parent_local_bridge_admission =
        crate::parent_local_bridge_admission::ParentLocalBridgeAdmission::mount_for_service(
            &network,
        );
    if let Err(error) = initialize_network_runtime().await {
        let reason = super::startup_error::network_runtime_startup_reason(&error);
        let _ = crate::dev_log::write_agent_error(
            constants::error::AGENT_SERVICE_RUNS,
            super::startup_error_log_fields(&network, reason),
        );
        return;
    }
    let listener = match tokio::net::TcpListener::bind(network.bind_address()).await {
        Ok(listener) => listener,
        Err(error) => {
            let _ = crate::dev_log::write_agent_error(
                constants::error::LOCALHOST_BIND_SUCCEEDS,
                super::startup_error_log_fields(&network, StartupErrorReason(error.to_string())),
            );
            return;
        }
    };
    let _ = crate::dev_log::write_agent_info(
        constants::dev_log_message::AGENT_SERVICE_STARTED,
        super::startup_log_fields(&network),
    );
    crate::network_runtime_capture_loop::spawn_recurring_capture_loop();
    crate::screen_ai_cadence_runtime::spawn_screen_ai_cadence_runtime();
    crate::screen_ai_foreground_runtime::spawn_screen_ai_foreground_runtime();
    crate::screen_ai_analysis_runtime::spawn_screen_ai_analysis_runtime();
    crate::screen_ai_retention_sweeper_runtime::spawn_screen_ai_retention_sweeper_runtime();
    crate::screen_ai_service_event_subscription::live_view_service_runtime::spawn_screen_live_view_worker_runtime();
    let _screen_ai_service_event_runtime =
        match crate::screen_ai_service_event_subscription::ScreenAiServiceEventRuntime::start()
            .await
        {
            Ok(runtime) => runtime,
            Err(error) => {
                let _ = crate::dev_log::write_agent_error(
                    constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES,
                    super::startup_error_log_fields(
                        &network,
                        StartupErrorReason(error.to_string()),
                    ),
                );
                return;
            }
        };

    if let Err(error) = axum::serve(
        listener,
        crate::app::router(network.clone(), parent_local_bridge_admission)
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    {
        let _ = crate::dev_log::write_agent_error(
            constants::error::AGENT_SERVICE_RUNS,
            super::startup_error_log_fields(&network, StartupErrorReason(error.to_string())),
        );
    }
}
