use crate::{
    activity_network_flow_payload::network_flow_read_model_payload_with_runtime_delivery,
    activity_payload::{ingest_status_payload, recent_summary_payload},
    activity_store_path::activity_db_path,
    activity_surface_store::load_app_game_model,
    browser_evidence_payload::browser_evidence_read_model_payload,
    browser_inventory_read_model::{
        browser_inventory_read_model_from_platform_inventory,
        browser_inventory_read_model_from_windows_inventory, BrowserInventoryGeneratedAtText,
    },
    browser_payload::browser_inventory_read_model_payload,
    browser_runtime_paths::system_browser_candidate_paths,
    event_builder::build_event,
    network_runtime_delivery::read_network_runtime_delivery_for_read_model,
    network_runtime_stream_payload::{
        network_runtime_event_chain_stream_payload,
        stream_network_runtime_event_chain_for_read_model,
    },
    time::timestamp_now,
};
use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    browser_platform_inventory::{
        browser_platform_inventory_observations, BrowserPlatformInventoryObservation,
    },
    browser_windows_package_inventory::windows_browser_package_observations,
    browser_windows_package_source::live_windows_browser_package_entries_with_limit,
    process_capture::{collect_process_snapshot, ProcessObservation},
    tracking::tracking_read_model_for_store,
};
use ocentra_parent_agent_protocol::activity_query::{ActivityIngestStatus, ActivityRecentSummary};
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFields, LogLevel};
use ocentra_parent_agent_protocol::tracking_read_model_payload;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use std::future::Future;

#[path = "activity_api/activity_store_error_event.rs"]
pub(crate) mod activity_store_error_event;
use self::activity_store_error_event::activity_store_error_event;

#[path = "activity_api/activity_memory_graph_report.rs"]
pub(crate) mod activity_memory_graph_report;
#[path = "activity_api/app_game_adapter_dispatch_execute_payload.rs"]
pub(crate) mod app_game_adapter_dispatch_execute_payload;
#[path = "activity_api/app_game_adapter_dispatch_preflight_payload.rs"]
pub(crate) mod app_game_adapter_dispatch_preflight_payload;
#[path = "activity_api/app_game_adapter_dispatch_result_fields.rs"]
pub(crate) mod app_game_adapter_dispatch_result_fields;
#[path = "activity_api/app_game_adapter_dispatch_result_payload.rs"]
pub(crate) mod app_game_adapter_dispatch_result_payload;
#[path = "activity_api/app_game_adapter_execution_readiness_payload.rs"]
pub(crate) mod app_game_adapter_execution_readiness_payload;
#[path = "activity_api/app_game_adapter_host_capabilities.rs"]
mod app_game_adapter_host_capabilities;
#[path = "activity_api/app_game_adapter_host_capabilities_paths.rs"]
mod app_game_adapter_host_capabilities_paths;
#[path = "activity_api/app_game_boundary_read_model_payload.rs"]
mod app_game_boundary_read_model_payload;
#[path = "activity_api/app_game_boundary_read_model_payload_rows.rs"]
mod app_game_boundary_read_model_payload_rows;
#[path = "activity_api/app_game_child_runtime_transport_receipt_payload.rs"]
pub(crate) mod app_game_child_runtime_transport_receipt_payload;
#[path = "activity_api/app_game_notification_readiness_payload.rs"]
mod app_game_notification_readiness_payload;
#[path = "activity_api/app_game_platform_proof_status_payload.rs"]
pub(crate) mod app_game_platform_proof_status_payload;
#[path = "activity_api/app_game_policy_readiness_payload.rs"]
mod app_game_policy_readiness_payload;
#[path = "activity_api/app_game_policy_readiness_sources.rs"]
mod app_game_policy_readiness_sources;
#[path = "activity_api/app_game_timer_parent_preference_setup_request.rs"]
pub(crate) mod app_game_timer_parent_preference_setup_request;
#[path = "activity_api/app_game_timer_parent_preference_setup_request_outbox.rs"]
mod app_game_timer_parent_preference_setup_request_outbox;
#[path = "activity_api/app_game_timer_parent_preference_setup_request_persistence.rs"]
mod app_game_timer_parent_preference_setup_request_persistence;
#[path = "activity_api/app_game_timer_parent_preference_setup_request_status.rs"]
mod app_game_timer_parent_preference_setup_request_status;
#[path = "activity_api/app_game_timer_parent_surface_action_results.rs"]
mod app_game_timer_parent_surface_action_results;
#[path = "activity_api/app_game_timer_parent_surface_payload.rs"]
pub(crate) mod app_game_timer_parent_surface_payload;
#[path = "activity_api/browser_intervention_payload.rs"]
mod browser_intervention_payload;
#[path = "activity_api/browser_intervention_report.rs"]
pub(crate) mod browser_intervention_report;
#[path = "activity_api/social_alert_report_parent_surface_read_model_payload.rs"]
pub(crate) mod social_alert_report_parent_surface_read_model_payload;
#[path = "activity_api/social_alert_report_read_model_payload.rs"]
pub(crate) mod social_alert_report_read_model_payload;
#[path = "activity_api/social_audit_explanation_read_model_payload.rs"]
pub(crate) mod social_audit_explanation_read_model_payload;
#[path = "activity_api/social_dashboard_read_model_payload.rs"]
pub(crate) mod social_dashboard_read_model_payload;
#[path = "activity_api/social_parent_notification_delivery_read_model_payload.rs"]
pub(crate) mod social_parent_notification_delivery_read_model_payload;
#[path = "activity_api/social_source_custody_mutation_payload.rs"]
pub(crate) mod social_source_custody_mutation_payload;

