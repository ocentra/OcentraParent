use axum::extract::ws::{Message, WebSocket};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
};
use std::path::Path;

mod basic_reports;
mod command_classifiers;
mod policy_request_confirm;
mod tracking_retention_settings_write;

use self::basic_reports::{
    build_dev_echo_report, build_health_report, build_log_snapshot_report,
    build_watcher_status_report, temp_runtime_store_path,
};
use self::command_classifiers::{
    is_activity_command, is_browser_policy_command, is_lan_runtime_command,
};
use self::policy_request_confirm::build_policy_request_assistant_preview_confirm_report;
use self::tracking_retention_settings_write::build_tracking_retention_settings_write_report;

use crate::{
    activity_api::activity_memory_graph_report::build_activity_memory_graph_report,
    activity_api::app_game_adapter_dispatch_execute_payload::build_activity_app_game_adapter_dispatch_execute_report,
    activity_api::app_game_adapter_dispatch_preflight_payload::build_activity_app_game_adapter_dispatch_preflight_report,
    activity_api::app_game_adapter_dispatch_result_payload::build_activity_app_game_adapter_dispatch_result_report,
    activity_api::app_game_adapter_execution_readiness_payload::build_activity_app_game_adapter_execution_readiness_report,
    activity_api::app_game_child_runtime_transport_receipt_payload::build_activity_app_game_child_runtime_transport_receipt_report,
    activity_api::app_game_platform_proof_status_payload::build_activity_app_game_platform_proof_status_report,
    activity_api::app_game_timer_parent_preference_setup_request::build_activity_app_game_timer_parent_preference_setup_request_report,
    activity_api::app_game_timer_parent_surface_payload::build_activity_app_game_timer_parent_surface_report,
    activity_api::browser_intervention_report::build_browser_intervention_read_model_report,
    activity_api::social_alert_report_parent_surface_read_model_payload::build_browser_social_alert_report_parent_surface_read_model_report,
    activity_api::social_alert_report_read_model_payload::build_browser_social_alert_report_read_model_report,
    activity_api::social_audit_explanation_read_model_payload::build_browser_social_audit_explanation_read_model_report,
    activity_api::social_dashboard_read_model_payload::build_browser_social_dashboard_read_model_report,
    activity_api::social_parent_notification_delivery_read_model_payload::build_browser_social_parent_notification_delivery_read_model_report,
    activity_api::social_source_custody_mutation_payload::build_browser_social_source_custody_mutation_report,
    activity_api::{
        build_activity_app_game_boundary_read_model_report,
        build_activity_app_game_notification_readiness_report,
        build_activity_app_game_policy_readiness_report, build_activity_ingest_status_report,
        build_activity_recent_summary_report, build_activity_tracking_read_model_report,
        build_browser_evidence_recent_report, build_browser_inventory_read_model_report,
        build_network_flow_read_model_report, build_network_runtime_event_chain_stream_report,
    },
    activity_surface_api::{
        build_activity_app_use_read_model, build_activity_browser_read_model,
        build_activity_daily_report, build_activity_games_read_model,
        build_activity_monthly_report, build_activity_network_read_model,
        build_activity_report_history, build_activity_report_save,
        build_activity_screen_read_model, build_activity_weekly_report,
    },
    browser_policy_api::build_browser_policy_event,
    browser_policy_runtime::BrowserPolicyRuntime,
    browser_runtime::build_browser_managed_status_report,
    browser_runtime_stream_api::build_browser_runtime_event_chain_stream_report,
    enforcement_api::enforcement_broad_adapter_proof_report::build_enforcement_broad_adapter_proof_report,
    enforcement_api::enforcement_supported_adapter_runtime_proof_report::build_enforcement_supported_adapter_runtime_proof_report,
    enforcement_api::{
        build_enforcement_audit_report, build_enforcement_policy_dispatch_report,
        build_enforcement_product_control_spine_report,
    },
    enforcement_timer_api::build_enforcement_timer_report,
    event_builder::{build_event, portal_peer},
    fields::fields_from_pairs,
    lan_pairing::{
        build_lan_pairing_status_report, command_routing::route_lan_command, extend_log_fields,
        LanCommandDecision, LanPairingRuntime,
    },
    lan_runtime_stream_api::build_lan_runtime_event_chain_stream_report,
    local_ai_chat_generation::build_local_ai_chat_generation_report,
    local_ai_runtime_status::build_local_ai_runtime_status_report,
    network_android_vpn_service_gate_status_bridge::build_network_android_vpn_service_gate_status_report,
    network_apple_network_extension_gate_status_bridge::build_network_apple_network_extension_gate_status_report,
    network_linux_nftables_lab_status_bridge::build_network_linux_nftables_lab_status_report,
    network_live_capture_readiness_bridge::build_network_live_capture_status_report,
    network_remote_delivery_status_payload::build_network_remote_delivery_status_report,
    network_windows_firewall_lab_status_bridge::build_network_windows_firewall_lab_status_report,
    network_windows_wfp_gate_status_bridge::build_network_windows_wfp_gate_status_report,
    parent_assistant_api::build_parent_assistant_scaffold_event,
    parent_assistant_runtime::build_parent_assistant_answer_report,
    policy_preview_api::build_policy_preview_read_model_report,
    screen_settings_api::build_screen_settings_event,
    screen_settings_runtime::ScreenSettingsRuntime,
    snapshot::build_dev_log_snapshot,
};
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;

