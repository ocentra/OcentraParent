use crate::{
    activity_network_flow_payload::network_flow_read_model_payload_with_runtime_delivery,
    activity_payload::{
        activity_store_error_payload, ingest_status_payload, recent_summary_payload,
    },
    activity_store_path::activity_db_path,
    activity_surface_store::load_app_game_model,
    browser_evidence_payload::browser_evidence_read_model_payload,
    browser_inventory_read_model::browser_inventory_read_model_from_windows_inventory,
    browser_payload::browser_inventory_read_model_payload,
    browser_runtime_paths::system_browser_candidate_paths,
    event_builder::build_event,
    network_product_path_bridge::prove_network_product_path_for_read_model,
    network_runtime_delivery::deliver_network_runtime_for_read_model,
    network_runtime_stream_payload::{
        network_runtime_event_chain_stream_payload,
        stream_network_runtime_event_chain_for_read_model,
    },
    time::timestamp_now,
};
use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    browser_windows_inventory::windows_browser_inventory_observations,
    browser_windows_package_inventory::windows_browser_package_observations,
    browser_windows_package_source::live_windows_browser_package_entries_with_limit,
    process_capture::{collect_process_snapshot, ProcessObservation},
    tracking::tracking_read_model_for_store,
};
use ocentra_parent_agent_protocol::activity_query::{ActivityIngestStatus, ActivityRecentSummary};
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::tracking_read_model_payload;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

pub(crate) mod activity_memory_graph_report;
pub(crate) mod app_game_adapter_dispatch_execute_payload;
pub(crate) mod app_game_adapter_dispatch_preflight_payload;
#[cfg(test)]
mod app_game_adapter_dispatch_preflight_payload_tests;
#[cfg(test)]
mod app_game_adapter_dispatch_preflight_service_tests;
pub(crate) mod app_game_adapter_dispatch_result_payload;
#[cfg(test)]
mod app_game_adapter_dispatch_result_payload_tests;
#[cfg(test)]
mod app_game_adapter_dispatch_result_service_tests;
pub(crate) mod app_game_adapter_execution_readiness_payload;
#[cfg(test)]
mod app_game_adapter_execution_readiness_payload_tests;
#[cfg(test)]
mod app_game_adapter_execution_readiness_service_tests;
mod app_game_adapter_host_capabilities;
mod app_game_boundary_read_model_payload;
#[cfg(test)]
mod app_game_boundary_read_model_payload_tests;
#[cfg(test)]
mod app_game_boundary_read_model_service_tests;
pub(crate) mod app_game_child_runtime_transport_receipt_payload;
#[cfg(test)]
mod app_game_child_runtime_transport_receipt_payload_tests;
#[cfg(test)]
mod app_game_child_runtime_transport_receipt_service_tests;
mod app_game_notification_readiness_payload;
#[cfg(test)]
mod app_game_notification_readiness_payload_tests;
#[cfg(test)]
mod app_game_notification_readiness_service_tests;
pub(crate) mod app_game_platform_proof_status_payload;
#[cfg(test)]
mod app_game_platform_proof_status_payload_tests;
#[cfg(test)]
mod app_game_platform_proof_status_service_tests;
mod app_game_policy_readiness_payload;
#[cfg(test)]
mod app_game_policy_readiness_payload_tests;
#[cfg(test)]
mod app_game_policy_readiness_service_tests;
mod app_game_policy_readiness_sources;
pub(crate) mod app_game_timer_parent_preference_setup_request;
mod app_game_timer_parent_preference_setup_request_outbox;
mod app_game_timer_parent_preference_setup_request_persistence;
mod app_game_timer_parent_preference_setup_request_status;
#[cfg(test)]
mod app_game_timer_parent_preference_setup_request_tests;
mod app_game_timer_parent_surface_action_results;
pub(crate) mod app_game_timer_parent_surface_payload;
#[cfg(test)]
mod app_game_timer_parent_surface_payload_tests;
#[cfg(test)]
mod app_game_timer_parent_surface_service_tests;
mod browser_intervention_payload;
pub(crate) mod browser_intervention_report;
pub(crate) mod social_alert_report_parent_surface_read_model_payload;
#[cfg(test)]
mod social_alert_report_parent_surface_read_model_payload_tests;
pub(crate) mod social_alert_report_read_model_payload;
#[cfg(test)]
mod social_alert_report_read_model_payload_tests;
#[cfg(test)]
mod social_alert_report_read_model_service_tests;
pub(crate) mod social_audit_explanation_read_model_payload;
#[cfg(test)]
mod social_audit_explanation_read_model_payload_tests;
#[cfg(test)]
mod social_audit_explanation_read_model_service_tests;
pub(crate) mod social_dashboard_read_model_payload;
#[cfg(test)]
mod social_dashboard_read_model_payload_tests;
#[cfg(test)]
mod social_dashboard_read_model_service_tests;
pub(crate) mod social_parent_notification_delivery_read_model_payload;
#[cfg(test)]
mod social_parent_notification_delivery_read_model_payload_tests;
#[cfg(test)]
mod social_parent_notification_delivery_read_model_service_tests;
pub(crate) mod social_source_custody_mutation_payload;
#[cfg(test)]
mod social_source_custody_mutation_payload_tests;
#[cfg(test)]
mod social_source_custody_mutation_service_tests;

