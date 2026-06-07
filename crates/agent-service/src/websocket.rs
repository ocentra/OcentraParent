use axum::extract::ws::{Message, WebSocket};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
    LogFieldValue, LogLevel,
};

use crate::{
    activity_api::social_alert_report_read_model_payload::build_browser_social_alert_report_read_model_report,
    activity_api::social_audit_explanation_read_model_payload::build_browser_social_audit_explanation_read_model_report,
    activity_api::social_dashboard_read_model_payload::build_browser_social_dashboard_read_model_report,
    activity_api::social_source_custody_mutation_payload::build_browser_social_source_custody_mutation_report,
    activity_api::{
        build_activity_app_game_boundary_read_model_report,
        build_activity_app_game_notification_readiness_report,
        build_activity_app_game_policy_readiness_report, build_activity_ingest_status_report,
        build_activity_memory_graph_report, build_activity_recent_summary_report,
        build_activity_tracking_read_model_report, build_browser_evidence_recent_report,
        build_browser_intervention_read_model_report, build_browser_inventory_read_model_report,
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
    enforcement_api::{
        build_enforcement_audit_report, build_enforcement_broad_adapter_proof_report,
        build_enforcement_policy_dispatch_report, build_enforcement_product_control_spine_report,
        build_enforcement_supported_adapter_runtime_proof_report,
    },
    enforcement_timer_api::build_enforcement_timer_report,
    event_builder::{build_event, portal_peer},
    fields::fields_from_pairs,
    lan_pairing::{
        build_lan_pairing_status_report, route_lan_command, LanCommandDecision, LanPairingRuntime,
    },
    local_ai_chat_generation::build_local_ai_chat_generation_report,
    local_ai_runtime_status::build_local_ai_runtime_status_report,
    network_remote_delivery_status_payload::build_network_remote_delivery_status_report,
    parent_assistant_api::build_parent_assistant_scaffold_event,
    parent_assistant_runtime::build_parent_assistant_answer_report,
    policy_preview_api::build_policy_preview_read_model_report,
    screen_settings_api::build_screen_settings_event,
    screen_settings_runtime::ScreenSettingsRuntime,
    snapshot::build_dev_log_snapshot,
};

pub async fn handle_socket(
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
                let event = handle_command_text(
                    text.as_str(),
                    lan_pairing.clone(),
                    browser_policy.clone(),
                    screen_settings.clone(),
                    origin.clone(),
                )
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
    match serde_json::from_str::<AgentCommandEnvelope>(text) {
        Ok(command) => {
            handle_command(
                command,
                lan_pairing,
                browser_policy,
                screen_settings,
                origin,
            )
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

#[cfg(test)]
pub(crate) async fn handle_command_text_for_test(
    text: &str,
    lan_pairing: LanPairingRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    handle_command_text(
        text,
        lan_pairing,
        BrowserPolicyRuntime::in_memory(),
        ScreenSettingsRuntime::in_memory(),
        origin,
    )
    .await
}

#[cfg(test)]
pub(crate) async fn handle_command_text_with_browser_policy_for_test(
    text: &str,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    handle_command_text(
        text,
        lan_pairing,
        browser_policy,
        ScreenSettingsRuntime::in_memory(),
        origin,
    )
    .await
}

#[cfg(test)]
pub(crate) async fn handle_command_text_with_screen_settings_for_test(
    text: &str,
    lan_pairing: LanPairingRuntime,
    screen_settings: ScreenSettingsRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    handle_command_text(
        text,
        lan_pairing,
        BrowserPolicyRuntime::in_memory(),
        screen_settings,
        origin,
    )
    .await
}

async fn handle_command(
    command: AgentCommandEnvelope,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    screen_settings: ScreenSettingsRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    let (command, audit_fields) =
        match route_lan_command(lan_pairing.clone(), origin, command).await {
            LanCommandDecision::Continue {
                command,
                audit_fields,
            } => (command, audit_fields),
            LanCommandDecision::Respond(event) => return event,
        };

    let mut event =
        build_command_event(command, lan_pairing, browser_policy, screen_settings).await;

    if let Some(audit_fields) = audit_fields {
        event.payload.extend(audit_fields);
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
        | AgentCommandName::AgentNetworkFlowReadModelGet
        | AgentCommandName::AgentNetworkRuntimeEventChainStreamGet
        | AgentCommandName::AgentNetworkRemoteDeliveryStatusGet => {
            build_browser_network_command_report(command).await
        }
        AgentCommandName::AgentLocalAiRuntimeStatusGet
        | AgentCommandName::AgentLocalAiChatGenerate
        | AgentCommandName::AgentParentAssistantAnswerGenerate
        | AgentCommandName::AgentParentAssistantMessageSend
        | AgentCommandName::AgentParentAssistantQuickActionStart
        | AgentCommandName::AgentPolicyPreviewReadModelGet => {
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
        command_name if is_lan_runtime_command(&command_name) => {
            build_lan_pairing_status_report(lan_pairing, command)
        }
        _ => build_log_snapshot_report(command),
    }
}

fn is_activity_command(command: &AgentCommandName) -> bool {
    matches!(
        command,
        AgentCommandName::AgentActivityIngestStatusGet
            | AgentCommandName::AgentActivityRecentSummaryGet
            | AgentCommandName::AgentActivityMemoryGraphGet
            | AgentCommandName::AgentActivityReportDailyGenerate
            | AgentCommandName::AgentActivityReportWeeklyGenerate
            | AgentCommandName::AgentActivityReportMonthlyGenerate
            | AgentCommandName::AgentActivityReportSave
            | AgentCommandName::AgentActivityReportHistoryList
            | AgentCommandName::AgentActivityScreenReadModelGet
            | AgentCommandName::AgentActivityAppUseReadModelGet
            | AgentCommandName::AgentActivityBrowserReadModelGet
            | AgentCommandName::AgentActivityGamesReadModelGet
            | AgentCommandName::AgentActivityAppGameBoundaryReadModelGet
            | AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet
            | AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet
            | AgentCommandName::AgentBrowserSocialDashboardReadModelGet
            | AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet
            | AgentCommandName::AgentBrowserSocialAlertReportReadModelGet
            | AgentCommandName::AgentActivityNetworkReadModelGet
            | AgentCommandName::AgentActivityTrackingReadModelGet
    )
}

fn is_lan_runtime_command(command: &AgentCommandName) -> bool {
    matches!(
        command,
        AgentCommandName::AgentLanPairingProofSubmit
            | AgentCommandName::AgentLanPairingRouteSelect
            | AgentCommandName::AgentLanPairingRouteRevoke
            | AgentCommandName::AgentLanPairingStatusGet
            | AgentCommandName::AgentLanPairingBrowserDiscoveryScan
            | AgentCommandName::AgentLanPairingAddDeviceRequest
            | AgentCommandName::AgentLanPairingControllerLeaseRenew
            | AgentCommandName::AgentLanPairingControllerLeaseRelease
            | AgentCommandName::AgentLanPairingControllerLeaseTakeover
            | AgentCommandName::AgentLanAiProviderStatusGet
            | AgentCommandName::AgentLanAiJobSubmit
    )
}

fn is_browser_policy_command(command: &AgentCommandName) -> bool {
    matches!(
        command,
        AgentCommandName::AgentBrowserPolicyGet
            | AgentCommandName::AgentBrowserPolicyPreview
            | AgentCommandName::AgentBrowserPolicyPatch
            | AgentCommandName::AgentBrowserPolicyReplace
            | AgentCommandName::AgentBrowserPolicyRollback
    )
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
        AgentCommandName::AgentActivityAppGameBoundaryReadModelGet => {
            build_activity_app_game_boundary_read_model_report(command).await
        }
        AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet => {
            build_activity_app_game_policy_readiness_report(command).await
        }
        AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet => {
            build_activity_app_game_notification_readiness_report(command).await
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
        AgentCommandName::AgentActivityNetworkReadModelGet => {
            build_activity_network_read_model(command).await
        }
        AgentCommandName::AgentActivityTrackingReadModelGet => {
            build_activity_tracking_read_model_report(command).await
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
        AgentCommandName::AgentNetworkFlowReadModelGet => {
            build_network_flow_read_model_report(command).await
        }
        AgentCommandName::AgentNetworkRuntimeEventChainStreamGet => {
            build_network_runtime_event_chain_stream_report(command).await
        }
        AgentCommandName::AgentNetworkRemoteDeliveryStatusGet => {
            build_network_remote_delivery_status_report(command).await
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
        _ => build_log_snapshot_report(command),
    }
}

fn build_dev_echo_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::DEV_ECHOED,
        &command.message_id,
        command.source,
        AgentEventName::AgentDevEchoed,
        LogLevel::Info,
        command.payload,
        None,
    )
}

fn build_health_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::HEALTH_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentHealthReported,
        LogLevel::Info,
        fields_from_pairs(vec![
            (constants::field::ONLINE, LogFieldValue::Boolean(true)),
            (
                constants::field::TRANSPORT,
                LogFieldValue::String(constants::value::TRANSPORT_WEBSOCKET.to_string()),
            ),
        ]),
        Some(build_dev_log_snapshot()),
    )
}

fn build_log_snapshot_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::LOG_SNAPSHOT_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLogSnapshotReported,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::ENTRIES,
            LogFieldValue::Number(1.0),
        )]),
        Some(build_dev_log_snapshot()),
    )
}

fn build_watcher_status_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::WATCH_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentWatchStatusReported,
        LogLevel::Info,
        fields_from_pairs(vec![
            (constants::field::AVAILABLE, LogFieldValue::Boolean(false)),
            (
                constants::field::NOTE,
                LogFieldValue::String(constants::value::WATCHER_STATUS_ONLY.to_string()),
            ),
        ]),
        None,
    )
}

async fn send_event(socket: &mut WebSocket, event: AgentEventEnvelope) -> Result<(), axum::Error> {
    let text = serde_json::to_string(&event).expect(constants::error::AGENT_EVENT_SERIALIZES);
    socket.send(Message::Text(text.into())).await
}