const BROWSER_POLICY_TEST_STORE_PREFIX: &str = "browser-policy";
const SCREEN_SETTINGS_TEST_STORE_PREFIX: &str = "screen-settings";

pub(crate) async fn handle_command_text_for_test(
    text: &str,
    lan_pairing: LanPairingRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    Box::pin(handle_command_text(
        text,
        lan_pairing,
        BrowserPolicyRuntime::for_store_path(temp_runtime_store_path(
            BROWSER_POLICY_TEST_STORE_PREFIX,
        )),
        ScreenSettingsRuntime::for_store_path(temp_runtime_store_path(
            SCREEN_SETTINGS_TEST_STORE_PREFIX,
        )),
        origin,
    ))
    .await
}

pub(crate) async fn handle_command_text_with_browser_policy_for_test(
    text: &str,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    Box::pin(handle_command_text(
        text,
        lan_pairing,
        browser_policy,
        ScreenSettingsRuntime::for_store_path(temp_runtime_store_path(
            SCREEN_SETTINGS_TEST_STORE_PREFIX,
        )),
        origin,
    ))
    .await
}

pub async fn dispatch_local_command_text(text: &str) -> AgentEventEnvelope {
    handle_command_text_for_test(text, LanPairingRuntime::empty(), None).await
}

pub async fn dispatch_local_command_text_with_browser_policy_store(
    text: &str,
    store_path: &Path,
) -> AgentEventEnvelope {
    handle_command_text_with_browser_policy_for_test(
        text,
        LanPairingRuntime::empty(),
        BrowserPolicyRuntime::for_store_path(store_path),
        None,
    )
    .await
}