use self::app_game_boundary_read_model_payload::{
    app_game_boundary_read_model_from_service_model, app_game_boundary_read_model_payload,
};
use self::app_game_notification_readiness_payload::{
    app_game_notification_readiness_from_service_model, app_game_notification_readiness_payload,
};
use self::app_game_policy_readiness_payload::{
    app_game_policy_readiness_from_service_model, app_game_policy_readiness_payload,
};
use self::app_game_timer_parent_preference_setup_request_outbox::setup_outbox_has_records;
pub async fn build_activity_ingest_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_activity_ingest_status().await {
        Some(status) => build_event(
            constants::event_id::ACTIVITY_INGEST_STATUS_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentActivityIngestStatusReported,
            LogLevel::Info,
            ingest_status_payload(&status),
            None,
        ),
        None => activity_store_error_event(
            command,
            constants::event_id::ACTIVITY_INGEST_STATUS_REPORTED,
            AgentEventName::AgentActivityIngestStatusReported,
        ),
    }
}

pub async fn build_activity_recent_summary_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_activity_recent_summary().await {
        Some(summary) => build_event(
            constants::event_id::ACTIVITY_RECENT_SUMMARY_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentActivityRecentSummaryReported,
            LogLevel::Info,
            recent_summary_payload(&summary),
            None,
        ),
        None => activity_store_error_event(
            command,
            constants::event_id::ACTIVITY_RECENT_SUMMARY_REPORTED,
            AgentEventName::AgentActivityRecentSummaryReported,
        ),
    }
}

pub async fn build_browser_evidence_recent_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_browser_evidence_read_model().await {
        Some(read_model) => build_event(
            constants::event_id::BROWSER_EVIDENCE_RECENT_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentBrowserEvidenceRecentReported,
            LogLevel::Info,
            browser_evidence_read_model_payload(&read_model),
            None,
        ),
        None => activity_store_error_event(
            command,
            constants::event_id::BROWSER_EVIDENCE_RECENT_REPORTED,
            AgentEventName::AgentBrowserEvidenceRecentReported,
        ),
    }
}

pub async fn build_browser_inventory_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = load_browser_inventory_read_model().await;
    build_event(
        constants::event_id::BROWSER_INVENTORY_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentBrowserInventoryReadModelReported,
        LogLevel::Info,
        browser_inventory_read_model_payload(&read_model),
        None,
    )
}

pub async fn build_network_flow_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_network_flow_read_model().await {
        Some(read_model) => {
            let delivery = deliver_network_runtime_for_read_model(&read_model).await;
            let product_path = prove_network_product_path_for_read_model(&read_model);
            build_event(
                constants::event_id::NETWORK_FLOW_READ_MODEL_REPORTED,
                &command.message_id,
                command.source,
                AgentEventName::AgentNetworkFlowReadModelReported,
                LogLevel::Info,
                network_flow_read_model_payload_with_runtime_delivery(
                    &read_model,
                    Some(&delivery),
                    Some(&product_path),
                ),
                None,
            )
        }
        None => activity_store_error_event(
            command,
            constants::event_id::NETWORK_FLOW_READ_MODEL_REPORTED,
            AgentEventName::AgentNetworkFlowReadModelReported,
        ),
    }
}

pub async fn build_network_runtime_event_chain_stream_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_network_flow_read_model().await {
        Some(read_model) => {
            let stream = stream_network_runtime_event_chain_for_read_model(&read_model).await;
            build_event(
                constants::event_id::NETWORK_RUNTIME_EVENT_CHAIN_STREAM_REPORTED,
                &command.message_id,
                command.source,
                AgentEventName::AgentNetworkRuntimeEventChainStreamReported,
                LogLevel::Info,
                network_runtime_event_chain_stream_payload(&stream),
                None,
            )
        }
        None => activity_store_error_event(
            command,
            constants::event_id::NETWORK_RUNTIME_EVENT_CHAIN_STREAM_REPORTED,
            AgentEventName::AgentNetworkRuntimeEventChainStreamReported,
        ),
    }
}

pub async fn build_activity_tracking_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_activity_tracking_read_model().await {
        Some(read_model) => build_event(
            constants::event_id::ACTIVITY_TRACKING_READ_MODEL_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentActivityTrackingReadModelReported,
            LogLevel::Info,
            tracking_read_model_payload(&read_model),
            None,
        ),
        None => activity_store_error_event(
            command,
            constants::event_id::ACTIVITY_TRACKING_READ_MODEL_REPORTED,
            AgentEventName::AgentActivityTrackingReadModelReported,
        ),
    }
}