use self::app_game_boundary_read_model_payload::{
    app_game_boundary_read_model_from_service_model, app_game_boundary_read_model_payload,
};
use self::app_game_notification_readiness_payload::{
    app_game_notification_readiness_report_from_service_model,
    app_game_notification_readiness_report_payload,
};
use self::app_game_policy_readiness_payload::{
    app_game_policy_readiness_from_service_model, app_game_policy_readiness_payload,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ActivityEventId(pub(crate) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct GeneratedAtText(pub(crate) String);
pub async fn build_activity_ingest_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_optional_activity_report(
        command,
        ActivityEventId(constants::event_id::ACTIVITY_INGEST_STATUS_REPORTED),
        AgentEventName::AgentActivityIngestStatusReported,
        load_activity_ingest_status(),
        ingest_status_payload,
    )
    .await
}

pub async fn build_activity_recent_summary_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_optional_activity_report(
        command,
        ActivityEventId(constants::event_id::ACTIVITY_RECENT_SUMMARY_REPORTED),
        AgentEventName::AgentActivityRecentSummaryReported,
        load_activity_recent_summary(),
        recent_summary_payload,
    )
    .await
}

pub async fn build_browser_evidence_recent_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_optional_activity_report(
        command,
        ActivityEventId(constants::event_id::BROWSER_EVIDENCE_RECENT_REPORTED),
        AgentEventName::AgentBrowserEvidenceRecentReported,
        load_browser_evidence_read_model(),
        browser_evidence_read_model_payload,
    )
    .await
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
            let delivery = read_network_runtime_delivery_for_read_model(&read_model).await;
            build_event(
                constants::event_id::NETWORK_FLOW_READ_MODEL_REPORTED,
                &command.message_id,
                command.source,
                AgentEventName::AgentNetworkFlowReadModelReported,
                LogLevel::Info,
                network_flow_read_model_payload_with_runtime_delivery(&read_model, Some(&delivery)),
                None,
            )
        }
        None => activity_store_error_event(
            command,
            ActivityEventId(constants::event_id::NETWORK_FLOW_READ_MODEL_REPORTED),
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
            ActivityEventId(constants::event_id::NETWORK_RUNTIME_EVENT_CHAIN_STREAM_REPORTED),
            AgentEventName::AgentNetworkRuntimeEventChainStreamReported,
        ),
    }
}