pub(crate) async fn handle_socket(
    mut socket: WebSocket,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    screen_settings: ScreenSettingsRuntime,
    origin: Option<String>,
) {
    let ready_event = build_event(
        constants::event_id::CONNECTION_READY,
        constants::event_id::CONNECTION_READY,
        portal_peer(),
        AgentEventName::AgentConnectionReady,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::ONLINE,
            LogFieldValue::Boolean(true),
        )]),
        Some(build_dev_log_snapshot()),
    );

    if send_event(&mut socket, ready_event).await.is_err() {
        return;
    }

    while let Some(result) = socket.recv().await {
        let message = match result {
            Ok(message) => message,
            Err(_) => break,
        };

        match message {
            Message::Text(text) => {
                let event = Box::pin(handle_command_text(
                    text.as_str(),
                    lan_pairing.clone(),
                    browser_policy.clone(),
                    screen_settings.clone(),
                    origin.clone(),
                ))
                .await;
                if send_event(&mut socket, event).await.is_err() {
                    break;
                }
            }
            Message::Ping(bytes) => {
                if socket.send(Message::Pong(bytes)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}

async fn handle_command_text(
    text: &str,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    screen_settings: ScreenSettingsRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    if text.len() > constants::lan_pairing::LAN_WEBSOCKET_COMMAND_MAX_BYTES {
        return oversized_command_text_rejected();
    }

    match serde_json::from_str::<AgentCommandEnvelope>(text) {
        Ok(command) => {
            Box::pin(handle_command(
                command,
                lan_pairing,
                browser_policy,
                screen_settings,
                origin,
            ))
            .await
        }
        Err(error) => build_event(
            constants::event_id::COMMAND_REJECTED,
            constants::event_id::UNKNOWN_COMMAND,
            portal_peer(),
            AgentEventName::AgentCommandRejected,
            LogLevel::Warn,
            fields_from_pairs(vec![(
                constants::field::REASON,
                LogFieldValue::String(error.to_string()),
            )]),
            None,
        ),
    }
}

fn oversized_command_text_rejected() -> AgentEventEnvelope {
    build_event(
        constants::event_id::COMMAND_REJECTED,
        constants::event_id::UNKNOWN_COMMAND,
        portal_peer(),
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        fields_from_pairs(vec![
            (
                constants::field::LAN_CONTROL_STATE,
                LogFieldValue::String(constants::value::LAN_CONTROL_REJECTED.to_string()),
            ),
            (
                constants::field::LAN_AUDIT_EVENT_TYPE,
                LogFieldValue::String(constants::value::LAN_AUDIT_CONTROL_REJECTED.to_string()),
            ),
            (
                constants::field::LAN_REJECTION_REASON,
                LogFieldValue::String(constants::value::LAN_REASON_PAYLOAD_TOO_LARGE.to_string()),
            ),
            (
                constants::field::LAN_AUTHENTICATION_STATE,
                LogFieldValue::String(constants::value::LAN_AUTH_UNAUTHENTICATED.to_string()),
            ),
            (
                constants::field::REASON,
                LogFieldValue::String(constants::value::LAN_REASON_PAYLOAD_TOO_LARGE.to_string()),
            ),
        ]),
        None,
    )
}

async fn handle_command(
    command: AgentCommandEnvelope,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    screen_settings: ScreenSettingsRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    let (command, audit_fields) = match route_lan_command(
        lan_pairing.clone(),
        crate::lan_pairing::command_routing::LanCommandOrigin(LanPairingOptionalText(origin)),
        command,
    )
    .await
    {
        LanCommandDecision::Continue {
            command,
            audit_fields,
        } => (command, audit_fields),
        LanCommandDecision::Respond(event) => return event,
    };

    let mut event = Box::pin(build_command_event(
        command,
        lan_pairing,
        browser_policy,
        screen_settings,
    ))
    .await;

    if let Some(audit_fields) = audit_fields {
        extend_log_fields(&mut event.payload, audit_fields);
    }
    event
}

async fn build_command_event(
    command: AgentCommandEnvelope,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    screen_settings: ScreenSettingsRuntime,
) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentHealthCheck => build_health_report(command),
        AgentCommandName::AgentLogSnapshotGet => build_log_snapshot_report(command),
        AgentCommandName::AgentDevEcho => build_dev_echo_report(command),
        AgentCommandName::AgentWatchStatusGet => build_watcher_status_report(command),
        AgentCommandName::AgentBrowserSocialSourceCustodyMutationApply => {
            build_browser_social_source_custody_mutation_report(command).await
        }
        command_name if is_activity_command(&command_name) => {
            build_activity_command_report(command).await
        }
        AgentCommandName::AgentBrowserInventoryReadModelGet
        | AgentCommandName::AgentBrowserEvidenceRecentGet
        | AgentCommandName::AgentBrowserManagedBridgePoll
        | AgentCommandName::AgentBrowserInterventionReadModelGet
        | AgentCommandName::AgentBrowserRuntimeEventChainStreamGet
        | AgentCommandName::AgentNetworkFlowReadModelGet
        | AgentCommandName::AgentNetworkRuntimeEventChainStreamGet
        | AgentCommandName::AgentNetworkRemoteDeliveryStatusGet
        | AgentCommandName::AgentNetworkLiveCaptureStatusGet
        | AgentCommandName::AgentNetworkAndroidVpnServiceGateStatusGet
        | AgentCommandName::AgentNetworkAppleNetworkExtensionGateStatusGet
        | AgentCommandName::AgentNetworkLinuxNftablesLabStatusGet
        | AgentCommandName::AgentNetworkWindowsFirewallLabStatusGet
        | AgentCommandName::AgentNetworkWindowsWfpGateStatusGet => {
            build_browser_network_command_report(command).await
        }
        AgentCommandName::AgentLocalAiRuntimeStatusGet
        | AgentCommandName::AgentLocalAiChatGenerate
        | AgentCommandName::AgentParentAssistantAnswerGenerate
        | AgentCommandName::AgentParentAssistantMessageSend
        | AgentCommandName::AgentParentAssistantQuickActionStart
        | AgentCommandName::AgentPolicyPreviewReadModelGet
        | AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm => {
            build_ai_command_report(command).await
        }
        command_name if is_browser_policy_command(&command_name) => {
            build_browser_policy_event(browser_policy, command).await
        }
        AgentCommandName::AgentScreenSettingsGet | AgentCommandName::AgentScreenSettingsReplace => {
            build_screen_settings_event(screen_settings, command).await
        }
        AgentCommandName::AgentParentAssistantThreadList
        | AgentCommandName::AgentParentAssistantThreadCreate
        | AgentCommandName::AgentParentAssistantThreadOpen
        | AgentCommandName::AgentParentAssistantThreadArchive
        | AgentCommandName::AgentParentAssistantRunCancel
        | AgentCommandName::AgentParentAssistantActionPreview
        | AgentCommandName::AgentParentAssistantActionConfirm
        | AgentCommandName::AgentParentAssistantProviderStatusGet => {
            build_parent_assistant_scaffold_event(command)
        }
        AgentCommandName::AgentEnforcementTimerRecover
        | AgentCommandName::AgentEnforcementTimerExpire
        | AgentCommandName::AgentEnforcementOverrideCancel
        | AgentCommandName::AgentEnforcementExecute
        | AgentCommandName::AgentEnforcementProductControlSpineGet
        | AgentCommandName::AgentEnforcementPolicyDispatchGet
        | AgentCommandName::AgentEnforcementBroadAdapterProofGet
        | AgentCommandName::AgentEnforcementSupportedAdapterRuntimeProofGet => {
            build_enforcement_command_report(command).await
        }
        command_name if is_lan_runtime_command(&command_name) => match command_name {
            AgentCommandName::AgentLanRuntimeEventChainStreamGet => {
                build_lan_runtime_event_chain_stream_report(&lan_pairing, command).await
            }
            _ => build_lan_pairing_status_report(&lan_pairing, command),
        },
        _ => build_log_snapshot_report(command),
    }
}

async fn build_enforcement_command_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentEnforcementExecute => build_enforcement_audit_report(command).await,
        AgentCommandName::AgentEnforcementProductControlSpineGet => {
            build_enforcement_product_control_spine_report(command).await
        }
        AgentCommandName::AgentEnforcementPolicyDispatchGet => {
            build_enforcement_policy_dispatch_report(command).await
        }
        AgentCommandName::AgentEnforcementBroadAdapterProofGet => {
            build_enforcement_broad_adapter_proof_report(command).await
        }
        AgentCommandName::AgentEnforcementSupportedAdapterRuntimeProofGet => {
            build_enforcement_supported_adapter_runtime_proof_report(command).await
        }
        AgentCommandName::AgentEnforcementTimerRecover
        | AgentCommandName::AgentEnforcementTimerExpire
        | AgentCommandName::AgentEnforcementOverrideCancel => {
            build_enforcement_timer_report(command).await
        }
        _ => build_log_snapshot_report(command),
    }
}