pub async fn build_activity_app_game_boundary_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_app_game_model().await {
        Some(model) => {
            let read_model = app_game_boundary_read_model_from_service_model(model);
            build_event(
                constants::event_id::ACTIVITY_APP_GAME_BOUNDARY_READ_MODEL_REPORTED,
                &command.message_id,
                command.source,
                AgentEventName::AgentActivityAppGameBoundaryReadModelReported,
                LogLevel::Info,
                app_game_boundary_read_model_payload(&read_model),
                None,
            )
        }
        None => activity_store_error_event(
            command,
            constants::event_id::ACTIVITY_APP_GAME_BOUNDARY_READ_MODEL_REPORTED,
            AgentEventName::AgentActivityAppGameBoundaryReadModelReported,
        ),
    }
}

pub async fn build_activity_app_game_policy_readiness_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_app_game_model().await {
        Some(model) => {
            let read_model = app_game_policy_readiness_from_service_model(model);
            build_event(
                constants::event_id::ACTIVITY_APP_GAME_POLICY_READINESS_READ_MODEL_REPORTED,
                &command.message_id,
                command.source,
                AgentEventName::AgentActivityAppGamePolicyReadinessReadModelReported,
                LogLevel::Info,
                app_game_policy_readiness_payload(&read_model),
                None,
            )
        }
        None => activity_store_error_event(
            command,
            constants::event_id::ACTIVITY_APP_GAME_POLICY_READINESS_READ_MODEL_REPORTED,
            AgentEventName::AgentActivityAppGamePolicyReadinessReadModelReported,
        ),
    }
}

pub async fn build_activity_app_game_notification_readiness_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_app_game_model().await {
        Some(model) => {
            let local_outbox_runtime_claimed = setup_outbox_has_records(&activity_db_path());
            let read_model = app_game_notification_readiness_from_service_model(
                model,
                local_outbox_runtime_claimed,
            );
            build_event(
                constants::event_id::ACTIVITY_APP_GAME_NOTIFICATION_READINESS_READ_MODEL_REPORTED,
                &command.message_id,
                command.source,
                AgentEventName::AgentActivityAppGameNotificationReadinessReadModelReported,
                LogLevel::Info,
                app_game_notification_readiness_payload(&read_model),
                None,
            )
        }
        None => activity_store_error_event(
            command,
            constants::event_id::ACTIVITY_APP_GAME_NOTIFICATION_READINESS_READ_MODEL_REPORTED,
            AgentEventName::AgentActivityAppGameNotificationReadinessReadModelReported,
        ),
    }
}

async fn load_browser_inventory_read_model() -> BrowserInventoryReadModel {
    let generated_at = timestamp_now();
    let fallback_generated_at = generated_at.clone();
    tokio::task::spawn_blocking(move || {
        let process_observations =
            collect_process_snapshot(constants::browser::PROCESS_SCAN_LIMIT_BROWSER_DISCOVERY);
        browser_inventory_read_model_from_service_defaults(&generated_at, &process_observations)
    })
    .await
    .unwrap_or_else(|_| {
        browser_inventory_read_model_from_windows_inventory(fallback_generated_at, &[])
    })
}

pub(crate) fn browser_inventory_read_model_from_service_defaults(
    generated_at: &str,
    process_observations: &[ProcessObservation],
) -> BrowserInventoryReadModel {
    let candidate_paths = system_browser_candidate_paths();
    let mut observations =
        windows_browser_inventory_observations(&candidate_paths, process_observations, None);
    let package_identities = live_windows_browser_package_entries_with_limit(
        constants::browser::PACKAGE_SCAN_LIMIT_BROWSER_DISCOVERY,
    );
    observations.extend(windows_browser_package_observations(&package_identities));
    browser_inventory_read_model_from_windows_inventory(generated_at.to_string(), &observations)
}

async fn load_activity_ingest_status() -> Option<ActivityIngestStatus> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store.status().ok()
    })
    .await
    .ok()
    .flatten()
}

async fn load_activity_recent_summary() -> Option<ActivityRecentSummary> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store
            .recent_summary(constants::activity_store::DEFAULT_RECENT_LIMIT)
            .ok()
    })
    .await
    .ok()
    .flatten()
}

pub(crate) async fn load_browser_evidence_read_model(
) -> Option<ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store
            .browser_evidence_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &timestamp_now(),
            )
            .ok()
    })
    .await
    .ok()
    .flatten()
}

pub(crate) async fn load_network_flow_read_model(
) -> Option<ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store
            .network_flow_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &timestamp_now(),
            )
            .ok()
    })
    .await
    .ok()
    .flatten()
}

async fn load_activity_tracking_read_model(
) -> Option<ocentra_parent_agent_protocol::tracking::read_model::TrackingReadModel> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        tracking_read_model_for_store(
            &store,
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            &timestamp_now(),
        )
        .ok()
    })
    .await
    .ok()
    .flatten()
}

pub(crate) fn activity_store_error_event(
    command: AgentCommandEnvelope,
    event_id_suffix: &str,
    event: AgentEventName,
) -> AgentEventEnvelope {
    build_event(
        event_id_suffix,
        &command.message_id,
        command.source,
        event,
        LogLevel::Error,
        activity_store_error_payload(),
        None,
    )
}