pub async fn build_activity_tracking_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_optional_activity_report(
        command,
        ActivityEventId(constants::event_id::ACTIVITY_TRACKING_READ_MODEL_REPORTED),
        AgentEventName::AgentActivityTrackingReadModelReported,
        load_activity_tracking_read_model(),
        tracking_read_model_payload,
    )
    .await
}

pub async fn build_activity_app_game_boundary_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_optional_activity_report(
        command,
        ActivityEventId(constants::event_id::ACTIVITY_APP_GAME_BOUNDARY_READ_MODEL_REPORTED),
        AgentEventName::AgentActivityAppGameBoundaryReadModelReported,
        async {
            load_app_game_model()
                .await
                .map(app_game_boundary_read_model_from_service_model)
        },
        app_game_boundary_read_model_payload,
    )
    .await
}

pub async fn build_activity_app_game_policy_readiness_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_optional_activity_report(
        command,
        ActivityEventId(
            constants::event_id::ACTIVITY_APP_GAME_POLICY_READINESS_READ_MODEL_REPORTED,
        ),
        AgentEventName::AgentActivityAppGamePolicyReadinessReadModelReported,
        async {
            load_app_game_model()
                .await
                .map(app_game_policy_readiness_from_service_model)
        },
        app_game_policy_readiness_payload,
    )
    .await
}

pub async fn build_activity_app_game_notification_readiness_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_optional_activity_report(
        command,
        ActivityEventId(
            constants::event_id::ACTIVITY_APP_GAME_NOTIFICATION_READINESS_READ_MODEL_REPORTED,
        ),
        AgentEventName::AgentActivityAppGameNotificationReadinessReadModelReported,
        async {
            load_app_game_model().await.map(|model| {
                app_game_notification_readiness_report_from_service_model(model, false)
            })
        },
        app_game_notification_readiness_report_payload,
    )
    .await
}

async fn build_optional_activity_report<T, Load, Payload>(
    command: AgentCommandEnvelope,
    event_id: ActivityEventId,
    event: AgentEventName,
    load: Load,
    payload: Payload,
) -> AgentEventEnvelope
where
    Load: Future<Output = Option<T>>,
    Payload: FnOnce(&T) -> LogFields,
{
    match load.await {
        Some(value) => build_event(
            event_id.0,
            &command.message_id,
            command.source,
            event,
            LogLevel::Info,
            payload(&value),
            None,
        ),
        None => activity_store_error_event(command, event_id, event),
    }
}

async fn load_browser_inventory_read_model() -> BrowserInventoryReadModel {
    let generated_at = GeneratedAtText(timestamp_now());
    let fallback_generated_at = BrowserInventoryGeneratedAtText(generated_at.clone().0);
    tokio::task::spawn_blocking(move || {
        let process_observations =
            collect_process_snapshot(constants::browser::PROCESS_SCAN_LIMIT_BROWSER_DISCOVERY);
        browser_inventory_read_model_from_service_defaults(generated_at, &process_observations)
    })
    .await
    .unwrap_or_else(|_| {
        browser_inventory_read_model_from_windows_inventory(fallback_generated_at, &[])
    })
}

pub(crate) fn browser_inventory_read_model_from_service_defaults(
    generated_at: GeneratedAtText,
    process_observations: &[ProcessObservation],
) -> BrowserInventoryReadModel {
    let candidate_paths = system_browser_candidate_paths();
    let mut observations =
        browser_platform_inventory_observations(&candidate_paths.0, process_observations, None);
    let package_identities = live_windows_browser_package_entries_with_limit(
        constants::browser::PACKAGE_SCAN_LIMIT_BROWSER_DISCOVERY,
    );
    observations.extend(
        windows_browser_package_observations(&package_identities)
            .iter()
            .map(BrowserPlatformInventoryObservation::from),
    );
    browser_inventory_read_model_from_platform_inventory(
        BrowserInventoryGeneratedAtText(generated_at.0),
        &observations,
    )
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
                timestamp_now::<String>().as_str(),
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
                timestamp_now::<String>().as_str(),
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
            timestamp_now::<String>().as_str(),
        )
        .ok()
    })
    .await
    .ok()
    .flatten()
}