async fn build_activity_command_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentActivityIngestStatusGet => {
            build_activity_ingest_status_report(command).await
        }
        AgentCommandName::AgentActivityRecentSummaryGet => {
            build_activity_recent_summary_report(command).await
        }
        AgentCommandName::AgentActivityMemoryGraphGet => {
            build_activity_memory_graph_report(command).await
        }
        AgentCommandName::AgentActivityReportDailyGenerate => {
            build_activity_daily_report(command).await
        }
        AgentCommandName::AgentActivityReportWeeklyGenerate => {
            build_activity_weekly_report(command).await
        }
        AgentCommandName::AgentActivityReportMonthlyGenerate => {
            build_activity_monthly_report(command).await
        }
        AgentCommandName::AgentActivityReportSave => build_activity_report_save(command).await,
        AgentCommandName::AgentActivityReportHistoryList => {
            build_activity_report_history(command).await
        }
        AgentCommandName::AgentActivityScreenReadModelGet => {
            build_activity_screen_read_model(command).await
        }
        AgentCommandName::AgentActivityAppUseReadModelGet => {
            build_activity_app_use_read_model(command).await
        }
        AgentCommandName::AgentActivityBrowserReadModelGet => {
            build_activity_browser_read_model(command).await
        }
        AgentCommandName::AgentActivityGamesReadModelGet => {
            build_activity_games_read_model(command).await
        }
        AgentCommandName::AgentActivityAppGameBoundaryReadModelGet
        | AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet
        | AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet
        | AgentCommandName::AgentActivityAppGameAdapterExecutionReadinessReadModelGet
        | AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet
        | AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet
        | AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet
        | AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet
        | AgentCommandName::AgentActivityAppGameAdapterDispatchExecute
        | AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet
        | AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest => {
            build_activity_app_game_command_report(command).await
        }
        AgentCommandName::AgentBrowserSocialDashboardReadModelGet => {
            build_browser_social_dashboard_read_model_report(command).await
        }
        AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet => {
            build_browser_social_audit_explanation_read_model_report(command).await
        }
        AgentCommandName::AgentBrowserSocialAlertReportReadModelGet => {
            build_browser_social_alert_report_read_model_report(command).await
        }
        AgentCommandName::AgentBrowserSocialAlertReportParentSurfaceReadModelGet => {
            build_browser_social_alert_report_parent_surface_read_model_report(command).await
        }
        AgentCommandName::AgentBrowserSocialParentNotificationDeliveryReadModelGet => {
            build_browser_social_parent_notification_delivery_read_model_report(command).await
        }
        AgentCommandName::AgentActivityNetworkReadModelGet => {
            build_activity_network_read_model(command).await
        }
        AgentCommandName::AgentActivityTrackingReadModelGet => {
            build_activity_tracking_read_model_report(command).await
        }
        AgentCommandName::AgentActivityTrackingRetentionSettingsWrite => {
            build_tracking_retention_settings_write_report(command).await
        }
        _ => build_log_snapshot_report(command),
    }
}

async fn build_activity_app_game_command_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentActivityAppGameBoundaryReadModelGet => {
            build_activity_app_game_boundary_read_model_report(command).await
        }
        AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet => {
            build_activity_app_game_policy_readiness_report(command).await
        }
        AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet => {
            build_activity_app_game_notification_readiness_report(command).await
        }
        AgentCommandName::AgentActivityAppGameAdapterExecutionReadinessReadModelGet => {
            build_activity_app_game_adapter_execution_readiness_report(command).await
        }
        AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet => {
            build_activity_app_game_platform_proof_status_report(command).await
        }
        AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet => {
            build_activity_app_game_child_runtime_transport_receipt_report(command).await
        }
        AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet => {
            build_activity_app_game_adapter_dispatch_preflight_report(command).await
        }
        AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet => {
            build_activity_app_game_adapter_dispatch_result_report(command).await
        }
        AgentCommandName::AgentActivityAppGameAdapterDispatchExecute => {
            build_activity_app_game_adapter_dispatch_execute_report(command).await
        }
        AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet => {
            build_activity_app_game_timer_parent_surface_report(command).await
        }
        AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest => {
            build_activity_app_game_timer_parent_preference_setup_request_report(command).await
        }
        _ => build_log_snapshot_report(command),
    }
}

async fn build_browser_network_command_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentBrowserInventoryReadModelGet => {
            build_browser_inventory_read_model_report(command).await
        }
        AgentCommandName::AgentBrowserEvidenceRecentGet => {
            build_browser_evidence_recent_report(command).await
        }
        AgentCommandName::AgentBrowserManagedBridgePoll => {
            build_browser_managed_status_report(command).await
        }
        AgentCommandName::AgentBrowserInterventionReadModelGet => {
            build_browser_intervention_read_model_report(command).await
        }
        AgentCommandName::AgentBrowserRuntimeEventChainStreamGet => {
            build_browser_runtime_event_chain_stream_report(command).await
        }
        AgentCommandName::AgentNetworkFlowReadModelGet => {
            build_network_flow_read_model_report(command).await
        }
        AgentCommandName::AgentNetworkRuntimeEventChainStreamGet => {
            build_network_runtime_event_chain_stream_report(command).await
        }
        AgentCommandName::AgentNetworkRemoteDeliveryStatusGet => {
            build_network_remote_delivery_status_report(command).await
        }
        AgentCommandName::AgentNetworkLiveCaptureStatusGet => {
            build_network_live_capture_status_report(command)
        }
        AgentCommandName::AgentNetworkAndroidVpnServiceGateStatusGet => {
            build_network_android_vpn_service_gate_status_report(command)
        }
        AgentCommandName::AgentNetworkAppleNetworkExtensionGateStatusGet => {
            build_network_apple_network_extension_gate_status_report(command)
        }
        AgentCommandName::AgentNetworkLinuxNftablesLabStatusGet => {
            build_network_linux_nftables_lab_status_report(command)
        }
        AgentCommandName::AgentNetworkWindowsFirewallLabStatusGet => {
            build_network_windows_firewall_lab_status_report(command)
        }
        AgentCommandName::AgentNetworkWindowsWfpGateStatusGet => {
            build_network_windows_wfp_gate_status_report(command)
        }
        _ => build_log_snapshot_report(command),
    }
}

async fn build_ai_command_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentLocalAiRuntimeStatusGet => {
            build_local_ai_runtime_status_report(command).await
        }
        AgentCommandName::AgentLocalAiChatGenerate => {
            build_local_ai_chat_generation_report(command).await
        }
        AgentCommandName::AgentParentAssistantAnswerGenerate
        | AgentCommandName::AgentParentAssistantMessageSend
        | AgentCommandName::AgentParentAssistantQuickActionStart => {
            build_parent_assistant_answer_report(command).await
        }
        AgentCommandName::AgentPolicyPreviewReadModelGet => {
            build_policy_preview_read_model_report(command).await
        }
        AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm => {
            build_policy_request_assistant_preview_confirm_report(command).await
        }
        _ => build_log_snapshot_report(command),
    }
}

async fn send_event(socket: &mut WebSocket, event: AgentEventEnvelope) -> Result<(), axum::Error> {
    let text = serde_json::to_string(&event).map_err(axum::Error::new)?;
    socket.send(Message::Text(text.into())).await
}
